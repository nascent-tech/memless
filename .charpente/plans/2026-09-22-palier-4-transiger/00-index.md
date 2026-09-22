---
type: plan
titre: Palier 4 — Transiger
slug: palier-4-transiger
cree_le: 2026-09-22T02:00:00+0000
mis_a_jour_le: 2026-09-22T02:00:00+0000
branche: main
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 4 — Transiger](../../cadrage/2026-09-22-palier-4-transiger.md) › [conception — Palier 4 — Transiger](../../conceptions/2026-09-22-palier-4-transiger/00-index.md) › **plan — Palier 4 — Transiger**  
**Parties** : [01 — Phases, critères et tests par PR](01-phases-et-tests.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/conceptions/2026-09-22-palier-4-transiger/00-index.md

# Plan — Palier 4 « Transiger »

Répartit en phases et pull requests les fichiers **déjà figés** par la conception. Périmètre :
**grouper** des écritures en une transaction, **lire ses propres écritures**, **valider** (une seule
réécriture) ou **abandonner**, avec toutes les issues. Ni transaction imbriquée, ni `SAVEPOINT`, ni
pont Node.

## La voie retenue : 6 pull requests

| Ordre | PR | Périmètre | Dépend de |
|---|---|---|---|
| 1 | **PR1** domaine | `Base::apply_write` (apply sans verify), `Base::verify_state`, `TransactionRefusal`, `Refusal::Transaction` (+ sa branche `Display`). **Pas** les variantes `Statement` (couplage d'exhaustivité → PR2) | — |
| 2 | **PR2** moteur | `Statement::{Begin,Commit,Rollback}` + `sql/` descend les verbes, `Instance` gagne `transaction`, `application::{execute,open,apply_write,validate,abandon}`, `query(instance)` lit l'état de travail, **et l'adaptation des appelants capi** (`run_execute`/`run_query`) pour garder le workspace vert | PR1 |
| 3 | **PR3** C ABI | `ABI_VERSION = 4`, en-tête étendu, `tests/contract.rs` (le recâblage capi est déjà en PR2) | PR2 |
| 4 | **PR4** PHP ∥ **PR5** Go | `begin`/`commit`/`rollback` minces, `ABI_VERSION = 4`, tests miroir | PR3 |
| 5 | **PR6** parité + banc | `transactions.txt`, mode transaction des pilotes, `run.sh`, `detects-transaction-divergence.sh`, `bench_transaction` + relevé | PR4 **et** PR5 |

Ordre imposé PR1 → PR2 → PR3 → {PR4 ∥ PR5} → PR6. Le statut `livre` est posé en PR6.

**Prérequis (branche de départ).** Le code des paliers 2–3 n'est **pas** fusionné dans `main` (pile
`palier-2-pr1-domaine` → … → `palier-3-pr7-parite`). PR1 du palier 4 part donc du tip
**`palier-3-pr7-parite`** ; chaque PR suivante part du tip de la précédente (stack). PR5 (Go) part de
PR3 et se rebase sur PR4 fusionnée ; PR6 part de ce tip. Rebaser la pile sur `main` quand les paliers
antérieurs fusionnent.

**Portes au workspace.** Chaque PR lance les quatre portes **sur tout le workspace**
(`cargo check --all-targets`, `clippy --all-targets`, `test`, `build --release`), pas `-p <crate>` :
c'est la seule façon de voir une cassure inter-crates (un changement de signature de l'engine dont
capi dépend).

## Les parties

- [01 — Phases, critères et tests par PR](01-phases-et-tests.md)

## Convention de renvoi

« conception §X » aux parties 01–04 ; « C-<n> »/« N-<n> » à la partie 04 ; « cadrage D<n> »/« T<n> »/
« W<n> » au cadrage. Quatre **portes** Cargo par crate ; PHP `phpunit`, Go `go test`/`go vet`, parité
`sh run.sh`, banc `bash harness/bench/run.sh` + relevé. Code et messages en **anglais**.
