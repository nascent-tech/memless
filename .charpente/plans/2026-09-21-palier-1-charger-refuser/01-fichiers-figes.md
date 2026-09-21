# 01 — Fichiers figés et points tranchés par le plan

<!-- charpente-navigation -->
**Index** : [plan — Palier 1 — Charger et refuser](00-index.md)  
**Suivant** : [02 — PR1 : le domaine (`memless-domain`)](02-pr1-domaine.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## Les fichiers figés, par chemin exact

La conception a figé la surface publique de chacun (signatures, refus typés, ce qu'il ne
fait pas). Ce plan **ne les rouvre pas** ; il les reprend par leur chemin pour la
répartition. **30 fichiers de code figés par la conception**, 18 fixtures + 1 résidu, plus
**3 fichiers ajoutés par le plan** (voir la section « Fichiers ajoutés » ci-dessous), sur
4 chaînes d'outils.

**Racine** (grandit PR après PR, voir ajustement 1) : `Cargo.toml` · `.charpente.json`

**`memless-domain`** (PR1) : `crates/memless-domain/Cargo.toml` · `src/lib.rs` ·
`src/document.rs` · `src/scalar.rs` · `src/id.rs` · `src/relation.rs` · `src/refusal.rs` ·
`src/shape.rs` · `src/base.rs`

**`memless-engine`** (PR2) : `crates/memless-engine/Cargo.toml` · `src/lib.rs` ·
`src/application/mod.rs` · `src/application/load.rs` · `src/yaml/mod.rs` ·
`src/yaml/reader.rs` · `src/yaml/parse.rs`

**`memless-capi`** (PR3) : `crates/memless-capi/Cargo.toml` · `include/memless.h` ·
`src/lib.rs` · `src/instances.rs` · `src/ffi.rs`

**`bindings/php`** (PR4) : `composer.json` · `src/MemlessRefusal.php` · `src/Instance.php`

**`bindings/go`** (PR5) : `go.mod` · `memless.go`

**`harness/parity`** (PR6) : `run.sh` · `README.md` · `fixtures/*.yaml` (18 fixtures,
écrites en PR2)

Aucun fichier figé n'a manqué : la liste des 30 est reprise à l'identique de la conception.

## Fichiers ajoutés par le plan (non figés par la conception)

La répartition a fait apparaître **trois fichiers** que la conception ne fige pas. Aucun ne
touche le modèle, une signature ou un refus : ce sont de l'outillage et des pilotes de
harness. Ils sont **soumis à ratification** (amendement de la conception, ou accord du
fondateur) avant le passage en `en_cours`.

| Fichier | PR | Pourquoi le plan l'ajoute |
|---|---|---|
| `.gitignore` (racine) | PR3 | sans lui, `target/` et `vendor/` polluent l'arbre et créent un conflit trivial entre PR4 et PR5 ; posé en PR3, avant les deux ponts |
| `harness/parity/php/load.php` | PR6 | `run.sh` doit **exécuter** le pont PHP, or `bindings/php` est une **bibliothèque**, pas un exécutable : il faut un pilote qui charge un chemin, imprime l'issue, libère |
| `harness/parity/go/main.go` (+ `go.mod`) | PR6 | idem côté Go ; un pilote exécutable importe `bindings/go` |

Décompte : **30 figés + 3 ajoutés = 33 fichiers de code**, plus 18 fixtures + 1 résidu.

## Versions et éditions figées

Le plan honore la stack de l'architecture (archi §7, `stack.md`) et fige ce que le build ne
doit pas réinventer. Les correctifs exacts sont consignés dans les fichiers de verrouillage
(`Cargo.lock`, `go.sum`, `composer.lock`) au build ; ce qui suit est la contrainte figée.

| Chaîne | Figé | Source / motif |
|---|---|---|
| Rust | **édition 2021** pour les trois crates ; `resolver = "2"` au workspace | édition stable conservatrice ; le cœur n'exige aucune construction d'édition 2024 |
| Bibliothèque YAML | **`serde-saphyr = "=1.3.0"`**, features par défaut, en `[workspace.dependencies]` | archi `stack.md`, `dependencies.md` (retenu après recherche) |
| Go | module en **`go 1.21`** ; dépendance `github.com/ebitengine/purego` **0.10.2** | purego 0.10.2 exige Go ≥ 1.18 (`stack.md`, `dependencies.md` figent 0.10.2) ; 1.21 est une base maintenue au-dessus de ce minimum |
| PHP | **`>=7.4`** avec `ext-ffi` (conception §04) ; **PHPUnit ligne 9.x** en `require-dev` | archi (brief §18) ; PHPUnit 10+ exige php 8.1, la ligne 9.x couvre php 7.4 |

`IndexMap` (archi `stack.md`) n'entre **pas** au palier 1 : l'ordre vit dans les `Vec` de
`RawNode` (conception §03, N-11) ; il reviendra à la réécriture du palier 3.

## Point tranché A — Le point d'entrée `serde-saphyr` et le budget (renvoi conception §03, C-8, C-11)

C'est le seul poste que la conception n'avait pas transcrit ; elle l'a **renvoyé au plan**.
Vérification faite sur la source en cache de la version 1.3.0 et sur docs.rs.

**Le point d'entrée.** `serde-saphyr` 1.3.0 n'a **aucune API événements propre** : il
réexporte l'analyseur `granit-parser` sous `serde_saphyr::granit_parser` (réexport actif via
la feature par défaut `deserialize`). `parse.rs` lit au niveau événements par :

