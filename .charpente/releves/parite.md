---
type: releve
titre: Relevé de parité — §17.1
cree_le: 2026-09-22T00:00:00+0000
statut: vivant
---

# Relevé de parité (§17.1)

La trace qui tranche l'hypothèse la plus risquée du brief : **PHP atteint une parité réelle avec
Go, et maintenant avec Node**. Ouvert au palier 1, complété au palier 4 (MVP §6), étendu au pont
Node. Il consigne, pour **chaque geste de la batterie × chaque langage × chaque famille de
systèmes**, ce que le brief demande de comparer : **le résultat, le message d'erreur, et
l'empreinte du fichier réécrit** (MVP §8, §17.1).

La batterie a quatre parts (voir `harness/parity/README.md`) :

- **charger** — les fixtures de `harness/parity/fixtures/` qui doivent charger ou se refuser (palier 1) ;
- **interroger** — les requêtes de `queries.txt` qui doivent rendre les mêmes lignes ou le même refus (palier 2) ;
- **écrire** — les écritures de `writes.txt` qui doivent s'appliquer, se refuser ou échouer à l'identique et réécrire les mêmes octets (palier 3) ;
- **transiger** — les transactions de `transactions.txt` qui doivent s'accorder instruction par instruction et valider les mêmes octets (palier 4).

Le lanceur `bash harness/parity/run.sh` rejoue chaque part à travers le pont PHP, le pont Go **et**
le pont Node et exige **la même issue et le même message** des trois. Sa sortie est le verdict : soit
`parity: all fixtures, queries, writes and transactions agree on all three bridges` (code 0), soit
une ligne `DIVERGENCE` par problème (code non nul). Recopier ici, à la main, le verdict et la
machine (MVP §8).

## Ce que ce relevé ferme, et ce qu'il ne ferme pas

Zéro divergence sur **une** famille de systèmes montre que les trois ponts parlent d'une seule voix
**sur cette famille**. Cela **ouvre** §17.1, sans la fermer. La fermeture exige : zéro divergence
sur **chaque famille de systèmes** retenue par l'architecture (OS × famille de processeurs), **et**
les séances du §7 (trois développeuses, leurs vraies fixtures, PHP, Go et Node).

**Le seuil qui fait changer de route (MVP §8) :** *1* écart (résultat, message ou fichier) que
l'architecture juge impossible à corriger dans le cœur — c'est-à-dire qui exigerait une décision
dans un pont —, ou *1* geste du §1 inaccessible depuis PHP sans une couche réseau déguisée en natif.
Qui tranche « corrigeable dans le cœur ou non » : l'architecture. Route alors : revoir le cœur ou la
voie d'appel avant tout palier suivant.

## Relevé automatisé — par famille de systèmes

Une ligne par exécution de `run.sh`. « Familles retenues » : à figer par l'architecture (question
ouverte 7 de la roadmap). Verdict : `all agree` ou la ou les lignes de divergence.

| Date | Famille (OS × processeur) | Révision (git) | Verdict | Écart(s) constaté(s) |
|---|---|---|---|---|
| 2026-09-26 | Darwin × arm64 | 114c180 | all fixtures, queries, writes and transactions agree on all three bridges | aucun |

## Séances du §7 — par développeuse

Chaque développeuse, sur **ses vraies fixtures** déjà suivies par Git, joue l'équivalent du geste du
§1 dans les trois langages (MVP §7) : une transaction d'au moins deux écritures validée, une
jointure sur une relation devinée de ses fixtures, un `DELETE` d'une ligne encore référencée. Elle
voit les mêmes résultats dans les trois langages, ouvre `git diff` et n'y trouve que les valeurs
changées, et lit le même message de refus dans les trois. Une entrée par développeuse et par séance.

| Date | Développeuse | Famille (OS × processeur) | Geste joué | Même résultat PHP/Go/Node ? | Même refus PHP/Go/Node ? | Empreinte identique ? | Note |
|---|---|---|---|---|---|---|---|
| _(à remplir)_ | | | | | | | |

## Verdict §17.1

- **Statut :** ouvert.
- **Ferme quand :** zéro écart sur chaque famille retenue **et** les trois séances du §7 accomplies
  sans écart non corrigeable dans le cœur.
- **Bascule quand :** un seul écart non corrigeable dans le cœur, ou un geste du §1 inaccessible
  depuis PHP sans réseau déguisé.
