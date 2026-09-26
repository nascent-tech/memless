---
type: mvp
titre: Prouver la promesse de Memless
cree_le: 2026-09-21T14:08:56+0000
mis_a_jour_le: 2026-09-26T09:04:58+0000
branche: docs/deliver-all
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../brief/00-index.md) › **mvp — Prouver la promesse de Memless** › [cadrage — Palier 1 — Charger et refuser](../cadrage/2026-09-21-palier-1-charger-refuser.md) › [conception — Palier 1 — Charger et refuser](../conceptions/2026-09-21-palier-1-charger-refuser/00-index.md) › [plan — Palier 1 — Charger et refuser](../plans/2026-09-21-palier-1-charger-refuser/00-index.md)  
**Aval** : [cadrage — Palier 1 — Charger et refuser](../cadrage/2026-09-21-palier-1-charger-refuser.md) · [cadrage — Palier 2 — Interroger](../cadrage/2026-09-21-palier-2-interroger.md) · [cadrage — Palier 3 — Écrire](../cadrage/2026-09-21-palier-3-ecrire.md) · [cadrage — Palier 4 — Transiger](../cadrage/2026-09-22-palier-4-transiger.md) · [cadrage — Palier 5 — Recharger, trier et tenir trois ponts](../cadrage/2026-09-26-palier-5-recharger-trier.md) · [cadrage — Palier 6 — Publier sur les registres](../cadrage/2026-09-26-palier-6-publier.md)  
**Parties** : [1. Le geste qui prouve](01-le-geste-et-le-tri.md) · [4. Ce qu'on accepte de faire mal](02-faiblesses-et-garanties.md) · [6. L'ordre, et ce que chaque étape rend démontrable](03-paliers-fini-trompe.md)  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

# Memless — MVP

Depuis : .charpente/brief/00-index.md

Ce MVP découpe le brief validé (`.charpente/brief/00-index.md`) : tout ce qu'il retient vient des §8
et §9 du brief, rien n'y est ajouté, et il ne dit ni avec quoi ni comment construire — le plan et
l'architecture le portent. Le brief laisse peu de gras à couper : le MVP retient **dix fonctionnalités
sur dix**. Sa légèreté vient du fichier de démonstration et du sous-ensemble SQL, pas du nombre de
fonctionnalités.

C'est l'expérience la moins chère qui prouve ou réfute la promesse du §1 : **un seul fichier, uniquement
des données, que trois langages peuvent interroger et modifier de façon identique — sans jamais avoir à
décrire la forme des tables à part.** Elle la réduit à un sous-ensemble SQL limité au geste et à un
fichier de quelques lignes — et dit, pour chaque réduction, la condition observable qui la lève.

## Sommaire

1. [Le geste et le tri](01-le-geste-et-le-tri.md) — §1 le geste qui prouve · §2 ce qui entre · §3 ce
   qui sort, et à quelle condition ça rentre.
2. [Faiblesses et garanties](02-faiblesses-et-garanties.md) — §4 ce qu'on accepte de faire mal · §5 ce
   qui ne se coupe jamais.
3. [Paliers, fini, trompé](03-paliers-fini-trompe.md) — §6 l'ordre et ce que chaque étape rend
   démontrable · §7 ce qui dit que c'est livré · §7 bis ce qui dit que c'est prouvé · §8 ce qui dit
   qu'on s'est trompé.
