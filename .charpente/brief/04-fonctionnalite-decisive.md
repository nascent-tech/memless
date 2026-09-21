<!-- charpente-navigation -->
**Index** : [brief — Moteur de base de données en mémoire pilotée par YAML](00-index.md)  
**Précédent** : [7. Les besoins, séparés des solutions](03-besoins-et-fonctionnalites.md)  
**Suivant** : [10. Les règles transverses](05-regles-et-limites.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 9. Les fonctionnalités décisives

Une seule, arbitrée par l'humain avant d'entrer dans ce document. Elle n'a été demandée par
personne : elle devient possible avec ce que les fonctionnalités du §8 construisent déjà, sans
moteur séparé à écrire pour elle.

### 9.1 L'historique Git comme journal d'audit des données

**Ce que c'est.** Git est l'outil que la plupart des développeuses utilisent déjà pour suivre les
changements de leur code — un peu comme le « suivi des modifications » d'un traitement de texte,
mais pour des fichiers entiers et avec un historique permanent : `git log` liste qui a changé quoi et
quand, `git diff` montre exactement quelles lignes ont bougé, `git blame` dit qui a écrit chaque
ligne encore présente, et une **pull request** est la proposition de changement qu'une équipe relit
avant de l'accepter.

Parce que le fichier YAML est réécrit à chaque transaction validée (§8.6), et parce que YAML est un
texte que Git sait comparer ligne à ligne, chaque changement de donnée peut devenir une ligne
d'historique consultable avec ces mêmes outils. Retirer d'un portefeuille et ajouter à un autre (§7)
laisse une trace lisible par une personne, exactement comme un changement de code — aucun outil
trouvé en recherche ne réunit ce bénéfice avec les quatre traits du §3.3 (§18).

Cette confiance dans l'historique suppose qu'un seul acteur écrive le fichier à la fois : Memless
n'arbitre pas deux écritures concurrentes (§12.2, §14) et fait gagner la dernière en silence. Deux
processus qui modifieraient le même fichier en même temps ne laisseraient donc aucune trace du
changement perdu — ni dans le fichier, ni dans l'historique Git, qui n'en verrait jamais qu'un des
deux passer. La fonctionnalité décisive hérite ainsi de la même limite que le reste du produit : elle
documente ce qu'un seul acteur a changé, pas ce que plusieurs auraient tenté en même temps.

**Ce qu'elle exige.** Le moteur réécrit le fichier de façon stable : le même ordre de tables, le même
ordre de lignes — une ligne nouvellement ajoutée se place à la fin de sa table, sans réordonner ce qui
existe déjà (§15, décision 27), pour respecter l'ordre qu'une auteure a pu donner elle-même à ses
fixtures en les écrivant à la main. Seule la valeur qui a réellement changé doit ainsi apparaître dans
le diff : une réécriture qui réordonnerait les lignes à chaque fois produirait un diff illisible, qui
listerait tout comme changé alors que presque rien ne l'est — la fonctionnalité se démontrerait
fausse par son propre résultat. C'est une exigence du moteur, pas une option qu'on pourrait désactiver
pour aller plus vite (§10, §15 décision 13). Puisqu'un seul cœur partagé écrit le fichier, les trois
langages produisent aussi le même fichier à l'octet près pour un même état (§8.8, §15 décision 25).

Cette stabilité porte sur les données elles-mêmes — pas sur tout ce que l'auteure aurait pu ajouter à
la main dans le fichier. Un commentaire ou une mise en forme libre n'est pas garanti de
survivre à une réécriture automatique (§15, décision 26) : ce que Git suit fidèlement, c'est le
contenu déclaré, pas la décoration qui l'entoure. La première réécriture d'un fichier commenté à la
main en est la conséquence directe et connue : elle peut faire disparaître ces commentaires en une
seule fois, avant que l'historique ne redevienne un journal des seuls changements de donnée.

Deux conséquences du modèle de données rejoignent les commentaires dans cette même liste d'exceptions
connues. Un `null` écrit à la main pour une colonne est, dès le chargement, indiscernable de l'absence
de cette colonne (§15, décision 35) : la première réécriture peut donc faire disparaître la ligne
`colonne: null` du fichier, sans que rien n'ait, au sens du produit, changé. Et un nombre décimal qui
ne se représente pas exactement (§15, décision 20) peut ressortir de la réécriture sous une forme
textuelle différente de celle écrite à la main, sans que sa valeur ait changé. Dans les deux cas, ce
n'est pas la donnée qui change : c'est son apparence à l'écran.

Enfin, ce bénéfice suppose un geste que Memless ne fait pas lui-même : quelqu'un doit committer le
fichier, avec une fréquence qui rend l'historique utile. Memless produit un fichier prêt à être
committé ; committer reste une pratique de l'équipe, pas une action de Memless (§15, décision 28).

**Ce que ça coûte.** Committer le fichier assez souvent pour que l'historique reste utile (§15,
décision 28), et la stabilité d'écriture imposée au moteur — ordre des tables, des lignes et des
colonnes conservé, aucun réordonnancement (§10, §15 décision 27).

**Qui porte la charge.** L'équipe, pour le commit ; le moteur, pour la stabilité de la réécriture.

**Ce qui est refusé.** Elle ne remplace pas un système d'audit applicatif — qui a lancé le test, avec
quelle autorisation : elle montre ce qui a changé dans les données, pas qui a déclenché le
changement. Cette seconde question reste hors périmètre (§14).
