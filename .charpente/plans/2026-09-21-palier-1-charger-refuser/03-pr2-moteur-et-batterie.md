# 03 — PR2 : le moteur et la batterie (`memless-engine`)

<!-- charpente-navigation -->
**Index** : [plan — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [02 — PR1 : le domaine (`memless-domain`)](02-pr1-domaine.md)  
**Suivant** : [04 — PR3, PR4, PR5 : la surface native et les ponts](04-capi-et-ponts.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

**PR2** ajoute le contexte borné « le moteur » : l'application (le cas d'usage `charger`),
l'adaptateur sortant `yaml`, et la **batterie** de fixtures qui fige le golden des textes de
refus. Elle importe `memless-domain` (PR1). Comme PR1, elle est à la limite d'une session et
se mène en **deux phases vertes** (2a puis 2b), **sans se scinder en deux PR**.

Cette PR ajoute `memless-engine` à `members` de la racine `Cargo.toml` et pose la
`[workspace.dependencies]` de la bibliothèque YAML (lignes disjointes de PR1).

Fichiers figés écrits ici (conception §03, §05) :

- `crates/memless-engine/Cargo.toml`
- `crates/memless-engine/src/lib.rs`
- `crates/memless-engine/src/application/mod.rs`
- `crates/memless-engine/src/application/load.rs`
- `crates/memless-engine/src/yaml/mod.rs`
- `crates/memless-engine/src/yaml/reader.rs`
- `crates/memless-engine/src/yaml/parse.rs`
- `harness/parity/fixtures/*.yaml` (18 fixtures + le résidu voisin)

Le point d'entrée exact de `serde-saphyr` 1.3.0 et le réglage des `Options` de l'analyseur
saturées sont **figés en [partie 01](01-fichiers-figes.md)** (c'est la seule API que la
conception n'a pas transcrite) ; les phases ci-dessous s'y réfèrent.

---

## Phase 2a — L'application et la lecture YAML

**Statut : À faire**

Elle pose le crate, le cas d'usage `charger` (une fonction qui reçoit la lecture en
paramètre — le seul « port », C-5), et l'adaptateur `yaml` (lecture du fichier, puis analyse
au niveau **événements/arbre** — jamais par désérialisation `serde`, C-11 — préservant ordre,
doublons de clés, clés non textuelles et style de scalaire). Elle se teste **sur des chaînes
en ligne**, sans fixture disque.

**Fichiers écrits** : `Cargo.toml` (dépend de `memless-domain` par chemin et de la
bibliothèque YAML ; **sans** `gluesql` ni `indexmap`) ; `src/lib.rs` (réexports nommés) ;
`src/application/mod.rs` ; `src/application/load.rs` ; `src/yaml/mod.rs` ;
`src/yaml/reader.rs` ; `src/yaml/parse.rs`.

**Tests, avant le code** :

*`application/load.rs` — l'enchaînement (conception §03) :*
- returns the Base when `read` yields a document that `Base::load` accepts (read stubbed, no disk)
- returns a `Refusal::Source` when the stubbed `read` fails (A assembled via `From`)
- returns a `Refusal::Structure` when the stubbed `read` yields an incoherent document (B/C via `From`)

*`yaml/parse.rs` — l'analyse préservante (conception §03, C-11) :*
- preserves key order in a mapping
- preserves duplicated keys (does not merge them) so B5/B6 stay pronounceable
- preserves a non-text key (integer/boolean/null/structure) so B7 stays pronounceable
- preserves a nested value (sequence or mapping) where a scalar is expected
- tags each scalar with its style (plain vs quoted/block/`!!str`)
- fails as `ParseFailure` on syntactically invalid YAML, with a position when available
- fails as `ParseFailure` on a non-decodable encoding or unexpected BOM (D17)
- fails as `ParseFailure` on several `---` documents in one stream (D9)
- guesses no type (each scalar stays a raw lexeme)

*`yaml/reader.rs` — l'adaptateur (conception §03) :*
- refuses a path with no ordinary file as `PathHasNoFile` (A1)
- refuses an unreadable file as `FileNotReadable` (A2)
- refuses an empty-or-blank file as `FileEmpty` (A3)
- refuses an unparsable file as `InvalidYaml` naming path and position (A4)
- reads exactly the given path, never a neighbouring file (D16)
- builds the `RawDocument` with its provenance label and parsed root
- accepts a bounded number of anchors/aliases with the saturated `Options` (no numeric refusal, D10/C-8)

