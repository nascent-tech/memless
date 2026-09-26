---
type: releve
titre: Journal des retours — §17.3
cree_le: 2026-09-22T00:00:00+0000
statut: vivant
---

# Journal des retours (§17.3)

La trace qui écoute les développeuses au lieu de deviner à leur place. Ouvert au palier 1, alimenté
aux séances du §7 (MVP §6). Il porte deux hypothèses du brief, chacune avec le seuil qui fait
changer de route :

- **§17.3 (a) — les développeuses acceptent qu'un test réécrive le fichier pointé.**
- **§17.3 (b) — les relations devinées par le seul nom sont acceptées.**

**Une entrée par développeuse et par séance** : date, événement, verbatim, fichier concerné (MVP
§8). Quand un refus est en cause, le consigner **tel que le moteur le nomme** (règle, table, ligne —
`ARCHITECTURE.md` §10), à côté de l'entrée.

Ce qui **ne compte pas** comme perte : le bruit d'une **première** réécriture de normalisation
(commentaires, mise en forme, `null`, forme d'un décimal — décision 26). Une fixture **sans** relation
devinée n'est pas un échec : c'est elle-même un retour sur §17.3 (b), à noter.

## §17.3 (a) — la réécriture est-elle acceptée

Ce qu'on observe pour chaque développeuse : a-t-elle accepté de pointer Memless sur ses vraies
fixtures ; a-t-elle vécu une réécriture comme une perte ; a-t-elle demandé un mode lecture seule
(décision 3).

**Le seuil qui fait changer de route :** *1* modification non committée écrasée par une réécriture
(une vraie perte, puisqu'une fixture suivie par Git est sinon restaurable), **ou** *au moins deux
des trois* développeuses qui refusent de pointer Memless sur leurs vraies fixtures ou demandent un
mode lecture seule. Route alors : revoir la décision 3.

| Date | Développeuse | A pointé ses vraies fixtures ? | Réécriture vécue comme perte ? | A demandé un mode lecture seule ? | Fichier concerné | Verbatim |
|---|---|---|---|---|---|---|
| _(à remplir)_ | | | | | | |

## §17.3 (b) — les relations devinées sont-elles acceptées

Ce qu'on observe sur les vraies fixtures : **fausses détections** (chargement refusé, ou suppression
refusée, sur une colonne `_id` qui n'est pas une relation — ex. `stripe_id`) et **relations
manquées** (ligne orpheline passée, pluriel irrégulier — jamais un refus, mais un oubli constaté
après coup).

**Le seuil qui fait changer de route :** *1* fichier réel refusé au chargement par fausse détection
(le pire cas nommé au §17.3), **ou** *1* relation manquée constatée après coup. Route alors : faire
entrer l'inventaire ou la déclaration explicite des relations (roadmap §12.1) au lieu de poursuivre
en convention seule.

| Date | Développeuse | Type (fausse détection / relation manquée / aucune relation) | Colonne ou tables | Refus tel que nommé par le moteur | Fichier concerné | Verbatim |
|---|---|---|---|---|---|---|
| _(à remplir)_ | | | | | | |

## Verdict §17.3

- **(a) Statut :** ouvert. **Bascule quand :** une perte non committée, ou deux développeuses sur
  trois qui refusent la réécriture ou réclament la lecture seule.
- **(b) Statut :** ouvert. **Bascule quand :** un fichier réel refusé par fausse détection, ou une
  relation manquée constatée après coup.
