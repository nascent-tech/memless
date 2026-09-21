# 03 — Application et lecture YAML (`memless-engine`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [02 — Domaine (`memless-domain`)](02-domaine.md)  
**Suivant** : [04 — Surface native et ponts](04-capi-et-ponts.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Crate `memless-engine` — le contexte borné « le moteur ». Il porte l'application (le cas
d'usage `charger`) et l'adaptateur sortant `yaml`. Le nom `core` est écarté : c'est un
fourre-tout interdit, et le crate contient de l'infrastructure. Dépendances :
`memless-domain` et la bibliothèque YAML. **Pas de GlueSQL** au palier 1.

> Désync à reprendre : `ARCHITECTURE.md` §2 nomme ce crate `memless-core` (partie 6).

---

## `crates/memless-engine/Cargo.toml`

Nom `memless-engine`. `[dependencies]` : `memless-domain` (chemin) et la **bibliothèque
YAML**. **Ne porte pas** `gluesql` (palier 2), ni `indexmap` : l'ordre vit dans les `Vec`
de `RawNode`, aucune carte ordonnée n'est nécessaire au palier 1 (elle reviendra avec son
premier appelant, la réécriture du palier 3).

La bibliothèque est **`serde-saphyr` 1.3.0** (retenue par l'architecture, stack §7) ;
seuls **son point d'entrée et le réglage du budget** se figent au plan. Contrainte que le
plan doit honorer : le palier 1 lit au niveau **événements/arbre**, jamais par
désérialisation `serde` vers une carte — celle-ci fusionnerait les clés dupliquées et
rejetterait les clés non textuelles, rendant B5/B6/B7 impossibles à prononcer. Un arbre de
syntaxe abstraite (*AST* — la structure d'un document telle que l'analyseur la voit, avant
toute interprétation) préserve ce qu'il faut ; `serde-saphyr` expose cet accès sous son
analyseur.

---

## `crates/memless-engine/src/lib.rs`

```rust
pub mod application;
pub mod yaml;

pub use application::load::{load, ReadSource};
pub use yaml::read;
pub use memless_domain::{Base, Refusal}; // ce que memless-capi consomme, réexporté nommément
```

**Ne fait pas** : aucune logique.

---

## `crates/memless-engine/src/application/mod.rs`

```rust
pub mod load;
```

---

## `crates/memless-engine/src/application/load.rs`

Le cas d'usage **charger**. Il reçoit la lecture **en paramètre** : une fonction, pas un
`trait`. C'est le seul « port » du palier — une signature dont le cœur dépend, jamais du
module `yaml`. Ainsi l'application n'importe pas l'infrastructure ; la racine de composition
(dans `memless-capi`) lui passe `yaml::read`. Le sens des dépendances tient sans indirection
de `trait`.

```rust
use memless_domain::document::RawDocument;
use memless_domain::refusal::{Refusal, SourceRefusal};
use memless_domain::Base;

/// Le « port » de lecture, une fonction : le cœur dépend de cette signature, pas du module.
pub type ReadSource = fn(&str) -> Result<RawDocument, SourceRefusal>;

/// Lit via `read` (refus A possible), puis devine et vérifie via `Base::load` (refus B/C).
/// Aucune autre décision : un simple enchaînement, les deux erreurs assemblées via `From`.
pub fn load(read: ReadSource, path: &str) -> Result<Base, Refusal>;
```

**Refus typés** : `Refusal` (A par la lecture, B/C par le domaine), assemblé via `From`.

**Ne fait pas** : n'importe pas le module `yaml` (il reçoit la lecture en paramètre) ; ne
tient aucun état entre appels (le handle est l'affaire de la surface native) ; ne rend
jamais un objet de transport — au palier 1 la surface native est dans le même processus et
range la `Base` (C-5). Alternative écartée : un `trait` de port + `Arc<dyn>` et une racine
de composition dédiée (plus lourd), ou un appel direct `application → yaml` (qui casserait
l'inversion des dépendances) — voir C-5.

---

## `crates/memless-engine/src/yaml/mod.rs`

```rust
pub mod reader;
mod parse;

pub use reader::read;
```

---

## `crates/memless-engine/src/yaml/reader.rs`

L'**adaptateur sortant** : la lecture du fichier et son analyse, derrière une fonction.

```rust
use memless_domain::document::RawDocument;
use memless_domain::refusal::SourceRefusal;

/// Lit **exactement** le chemin donné. I/O d'abord : chemin absent ou non-fichier =>
/// `PathHasNoFile` (A1) ; illisible => `FileNotReadable` (A2) ; vide ou blancs =>
/// `FileEmpty` (A3). Puis analyse (`parse`) : échec => `InvalidYaml` (A4). Ne lit rien
/// d'autre : le résidu voisin n'est jamais touché (décision 17, D16).
pub fn read(path: &str) -> Result<RawDocument, SourceRefusal>;
```

**Anti-« YAML bomb » (D10, C-8).** Le palier 1 ne pose **aucun refus chiffré** (§12.3).
La bibliothèque retenue ne permet pas de tout désactiver d'un coup : la recherche montre
que `serde-saphyr` 1.3.0 accepte un budget saturé (toutes limites à `usize::MAX`,
`AliasLimits` saturés), ce qui approche §12.3 mais laisse deux limites de directives et le
risque de débordement de pile. Le **réglage exact** (budget saturé) se fige au plan ; le
résidu chiffré non éliminable est **assumé en dette** (partie 6, C-8). C'est ici, et
nulle part ailleurs, que ce réglage vit.

**Refus typés** : `SourceRefusal` A1–A4.

**Ne fait pas** : ne juge ni la forme (B) ni la cohérence (C) ; ne lit qu'un seul chemin
(aucune recherche de fichier voisin, D11/D16).

---

## `crates/memless-engine/src/yaml/parse.rs`

Module privé : construit `RawNode` depuis l'arbre de l'analyseur, **en préservant** ce
qu'une désérialisation `serde` perdrait — l'ordre, les **doublons de clés** (que la lecture
ordinaire fusionnerait), les valeurs imbriquées, les clés non textuelles, et le style de
chaque scalaire (nu, cité, bloc).

```rust
/// Échec d'analyse local, sans chemin : `read` l'habille en `InvalidYaml { path, at }`.
pub(crate) struct ParseFailure { pub at: Option<TextPosition> }

/// Analyse un texte source vers `RawNode`. Un YAML syntaxiquement invalide, un encodage
/// non décodable ou un BOM inattendu (D17), ou plusieurs documents `---` (D9) donnent une
/// `ParseFailure`. Aucune interprétation de type ici : chaque scalaire devient un
/// `RawScalar { lexeme, style }`, son type sera deviné dans le domaine.
pub(crate) fn parse(source: &str) -> Result<RawNode, ParseFailure>;
```

`parse` ne connaît pas le chemin : c'est `read` qui construit le `RawDocument` (provenance
+ racine analysée) et convertit une `ParseFailure` en `InvalidYaml { path, at }`.

**Refus typés** : `ParseFailure`, que `read` élève en `SourceRefusal::InvalidYaml` (A4),
avec position si l'analyseur la fournit.

**Ne fait pas** : ne devine aucun type (c'est `Scalar::guess` au domaine) ; ne déduplique
pas les clés (il les préserve pour que le domaine prononce B5/B6) ; ne touche pas au disque
(il reçoit le texte de `read`).

---

## La racine de composition

Le seul câblage du palier tient en un passage d'argument : la surface d'entrée
`memless-capi` appelle `engine::load(engine::read, path)` — elle relie l'adaptateur `read`
au cas d'usage. C'est la racine de composition, réduite à une ligne dans le crate d'entrée,
sans fichier `composition.rs` dédié (C-5, partie 6). Une racine plus étoffée renaît au
palier 2, quand l'exécution SQL aura une implémentation à choisir.