**Critères vérifiables** :
- `cargo build -p memless-engine` sort 0.
- `cargo test -p memless-engine` sort 0 (tests 2a verts) — dont les tests B5/B6/B7 qui
  prouvent C-11 : ils tombent si l'analyse fusionne les clés dupliquées.
- `cargo clippy -p memless-engine -- -D warnings` sort 0.
- `load.rs` n'importe pas l'infrastructure (le port est un paramètre, C-5) :
  `! grep -q 'yaml' crates/memless-engine/src/application/load.rs` sort 0.

**Ne fait pas** : n'écrit aucune fixture disque (phase 2b) ; ne juge ni forme ni cohérence
(c'est le domaine) ; n'importe pas `gluesql`.

---

## Phase 2b — La batterie et le golden des textes

**Statut : À faire**

Elle pose les 18 fixtures (plus le résidu voisin) et le **test golden d'intégration** qui
charge chaque fixture par la vraie chaîne (`read` réel, disque) et vérifie l'issue et le
**texte exact** du message. C'est ici que les textes de refus figés en PR1 (`refusal.rs`)
sont éprouvés de bout en bout.

**Fichiers écrits** — un fichier par cas, nommé en anglais (conception §05) :

| Fixture | Cas |
|---|---|
| `start.yaml` | fichier de départ, valide |
| `.start.yaml.memless-tmp` (voisin de `start.yaml`) | résidu ignoré, intact après chargement |
| `empty.yaml` | A3 |
| `invalid-yaml.yaml` | A4 |
| `no-table.yaml` | B1 |
| `table-not-list.yaml` | B2 |
| `row-not-map.yaml` | B3 |
| `nested-value.yaml` | B4 |
| `dup-table-key.yaml` | B5 (D18) |
| `dup-column-key.yaml` | B6 (D18) |
| `non-text-key.yaml` | B7 (D19) |
| `missing-id.yaml` | C1 |
| `id-decimal.yaml` | C2 |
| `id-boolean.yaml` | C3 |
| `dup-id.yaml` | C5 |
| `broken-relation.yaml` | C6 |
| `self-relation.yaml` | C7 |
| `loads-mixed-id.yaml` | non-refus : `id: 5` et `id: "5"` = deux lignes |
| `loads-anchors.yaml` | non-refus : ancres/alias en nombre borné (prouve « aucun refus chiffré ») |

**Tests, avant le code** (intégration golden ; A1/A5 et A2 montés par le test, sans fichier) :
- loads `start.yaml` and returns a Base (no refusal)
- leaves the neighbouring residue intact after loading `start.yaml` (byte compare, D16)
- refuses `empty.yaml` with the exact A3 text
- refuses `invalid-yaml.yaml` with the exact A4 text
- refuses each B fixture (`no-table` … `non-text-key`) with its exact B text
- refuses each C fixture (`missing-id` … `self-relation`) with its exact C text
- loads `loads-mixed-id.yaml` as two rows (accepted)
- loads `loads-anchors.yaml` (accepted, no numeric refusal — D10/C-8)
- refuses a non-existent path as A1 and a permission-stripped file as A2 (situations built by the test)

**Critères vérifiables** :
- `cargo test -p memless-engine` sort 0 avec **tous** les tests 2a **et** 2b verts.
- `cargo clippy -p memless-engine -- -D warnings` sort 0.
- `cargo build` (workspace : `memless-domain` + `memless-engine`) sort 0.
- Les 18 fixtures existent (elles serviront au lanceur de PR6) :
  `for f in start empty invalid-yaml no-table table-not-list row-not-map nested-value dup-table-key dup-column-key non-text-key missing-id id-decimal id-boolean dup-id broken-relation self-relation loads-mixed-id loads-anchors; do test -f harness/parity/fixtures/$f.yaml || exit 1; done` sort 0.

**Ne fait pas** : aucune fixture de requête, d'écriture ou de transaction ; le lanceur de
parité n'arrive qu'en PR6 (les fixtures **spécifient** ici les textes, elles ne se comparent
pas encore entre ponts).

---

## Fin de PR2

Les quatre portes cargo vertes, PR2 se clôt par **un commit** (`feat(engine): …`). Les 18
fixtures vivent désormais sous `harness/parity/fixtures/`, prêtes pour le lanceur de PR6.
