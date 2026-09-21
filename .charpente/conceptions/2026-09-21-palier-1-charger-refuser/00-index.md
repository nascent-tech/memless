---
type: conception
titre: Palier 1 — Charger et refuser
slug: palier-1-charger-refuser
cree_le: 2026-09-21T15:42:28+0000
mis_a_jour_le: 2026-09-21T16:19:08+0000
branche: main
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../../mvp/00-index.md) › [cadrage — Palier 1 — Charger et refuser](../../cadrage/2026-09-21-palier-1-charger-refuser.md) › **conception — Palier 1 — Charger et refuser** › [plan — Palier 1 — Charger et refuser](../../plans/2026-09-21-palier-1-charger-refuser/00-index.md)  
**Parties** : [01 — Modèle et ports](01-modele-et-ports.md) · [02 — Domaine (`memless-domain`)](02-domaine.md) · [03 — Application et lecture YAML (`memless-engine`)](03-application-et-yaml.md) · [04 — Surface native et ponts](04-capi-et-ponts.md) · [05 — La batterie, le lanceur, l'espace de travail](05-banc-de-parite-et-workspace.md) · [06 — Décisions, non-créés, hors périmètre](06-decisions-et-hors-perimetre.md)  
**Maillage** : [maillage.md](../../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md

# Conception — Palier 1 « Charger et refuser »

Cette conception **fige les fichiers, leurs surfaces publiques et leurs refus**
avant toute ligne de code. Le build ne fera que transcrire : aucune décision n'est
reportée. Le périmètre est celui du cadrage — **charger** un YAML valide et le
**refuser** quand il ne l'est pas, depuis **PHP puis Go**, avec un message
identique. Ni requête SQL, ni écriture, ni transaction, ni pont Node : tout cela
entre aux paliers suivants.

Le dépôt est vierge de code. Cette conception pose donc, au plus petit, le squelette
de l'espace de travail Cargo que le palier exige, et rien de plus : trois crates
Rust (`memless-domain`, `memless-engine`, `memless-capi`), deux ponts (`bindings/php`,
`bindings/go`) et le lanceur de parité (`harness/parity`). GlueSQL, l'écrivain YAML, la
garde SQL et le pont Node **ne sont pas créés** (partie 6).

## Les parties

- [01 — Modèle et ports](01-modele-et-ports.md) — l'agrégat `Base`, ses contraintes,
  ses objets-valeur, et pourquoi le palier n'a **aucun port**.
- [02 — Domaine](02-domaine.md) — `memless-domain` : la représentation d'entrée du
  fichier lu, le scalaire et sa règle de comparaison, l'`id`, la relation devinée,
  l'énumération des refus, la vérification de forme, l'agrégat `Base`.
- [03 — Application et lecture YAML](03-application-et-yaml.md) — `memless-engine` : le
  cas d'usage `charger` (une fonction), l'adaptateur YAML, sans racine de composition.
- [04 — Surface native et ponts](04-capi-et-ponts.md) — `memless-capi` (C ABI) et les
  ponts PHP (FFI) et Go (purego).
- [05 — La batterie, le lanceur, l'espace de travail](05-banc-de-parite-et-workspace.md) —
  la batterie de fichiers, le lanceur qui rejoue PHP puis Go, le manifeste du workspace,
  `.charpente.json`.
- [06 — Décisions, non-créés, hors périmètre](06-decisions-et-hors-perimetre.md) — les
  décisions figées et leurs alternatives écartées, ce qui est **choisi de ne pas être
  créé**, la voie retenue vers le plan.

## Convention de renvoi

« §X » / « décision N » renvoient au **brief validé** ; « archi §X » renvoie à
l'`ARCHITECTURE.md` ; « cadrage » renvoie au cadrage amont, et « D<n> » à ses décisions
restantes ; « C-<n> » et « N-<n> » renvoient à la partie 6. Les termes métier
sont ceux du glossaire [le moteur](../../glossaire/le-moteur.md). Le code, les
identifiants et les messages d'erreur sont en **anglais** (règle de langage) ; le
reste, en français.
