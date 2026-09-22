---
type: conception
titre: Palier 4 — Transiger
slug: palier-4-transiger
cree_le: 2026-09-22T01:45:00+0000
mis_a_jour_le: 2026-09-22T01:45:00+0000
branche: main
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 4 — Transiger](../../cadrage/2026-09-22-palier-4-transiger.md) › **conception — Palier 4 — Transiger** › [plan — Palier 4 — Transiger](../../plans/2026-09-22-palier-4-transiger/00-index.md)  
**Parties** : [01 — Domaine : scission apply/verify, refus de transaction (`memless-domain`)](01-domaine.md) · [02 — État de travail et cas d'usage (`memless-engine`)](02-moteur.md) · [03 — Surface C ABI](03-capi.md) · [04 — Ponts, parité, banc et décisions](04-ponts-parite-banc.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/cadrage/2026-09-22-palier-4-transiger.md

# Conception — Palier 4 « Transiger »

Fige, **avant toute ligne de code**, chaque fichier ajouté ou touché, sa surface exacte, ses refus
et ce qu'il ne fait pas. Le périmètre est celui du cadrage : **grouper** des écritures en une
**transaction** tout ou rien, **lire ses propres écritures**, **valider** (une seule réécriture par
substitution) ou **abandonner**, avec toutes les issues (instruction refusée → ouverte ; validation
échouée → refermée, état d'avant). La stratégie est tranchée (cadrage D1–D8, décision Fable du
2026-09-22).

## Principes hérités (rappel, non rouverts)

- **Domaine pur** ; **1 type public/fichier ; ≤ 10 instructions ; gardes ; aucun commentaire ;
  anglais** (garde Charpente). **`sqlparser` reste dans `sql/`** de `memless-engine` ; l'écrivain par
  substitution du palier 3 est réutilisé **tel quel**.
- **Exécution par copie** (palier 3, C-1) : une écriture s'applique sur une **copie** de `Base`. La
  transaction en fait son **état de travail** : la copie vit tant que la transaction est ouverte.
- **La contrainte maîtresse** (§8.6) : *la mémoire n'est jamais avancée avant le point de
  substitution*. À la validation, on vérifie la structure d'ensemble de l'état de travail, on réécrit
  le fichier, et **seulement ensuite** on substitue l'état de travail à l'état en mémoire.
- **Le report de la vérification** (décision 15) : en transaction, on **applique sans vérifier**
  l'unicité (W8) et les relations (W9) ; c'est la **validation** qui les vérifie. L'`id` (W6/W7), la
  forme (W4/W5) et les noms (Q3/Q4) restent prononcés **à l'écriture** (cadrage point 6).

## Les parties

- [01 — Domaine : scission apply/verify, refus de transaction (`memless-domain`)](01-domaine.md) —
  `Base::apply_write` (apply sans verify), `Base::verify_state` (structure d'ensemble), `TransactionRefusal`.
  Porte **PR1**.
- [02 — État de travail et cas d'usage (`memless-engine`)](02-moteur.md) — `Statement::Begin/Commit/
  Rollback`, `Instance` gagne l'état de travail, `application::{open,apply_write,validate,abandon}`, la
  lecture de ses propres écritures. Porte **PR2**.
- [03 — Surface C ABI](03-capi.md) — `memless_execute` reconnaît les verbes, `memless_query` consulte
  l'état de travail, `ABI_VERSION = 4`. Porte **PR3**.
- [04 — Ponts, parité, banc et décisions](04-ponts-parite-banc.md) — `begin`/`commit`/`rollback`
  PHP/Go, la batterie de transactions octet-près, le banc, les choix C-*/N-*. Porte **PR4, PR5, PR6**.

## Convention de renvoi

« cadrage D<n> » / « T<n> » / « W<n> » renvoie au cadrage ; « décision <n> » / « §<n> » au **brief** ;
« palier 3 C-<n> » à la conception du palier 3 ; « C-<n> »/« N-<n> » à la partie 04. Les quatre
**portes** d'un crate : `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build`.
Code et messages en **anglais**.