```rust
use serde_saphyr::granit_parser::{Parser, Event, Span, ScalarStyle, Tag, Options, ScanError};
```

- **Construction** : `Parser::new_from_str_with_options(source: &str, options)` (ou
  `new_from_str`). **Entrée `&str` uniquement** — pas de `&[u8]` ni de `Read` : `reader.rs`
  lit d'abord le fichier en `String` (un décodage impossible ou un BOM inattendu y devient
  A4, D17), puis passe la chaîne à `parse`.
- **Consommation** : `Parser` implémente `Iterator<Item = Result<(Event, Span), ScanError>>`.
- **Style du scalaire** (C-12) : `Event::Scalar(Cow<str>, ScalarStyle, usize, Option<Cow<'_, Tag>>)`.
  `ScalarStyle` ∈ `{ Plain, SingleQuoted, DoubleQuoted, Literal, Folded }`. `RawStyle::Plain`
  = `ScalarStyle::Plain` **et** tag `None` ; tout le reste (cité, bloc `|`/`>`, ou tagué
  `!!str` — reconnu par `Tag::is_yaml_core_schema_tag("str")`) devient `RawStyle::ExplicitText`.
- **Position** (A4) : chaque événement porte `Span { start: Marker, end: Marker, .. }` ;
  `Marker::line()` est **1-indexé**, `Marker::col()` **0-indexé** — `TextPosition` du domaine
  attend une colonne 1-indexée, `parse.rs` ajoute donc 1 à la colonne. Une `ScanError` porte
  `marker()` et `kind()`, habillés par `read` en `InvalidYaml { path, at }` (A4).
