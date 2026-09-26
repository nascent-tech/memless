<!-- charpente-navigation -->
**Index** : [mvp — Prouver la promesse de Memless](00-index.md)  
**Suivant** : [4. Ce qu'on accepte de faire mal](02-faiblesses-et-garanties.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 1. Le geste qui prouve

**Une développeuse écrit un fichier YAML de deux tables, `users` et `wallets`, sans rien déclarer de
leur forme ; depuis chacun des trois langages tour à tour — PHP, Go et Node —, son test ouvre une
transaction, retire 1 000 d'un portefeuille et l'ajoute à un autre, valide, puis relit les soldes en
croisant `wallets` et `users` ; la personne à côté voit les mêmes résultats dans les trois langages, un
`git diff` qui ne montre que les deux valeurs changées, et un `DELETE` de l'utilisateur encore référencé
refusé — avec le même message, nommant la règle, la table et la ligne, dans les trois langages.**

Acteur : l'auteure du fichier et son code appelant (§6 du brief, une seule personne). Déclencheur :
lancer le test. Résultat observable : les résultats à l'écran, le diff, le refus.

**Le fichier de départ** est celui du §8.1 du brief — deux `users` (`01H7B2`, `01H7B3`) et un
portefeuille (`w_123`, appartenant à `01H7B3`) — **augmenté d'un second portefeuille** (`w_124`,
appartenant à `01H7B2`, solde initial fixé au plan) : deux tables, **quatre lignes**, montants en entiers
(décision 20). Le virement va de `w_123` à `w_124`.

**Le diff observé est celui de la seconde réécriture.** Un fichier écrit à la main peut, à sa première
réécriture, perdre commentaires, mise en forme, un `null` écrit à la main ou la forme d'un décimal
(§9.1, décision 26). Avant le geste, on commit donc une première réécriture de normalisation ; c'est à
partir d'elle que le `git diff` du virement ne montre plus que les deux valeurs changées.

Le geste se montre à une personne à côté, sur une seule machine, sans réseau, sans deuxième acteur
humain : les trois exécutions ne sont ni trois services ni trois jours. Entre deux d'entre elles, la
remise à zéro suit la séquence du §3.

## 2. Ce qui entre, et rien d'autre

**Dix retenues sur dix** — les neuf fonctionnalités du §8 du brief et la décisive du §9 ; rien ne sort.

| La fonctionnalité | Au brief | Ce qui casse dans le geste sans elle |
|---|---|---|
| Un seul fichier, uniquement des données — Memless devine le reste | §8.1, décisions 20, 30, 31, 34–36 | Il faudrait déclarer la forme : la promesse « sans jamais décrire la forme des tables à part » est fausse par construction ; le fichier de deux tables ne se charge pas. |
| Le chargement vérifie la structure devinée et refuse un fichier incohérent | §8.2, décisions 21, 22 | Sans lui, la règle §10 « impossible d'écrire, par une transaction acceptée, un fichier que Memless refuserait ensuite de charger » perd son sens : une relation cassée par une écriture pourrait entrer, le `DELETE` référencé (§8.7) ne serait plus refusé, et le `git diff` ne prouverait plus que le fichier est la vérité. Le palier 1 le montre en refusant trois fichiers incohérents. |
| Interroger l'état en SQL | §8.3, décision 23 | Impossible de relire les soldes ni de croiser `wallets` et `users` par la relation devinée. |
| Modifier l'état en SQL | §8.4, décision 24 | Aucun virement ; « modifier » (§1) n'est pas prouvé. |
| Regrouper plusieurs écritures en une transaction tout ou rien | §8.5, décisions 9, 15, 16, 29 | Le virement devient deux réécritures : entre les deux, le fichier sur disque porte l'état où l'argent a quitté la première tirelire sans arriver dans la seconde — ce que §5.1 définit comme ce qu'une transaction empêche ; le diff montre deux changements pour un geste. |
| Toute transaction validée est aussitôt réécrite dans le fichier YAML | §8.6, décisions 3, 5, 17, 18, 19 | Le fichier n'est pas la vérité après le virement (§1) : rien à montrer sur le disque. |
| Casser une relation devinée est refusé | §8.7, décision 10 | Le `DELETE` de l'utilisateur référencé passe : le refus qui prouve que la relation devinée **protège** n'a pas lieu. |
| Le même moteur, utilisable nativement dans trois langages | §8.8, §5.1, décisions 11, 25 | « Identique dans les trois langages » ne se montre pas ; l'hypothèse §17.1 (parité) n'est pas éprouvée. |
| L'historique Git comme journal d'audit des données ·décisive· | §9.1, décisions 13, 26, 27, 28 | Sans réécriture stable (ordre des tables, lignes, colonnes), le diff liste tout comme changé : la personne à côté ne peut pas voir que seules deux valeurs ont bougé — la preuve visible que le fichier est la vérité devient illisible, et §17.3 (a) ne se teste pas. |
| Recharger l'état depuis le fichier, à la demande | §8.9, décision 14 | Une suite qui tient une instance pour toute sa durée ne peut pas revenir au fichier restauré sans recréer un moteur ; la remise à zéro du §3 exigerait une instance par test. |

## 3. Ce qui sort, et à quelle condition ça rentre

**Rien ne sort.** Aucune fonctionnalité du §8/§9 du brief ne sort du MVP ; les restrictions **à l'intérieur** d'une
fonctionnalité retenue — étendue du SQL, familles de systèmes, fichier de démonstration — sont au §4.

**La remise à zéro entre deux exécutions** se fait ainsi, dans cet ordre (le brief avertit qu'une
transaction validée après restauration, par une instance encore vivante, réécrirait le fichier restauré
avec l'ancien état — §8.6, §12.2) : **(1)** observer le `git diff` de l'exécution qui vient de finir ;
**(2)** restaurer le fichier par Git ; **(3)** `reload` sur l'instance vivante — ou, entre deux langages,
libérer l'instance et en créer une nouvelle dans le langage suivant. Jamais deux instances vivantes sur
le même fichier en même temps : le brief ne coordonne pas deux instances (§12.2, §14).
