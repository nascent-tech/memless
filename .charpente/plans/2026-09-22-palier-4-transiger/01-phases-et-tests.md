# 01 — Phases, critères et tests par PR

<!-- charpente-navigation -->
**Index** : [plan — Palier 4 — Transiger](00-index.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Une phase = une pull request. Chaque phase donne son périmètre (fichiers figés par la conception),
ses **tests d'abord**, ses **critères vérifiables**, ses **portes**. Statut par phase tenu à jour.

## PR1 — domaine : scission apply/verify, refus de transaction

**Statut : À faire.** Depuis conception §01. Crate `memless-domain`. **Dépend de** : — (part du tip
`palier-3-pr7-parite`).

Périmètre : `Base::apply_write` (expose l'`apply` interne, sans `verify` ; nom évitant « stager ») ;
`Base::verify_state` (méthode publique sur `base/verify.rs`, W8/W9, mappe en `Refusal`) ; `Base::write`
= `apply_write` + `verify_state` (comportement inchangé) ; `refusal/transaction/` (`TransactionRefusal
{ AlreadyOpen, NoOpenTransaction }`, `Refusal::Transaction`, `From`, branche `Display` mise à jour) ;
réexport `TransactionRefusal`. **Pas** les variantes `Statement` (elles rendraient les `match` de
l'engine non exhaustifs → PR2).

Tests d'abord : `tests/apply_write.rs` (`apply_write` applique sans vérifier — id dupliqué / relation
cassée passent ; W6 échoue quand même ; `verify_state` prononce W8/W9 sur l'état entier) ;
`tests/transaction_refusal.rs` (gabarits) ; la batterie `tests/write.rs` du palier 3 reste verte.

Critères : `apply_write(insert dupliquant un id)` rend `Ok` ; `verify_state` sur ce résultat rend
`DuplicateId` ; `write` (palier 3) refuse toujours le même insert ; **le workspace entier compile**
(les paliers 2–3 sont sous la branche de départ).

Portes : les quatre **au workspace** (`--all-targets`).

## PR2 — moteur : grammaire, état de travail, cas d'usage

**Statut : À faire.** Depuis conception §02. Crate `memless-engine`. **Dépend de** : PR1.

Périmètre : `query/statement.rs` gagne `Begin`/`Commit`/`Rollback` (+ les `match` de l'engine mis à
jour, même PR) ; `sql/lower_statement` aiguille `StartTransaction`/`Commit`/`Rollback` avec le rejet
exact (`modes`/`modifier` pour begin ; `chain`/`end`/`modifier` pour commit — **`END` refusé** ;
`chain`/`savepoint` pour rollback) ; `sql/reject_transaction_options` ; `sql/statement_label` gagne
`SAVEPOINT`/`RELEASE` ; `application/instance.rs` gagne `transaction: Option<Base>` ;
`application/execute.rs` (aiguille Begin/Commit/Rollback/Write/Select) ; `application/{open,apply_write,
validate,abandon}.rs` (ordre strict, `take` en tête de `validate` ; le chemin isolé = corps du palier
3 tel quel, étape « sans changement » comprise) ; `application/query.rs` lit `transaction.unwrap_or(
base)` et refuse un non-`SELECT` ; `lib.rs` réexporte `execute` (remplace `write`). **Adaptation des
appelants capi dans cette PR** : `run_execute.rs` → `application::execute`, `run_query.rs` →
`application::query(instance)` (frontière de compilation — sinon `memless-capi` casse au workspace).

Tests d'abord : `tests/sql.rs` (verbes nus → `Statement`, options → Q2) ; `tests/transaction.rs`
(virement octet-près, abandon, lecture de ses écritures, instruction refusée laisse ouverte, état
intermédiaire invalide toléré, validation échouée referme+mémoire d'avant, échec disque referme,
gardes T1/T2).

Critères : après un `COMMIT` d'un virement, le fichier est réécrit **une** fois à l'octet attendu ;
après un `ROLLBACK`, le fichier est inchangé et une lecture rend l'état d'avant ; un `replace` qui
échoue au `COMMIT` laisse `instance.base` inchangé et permet un nouveau `BEGIN`.

Portes : les quatre **au workspace** (`--all-targets`).

## PR3 — surface C ABI

**Statut : À faire.** Depuis conception §03. Crate `memless-capi`. **Dépend de** : PR2.

Périmètre : `ABI_VERSION = 4` ; en-tête `include/memless.h` étendu (verbes acceptés, read-your-writes,
T1/T2 → `Refused`). Le **recâblage** `run_execute`→`execute` / `run_query`→`query(instance)` est
**déjà en PR2** (frontière de compilation) ; PR3 n'ajoute que la version, l'en-tête et le contrat.

Tests d'abord : `tests/contract.rs` (`abi_version == 4` ; transaction complète par le handle avec
read-your-writes ; abandon ; gardes T1/T2 `Refused` ; validation échouée `Refused` ; sur copie de
fixture).

Critères : `memless_execute(h,"BEGIN")` puis un `UPDATE` puis `memless_query(h,…)` voit l'écriture
non validée ; `memless_execute(h,"COMMIT")` réécrit une fois ; deuxième `BEGIN` → `Refused`.

Portes : les quatre **au workspace** (`--all-targets`) ; advisory `avoidUnsafeBlocks` toléré.

## PR4 — pont PHP ∥ PR5 — pont Go

**Statut : À faire.** Depuis conception §04. `bindings/php` / `bindings/go`. **Dépend de** : PR3.
**Parallèles.**

- **PR4 PHP** : `Instance::begin/commit/rollback` (façades sur `Execute::run`) ; `Library::ABI_VERSION
  = 4` ; `tests/TransactionTest.php` (virement accepté + fichier réécrit, abandon, lecture de ses
  écritures, T1/T2). Portes : `phpunit` ; `php -l`.
- **PR5 Go** : `Instance.Begin/Commit/Rollback` (méthodes sur `Execute`) ; `abiVersion = 4` ;
  `transaction_test.go` (mêmes cas). Portes : `go test ./...` ; `go vet ./...` ; `gofmt -l`.

## PR6 — parité et banc

**Statut : À faire.** Depuis conception §04 (parité/banc). `harness/parity`, `harness/bench`.
**Dépend de** : PR4 **et** PR5.

Périmètre : `parity/transactions.txt` (une transaction par ligne, `;;` entre instructions, `!disk`) ;
**mode transaction** des pilotes `go/main.go` et `php/load.php` (copie tmp, exécute la suite, imprime
chaque issue + SHA-256 + résidu) ; `run.sh` (boucle de transactions, octets, `!disk` par `chmod 0555`,
saut root) ; `tests/detects-transaction-divergence.sh` ; `examples/bench_transaction.rs` +
relevé dans `.charpente/releves/banc.md` (une transaction paie un `fsync`, non k).

Tests d'abord : `detects-transaction-divergence.sh` échoue avec un pilote trafiqué ; la boucle
`transactions.txt` passe sur les deux ponts.

Critères : `sh run.sh` imprime l'accord sur toutes les transactions (issues **et** octets) ;
`banc.md` porte un relevé daté. Le statut du document passe à `livre` (posé par build en PR6).

Portes : `sh harness/parity/run.sh` (exit 0) ; `bash harness/bench/run.sh` + relevé ; les quatre
portes Cargo **au workspace**.

## Ordre et parallélisme

PR1 → PR2 → PR3 → {PR4 ∥ PR5} → PR6. PR4 et PR5 ne dépendent que de PR3 (langages disjoints) ; dans
la pile, PR5 part de PR3 et se rebase sur PR4 fusionnée, PR6 part de ce tip (voir « Prérequis » à
l'index). PR1 part du tip `palier-3-pr7-parite`.
