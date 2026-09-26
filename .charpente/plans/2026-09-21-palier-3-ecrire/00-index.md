---
type: plan
titre: Palier 3 — Écrire
slug: palier-3-ecrire
cree_le: 2026-09-21T21:50:00+0000
mis_a_jour_le: 2026-09-26T11:36:49+0000
branche: main
statut: perime
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 3 — Écrire](../../cadrage/2026-09-21-palier-3-ecrire.md) › [conception — Palier 3 — Écrire](../../conceptions/2026-09-21-palier-3-ecrire/00-index.md) › **plan — Palier 3 — Écrire**  
**Parties** : [01 — Phases, critères et tests par PR](01-phases-et-tests.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/conceptions/2026-09-21-palier-3-ecrire/00-index.md

# Plan — Palier 3 « Écrire »

Répartit en phases et pull requests les fichiers **déjà figés** par la conception. Il ne rouvre ni
le modèle, ni un refus, ni une signature. Périmètre : **modifier** l'état (`INSERT`/`UPDATE`/`DELETE`
isolés), **réécrire** le fichier par substitution, **refuser** (W1–W10), **échec disque** sans perte
mémoire. Ni transaction multi-instructions, ni pont Node.

## La voie retenue : 7 pull requests

| Ordre | PR | Périmètre | Dépend de |
|---|---|---|---|
| 1 | **PR1** domaine | `query/{write,insert,update,delete,statement}`, `refusal/write/`, `base/filter/` (déplacé), `base/write/`, `base/verify` (scindé de `load`), `base/render/`, `Clone`/`PartialEq` | — |
| 2 | **PR2** écrivain | `yaml/render/` (maison, `quote.rs`), `yaml/writer/` (résidu+fsync+rename), `application/replace_file.rs`, `application/instance.rs`, `load`→`Instance` | PR1 |
| 3 | **PR3** moteur SQL | `sql/lower/{statement,insert,update,delete}`, `parse`→`Statement`, `application::write`, `application::query`/`load` adaptés | PR2 |
| 4 | **PR4** C ABI | `memless_execute`, `Instance` dans `instances.rs` (`with_instance`/`_mut`), en-tête, `ABI_VERSION=3` | PR3 |
| 5 | **PR5** PHP ∥ **PR6** Go | `execute` dans chaque pont, tests miroir (accepté/compte, refusé, disque) | PR4 |
| 6 | **PR7** parité + banc | `writes.txt`, pilotes mode écriture (copie tmp, SHA-256, résidu), `run.sh` (octets, `!disk`), régression ; `bench_write` + premier relevé | PR5 **et** PR6 |

Ordre imposé PR1 → PR2 → PR3 → PR4 → {PR5 ∥ PR6} → PR7. **PR2 est séparée de PR3** : l'écrivain
par substitution est la pièce la plus risquée, fusionnable et prouvable seule (aller-retour, golden,
disque). Le statut `livre` est posé en PR7.

## Les parties

- [01 — Phases, critères et tests par PR](01-phases-et-tests.md)

## Convention de renvoi

« conception §X » aux parties 01–04 ; « C-<n> »/« N-<n> » à la partie 04 ; « cadrage D<n> »/« W<n> »
au cadrage ; « décision <n> »/« §<n> » au brief. Quatre **portes** Cargo par crate ; PHP
`composer test`, Go `go test`/`go vet`, parité `sh run.sh`, banc `sh harness/bench/run.sh` + relevé.
Code et messages en **anglais**.
