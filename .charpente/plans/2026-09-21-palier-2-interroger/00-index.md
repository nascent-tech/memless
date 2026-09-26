---
type: plan
titre: Palier 2 — Interroger
slug: palier-2-interroger
cree_le: 2026-09-21T19:50:00+0000
mis_a_jour_le: 2026-09-26T11:36:49+0000
branche: main
statut: perime
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 2 — Interroger](../../cadrage/2026-09-21-palier-2-interroger.md) › [conception — Palier 2 — Interroger](../../conceptions/2026-09-21-palier-2-interroger/00-index.md) › **plan — Palier 2 — Interroger**  
**Parties** : [01 — Phases, critères et tests par PR](01-phases-et-tests.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/conceptions/2026-09-21-palier-2-interroger/00-index.md

# Plan — Palier 2 « Interroger »

Répartit en phases et pull requests les fichiers **déjà figés** par la conception — il ne
rouvre ni le modèle, ni un refus, ni une signature. Il ajoute l'**ordre d'écriture** vertical,
les **critères vérifiables**, la **liste des tests** (un refus par ligne, avant le code), et le
**découpage en PR**.

Périmètre : **lire** un état chargé par un `SELECT` du sous-ensemble (cadrage D1) et le
**refuser** (Q1–Q8), depuis PHP puis Go. Ni écriture, ni transaction, ni pont Node.

## La voie retenue : 6 pull requests

| Ordre | PR | Périmètre | Dépend de | Phases |
|---|---|---|---|---|
| 1 | **PR1** | `memless-domain` : modèle `query/`, `Rows`, `QueryRefusal`, `scalar_order`, exécution `base/select/` | — | 1a modèle+refus, 1b exécution |
| 2 | **PR2** | `memless-engine` : adaptateur `sql/` (sqlparser), `application::query`, fixtures + batterie | PR1 | 2a analyse, 2b cas d'usage+batterie |
| 3 | **PR3** | `memless-capi` : `memless_query` + accesseurs de résultat, en-tête, `ABI_VERSION=2` | PR2 | une phase |
| 4 | **PR4** | pont PHP `query` | PR3 | une phase |
| 4 | **PR5** | pont Go `Query` | PR3 | une phase |
| 5 | **PR6** | lanceur de parité : `queries.txt`, pilotes, `run.sh`, régression | PR4 **et** PR5 | une phase |

Ordre imposé : **PR1 → PR2 → PR3 → {PR4 ∥ PR5} → PR6**. PR4 et PR5 sont d'ordre libre (fichiers
et chaînes d'outils disjoints).

## Les parties

- [01 — Phases, critères et tests par PR](01-phases-et-tests.md) — l'ordre d'écriture, les
  quatre portes par phase, et la liste des tests avant le code.

## Convention de renvoi

« conception §X » renvoie aux parties 01–04 de la conception ; « C-<n> »/« N-<n> » à sa partie
04 ; « cadrage D<n> » aux décisions du cadrage ; « Q<n> » aux refus du cadrage point 6. Les
quatre **portes** d'un crate Cargo : `cargo check`, `cargo clippy -- -D warnings`, `cargo test`,
`cargo build` ; PHP porte `composer test`, Go `go test`/`go vet`, la parité `sh run.sh`. Code et
messages en **anglais**.
