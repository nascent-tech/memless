# 01 — Domaine : scission apply/verify, refus de transaction (`memless-domain`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 4 — Transiger](00-index.md)  
**Suivant** : [02 — État de travail et cas d'usage (`memless-engine`)](02-moteur.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Crate **pur, sans dépendance**. Le palier 4 n'ajoute au domaine que **trois choses** : exposer
l'**application sans vérification** (pour l'état de travail), exposer la **vérification de structure
d'ensemble** (pour la validation), et les **refus de garde de transaction**. Le reste (grammaire,
état de travail, cas d'usage) vit dans `memless-engine` (parties 02–03). Porte **PR1**.

## La scission apply / verify — `crates/memless-domain/src/base/`

Le palier 3 a `Base::write` = **apply** (forme W4/W5, noms Q3/Q4, `id` W6/W7 par construction de
ligne) **puis** verify (structure d'ensemble W8/W9), rendant `Applied { base, affected }`. Le palier
4 **expose les deux moitiés** :

```rust
// base/write/mod.rs — la moitié « appliquer » devient publique
impl Base {
    pub fn apply_write(&self, statement: &Write) -> Result<Applied, Refusal>; // apply seul, PAS de verify
    pub fn write(&self, statement: &Write) -> Result<Applied, Refusal>;       // apply_write + verify (palier 3)
}

// base/verify.rs — la moitié « vérifier » devient une méthode publique
impl Base {
    pub fn verify_state(&self) -> Result<(), Refusal>; // W8/W9 sur l'état entier
}
```

`apply_write` évite le terme exclu « stager » (cadrage §3) ; il **expose** l'`apply` interne du
palier 3 sans le rappeler (pas de collision de nom : le libre `apply(tables, statement)` reste
interne, la méthode `apply_write` le délègue).

- **`apply_write`** réutilise le `apply` interne (`base/write/apply.rs` : insert/update/delete, qui
  prononce Q3/Q4, W4/W5, et W6/W7 via `build_row`). Il **ne** vérifie **pas** l'unicité ni les
  relations — un état de travail peut être transitoirement invalide (décision 15). Rend `Applied`.
- **`verify_state`** enveloppe le `verify(tables)` interne (`check_uniqueness` → `build_index` →
  `check_relations`) et mappe `StructureRefusal` → `Refusal`. C'est ce que la **validation** (partie
  02) appelle sur l'état de travail final.
- **`write`** (palier 3) devient `let applied = self.apply_write(statement)?; applied.base
  .verify_state()?; Ok(applied)` — comportement **inchangé** pour l'écriture isolée.

**Ne fait pas** : ne connaît ni transaction, ni état de travail, ni `Instance` (application) ; ne
lit ni n'écrit aucun fichier. `apply_write` ne décide pas de réécrire.

**Les variantes `Statement::Begin`/`Commit`/`Rollback` ne sont PAS ajoutées ici.** Ajouter une
variante à l'enum `Statement` rendrait non exhaustifs les `match` de `memless-engine` (palier 3 :
`application::query`/`write`) — l'exhaustivité couple la variante à ses consommateurs. Les trois
variantes **et** leur traitement atterrissent donc **ensemble en PR2** (conception §02), pour que le
workspace reste vert. PR1 se limite aux ajouts **sans couplage d'exhaustivité** : `apply_write`,
`verify_state`, `TransactionRefusal`, `Refusal::Transaction` (la branche `Display` de `Refusal` est
mise à jour dans la même PR — c'est le seul `match` exhaustif sur `Refusal`, et il vit au domaine).

## Les refus de garde — `crates/memless-domain/src/refusal/transaction/`

Deux refus neufs (cadrage T1, T2). Le reste (T3 = Q2 ; W1–W7 par instruction ; W8/W9 par
`verify_state` ; W10 par l'écrivain) **réutilise** les types des paliers 2–3.

```rust
// refusal/transaction/mod.rs
pub enum TransactionRefusal {
    AlreadyOpen,        // T1 : BEGIN alors qu'une transaction est ouverte
    NoOpenTransaction,  // T2 : COMMIT/ROLLBACK sans transaction ouverte
}
```

Gabarits (anglais), messages en fonctions comme `structure/` (≤ 10 instructions) :

| Variante | Message |
|---|---|
| `AlreadyOpen` | `a transaction is already open` |
| `NoOpenTransaction` | `no open transaction` |

`refusal/mod.rs` gagne `Refusal::Transaction(TransactionRefusal)`, `impl From<TransactionRefusal>`,
une branche `Display`.

**Ne fait pas** : ne décide pas **quand** ces refus surviennent (c'est le cas d'usage, partie 02) ;
ne connaît pas l'état ouvert/fermé (le domaine est sans état).

## `lib.rs` — réexports

`pub use refusal::TransactionRefusal;` ; `Statement`, `Base`, `Applied`, `Refusal` déjà exportés.

## Tests (avant le code)

- `tests/apply_write.rs` — `Base::apply_write` applique **sans** vérifier : un `INSERT` créant un `id`
  en doublon ou une relation cassée **passe** `apply_write` (rend `Applied`), là où `write` (palier 3)
  le refuse ; un `INSERT` sans `id` (W6) échoue **quand même** à `apply_write` (cadrage point 6).
  `verify_state` sur un état à `id` en doublon → `DuplicateId` ; sur un orphelin → `BrokenRelation` ;
  sur un état sain → Ok.
- `tests/transaction_refusal.rs` — les gabarits `AlreadyOpen`/`NoOpenTransaction` (messages exacts).
- La batterie du palier 3 (`tests/write.rs`) reste **verte** : `write` = `apply_write` + `verify_state` ne
  change aucun comportement de l'écriture isolée.
