---
type: conception
titre: Palier 3 — Écrire
slug: palier-3-ecrire
cree_le: 2026-09-21T21:45:00+0000
mis_a_jour_le: 2026-09-26T11:36:49+0000
branche: main
statut: perime
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 3 — Écrire](../../cadrage/2026-09-21-palier-3-ecrire.md) › **conception — Palier 3 — Écrire** › [plan — Palier 3 — Écrire](../../plans/2026-09-21-palier-3-ecrire/00-index.md)  
**Parties** : [01 — Domaine : modèle, exécution, contraintes, rendu (`memless-domain`)](01-domaine.md) · [02 — L'écrivain YAML par substitution (`memless-engine`)](02-ecrivain.md) · [03 — Moteur SQL, cas d'usage et surface native](03-moteur-capi.md) · [04 — Ponts, parité, banc et décisions](04-ponts-parite-banc.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/cadrage/2026-09-21-palier-3-ecrire.md

# Conception — Palier 3 « Écrire »

Fige, **avant toute ligne de code**, chaque fichier ajouté ou touché, sa surface exacte, ses
refus et ce qu'il ne fait pas. Le périmètre est celui du cadrage : **modifier** l'état par du SQL
(`INSERT`/`UPDATE`/`DELETE` isolés), **réécrire** le fichier par substitution, **refuser** (W1–W10),
et l'**échec disque** sans perte mémoire. La stratégie est tranchée (cadrage D1–D8, décision
Fable du 2026-09-21).

## Principes hérités (rappel, non rouverts)

- **Domaine pur** ; **1 type public/fichier ; ≤ 10 instructions ; gardes ; aucun commentaire ;
  anglais** (garde Charpente). **`sqlparser` reste dans `sql/`** de `memless-engine`.
- **Refus typés par couche** : `WriteRefusal` rejoint `Refusal` par une variante `Write` ; les
  autres refus **réutilisent** `QueryRefusal` (syntaxe/nom, palier 2) et `StructureRefusal`
  (contraintes, palier 1) — mêmes types, mêmes messages.
- **Ports fonction** : `ReplaceFile = fn(&str, &str) -> Result<(), io::Error>` câblé par
  `memless-capi`, même patron que `ReadSource`/`ParseSql`.
- **La contrainte maîtresse** (§8.6) : *la mémoire n'est jamais avancée avant le point de
  substitution*. On applique sur une **copie** du `Base`, on vérifie, on réécrit le fichier, et
  **seulement ensuite** on substitue la copie à l'état en mémoire.

## Les parties

- [01 — Domaine : modèle, exécution, contraintes, rendu (`memless-domain`)](01-domaine.md) — le
  modèle d'écriture, `WriteRefusal`, le déplacement de l'évaluation de filtre, `Base::write` par
  copie, `Base::verify` scindé de `load`, `Base::document`. Porte **PR1**.
- [02 — L'écrivain YAML par substitution (`memless-engine`)](02-ecrivain.md) — `yaml/render` maison,
  `yaml/writer` (résidu + fsync + substitution), le port `ReplaceFile`, `Instance` (avec adaptation
  des appelants existants de `load`). Porte **PR2**.
- [03 — Moteur SQL, cas d'usage et surface native](03-moteur-capi.md) — la descente
  `INSERT`/`UPDATE`/`DELETE`, `application::write`, `memless_execute` (`ABI_VERSION = 3`). Porte
  **PR3** et **PR4**.
- [04 — Ponts, parité, banc et décisions](04-ponts-parite-banc.md) — `execute` PHP/Go, la batterie
  d'écritures et la comparaison octet-près, le banc §17.2, les choix C-*/N-*. Porte **PR5, PR6, PR7**.

## Convention de renvoi

« cadrage D<n> » / « W<n> » renvoie au cadrage ; « décision <n> » / « §<n> » au **brief** ; « MVP
§<n> » au MVP ; « palier 2 C-<n> » à la conception du palier 2 ; « C-<n> »/« N-<n> » à la partie
04. Les quatre **portes** d'un crate : `cargo check`, `cargo clippy -- -D warnings`, `cargo test`,
`cargo build`. Code et messages en **anglais**.
