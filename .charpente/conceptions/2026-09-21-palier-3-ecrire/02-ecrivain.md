# 02 — L'écrivain YAML par substitution (`memless-engine`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 3 — Écrire](00-index.md)  
**Précédent** : [01 — Domaine : modèle, exécution, contraintes, rendu (`memless-domain`)](01-domaine.md)  
**Suivant** : [03 — Moteur SQL, cas d'usage et surface native](03-moteur-capi.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

**La pièce la plus risquée du palier**, séparée pour être revue et prouvée seule : un `RawDocument`
(rendu par le domaine, partie 01) devient un **texte YAML canonique déterministe à l'octet**, écrit
sur le disque par **résidu + fsync + rename**. Porte **PR2**.

## Le rendu texte — `crates/memless-engine/src/yaml/render/`

**Écrit à la main** (pas le sérialiseur `serde-saphyr`, qui ne maîtrise ni la citation ni
l'indentation et peut bouger à une mise à jour ; il reste **lecture seule**). Entrée : le
`RawDocument` de `Base::document`. Sortie : une `String`.

```rust
// yaml/render/mod.rs
pub fn render(document: &RawDocument) -> String;
```

Règles figées (un fichier par règle courte) :

- **Structure** : `table:\n  - id: v\n    c: v\n` ; table vide → `table: []\n` ; indentation **deux
  espaces**, fins de ligne **LF**, UTF-8 **sans BOM**, ni `---` ni `...`, **une seule** fin de ligne
  finale.
- **Scalaires** : entier `{}` ; décimal via `render_decimal` (déjà au domaine — plus court
  aller-retour, `.0` sur un entier flottant) ; `true`/`false`. Un scalaire ou une clé est **cité**
  (guillemets doubles, échappement `\"` `\\` `\n` `\uXXXX`) **ssi** son style est `ExplicitText`
  (posé par le domaine pour le devinage), **ou** si la syntaxe YAML l'exige : vide, blanc en
  tête/queue, caractère de contrôle, premier caractère indicateur (`- ? : , [ ] { } # & * ! | > ' " % @ \``),
  contient `: ` ou ` #`, ou est un mot nul (`null`, `Null`, `NULL`, `~`). **Cette liste est fermée,
  dans un seul fichier** `yaml/render/quote.rs`.
- Une valeur nue déjà canonique (`01H7B2`, `Ada`, `w_123`, `5`, `1.5`) reste **nue** : un fichier
  déjà canonique se réécrit sans diff.

**Ne fait pas** : ne devine aucun type (le style lui vient du domaine) ; ne conserve ni commentaire
ni mise en forme d'origine (décision 26) ; ne lit aucun fichier.

## L'écrivain par substitution — `crates/memless-engine/src/yaml/writer/`

Écrit un texte à un chemin par substitution (résidu puis substitution atomique du système de
fichiers), ou rend une erreur `io`.

```rust
// yaml/writer/mod.rs
pub fn replace_file(path: &str, text: &str) -> Result<(), std::io::Error>;
```

Séquence (un fichier par étape, gardes) :

1. **`residue.rs`** — dérive le chemin du résidu : `.<nom>.memless-tmp` **dans le même
   répertoire** que `path` (condition d'un `rename` atomique ; nom déjà fixé au palier 1, déjà
   ignoré par le chargement, motif Git `.*.memless-tmp`).
2. **`write_temp.rs`** — crée/tronque le résidu (un résidu laissé par un plantage antérieur est
   **écrasé** — le seul geste qui le touche) ; recopie les bits de permission du fichier réel s'il
   existe (unix) ; `write_all` ; `sync_all` (fsync du fichier).
3. **`swap.rs`** — `rename` du résidu sur `path` (**point de substitution** — atomique POSIX ;
   `MoveFileEx` avec remplacement sous Windows) ; puis fsync du répertoire (unix, non bloquant : une
   erreur ici n'est pas une incohérence, seulement une durabilité de l'entrée sous coupure — limite
   consignée).
4. **En cas d'erreur avant le `rename`** — `cleanup.rs` supprime le résidu (au mieux ; l'erreur
   d'origine prime), l'erreur `io` remonte. La mémoire n'a pas bougé (elle est avancée par le cas
   d'usage **après** le retour `Ok`).

**Ne fait pas** : n'écrit **jamais** par-dessus l'original morceau par morceau ; ne touche jamais
le résidu au chargement (palier 1, décision 17) ; ne décide d'aucune règle produit (il déplace des
octets).

## Le port et l'instance — `crates/memless-engine/src/application/`

```rust
// application/replace_file.rs
pub type ReplaceFile = fn(&str, &str) -> Result<(), std::io::Error>;

// application/instance.rs — l'état vivant : le chemin ET la base
pub struct Instance { pub path: String, pub base: Base }
```

`load` rend désormais un `Instance` (le chemin qu'il a lu + la base) au lieu d'un `Base` seul.
`ReplaceFile` est câblé par `memless-capi` avec `yaml::writer::replace_file` (même patron
fonction que `ReadSource`/`ParseSql`, palier 2 C-3). `application::load` : signature élargie à
`load(read, path) -> Result<Instance, Refusal>`.

**Adaptation des appelants existants — dans PR2.** Puisque `load` change de type de retour, tout
appelant du palier 2 est adapté **dans cette PR** pour que `memless-engine` (et donc `memless-capi`)
reste vert sur `main` : `application::query` prend l'`Instance` et lit `instance.base` ; le câblage
`capi` de `load` reçoit l'`Instance`. PR3 ne portera que l'adaptation liée à `parse → Statement`.
C'est la condition pour que PR2 soit fusionnable seule.

**Ne fait pas** : ne garde aucune règle ; l'écrivain concret vit dans `yaml/writer`, pas ici.

## Tests (avant le code)

- `tests/render.rs` — golden : chaque fixture rendue est égale à un attendu ; une fixture
  **déjà canonique** produit un rendu **identique à son entrée** (diff vide) ; corpus de valeurs
  limites (vide, espaces, indicateurs, mots nuls, décimal non représentable) **aller-retour**
  `render(document(load(text))) → load → égalité de Base`.
- `tests/writer.rs` — `replace_file` : écrit puis relit à l'octet ; un répertoire **non
  inscriptible** (`chmod 0555`) → `Err(io)` **et** aucun résidu laissé **et** original intact ; un
  résidu préexistant est écrasé ; les permissions de l'original sont conservées.

**Branche « échec après substitution » (cadrage l. 84–87).** Le fsync du répertoire, qui suit le
point de substitution, est **non bloquant** : `swap.rs` rend `Ok` même s'il échoue (le fichier est
déjà remplacé ; seule la durabilité sous coupure immédiate est moindre). Cette branche n'est **pas
testée directement** — l'échec du fsync d'un répertoire n'est pas simulable de façon portable sans
injection dans le système de fichiers. L'issue retenue (retour `Ok`, la mémoire avance) est fixée
ici et par l'ordre strict du cas d'usage (§03) ; le plan (PR2) la consigne comme limite assumée.
