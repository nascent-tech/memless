---
type: conception
titre: Palier 2 — Interroger
slug: palier-2-interroger
cree_le: 2026-09-21T19:48:00+0000
mis_a_jour_le: 2026-09-21T21:48:18+0000
branche: main
statut: livre
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 2 — Interroger](../../cadrage/2026-09-21-palier-2-interroger.md) › **conception — Palier 2 — Interroger** › [plan — Palier 2 — Interroger](../../plans/2026-09-21-palier-2-interroger/00-index.md)  
**Parties** : [01 — Domaine : modèle, exécution, refus (`memless-domain`)](01-domaine.md) · [02 — Moteur et surface native (`memless-engine`, `memless-capi`)](02-moteur-et-capi.md) · [03 — Ponts et parité (`bindings/php`, `bindings/go`, `harness/parity`)](03-ponts-et-parite.md) · [04 — Décisions et hors périmètre](04-decisions-et-hors-perimetre.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/cadrage/2026-09-21-palier-2-interroger.md

# Conception — Palier 2 « Interroger »

Fige, **avant toute ligne de code**, chaque fichier ajouté ou touché, sa surface exacte, ses
refus et ce qu'il ne fait pas. Le build ne fait que transcrire. Le périmètre est celui du
cadrage : **lire** un état chargé par un `SELECT` du sous-ensemble (cadrage D1), et **refuser**
(Q1–Q8). La stratégie est tranchée (cadrage D3) : `sqlparser` **analyse**, le **domaine
exécute** sous la règle de comparaison unique (décision 34) et l'absence (décision 35).

## Principes hérités (rappel, non rouverts)

- **Domaine pur** : `memless-domain` ne dépend de rien (archi §2). Le modèle de requête, sa
  vérification et son exécution y vivent. `sqlparser` reste **hors** du domaine, dans
  l'adaptateur `sql/` de `memless-engine`.
- **Un type public par fichier ; ≤ 10 instructions par fonction ; contrôle de flux par gardes ;
  aucun commentaire ; identifiants et messages en anglais** (garde Charpente).
- **Refus typés par couche** : `QueryRefusal` rejoint `Refusal` par une variante `Query`, et le
  `Display` unique existant produit le message anglais (parité, décision 25).
- **Port fonction** : `application::query` reçoit un `ParseSql = fn(&str) -> Result<Select,
  QueryRefusal>`, câblé par `memless-capi` — même patron que `ReadSource` au palier 1 (C-5).

## Les parties

- [01 — Domaine : modèle, exécution, refus (`memless-domain`)](01-domaine.md) — le modèle de
  requête, `Rows`, `QueryRefusal`, et l'exécution sur `Base.tables`. Porte **PR1**.
- [02 — Moteur et surface native (`memless-engine`, `memless-capi`)](02-moteur-et-capi.md) —
  l'adaptateur `sql/` sur `sqlparser`, le cas d'usage `query`, puis `memless_query` et les
  accesseurs de résultat. Porte **PR2** et **PR3**.
- [03 — Ponts et parité (`bindings/php`, `bindings/go`, `harness/parity`)](03-ponts-et-parite.md)
  — `query` côté PHP et Go, la batterie de requêtes et le lanceur de parité étendu. Porte
  **PR4, PR5, PR6**.
- [04 — Décisions et hors périmètre](04-decisions-et-hors-perimetre.md) — les points tranchés
  par la conception (C-*, N-*) et ce qui reste dehors.

## Convention de renvoi

« cadrage D<n> » renvoie aux décisions du cadrage ; « décision <n> » et « §<n> » au **brief** ;
« MVP §<n> » au MVP ; « C-<n> »/« N-<n> » aux choix et non-choix de la partie 04. Les quatre
**portes** d'un crate Cargo : `cargo check`, `cargo clippy -- -D warnings`, `cargo test`,
`cargo build`. Code et messages en **anglais** ; le reste en français.