- **Clés dupliquées et non textuelles** (B5/B6/B7) : entre `MappingStart` et `MappingEnd`, les
  nœuds alternent clé/valeur sans marqueur ; `parse.rs` compte. Aucune fusion des doublons
  (elle n'existe que sur la voie serde) : B5/B6 restent prononçables. Une clé structurée
  arrive comme `SequenceStart`/`MappingStart` en position de clé → `RawKey::NonText` (B7). Les
  `Event::Comment` sont ignorés (ou `emit_comments: false` dans `Options`).
- **Multi-documents** (`---`, D9) : `parse.rs` **compte les `Event::DocumentStart`** ; le
  second est un refus (A4). Le drapeau « explicite » ne suffit pas (un document unique ouvert
  par `---` le porte aussi) ; `Parser::load(multi = false)` est inadapté (il tait le second).

**Le budget anti-« YAML bomb ».** Le réglage figé, en quatre points :

- **Constat.** Le `serde_saphyr::Budget` et ses `AliasLimits` ne s'appliquent **qu'à la voie
  serde** : le vérificateur est `pub(crate)`, non branchable sur un `Parser` piloté à la main.
- **Ce qui s'applique en lecture événements.** Seules les `granit_parser::Options` limitent —
  tous des `usize`, **saturables à `usize::MAX`** : `flow_nesting_limit`, `block_nesting_limit`,
  `simple_key_max_lookahead`, `max_buffered_comment_events`, `max_directive_bytes`,
  `max_reserved_directive_params`.
- **Réglage.** Construire les `Options` avec ces limites saturées
  (`granit_parser::options!{ .. }`). Le parseur **n'expanse pas les alias** : il émet
  `Event::Alias(id)` et le consommateur rejoue. Aucun refus chiffré d'alias n'existe donc au
  niveau parseur — §12.3 (brief) tient, `loads-anchors.yaml` charge sans refus chiffré.
- **Dette C-8 assumée.** Le risque « billion laughs » se déplace dans `parse.rs` (résolution
  d'alias) et dans la récursion de son constructeur d'arbre. `parse.rs` **ne pose aucune
  limite** ; les fixtures du palier portent un nombre borné d'alias, non hostiles (MVP §5).
  Deux résidus non éliminables demeurent : le débordement de pile d'un arbre profond (saturer
  `block_nesting_limit` retire la seule garde) et la limite fixe à neuf chiffres des
  composantes `%YAML`.

**Désync à refléter.** La formulation de la conception §03 et C-8 (« budget saturé,
`AliasLimits` saturés ») vise la voie serde ; le plan retient « `granit_parser::Options`
saturées + résolution d'alias non bornée, dette C-8 ». C'est une **imprécision de formulation
de la conception**, à corriger par un amendement de la conception §03/C-8 sans changer la
décision (« aucun refus chiffré + dette assumée » tient) — pas au chantier d'architecture.

**Décision de dépendance.** Le contrat public consommé (`Event`, `Span`) appartient à
`granit-parser`, réexporté par `serde-saphyr`. Le plan **garde `serde-saphyr` comme seule
dépendance déclarée** (choix de l'architecture) et accède au parseur via
`serde_saphyr::granit_parser`. Alternative écartée : déclarer `granit-parser = "=1.3.0"` en
direct — le build ne la retiendra que si la feature `deserialize` cessait de réexporter le
parseur (active par défaut).

## Point tranché B — L'outillage de test PHP (renvoi du découpage)

La conception nomme le paquet Composer (`php >=7.4`, `ext-ffi`) mais aucun outil de test. Le
plan fige les **quatre portes de la chaîne composer** :

| Porte | Commande |
|---|---|
| syntaxe | `php -l` sur chaque source PHP (la chaîne PHP n'a pas de vérificateur de types) |
| lint | `composer validate --strict` |
| test | **PHPUnit** (`vendor/bin/phpunit`), dépendance `require-dev` du `composer.json` |
| build | chargement effectif de la cdylib bâtie par PR3 (`FFI::cdef` sur l'en-tête committé) |

PHPUnit est l'outil de test standard de l'écosystème et se contente d'un `require-dev` ; il
n'ajoute aucune dépendance de production au pont.

## Point tranché C — Les trois ajustements de contenu du découpage

Relevés par l'analyse de découpage, ils ne changent ni le modèle ni les signatures ; ils
précisent **où** un fichier atterrit.

1. **La racine `Cargo.toml` grandit PR après PR.** Cargo refuse un `members` pointant vers un
   crate absent. PR1 déclare `members = ["crates/memless-domain"]` ; **PR2** y ajoute
   `memless-engine` et la `[workspace.dependencies]` YAML ; **PR3** y ajoute `memless-capi`,
   le profil de compilation **sans `panic = "abort"`** (sinon `guard` est inopérant) et le
   `.gitignore`. Chaque PR touche la racine, sur des **lignes disjointes** — aucun conflit.
2. **Les portes PHP sont figées** (point tranché B) : la conception ne les nommait pas.
3. **PR6 porte deux pilotes** (`harness/parity/php/load.php`, `harness/parity/go/main.go`) :
   voir la section « Fichiers ajoutés » et le détail de leur câblage en
   [partie 05](05-pr6-parite-et-decoupage.md).

Aucun de ces points ne rouvre une décision figée ; ils comblent ce que la conception avait
explicitement renvoyé au plan.
