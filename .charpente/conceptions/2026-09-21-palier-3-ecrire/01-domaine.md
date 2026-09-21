# 01 — Domaine : modèle, exécution, contraintes, rendu (`memless-domain`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 3 — Écrire](00-index.md)  
**Suivant** : [02 — L'écrivain YAML par substitution (`memless-engine`)](02-ecrivain.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Crate **pur, sans dépendance**. Ordre : le **modèle d'écriture**, les **refus neufs**, le
**déplacement** de l'évaluation de filtre (partagée lecture/écriture), l'**exécution par copie**,
la **redérivation des contraintes** scindée du chargement, le **rendu** de l'état en document.
Porte **PR1**.

## Le modèle d'écriture — `crates/memless-domain/src/query/`

`query/` gagne l'écriture, `pub` (l'adaptateur `sql/` la construit). Un type par fichier.

```rust
// query/write.rs
pub enum Write {
    Insert(Insert),
    Update(Update),
    Delete(Delete),
}

// query/insert.rs — une seule ligne ; colonnes explicites ; None = NULL = absent
// colonnes et valeurs SÉPARÉES : W4 (ColumnCountMismatch) a besoin des deux longueurs
pub struct Insert { pub table: String, pub columns: Vec<String>, pub values: Vec<Option<Scalar>> }

// query/update.rs
pub struct Update {
    pub table: String,
    pub assignments: Vec<(String, Option<Scalar>)>,
    pub filter: Option<Filter>,
}

// query/delete.rs
pub struct Delete { pub table: String, pub filter: Option<Filter> }

// query/statement.rs — ce que `parse` rend : une lecture ou une écriture
pub enum Statement { Select(Select), Write(Write) }
```

`None` porte `NULL` (décision 35). `Filter` est le type du palier 2, réutilisé tel quel.

**Ne fait pas** : n'analyse aucun texte ; ne mute rien ; ne connaît pas `sqlparser`.

## Les refus neufs — `crates/memless-domain/src/refusal/write/`

Seuls **trois** refus sont neufs (cadrage W4, W5, W10) ; le reste réutilise `QueryRefusal`
(Q1–Q4) et `StructureRefusal` (B).

```rust
// refusal/write/mod.rs
pub enum WriteRefusal {
    ColumnCountMismatch { table: String, columns: usize, values: usize }, // W4
    ColumnRepeated { table: String, column: String },                     // W5
    DiskWriteFailed { path: String, kind: String },                       // W10
}
```

Gabarits (anglais), messages découpés en fonctions comme `structure/` (≤ 10 instructions) :

| Variante | Message |
|---|---|
| `ColumnCountMismatch` | `INSERT into {table:?} names {columns} columns but gives {values} values` |
| `ColumnRepeated` | `{column:?} repeated in the write to {table:?}` |
| `DiskWriteFailed` | `cannot write file {path:?}: {kind}` — `kind` = **liste fermée** dérivée de `io::ErrorKind` : `permission denied`, `not found`, `storage full`, `already exists`, sinon `other` ; jamais un texte d'OS localisé (parité testable, conception §04) |

