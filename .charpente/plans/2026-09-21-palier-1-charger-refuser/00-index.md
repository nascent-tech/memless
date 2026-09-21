---
type: plan
titre: Palier 1 — Charger et refuser
slug: palier-1-charger-refuser
cree_le: 2026-09-21T16:27:03+0000
mis_a_jour_le: 2026-09-21T16:43:42+0000
branche: main
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 1 — Charger et refuser](../../cadrage/2026-09-21-palier-1-charger-refuser.md) › [conception — Palier 1 — Charger et refuser](../../conceptions/2026-09-21-palier-1-charger-refuser/00-index.md) › **plan — Palier 1 — Charger et refuser**  
**Parties** : [01 — Fichiers figés et points tranchés par le plan](01-fichiers-figes.md) · [02 — PR1 : le domaine (`memless-domain`)](02-pr1-domaine.md) · [03 — PR2 : le moteur et la batterie (`memless-engine`)](03-pr2-moteur-et-batterie.md) · [04 — PR3, PR4, PR5 : la surface native et les ponts](04-capi-et-ponts.md) · [05 — PR6, découpage et hors périmètre](05-pr6-parite-et-decoupage.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/conceptions/2026-09-21-palier-1-charger-refuser/00-index.md

# Plan — Palier 1 « Charger et refuser »

Ce plan **répartit en phases et en pull requests** les fichiers que la conception a
figés — il ne rouvre ni le modèle, ni une frontière, ni un port, ni une signature, ni un
refus : tout cela est validé (conception, statut `valide`). Il ajoute seulement ce qu'un
plan doit trancher : l'**ordre d'écriture** vertical, les **critères vérifiables par
commande**, la **liste des tests** (un refus par ligne, avant le code), le **découpage en
PR** avec dépendances et parallélisme, et les **trois points** que la conception lui a
explicitement renvoyés (point d'entrée `serde-saphyr` et `Options` de l'analyseur saturées ;
outillage de test PHP ; versions et éditions figées).

Le périmètre reste celui de la conception : **charger** un YAML valide et le **refuser**
sinon, depuis **PHP puis Go**, d'une seule voix. Ni requête SQL, ni écriture, ni
transaction, ni pont Node.

## La voie retenue : 6 pull requests, 4 sessions

La conception concluait « 6 PR, 3 sessions ». Le découpage technique **maintient les 6 PR et
leur ordre**, mais retient **4 sessions** : PR1 et PR2 sont
chacune à la limite d'une session et se mènent en **deux phases vertes** sans se scinder en
deux PR.

| Ordre | PR | Périmètre | Dépend de | Session |
|---|---|---|---|---|
| 1 | **PR1** | workspace (1 membre) + `.charpente.json` + `memless-domain` | — | S1 |
| 2 | **PR2** | `memless-engine` (application + `yaml`) + la batterie de fixtures | PR1 | S2 |
| 3 | **PR3** | `memless-capi` (contrat C ABI publié) | PR2 | S3 |
| 4 | **PR4** | pont PHP (`bindings/php`) | PR3 | S3 |
| 4 | **PR5** | pont Go (`bindings/go`) | PR3 | S4 |
| 5 | **PR6** | lanceur de parité (`harness/parity`) | PR4 **et** PR5 | S4 |

Ordre imposé : **PR1 → PR2 → PR3 → {PR4 ∥ PR5} → PR6**. PR4 et PR5 sont d'ordre libre
(chaînes d'outils et fichiers disjoints) ; une seule développeuse les mène l'une après
l'autre, PHP d'abord (l'hypothèse §17.1 la plus risquée). Détail, critères de fusion
autonomes et couplages en [partie 05](05-pr6-parite-et-decoupage.md).

## Les parties

- [01 — Fichiers figés et points tranchés par le plan](01-fichiers-figes.md) — la liste
  gelée par chemin exact, les versions et éditions, le point d'entrée `serde-saphyr` et les
  `Options` saturées, l'outillage de test PHP, les trois ajustements de contenu du découpage.
- [02 — PR1 : le domaine (`memless-domain`)](02-pr1-domaine.md) — phases 1a (types et
  textes de refus) et 1b (forme et agrégat), tests avant code, quatre portes.
- [03 — PR2 : le moteur et la batterie (`memless-engine`)](03-pr2-moteur-et-batterie.md) —
  phases 2a (application et lecture YAML) et 2b (les 18 fixtures et le golden des textes).
- [04 — PR3, PR4, PR5 : la surface native et les ponts](04-capi-et-ponts.md) —
  `memless-capi`, puis les ponts PHP et Go.
- [05 — PR6, découpage et hors périmètre](05-pr6-parite-et-decoupage.md) — le lanceur de
  parité, le découpage en PR avec dépendances et parallélisme, ce qui reste dehors.

## Convention de renvoi

Les renvois suivent la conception : « conception §X » renvoie à ses parties (01 à 06) ;
« C-<n> » et « N-<n> » à sa partie 06 ; « cadrage D<n> » aux décisions restantes du
cadrage. Un « §X » **seul** renvoie au **brief** ; « archi §X » à l'`ARCHITECTURE.md` ;
« MVP §X » au MVP. Les quatre **portes** sont, pour un crate Cargo : `cargo check`, `cargo clippy
-- -D warnings`, `cargo test`, `cargo build` ; les chaînes `composer`, `go` et `sh` portent
les leurs (partie 01). Le code, les identifiants et les messages sont en **anglais** ; le
reste, en français.
