<!-- charpente-navigation -->
**Index** : [mvp — Prouver la promesse de Memless](00-index.md)  
**Précédent** : [4. Ce qu'on accepte de faire mal](02-faiblesses-et-garanties.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 6. L'ordre, et ce que chaque étape rend démontrable

Tout le MVP vit dans **un seul contexte borné — le moteur** (comme l'établit `ARCHITECTURE.md`) ; les
paliers ne le scindent pas, ils se distinguent par la fonctionnalité du brief qu'ils exercent. Trois
traces les accompagnent, toutes tenues par le fondateur et ouvertes dès le palier 1 : le **relevé de
parité** (un geste de la batterie × langage × famille), le **relevé du banc** (§17.2) et le **journal
des retours** (les seuils des §3 et §4, puis les retours des développeuses au §7 bis).

| # | L'étape | Ce qu'on montre à la fin | Contexte borné |
|---|---|---|---|
| 1 | **Charger et refuser** — charger le fichier de départ depuis PHP, Go et Node ; charger une batterie de fichiers incohérents : relation vers une ligne absente, deux `id` identiques, une ligne sans `id`, un `id` mal typé (décimal, vrai/faux), une valeur imbriquée, aucune table, une clé dupliquée ; charger enfin un fichier accompagné d'un résidu (un « fichier à part » laissé par un arrêt brutal). | Le fichier valide charge dans les trois langages ; chaque fichier incohérent est refusé avec le **même** message — règle, table, ligne — dans les trois ; le résidu est ignoré, jamais confondu avec le fichier réel. Le relevé de parité est ouvert : c'est la **première épreuve** de §17.1 (mêmes refus, trois langages, une machine), pas encore sa tranche. Le brief exclut tout inventaire de ce qui est deviné (§14) : le refus est la seule preuve du devinage. | le moteur (§8.1, §8.2) |
| 2 | **Interroger** — filtre, tri (croissant et décroissant, colonne absente placée en dernier), jointure `wallets → users` par la relation devinée, compte, somme ; puis lecture sur une table absente, sur une colonne absente, un texte SQL invalide, une requête hors sous-ensemble, un tri sur une colonne dont deux lignes du résultat portent des types différents ; enfin un fichier piège **qui charge** — `users` portant à la fois `id: 5` et `id: "5"` (décision 30) — pour montrer une non-jointure en lecture. | Mêmes lignes, dans le même ordre — trié quand la requête le demande, celui du fichier sinon —, dans les trois langages ; mêmes refus, mêmes messages, y compris le refus du tri sur types mêlés ; `"5"` et `5` ne se rejoignent jamais (décision 34). | le moteur (§8.3) |
| 3 | **Écrire** — insérer, modifier, supprimer une ligne, chacune isolément ; puis les écritures qui violent : `INSERT` sans `id`, `INSERT` en doublon d'`id`, `UPDATE` d'un `id` vers un décimal, `UPDATE`/`DELETE` sur une table absente, un `INSERT` qui fait apparaître la table transformant une colonne en relation orpheline, un `DELETE` d'un `users` référencé ; enfin un disque rendu inaccessible sur une écriture. | Après chaque écriture validée, le fichier a changé d'un coup, seules les valeurs touchées, identique à l'octet près entre langages ; chaque écriture illégale est refusée en nommant la règle (et les deux tables pour l'orpheline) ; l'échec disque laisse mémoire et fichier inchangés et **aucun fichier à part** derrière lui. Premier relevé du banc (§17.2). | le moteur (§8.4, §8.6, §8.7) |
| 4 | **Le virement tout-ou-rien** — le geste complet du §1, plus toutes les issues d'une transaction : ouvrir, retirer, ajouter, relire dans la transaction, valider ; **abandonner** volontairement ; une **instruction refusée** (un `UPDATE` sur table absente) qui laisse la transaction ouverte ; une **validation qui échoue** (un `DELETE` référencé puis valider) ; un **échec disque** dans une transaction ; une **deuxième ouverture** ; **valider ou abandonner sans transaction ouverte**. Le tout depuis PHP, Go et Node, sur chacune des quatre familles de systèmes retenues (`ARCHITECTURE.md` §12.3 Q7), rejouées par la CI, sous la charge du banc. | Le geste du §1 devant une personne à côté, `git diff` compris. L'abandon : la relecture montre le fichier inchangé, aucun diff. L'instruction refusée : la transaction reste ouverte, on continue. La validation qui échoue et l'échec disque : la transaction est refermée, la mémoire revenue à l'état d'avant, le fichier intact. La deuxième ouverture, et la validation ou l'abandon sans transaction ouverte : refusés, chacun nommant la règle. **Relevé de parité complet sur les quatre familles → §17.1 tranchée sur la batterie du geste ; sa fermeture sur de vraies suites revient au §7 bis**, complété par la CI et non plus à la main ; relevé du banc complet (taille × écritures × transactions × chargements). | le moteur (§8.5, §9.1) |
| 5 | **Recharger** — restaurer le fichier par un moyen externe puis `reload` ; `reload` pendant une transaction ouverte ; `reload` sur un fichier rendu incohérent, puis introuvable ; une lecture après chaque échec. | Le rechargement réussi rend l'état du fichier restauré sans recréer d'instance ; le refus pendant une transaction laisse la transaction ouverte ; un rechargement raté laisse l'état précédent intact et l'instance répond encore — même issue et même message dans les trois langages. | le moteur (§8.9) |

Chaque palier n'a besoin que des précédents ; le palier 1 ouvre l'épreuve de l'hypothèse la plus risquée
(§17.1), le palier 4 la tranche en même temps qu'il réalise le geste complet du §1 ; le palier 5 clôt les
dix fonctionnalités du brief.

## 7. Ce qui dit que c'est livré

Le MVP est **livré** quand les cinq conditions suivantes tiennent toutes, chacune vérifiable sans
séance humaine : **(a)** les dix fonctionnalités passent leurs tests Rust ; **(b)** les trois ponts
passent leurs propres tests et `bash harness/parity/run.sh` rend zéro divergence sur les trois ponts et
sur chacune des quatre familles de systèmes retenues (CI verte sur le tag) ; **(c)** le banc §17.2 est
relevé aux paliers 3, 4 et 5 ; **(d)** le brief, le MVP, `ARCHITECTURE.md` et le glossaire sont
cohérents avec le code (revue document verte) ; **(e)** un dépôt public existe, avec un `README`, une
`LICENSE` et une version taguée. La réussite des tests automatisés du fondateur, seule, ne suffit pas :
il faut (b), (c), (d) et (e) en plus.

## 7 bis. Ce qui dit que c'est prouvé

Une étape d'apprentissage, **postérieure à la livraison** (§7) et qui ne la conditionne pas : chacune
des trois développeuses — aucune n'étant le fondateur —, sur sa propre machine et sur un fichier de
fixtures **de son projet** déjà suivi par Git, joue **l'équivalent du geste du §1 dans les trois
langages** (PHP, Go et Node) : une transaction d'au moins deux écritures qu'elle valide, une jointure
sur une relation devinée de ses fixtures, et un `DELETE` d'une ligne encore référencée. Elle commit
d'abord une réécriture de normalisation ; puis elle voit les mêmes résultats dans les trois langages,
ouvre `git diff` et n'y trouve que les valeurs que son test a changées, et lit le même message de refus
dans les trois langages. Si ses fixtures ne portent **aucune** relation devinée, la part « `DELETE`
référencé / jointure » ne peut pas être jouée : elle est notée au journal (une fixture sans relation est
elle-même un retour sur §17.3 b) et le reste du geste compte quand même. Le fondateur n'intervient
qu'autant que le §4 l'admet (une requête hors sous-ensemble). Le journal des retours est rempli à
chaque séance ; cette étape ferme les hypothèses §17.1 et §17.3, par `/charpente:learn`.

## 8. Ce qui dit qu'on s'est trompé

| L'hypothèse | Ce qu'on observe | Le seuil qui fait changer de route | Où ça se lit |
|---|---|---|---|
| §17.1 — PHP atteint une parité réelle | Pour chaque **geste de la batterie** (chargement, requête, ouverture/validation/abandon, rechargement) × **trois langages** × **quatre familles de systèmes** (`ARCHITECTURE.md` §12.3 Q7), rejoué par la CI : résultat, message d'erreur, empreinte du fichier réécrit. | **1** écart (résultat, message ou fichier) que l'architecture juge impossible à corriger dans le cœur — c'est-à-dire qui exigerait une décision dans un pont (§5.1) — ou **1** geste du §1 inaccessible depuis PHP sans une couche réseau déguisée en natif (§17.1). Qui tranche « corrigeable dans le cœur ou non » : l'architecture. Route : revoir le cœur ou la voie d'appel avant tout palier suivant. | Le **relevé de parité**, alimenté par la CI (une ligne par tag). |
| §17.2 — Réécrire le fichier entier reste imperceptible | Durée d'une transaction validée (réécriture comprise), d'un chargement complet, d'un rechargement, et d'une suite d'écritures isolées, en faisant varier la taille du fichier depuis celui du §1 jusqu'à la plus grande fixture réelle collectée au §7 bis. | Le critère que le brief pose lui-même (§4, §17.2 du brief) : la réécriture devient **le geste le plus lent de la suite de tests qui la contient** — durée de réécriture supérieure à la durée du reste de la suite — à une taille inférieure ou égale à la plus grande fixture réelle collectée. Aucun nombre n'est fixé d'avance ; c'est la fixture réelle qui le fixe. Route : revoir le pari du §4 du brief. | Le **relevé du banc**, tenu par le fondateur, rempli aux paliers 3, 4 et 5, rejoué au §7 bis. |
| §17.3 (a) — Les développeuses acceptent qu'un test réécrive le fichier pointé | Pour chacune des trois développeuses du §7 bis : a-t-elle accepté de pointer Memless sur ses vraies fixtures ; a-t-elle vécu une réécriture comme une perte ; a-t-elle demandé un mode lecture seule (décision 3). Le bruit d'une **première** réécriture (commentaires, mise en forme, `null`, forme d'un décimal — décision 26) n'est pas une perte. | **1** modification non committée écrasée par une réécriture (une vraie perte, puisque des fixtures suivies par Git sont sinon restaurables), ou **au moins deux des trois** développeuses qui refusent de pointer Memless sur leurs vraies fixtures ou demandent un mode lecture seule. Route : revoir la décision 3. | Le **journal des retours**, tenu par le fondateur, alimenté aux séances du §7 bis : une entrée par développeuse et par séance — date, événement, verbatim, fichier concerné. |
| §17.3 (b) — Les relations devinées par le seul nom sont acceptées | Sur les vraies fixtures du §7 bis : fausses détections (chargement refusé, ou suppression refusée, sur une colonne `_id` qui n'est pas une relation, ex. `stripe_id`) et relations manquées (ligne orpheline passée, pluriel irrégulier — jamais un refus, mais un oubli constaté après coup). | **1** fichier réel refusé au chargement par fausse détection (le pire cas nommé au §17.3), ou **1** relation manquée constatée après coup. Route : faire entrer l'inventaire ou la déclaration explicite (§16) au lieu de poursuivre en convention seule. | Le **journal des retours**, avec chaque refus tel que le moteur le nomme (règle, table, ligne — §10) consigné à côté de l'entrée, séances §7 bis. |
