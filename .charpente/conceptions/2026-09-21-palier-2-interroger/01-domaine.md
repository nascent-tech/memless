# 01 — Domaine : modèle, exécution, refus (`memless-domain`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 2 — Interroger](00-index.md)  
**Suivant** : [02 — Moteur et surface native (`memless-engine`, `memless-capi`)](02-moteur-et-capi.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Crate **pur, sans dépendance** — `sqlparser` n'entre jamais ici. Ordre des fichiers : le
**modèle de requête** (ce qu'une requête vérifiée est), le **résultat** (`Rows`), les **refus**
(`QueryRefusal`), puis l'**exécution** sur `Base.tables`. `Base.tables` perd son
`#[allow(dead_code)]` : le palier 2 est son premier lecteur.

---

## Le modèle de requête — `crates/memless-domain/src/query/`

Un module `pub` : l'adaptateur `sql/` de `memless-engine` construit ces types depuis l'AST de
`sqlparser`, puis les passe à l'exécution. Le modèle ne porte que ce que le sous-ensemble
(cadrage D1) autorise — il ne **peut pas** représenter une construction hors sous-ensemble, ce
qui rend un état hors-ensemble non constructible plutôt que rejeté tard.

Un type par fichier. `query/mod.rs` déclare les modules et réexporte les types.

```rust
// query/select.rs
pub struct Select {
    pub items: Items,
    pub from: String,
    pub join: Option<Join>,
    pub filter: Option<Filter>,
}

// query/items.rs — la projection : des colonnes, ou des agrégats ; jamais mêlés (cadrage Q2)
pub enum Items {
    All,                 // `*`
    Columns(Vec<ColumnRef>),
    Aggregates(Vec<Aggregate>),
}

// query/column_ref.rs
pub struct ColumnRef { pub table: Option<String>, pub column: String }

// query/aggregate.rs
pub enum Aggregate { CountStar, Count(ColumnRef), Sum(ColumnRef) }

// query/join.rs — une jointure inner ; `left`/`right` sont les deux côtés du `ON`
pub struct Join { pub table: String, pub left: ColumnRef, pub right: ColumnRef }

// query/filter.rs
pub enum Filter {
    Compare(Compare),
    IsNull(ColumnRef),
    IsNotNull(ColumnRef),
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
}

// query/compare.rs
pub struct Compare { pub column: ColumnRef, pub op: Op, pub literal: Scalar }

// query/op.rs
pub enum Op { Eq, Ne, Lt, Le, Gt, Ge }
```

Pas de `Order` ni de `Direction` : le tri est hors sous-ensemble (cadrage D1, MVP §4). Les
opérateurs `<`/`<=`/`>`/`>=` du `WHERE` comparent **au sein d'un même type** (voir
`scalar_order`), sans jamais introduire un `ORDER BY`.

Un **littéral** est un `Scalar` (réutilisé) : la grammaire des types d'un littéral SQL est
celle du devinage YAML (cadrage D1) — `'text'` → `Text`, entier → `Integer`, décimale →
`Decimal`, `TRUE`/`FALSE` → `Boolean`. Aucun `Literal` distinct (N-1).

**Ne fait pas** : n'analyse aucun texte (c'est `sql/`) ; ne connaît pas `sqlparser` ; ne porte
aucune écriture (paliers 3–4).

---

## Le résultat — `crates/memless-domain/src/rows.rs`

Le jeu de lignes rendu, `pub`, réexporté par `lib.rs`.

```rust
pub struct Rows {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<Scalar>>>,
}
```

Une **cellule** est `Option<Scalar>` : `None` = valeur absente sur cette ligne (décision 35),
rendue `null`/`nil` par les ponts ; jamais un cinquième scalaire (N-2). Un **agrégat** est un
`Rows` d'exactement une ligne, une cellule par agrégat — rien ne le distingue dans le type
(cadrage point 5). L'ordre des colonnes et des lignes est porté par les `Vec`, tel que
l'exécution le fixe (cadrage D4, D6).

