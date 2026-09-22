# 02 — État de travail et cas d'usage (`memless-engine`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 4 — Transiger](00-index.md)  
**Précédent** : [01 — Domaine : scission apply/verify, refus de transaction (`memless-domain`)](01-domaine.md)  
**Suivant** : [03 — Surface C ABI](03-capi.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

L'adaptateur `sql/` **descend** `BEGIN`/`COMMIT`/`ROLLBACK` vers les verbes du domaine ; l'`Instance`
porte l'**état de travail** ; les cas d'usage `execute`/`query` deviennent **conscients de la
transaction** (lecture de ses propres écritures, ordre strict à la validation). Porte **PR2**.

## Le verbe de transaction — `crates/memless-domain/src/query/statement.rs` (ajouté **ici**, PR2)

`Statement` gagne trois variantes **nues** (déplacées de PR1 : ajouter une variante d'enum couple à
ses consommateurs par l'exhaustivité, donc la variante **et** son traitement atterrissent ensemble) :

```rust
// query/statement.rs
pub enum Statement { Select(Select), Write(Write), Begin, Commit, Rollback }
```

`#[derive(Debug, Clone, PartialEq)]`. La même PR met à jour les `match` sur `Statement` de l'engine
(`application::query`/`execute`) — d'où le regroupement en PR2.

## La descente `sql/` — les verbes de transaction

`sql/lower_statement.rs` gagne trois branches. Les formes sqlparser 0.54 (vérifiées) :
`StartTransaction { modes, begin, transaction, modifier }`, `Commit { chain, end, modifier }`,
`Rollback { chain, savepoint }`. L'ensemble **fermé** accepté (le suffixe `TRANSACTION|WORK` est
consommé/toléré par le parseur, donc admis) :

- `BEGIN [TRANSACTION|WORK]` / `START TRANSACTION` → `Begin` **ssi** `modes.is_empty() &&
  modifier.is_none()` (sinon T3) ; `ISOLATION LEVEL …` remplit `modes`.
- `COMMIT [TRANSACTION|WORK]` → `Commit` **ssi** `!chain && !end && modifier.is_none()`. Attention :
  `END` rend `Commit { end: true }` → **refusé T3** (hors D1) ; `COMMIT AND CHAIN` remplit `chain`.
- `ROLLBACK [TRANSACTION|WORK]` → `Rollback` **ssi** `!chain && savepoint.is_none()` ; `ROLLBACK TO s`
  remplit `savepoint`.
- `sql/reject_transaction_options.rs` prononce T3 (`OutsideSubset`, libellé fermé « transaction
  options ») pour ces cas. `SAVEPOINT`/`RELEASE` sont d'**autres** `SqlStatement` → `other =>` → Q2.
  Un modificateur non supporté par le dialecte (`BEGIN DEFERRED`) échoue **au parse** → Q1
  (`InvalidSql`), pas T3 — il n'atteint pas la descente.
- `parse` rend donc `Statement::{Select,Write,Begin,Commit,Rollback}`.

**Ne fait pas** : n'exécute rien ; ne connaît pas l'état ouvert/fermé (c'est le cas d'usage).

## L'état de travail — `crates/memless-engine/src/application/instance.rs`

```rust
// application/instance.rs — l'instance gagne l'état de travail
pub struct Instance {
    pub path: String,
    pub base: Base,                 // l'état validé (l'état d'avant, pendant une transaction)
    pub transaction: Option<Base>,  // Some(état de travail) ssi une transaction est ouverte
}
```

`load` initialise `transaction: None`. **Invariant** : `base` est toujours le dernier état **validé**
(sur le disque) ; `transaction`, quand `Some`, est l'état de travail que les écritures et les
lectures de la transaction voient. Relâcher l'instance jette `transaction` avec elle (abandon
implicite, cadrage 3c) — rien n'a touché le disque.

## Les cas d'usage — `crates/memless-engine/src/application/`

### `execute` — le point d'entrée d'écriture, conscient de la transaction

`application::write` (palier 3) devient `application::execute`, qui **aiguille** sur le `Statement` :

```rust
// application/execute.rs
pub fn execute(parse: ParseSql, replace: ReplaceFile, instance: &mut Instance, text: &str)
    -> Result<u64, Refusal>;
```

1. `parse(text)?` → `Statement`.
2. `Begin` → `open(instance)` ; `Commit` → `validate(replace, instance)` ; `Rollback` →
   `abandon(instance)` ; `Write(w)` → `apply_write(replace, instance, w)` ; `Select` →
   `Refusal::Query(Q2 "a SELECT in execute")`.
3. Rend `u64` : le compte de lignes pour une écriture ; **0** pour `Begin`/`Commit`/`Rollback`.

- **`open.rs`** — si `instance.transaction.is_some()` → `TransactionRefusal::AlreadyOpen` (T1),
  l'ouverte intacte ; sinon `instance.transaction = Some(instance.base.clone())`. Rend `Ok(0)`.
- **`apply_write.rs`** — **remplace** l'`apply.rs` du palier 3 (la fn libre `apply` enveloppant
  `base.write` disparaît ; il n'y a **pas** deux `apply*.rs` côte à côte). **Avec** transaction ouverte : `working.apply_write(&w)?` (apply seul, W1–W7
  ici, laisse la transaction ouverte à l'échec), `instance.transaction = Some(applied.base)`, rend
  `affected`. **Sans** transaction : appelle **le corps de `application::write` du palier 3 déplacé
  tel quel** (`apply_write` → `verify_state` → **si l'état final égale `instance.base`, retour sans
  disque** → `render`+`replace` → avance mémoire) — inchangé, une transaction d'une instruction. Ne
  pas ré-énumérer l'ordre : c'est exactement le palier 3, l'étape « sans changement » comprise
  (test `a_write_without_a_change_touches_no_disk`).
