# 02 — PR1 : le domaine (`memless-domain`)

<!-- charpente-navigation -->
**Index** : [plan — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [01 — Fichiers figés et points tranchés par le plan](01-fichiers-figes.md)  
**Suivant** : [03 — PR2 : le moteur et la batterie (`memless-engine`)](03-pr2-moteur-et-batterie.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

**PR1** pose le squelette de l'espace de travail, la garde de frontière et le crate de
domaine **pur** — le cœur du palier : deviner un type, comparer selon la règle unique,
prononcer les refus B et C d'une seule voix anglaise. Aucune dépendance externe (pas même
`thiserror`). C'est le plus gros lot ; il se mène en **deux phases vertes** (1a puis 1b)
**sans se scinder en deux PR** : un domaine sans sa fabrique `Base::load` n'est pas jugeable.

Fichiers figés écrits ici (conception §02, §05) :

- `Cargo.toml` (racine, workspace — 1 seul membre à ce stade)
- `.charpente.json` (racine)
- `crates/memless-domain/Cargo.toml`
- `crates/memless-domain/src/lib.rs`
- `crates/memless-domain/src/document.rs`
- `crates/memless-domain/src/scalar.rs`
- `crates/memless-domain/src/id.rs`
- `crates/memless-domain/src/relation.rs`
- `crates/memless-domain/src/refusal.rs`
- `crates/memless-domain/src/shape.rs`
- `crates/memless-domain/src/base.rs`

Ordre d'écriture imposé par les couches et par la conception §02 : la représentation lue,
les briques de valeur, l'énumération des refus, la vérification de forme, enfin l'agrégat.
Les tests sont **en miroir** (`crates/memless-domain/tests/` pour l'intégration ; modules
`#[cfg(test)]` pour l'unité, selon l'usage Rust), écrits **avant** le code, observés rouges.

---

## Phase 1a — Le workspace, la garde, les types et les textes de refus

**Statut : À faire**

Elle pose l'espace de travail réduit à un membre, `.charpente.json`, puis les quatre
fichiers de domaine qui n'ont **aucune dépendance interne au reste du domaine** : la
représentation d'entrée, le scalaire deviné, l'`id`, et l'énumération des refus avec son
`Display` (la source unique des textes anglais). Tous sont `pub` ou réexportés, donc aucun
avertissement `dead_code` de clippy.

**Fichiers écrits** : `Cargo.toml` (racine, `members = ["crates/memless-domain"]`,
`resolver = "2"`) ; `.charpente.json` (conception §05 : `architecture: hexagonal`,
`layers: [["domain"],["application"],["interface"],["infrastructure"]]`) ;
`crates/memless-domain/Cargo.toml` (sans `[dependencies]`) ; `src/lib.rs` (carte des modules
et réexports nommés) ; `src/document.rs` ; `src/scalar.rs` ; `src/id.rs` ; `src/refusal.rs`.

**Tests, avant le code** (un comportement par ligne ; titres anglais état → résultat) :

*`scalar.rs` — la grammaire du devinage (conception §02, C-12) :*
- guesses a quoted scalar as text without inspecting its content
- guesses a block-scalar (`|`, `>`) as text
- guesses a `!!str`-tagged scalar as text
- guesses a bare `-?[0-9]+` fitting an i64 as integer
- guesses an integer overflowing i64 as text (not integer)
- guesses a bare `-?[0-9]+\.[0-9]+` as decimal
- guesses `1e3`, `.inf`, `.nan`, hexadecimal as text (not decimal)
- guesses exactly `true` / `false` as boolean
- guesses an unquoted date as text
- renders text quoted, integer bare, boolean as `true`/`false`, decimal canonical (Display)
- renders `Text("5")` and `Integer(5)` differently (parity requires it)

*`scalar.rs` — la règle de comparaison (décision 34, C-2) :*
- treats two different variants as never equal (`Text("5") != Integer(5)`)
- treats `Integer(5)` and `Decimal(5.0)` as never equal
- treats same-variant same-value as equal

*`id.rs` (conception §02) :*
- builds an Id from a text scalar
- builds an Id from an integer scalar
- rejects a decimal scalar as `None` (C2)
- rejects a boolean scalar as `None` (C3)
- gives equal-and-hashing Ids for same text (needed by C5 duplicate search)
- renders an Id by delegating to Scalar's rendering

*`refusal.rs` — un test par variante, le texte anglais figé (le golden vit ici) :*
- renders A1 `PathHasNoFile` naming the path
- renders A2 `FileNotReadable` naming the path
- renders A3 `FileEmpty` naming the path
- renders A4 `InvalidYaml` naming the path (and position when present)
- renders B1 `NoTableDeclared` naming the path
- renders B2 `TableNotRowList` naming the table
- renders B3 `RowNotFieldSet` naming table and position
- renders B4 `NestedValue` naming table, row, column
- renders B5 `DuplicateTableKey` naming the table
- renders B6 `DuplicateColumnKey` naming table, row, column
- renders B7 `NonTextKey` naming rendered key and position
- renders C1 `MissingId` naming table and position
- renders C2/C3 `IdNotTextOrInteger` naming table, position, offending value
- renders C5 `DuplicateId` naming table, shared id, both positions
- renders C6/C7 `BrokenRelation` naming holder, row, column, target table, orphan value
- converts `SourceRefusal` and `StructureRefusal` into `Refusal` via `From`
- exposes `Refusal` as `std::error::Error`

**Critères vérifiables** (depuis la racine du dépôt) :
- `cargo build -p memless-domain` sort 0.
- `cargo test -p memless-domain` sort 0, tous les tests ci-dessus verts.
- `cargo clippy -p memless-domain -- -D warnings` sort 0 (aucun `dead_code`).
- `test -f .charpente.json && test -f Cargo.toml` — les deux fichiers racine existent.

**Ne fait pas** : n'écrit ni `relation.rs`, ni `shape.rs`, ni `base.rs` (phase 1b) ; ne
prononce aucun refus (elle ne fait que les **rendre** en texte).

---

## Phase 1b — La forme, la relation et l'agrégat

**Statut : À faire**

Elle ajoute les trois fichiers qui **prononcent** les refus : la règle de nom des relations,
la vérification de forme (B), et l'agrégat `Base` avec sa fabrique faillible (C), dans
l'ordre figé par cadrage D1. `Base::load` est le seul endroit qui construit un `Id` et
prononce C.

**Fichiers écrits** : `crates/memless-domain/src/relation.rs` ;
`crates/memless-domain/src/shape.rs` ; `crates/memless-domain/src/base.rs`. Mise à jour de
`src/lib.rs` si un module manque à la carte.

**Tests, avant le code** :

*`relation.rs` (conception §02, décision 31, cadrage D12) :*
- maps `xxx_id` to target table `xxxs`
- maps exact `_id` literally to table `s`
- gives `None` for a column that is not a relation
- does not check target existence (that is Base)

*`shape.rs` — B1–B7 dans l'ordre du fichier (conception §02) :*
- refuses a document with no table declared (B1) naming the path
- refuses a root scalar / root sequence as B1
- refuses a top-level key whose value is not a row list (B2), null value included (D6)
- refuses a row that is not a field set (B3) naming table and position
- refuses a nested field value (B4) naming table, row, column
- refuses a duplicated table key (B5)
- refuses a duplicated column key in one row (B6)
- refuses a non-text table/column key (B7, D19)
- names a shaped row by its id when a bare convertible scalar is present, else by position (D2)
- produces `ShapedTable` with ordered rows of guessed fields, without any Id
- accepts an empty table list (`users: []`) as a table with no rows

*`base.rs` — la fabrique, dans l'ordre D1 (conception §02, cadrage §6) :*
- refuses a row without an id (C1), `id: null` included (D4), naming table and position
- refuses a decimal id (C2) naming table, position, value
- refuses a boolean id (C3) naming table, position, value
- refuses two rows sharing an id in one table (C5) naming table, id, both positions
- refuses a guessed relation pointing at no existing row (C6) naming all coordinates
- refuses a self-table guessed relation with no matching row (C7)
- accepts `users: [{id: 5}, {id: "5"}]` as two distinct rows (types never equal)
- accepts a guessed relation that is absent or null (D5), nothing to check
- accepts a `stripe_id` column with no `stripes` table as an ordinary column
- checks each rule over the whole file before the next, first file-order occurrence on tie (D1)
- names the first structure fault when a file cumulates several (B before C, then C1-3, C5, C6-7)
- returns `Err(StructureRefusal)` with no `Base` built when the file is incoherent

**Critères vérifiables** :
- `cargo test -p memless-domain` sort 0 avec **tous** les tests 1a **et** 1b verts (dont les
  tests d'ordre D1 « names the first structure fault » et « first file-order occurrence on
  tie », qui prouvent l'ordre par leur seule présence verte).
- `cargo clippy -p memless-domain -- -D warnings` sort 0.
- `cargo build` (workspace entier, un seul membre) sort 0.

**Ne fait pas** : ne lit aucun fichier (le `RawDocument` lui arrive) ; ne devine aucun type
lui-même (délègue à `Scalar::guess`) ; ne porte aucune transition d'écriture.

---

## Fin de PR1

Les quatre portes cargo vertes, PR1 se clôt par **un commit** (`feat(domain): …`, message
anglais impératif, une portée). La racine `Cargo.toml` ne liste qu'un membre : PR2 y ajoutera
`memless-engine`. La branche, sa création et sa fusion sont l'affaire de `/charpente:build`
et `/charpente:pr` ; ce plan ne crée ni branche ni commit.
