# 02 — Domaine (`memless-domain`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [01 — Modèle et ports](01-modele-et-ports.md)  
**Suivant** : [03 — Application et lecture YAML (`memless-engine`)](03-application-et-yaml.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Crate **pur, sans aucune dépendance externe** (archi §2, archi §6.1) — pas même
`thiserror` : `Display` et `Error` sont écrits à la main. Cargo impose la pureté (un cycle
vers un crate d'infrastructure ne compile pas). Les refus sont conçus **avant** le chemin
nominal.

Ordre des fichiers : la représentation lue, les briques de valeur, l'énumération des
refus, la vérification de forme, enfin l'agrégat.

---

## `crates/memless-domain/Cargo.toml`

Manifeste sans `[dependencies]`. Nom `memless-domain`, édition figée au plan.
**Ne porte pas** : aucune dépendance ; c'est la garantie de pureté.

---

## `crates/memless-domain/src/lib.rs`

Déclare les modules et réexporte **nommément** (jamais `pub use module::*`, qui effacerait
les frontières) les seuls types que l'extérieur consomme. Les modules qui ne portent que
du devinage interne restent `pub(crate)` (N-9 : le devinage n'est pas une surface
publique) ; les types qui entrent dans `Refusal` (`Scalar`, `Id`) sont réexportés parce
que ce type est public.

```rust
pub mod document;
pub mod refusal;
pub mod base;

pub(crate) mod scalar;
pub(crate) mod id;
pub(crate) mod relation;
pub(crate) mod shape;

pub use base::Base;
pub use refusal::Refusal;
pub use scalar::Scalar;
pub use id::Id;
```

**Ne fait pas** : aucune logique ; seulement la carte des modules.

---

## `crates/memless-domain/src/document.rs`

La représentation d'entrée : ce que l'analyseur YAML a lu, **avant tout jugement**. Elle
existe pour que les refus B soient prononcés par le domaine, pas par l'adaptateur ; elle
préserve donc fidèlement l'ordre, les **doublons** de clés, les valeurs imbriquées et les
clés non textuelles. Elle porte aussi une **étiquette de provenance** (`source`) — le
chemin, pour que le refus B1 puisse le nommer (cadrage B1) — qui est une simple chaîne,
pas une entrée-sortie.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawDocument {
    pub source: String, // étiquette de provenance (le chemin), pas un accès disque
    pub root: RawNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawNode {
    Scalar(RawScalar),
    Sequence(Vec<RawNode>),
    Mapping(Vec<(RawKey, RawNode)>), // ordre et doublons préservés
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawKey {
    Text(String),
    NonText { rendered: String }, // entier, vrai/faux, null ou structure en position de clé (B7)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawScalar {
    pub lexeme: String,   // le texte source, non interprété
    pub style: RawStyle,  // comment le YAML le présentait
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawStyle {
    Plain,        // scalaire nu : son type est à deviner
    ExplicitText, // cité, bloc (`|`, `>`) ou tagué `!!str` : texte, sans devinage
}
```

**Refus typés** : aucun — cette structure décrit, ne juge pas.

**Ne fait pas** : n'interprète aucun type (le lexème reste brut — c'est `scalar.rs` qui
devine) ; ne déduplique pas ; ne dérive **aucune** sérialisation (`Serialize` appartient à
un DTO — un objet de transport propre à un adaptateur —, jamais au domaine).

---

## `crates/memless-domain/src/scalar.rs`

Le scalaire deviné, **la grammaire du devinage** et **la règle de comparaison**, réunis là
où ils sont indivisibles.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Scalar {
    Text(String),
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
}

impl Scalar {
    /// Devine le type d'un scalaire nu (§8.1). Grammaire figée, appliquée dans cet ordre :
    ///   - style `ExplicitText` (cité, bloc, tagué `!!str`) => `Text` sans devinage ;
    ///   - `^[-+]?[0-9]+$` tenant dans un i64 => `Integer` ;
    ///   - décimale simple `^[-+]?[0-9]+\.[0-9]+$` => `Decimal` (ni `1e3`, ni `.inf`,
    ///     ni `.nan`, ni hexadécimal : ce sont du `Text`) ;
    ///   - `true` ou `false` (exactement) => `Boolean` ;
    ///   - tout le reste => `Text` (une date sans guillemets reste du texte).
    pub fn guess(raw: &RawScalar) -> Scalar;
}

impl std::fmt::Display for Scalar {
    // Rendu unique pour les messages de refus : texte entre guillemets, entier nu,
    // vrai/faux en `true`/`false`, décimal en forme canonique. `"5"` et `5` se lisent
    // donc différemment — c'est la parité qui l'exige.
}
```

**La règle de comparaison (décision 34) est l'égalité structurelle dérivée** : deux
variantes différentes ne sont jamais égales ; `Decimal` porte un `f64`, d'où `PartialEq` et
non `Eq`. Aucun ordre n'est défini — le palier n'a besoin que de l'égalité. Un module
`comparison` distinct est **choisi de ne pas être créé** (C-2).

**Ne fait pas** : ne compare jamais par ressemblance d'écriture ; ne convertit jamais un
type en un autre ; ne lit pas le fichier (le `RawScalar` lui arrive).

---

## `crates/memless-domain/src/id.rs`

L'`id`, seule colonne requise, restreinte à texte ou entier — porté par sa propre
énumération pour gagner `Eq` et `Hash` (la recherche de doublon C5 est une recherche
d'égal).

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Id {
    Text(String),
    Integer(i64),
}

impl Id {
    /// Rend `None` pour un `id` décimal (C2) ou vrai/faux (C3) ; `Base` traduit ce `None`
    /// en refus avec la table et la position. `null` et l'absence sont traités en amont
    /// par `Base` (C1), indiscernables (décision 35).
    pub fn from_scalar(value: Scalar) -> Option<Id>;
}

impl std::fmt::Display for Id { /* délègue au rendu de `Scalar` */ }
```

**Refus typés** : aucun ici — l'échec est un `None`, que `Base` habille en `Refusal`.

**Ne fait pas** : ne juge pas l'absence d'`id` ni `id: null` (c'est `Base`, C1) ; ne génère
jamais d'identifiant (§3.4). Aucune énumération d'erreur à une seule variante n'est créée
(C-2).

---

## `crates/memless-domain/src/relation.rs`

La relation devinée : une **règle de nom** pure, sans type ni objet à retenir.

```rust
/// Rend la table visée par une colonne relation : `xxx_id` -> `xxxs` (décision 31) ;
/// `_id` exact suit la même règle littérale (D12). Rend `None` quand la colonne n'est pas
/// une relation. L'**existence** de la table visée, elle, est vérifiée par `Base` (une
/// colonne `_id` sans table cible est ordinaire, jamais un refus, §8.1).
pub(crate) fn guessed_target(column: &str) -> Option<String>;
```

**Ne fait pas** : ne vérifie ni l'existence de la table visée ni celle de la ligne (c'est
`Base`) ; ne déclare aucune relation hors convention (§14).

---

## `crates/memless-domain/src/refusal.rs`

**L'énumération des refus** et la **source unique du texte des messages** (son `Display`
produit le message anglais, identique quel que soit le pont — décision 25, D3). Elle est
**typée par couche** pour que chaque signature ne promette que ce qu'elle peut produire :
l'adaptateur ne rend que des refus de **source**, la fabrique que des refus de
**structure**, et un seul `Display` couvre les deux (la parité tient — C-3).

Tous ces types dérivent `#[derive(Debug, Clone, PartialEq)]` (`PartialEq` seul, car
`Scalar` porte un `f64`) ; `Debug` est requis par `impl Error`.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SourceRefusal {          // A — le fichier n'est pas lisible
    PathHasNoFile { path: String },   // A1, A5
    FileNotReadable { path: String }, // A2
    FileEmpty { path: String },       // A3
    InvalidYaml { path: String, at: Option<TextPosition> }, // A4 (encodage/BOM => A4, D17)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructureRefusal {       // B + C — forme, puis cohérence
    // B — le fichier n'a pas la forme d'une base
    NoTableDeclared { source: String },                       // B1 (nomme le chemin)
    TableNotRowList { table: String },                        // B2 (valeur nulle incluse, D6)
    RowNotFieldSet { table: String, position: usize },        // B3
    NestedValue { table: String, row: RowLabel, column: String }, // B4 (et C4)
    DuplicateTableKey { table: String },                      // B5
    DuplicateColumnKey { table: String, row: RowLabel, column: String }, // B6
    NonTextKey { rendered: String, table: Option<String>, position: usize }, // B7, D19
    // C — la structure devinée se contredit
    MissingId { table: String, position: usize },             // C1 (id: null inclus, D4)
    IdNotTextOrInteger { table: String, position: usize, value: Scalar }, // C2, C3
    DuplicateId { table: String, id: Id, positions: (usize, usize) }, // C5
    BrokenRelation {                                           // C6, C7
        table: String, row: RowLabel, column: String,
        target_table: String, value: Scalar,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Refusal {
    Source(SourceRefusal),
    Structure(StructureRefusal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RowLabel {
    Id(Id),          // ligne nommée par son `id` valide (D2)
    Position(usize), // ligne sans `id` valide : sa position (1 = première)
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextPosition { pub line: usize, pub column: usize }
```

Les variantes portent des **objets-valeur** (`Scalar`, `Id`), jamais un texte déjà rendu :
le rendu est fait une seule fois, par `Display for Scalar`/`Id`. `impl From<SourceRefusal>`
et `From<StructureRefusal> for Refusal` ; un seul `impl Display for Refusal` (délègue aux
deux couches) ; `impl std::error::Error for Refusal`.

**Ne fait pas** : ne décide **pas** l'ordre entre deux fautes (c'est `shape`/`base`, D1) ;
ne connaît ni exception PHP ni valeur d'erreur Go (les ponts traduisent la forme, jamais le
texte, D13) ; ne porte aucune faute de frontière FFI (elle vit dans `memless-capi`, C-6).

---

## `crates/memless-domain/src/shape.rs`

La vérification de **forme** (refus B), séparée de la cohérence pour qu'aucun fichier ne
dépasse une raison de changer ni la limite de taille. Transforme la représentation
d'entrée en tables ordonnées de lignes de champs devinés, ou refuse. Elle ne produit
**aucun `Id`** — la ligne n'a pas encore d'`id` valide à ce stade (C1–C3 sont l'étape
suivante), d'où un type de sortie propre, sans `Row`.

```rust
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShapedTable {
    pub name: String,
    pub rows: Vec<Vec<(String, Scalar)>>, // lignes ordonnées de champs devinés, sans id
}

/// Vérifie B1–B7 dans l'ordre du fichier (table, puis ligne, puis colonne) et rend les
/// tables mises en forme, prêtes pour la vérification de cohérence. Chaque valeur de champ
/// est devinée en `Scalar` (`Scalar::guess`) ; une valeur imbriquée est refusée (B4).
pub(crate) fn check_shape(document: RawDocument)
    -> Result<Vec<ShapedTable>, StructureRefusal>;
```

`ShapedTable` est défini ici. `Table` et `Row` (qui, eux, portent l'`Id`) restent dans
`base.rs` et **ne sont pas** importés ici : seul `Base::load` construit un `Id`.

Un refus B qui nomme une ligne (B4, B6) porte un `RowLabel`. `shape` le calcule par
**lecture seule**, sans prononcer C : la ligne est nommée par son `id` s'il porte un
scalaire nu convertible (`Id::from_scalar` rend `Some`), sinon par sa position (D2). Cette
lecture n'établit pas la contrainte C1–C3 — elle ne fait que désigner la ligne fautive.

**Ne fait pas** : ne vérifie aucune cohérence (id, unicité, relations : c'est `base.rs`) ;
ne lit aucun fichier.

---

## `crates/memless-domain/src/base.rs`

L'agrégat racine et sa fabrique — le seul endroit qui **prononce** les refus C, dans
l'ordre figé par D1, après que `shape` a prononcé les refus B.

Tous dérivent `#[derive(Debug, Clone, PartialEq)]`. `Table` et `Row` portent des champs
`pub(crate)` (ou un constructeur `pub(crate)`) : c'est `Base::load` qui les construit depuis
les `ShapedTable`, après avoir prouvé l'`Id`.

```rust
pub struct Base { /* tables ordonnées, privées au module */ }

pub(crate) struct Table { /* pub(crate) name: String, rows: Vec<Row> */ }
pub(crate) struct Row { /* pub(crate) id: Id, columns: Vec<(String, Scalar)> */ }

impl Base {
    /// Fabrique unique. Applique, dans cet ordre (D1) :
    ///   1. forme (B) via `shape::check_shape` — sur tout le fichier ;
    ///   2. cohérence (C) — `id` présent (C1) et bien typé (C2/C3) sur toutes les lignes,
    ///      puis unicité par table (C5, via un ensemble d'`Id`), puis relations devinées
    ///      (C6/C7 : `relation::guessed_target`, existence de la table, puis de la ligne).
    /// À règle égale, la première occurrence dans l'ordre du fichier. Démarre sur l'état
    /// entier, ou refuse sans qu'aucune ligne n'entre.
    pub fn load(document: RawDocument) -> Result<Base, StructureRefusal>;
}
```

L'ordre des tables et des colonnes est capté ici depuis `RawDocument` (il servira à la
réécriture du palier 3 ; §8.6, sous-décision 1-bis renvoyée au palier 3, partie 6).

**Refus typés** : `StructureRefusal` (C). A ne remonte jamais jusqu'ici, B est prononcé par
`shape`.

**Ne fait pas** : ne lit aucun fichier ; ne devine aucun type lui-même (délègue à
`Scalar::guess`) ; ne touche jamais au résidu (il n'en connaît pas l'existence — partie 3) ;
ne porte aucune transition d'écriture (paliers 3–4) ; ne rend pas d'instantané au palier 1.