- **`abandon.rs`** — si `instance.transaction.is_none()` → `NoOpenTransaction` (T2) ; sinon
  `instance.transaction = None` (l'état de travail est jeté). Rend `Ok(0)`.
- **`validate.rs`** — l'ordre **strict** (§8.6) :
  1. `let Some(working) = instance.transaction.take()` sinon → `NoOpenTransaction` (T2). **`take`
     ferme la transaction d'abord** : quelle que soit la suite (échec compris), elle est close
     (cadrage 2c).
  2. `working.verify_state()?` — W8/W9 ici ; un échec rend `Err` (transaction déjà fermée, mémoire
     **inchangée** = état d'avant, `instance.base` intact).
  3. Si `working == instance.base` → `Ok(0)` **sans toucher le disque** (§8.6).
  4. Sinon `render(working.document())` → texte ; `replace(&instance.path, &text)` — un `Err(io)`
     devient `Refusal::Write(DiskWriteFailed)` (W10), `instance.base` **inchangé**.
  5. **Puis seulement** `instance.base = working` ; `Ok(0)`.

### `query` — la lecture, conscient de la transaction

`application::query` (palier 2) lit désormais l'**état de travail** si une transaction est ouverte :

```rust
// application/query.rs
pub fn query(parse: ParseSql, instance: &Instance, text: &str) -> Result<Rows, Refusal>;
```

- `parse(text)?` ; un `Statement::Write`/`Begin`/`Commit`/`Rollback` passé à la lecture →
  `Refusal::Query(Q2 "a non-SELECT in query")` (libellé générique, pas « write » — un verbe de
  transaction n'est pas une écriture ; une instruction refusée **laisse la transaction ouverte**,
  décision 29 : `query` ne touche jamais `instance.transaction`).
- Sinon `base_to_read(instance).select(&select)` où `base_to_read` rend `instance.transaction.as_ref()
  .unwrap_or(&instance.base)` — **lecture de ses propres écritures** (cadrage vocabulaire).

`sql/statement_label` gagne `SAVEPOINT`/`RELEASE` (aujourd'hui « this statement »).

## L'adaptation des appelants — dans **cette PR** (frontière de compilation)

`write` (palier 3) est **remplacé** par `execute`, et `query` prend une `&Instance` au lieu d'un
`&Base`. Ces deux signatures changent la surface publique de `memless-engine`, dont **`memless-capi`
dépend**. Pour que le **workspace** reste vert (pas seulement `-p memless-engine`), **PR2 embarque
l'adaptation mécanique des sites d'appel de capi** : `run_execute.rs` appelle `application::execute`,
`run_query.rs` appelle `application::query(parse, instance, sql)` (par `with_instance`). Le fond capi
riche (reconnaissance C des verbes déjà acquise via `execute`, `ABI_VERSION = 4`, en-tête,
`tests/contract.rs`) reste en **PR3**. Sans cette adaptation, PR2 n'est pas fusionnable seule.

`lib.rs` réexporte `execute`, `Instance`, `Statement`.

**Ne fait pas** : n'avance **jamais** la mémoire avant le succès disque ; ne connaît pas la
concurrence (l'état de travail est un état d'instance, la sérialisation est à l'appelant, décision 7).

## Tests (avant le code)

- **`tests/sql.rs`** étendu — `BEGIN`/`START TRANSACTION`/`COMMIT`/`ROLLBACK` nus **et** `BEGIN WORK`/
  `COMMIT WORK`/`ROLLBACK TRANSACTION` → les verbes ; `BEGIN ISOLATION LEVEL SERIALIZABLE`, `COMMIT
  AND CHAIN`, **`END`**, `ROLLBACK TO s`, `SAVEPOINT s` → Q2 (T3/hors sous-ensemble). Un modificateur
  refusé par le dialecte (`BEGIN DEFERRED`) → Q1.
- **`tests/transaction.rs`** d'intégration (via un `replace` de test capturant le texte) — le
  **virement** : `BEGIN`, deux `UPDATE`, une **lecture qui voit ses écritures**, `COMMIT` → un seul
  fichier réécrit **à l'octet**, `affected` par écriture. **Abandon** : `BEGIN`, `UPDATE`, `ROLLBACK`
  → fichier inchangé, une lecture après abandon voit l'état d'avant. **Instruction refusée** :
  `UPDATE` sur table absente en transaction → Q3, la transaction **reste ouverte** (une écriture
  suivante réussit). **État intermédiaire invalide toléré** : dans une transaction, supprimer deux
  lignes qui se référencent puis `COMMIT` → accepté (décision 15). **Validation échouée** : un état
  final à relation cassée → `COMMIT` échoue `BrokenRelation`, transaction fermée, mémoire à l'état
  d'avant. **Échec disque** : un `replace` qui échoue au `COMMIT` → `DiskWriteFailed`, transaction
  fermée, mémoire intacte, une nouvelle transaction possible. **Gardes** : deuxième `BEGIN` → T1 ;
  `COMMIT`/`ROLLBACK` sans transaction → T2.