**Ne fait pas** : ne porte aucune métadonnée de type par colonne (le type vit dans chaque
cellule, décision 36) ; ne se sérialise pas (pas de `Serialize` — c'est un DTO d'adaptateur qui
traversera la frontière, partie 02).

---

## Les refus — `crates/memless-domain/src/refusal/query/`

`QueryRefusal` et ses messages, dans le style des refus de chargement (minuscule, anglais, noms
entre `{:?}`). Le `Display` est **découpé** en fonctions de message pour tenir ≤ 10 instructions,
comme `structure/` : `syntax_message` (Q1–Q2), `name_message` (Q3–Q5), `execution_message`
(Q7–Q8).

```rust
// refusal/query/mod.rs
pub enum QueryRefusal {
    InvalidSql { detail: String },                              // Q1
    OutsideSubset { construct: String },                        // Q2
    UnknownTable { table: String },                             // Q3
    UnknownColumn { table: String, column: String },            // Q4
    JoinNotRelation { table: String, column: String, target: String }, // Q5
    SumNotNumber { table: String, column: String, row: RowLabel }, // Q7
    SumOverflow { table: String, column: String },              // Q8
}
```

Gabarits exacts (anglais), un par variante :

| Variante | Message |
|---|---|
| `InvalidSql` | `invalid SQL: {detail}` — `detail` = message de `sqlparser` sans son préfixe, position incluse quand il la donne (calculé dans `sql/`, transporté ici comme texte). |
| `OutsideSubset` | `{construct} is outside the supported SQL subset` — `construct` vient d'une table de correspondance de `sql/`, jamais un fragment du texte utilisateur. |
| `UnknownTable` | `no table {table:?}` |
| `UnknownColumn` | `no column {column:?} in table {table:?}` |
| `JoinNotRelation` | `{column:?} of {table:?} is not a guessed relation to the id of {target:?}` |
| `SumNotNumber` | `cannot sum {column:?} of {table:?} at row {row}` |
| `SumOverflow` | `sum of {column:?} in {table:?} overflows` |

`refusal/mod.rs` gagne une variante et son câblage :

```rust
pub enum Refusal {
    Source(SourceRefusal),
    Structure(StructureRefusal),
    Query(QueryRefusal),      // ajout
}
```

avec `impl From<QueryRefusal> for Refusal` et une branche de plus dans `Display for Refusal`.

**Ne fait pas** : ne calcule pas la position d'un SQL invalide (c'est `sql/` qui la lit de
`sqlparser` et la met dans `detail`) ; ne connaît ni exception PHP ni valeur d'erreur Go (D13).

---

## L'exécution — `crates/memless-domain/src/base/select/`

Le seul endroit qui **exécute** une requête, sur `Base.tables`. Entrée unique :

```rust
// base/select/mod.rs
impl Base {
    pub fn select(&self, query: &Select) -> Result<Rows, QueryRefusal>;
}
```

Étapes, chacune dans son fichier, dans l'ordre des refus du cadrage (noms → jointure →
exécution ; Q1–Q2 sont prononcés en amont par `sql/`) :

1. **`resolve.rs`** — résout la table `FROM` (Q3) puis, s'il y a une jointure, sa table (Q3) ;
   vérifie que le `ON` est `relation devinée = id de la table visée` : un côté est une **relation
   devinée** (`relation::guessed_target` pointe vers l'autre table, décision 31) **et** l'autre
   côté est la colonne `id` de cette table visée, sinon Q5. Rend un contexte de tables résolues.
2. **`columns.rs`** — l'ensemble des colonnes existantes par table (union des colonnes portées
   par au moins une ligne — définition du palier 1) ; une **table vide** n'a aucune colonne, pas
   même `id`. Toute `ColumnRef` de `items`, `filter`, et des deux côtés du `ON`, doit s'y
   trouver, sinon Q4. Avec jointure, une colonne non qualifiée est déjà refusée en amont (Q2),
   donc chaque `ColumnRef` porte sa table ici.