`refusal/mod.rs` gagne `Refusal::Write(WriteRefusal)`, `impl From<WriteRefusal>`, une branche
`Display`. `Base`, `Table`, `Row` gagnent `#[derive(Clone, PartialEq)]` (l'écriture par copie et
le « ne change rien » l'exigent).

**Ne fait pas** : ne duplique aucun message B en variante « write » ; ne connaît pas l'`io` réelle
(le `kind` lui arrive en chaîne depuis l'adaptateur — le domaine ne fait pas d'entrée-sortie).

## Le déplacement de l'évaluation de filtre — `crates/memless-domain/src/base/filter/`

L'évaluation d'un `Filter` sur **une ligne d'une seule table** (aujourd'hui dans `base/select/` :
`passes`, `step`, `enter`, `eval`, `frame`, `eval_leaf`, `compare_holds`, `holds`, `combine_and`,
`combine_or`) **monte** dans `base/filter/`, à l'identique — aucun changement de comportement,
verrouillé par la batterie du palier 2. `select/` et `write/` l'appellent. Le partage passe par un
**trait `Cells { fn cell(&self, &ColumnRef) -> Option<&Scalar> }`** : `keeps<C: Cells>(&C, &Filter)
-> bool` porte la traversée (pile explicite, `holds`, `combine_*` déplacés à l'octet) ; `select`
l'implémente par `Eval` (jointure comprise), `write` par `RowCells` (une seule table). La règle de
comparaison unique reste dans `scalar/scalar_order`, intacte.

**Ne fait pas** : ne rejuge aucune règle de comparaison (elle reste dans `scalar/scalar_order`).

## L'exécution par copie — `crates/memless-domain/src/base/write/`

Le seul endroit qui **applique** une écriture, sur une **copie** de `Base`.

```rust
// base/write/mod.rs
pub struct Applied { pub base: Base, pub affected: u64 }

impl Base {
    pub fn write(&self, statement: &Write) -> Result<Applied, Refusal>;
}
```

`write` (fichiers courts, un par étape) :

1. **`apply.rs`** — clone `self.tables`, applique la mutation demandée :
   - `insert.rs` : table absente → ajoutée **en fin** ; ligne construite par `build_row` (donc W6/W7
     gratuits), **en fin** de table (décision 27), colonnes dans l'ordre de la liste, `None` omis ;
     W4 (longueurs) et W5 (doublon de colonne) prononcés **avant** la construction.
   - `update.rs` : lignes retenues par `filter::keeps` ; colonne existante remplacée à sa place,
     colonne nouvelle **ajoutée en fin de ligne**, `None` retire la paire ; `SET id` passe par
     `Id::from_scalar` ; W5 sur les cibles de `SET`. `affected` = lignes retenues.
   - `delete.rs` : lignes retenues retirées, ordre des autres intact. `affected` = lignes retirées.
   - `UPDATE`/`DELETE` sur table absente → Q3 ; **qualificateur de `WHERE` nommant une autre table → Q3** (comme `select/check_qualifier`, palier 2) ; colonne de `WHERE` absente → Q4 (établis avant la mutation).
   - `insert.rs` s'appuie sur un **invariant d'ordre** : `build_fields` (`zip` colonnes/valeurs, qui tronque au plus court) n'est correct que parce que `check_count` (W4) s'exécute **avant** ; ne pas réordonner.
   - **Ordre des refus** (cadrage point 6) : **forme d'abord** (W4/W5), **puis les noms** (Q3/Q4). Ainsi `UPDATE t_absente SET c=1, c=2` prononce **W5** (`ColumnRepeated`), pas Q3 — un test de gabarit croisé le fixe.
2. **`verify` (voir plus bas)** — la copie mutée passe la **redérivation des contraintes** du
   chargement : W6/W7 (id présent/typé), W8 (unicité), W9 (relations, cas orphelin nommant deux
   tables). Un échec rend `Err(Refusal)` **sans** que `self` ait bougé (c'est une copie).
3. **`unchanged.rs`** — si `applied.base == *self`, `affected` est rendu mais l'appelant saura ne
   pas réécrire (`Applied` porte l'égalité implicitement ; le cas d'usage compare).

**Refus typés** : `Refusal` (Q2 pour l'aiguillage lecture/écriture est prononcé par `sql/` ;
Q3/Q4 ici pour les noms ; W4/W5 ici ; B via `verify`). W1/W2/W10 n'atteignent pas ce fichier.

**Ne fait pas** : ne mute **jamais** `self` ; ne lit ni n'écrit aucun fichier ; ne décide pas de
réécrire (c'est le cas d'usage) ; ne connaît pas les transactions multi-instructions (palier 4).

## La redérivation des contraintes — `crates/memless-domain/src/base/verify.rs`

`Base::load` se **scinde** : la construction de forme reste, et la vérification de cohérence
(aujourd'hui `check_uniqueness` → `build_index` → `check_relations`) devient
`verify(tables: &[Table]) -> Result<(), StructureRefusal>`, appelée **et** par `load` **et** par
`write`. C'est ce partage qui rend l'`INSERT` créant `users` capable de transformer `user_id` en
relation et de nommer l'orphelin avec ses deux tables, **sans code dédié** (décision 37,
redérivation depuis l'état entier). Aucun cache de structure (palier 2 N-5).

**Ne fait pas** : ne construit aucune table (c'est `build`) ; ne connaît pas l'écriture.

## Le rendu de l'état — `crates/memless-domain/src/base/render/`

Le miroir exact de `load`, **côté domaine** : produit la représentation d'entrée depuis l'état.

```rust
// base/render/mod.rs
impl Base {
    pub fn document(&self) -> RawDocument;
}
```

Tables dans l'ordre de `tables`, lignes dans l'ordre, colonnes dans l'ordre de la ligne ; une
colonne absente est **omise** (jamais `null`) ; une table vide → séquence vide. Un texte reçoit
`RawStyle::ExplicitText` **ssi** `Scalar::guess` en style `Plain` ne le rendrait pas tel quel
(`"5"`, `"1.5"`, `"true"` seraient re-devinés en nombre/booléen) — le domaine ne tranche que le
**devinage de type**, ce qu'il sait ; la citation YAML de syntaxe (indicateurs, espaces) est du
ressort de l'écrivain texte (partie 02). `RawDocument`/`RawNode`/`RawScalar`/`RawStyle` existent
déjà (palier 1) et sont réutilisés.

**Ne fait pas** : ne produit aucun texte (c'est `yaml/render`, partie 02) ; ne cite pas selon la
syntaxe YAML (il ne connaît que le devinage) ; ne dérive aucune sérialisation `serde`.

## `lib.rs` — réexports

Réexporte `pub use query::{Write, Insert, Update, Delete, Statement};`,
`pub use refusal::WriteRefusal;` ; `Base`, `Refusal`, `Rows`, `Select` inchangés.

## Tests (avant le code)

- `tests/write.rs` — un `Write` construit à la main contre une `Base` chargée : `INSERT` nominal
  (table nouvelle, colonne nouvelle, `None`=absent), `UPDATE` (filtre, `SET NULL` retire la
  colonne, `SET id`), `DELETE` (avec/sans filtre = table vidée), « ne change rien » (état égal), et
  chaque refus W4, W5, W6, W7, W8, W9 (dont l'orphelin par apparition de table, nommant deux tables).
- `tests/render.rs` — `Base::document` : ordre préservé, colonne absente omise, texte ambigu
  (`"5"`) reçoit `ExplicitText` ; **aller-retour** `load(document(base)) == base` sur les fixtures.
- `tests/write_refusal.rs` — les trois gabarits W4/W5/W10.