3. **`combine.rs`** — construit les **lignes candidates** : sans jointure, les lignes de `FROM`
   dans l'ordre du fichier ; avec jointure, chaque ligne de `FROM` appariée aux lignes de la
   table jointe dont l'égalité stricte tient (`scalar` du côté relation == `id` de la cible,
   règle de comparaison unique — décision 34), dans l'ordre du fichier de la table jointe
   (cadrage D4). Quand c'est la table `FROM` qui porte la relation, l'appariement trouve **au
   plus une** cible (relation vérifiée au chargement). Sur le fichier piège — `users` portant
   `id: 5` et `id: "5"`, `wallets.user_id: 5` — la jointure `wallets→users` rejoint `id: 5` et
   **jamais** `id: "5"` (types différents jamais égaux) : c'est la preuve du MVP §6. Une ligne
   dont la relation est absente ou `null` ne s'apparie à rien et **sort** de la jointure interne
   (décision 35). L'ordre des lignes suit `FROM`.
4. **`keep.rs`** — applique `filter` à chaque ligne candidate : `evaluate` d'un `Filter` rend
   un **booléen à deux valeurs** (`compare.rs`, `is_null`) ; une comparaison sur valeur absente
   ou de types différents vaut **faux** (décisions 34, 35) ; les opérateurs `<`/`<=`/`>`/`>=`
   comparent au sein d'un même type (`scalar_order`) ; `AND`/`OR` combinent, les parenthèses
   sont portées par l'arbre. Aucune erreur d'exécution ici.
5. **`project.rs`** — produit `Rows` : `Items::All` rend toutes les colonnes de `FROM` puis
   celles de la table jointe, dans l'ordre de première apparition dans le fichier (cadrage D6) ;
   `Columns` prend les colonnes nommées (qualifiées `table.column` si jointure, sinon le nom du
   fichier) ; `Aggregates` calcule chaque agrégat sur **les lignes retenues** — `CountStar`
   (nombre de lignes), `Count(col)` (lignes portant la colonne), `Sum(col)` : additionne les
   valeurs présentes, **toutes du même type numérique** (Integer additionné dans l'entier 64 bits
   signé avec débordement → Q8 ; Decimal en flottant) ; un texte, un vrai/faux ou un mélange
   entier/décimal parmi les lignes retenues → **Q7** (nomme la **première** ligne fautive par son
   `id`, dans l'ordre du fichier) ; aucune valeur présente → cellule **absente** (cadrage D5).

Chaque étape est une ou deux fonctions courtes ; la comparaison stricte de deux `Scalar`
(égalité, et ordre **au sein d'un même type**, refus entre types) est portée par un fichier
**`scalar_order.rs`** de `scalar/` : la règle de comparaison, jusqu'ici réduite à l'égalité
(`PartialEq`), gagne l'**ordre au sein d'un type** que `<`/`<=`/`>`/`>=` du `WHERE` exigent, en
rendant `None` entre deux types différents (jamais ordonnés — décision 34). C'est l'unique ajout
à la brique `scalar`. Il n'y a **pas** de `sort.rs` : le tri est hors sous-ensemble.

**Refus typés** : `QueryRefusal` (Q3–Q5, Q7, Q8). Q1–Q2 n'atteignent jamais ici.

**Ne fait pas** : n'analyse aucun texte ; ne modifie jamais `Base` (lecture seule) ; ne convertit
jamais un type en un autre ; n'invente aucun ordre entre types. Il **suppose un `Select` valide**
au sens du sous-ensemble : les constructions hors sous-ensemble (auto-jointure, colonne non
qualifiée dans une jointure, tri) sont refusées **en amont** par `sql/` (Q2) — le domaine ne les
rejuge pas. Une somme décimale qui déborde en `inf` est rendue telle quelle (le débordement
chiffré Q8 ne vise que l'entier — cadrage point 7).

---

## `lib.rs` — réexports

`lib.rs` déclare `pub mod query;`, `pub mod rows;` (ou réexporte `Rows`), garde `base`,
`refusal`, `scalar` inchangés en visibilité, et réexporte `pub use rows::Rows;` et
`pub use refusal::QueryRefusal;`. `Base` reste réexporté. `query` est `pub` (l'engine le
consomme).

**Ne fait pas** : aucune logique ; la carte des modules seulement.
