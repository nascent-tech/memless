<!-- charpente-navigation -->
**Index** : [mvp — Prouver la promesse de Memless](00-index.md)  
**Précédent** : [1. Le geste qui prouve](01-le-geste-et-le-tri.md)  
**Suivant** : [6. L'ordre, et ce que chaque étape rend démontrable](03-paliers-fini-trompe.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 4. Ce qu'on accepte de faire mal

| Ce qu'on fait mal | Ce que ça économise | Le seuil auquel ça casse |
|---|---|---|
| Deux ponts sur trois — PHP et Go (§8.8) | Un pont natif entier. | **1** développeuse du troisième langage (JavaScript/TypeScript) qui demande à l'essayer, une fois la parité PHP/Go confirmée — 0 écart au relevé de parité (journal des retours). |
| Le sous-ensemble SQL est réduit à ce que le geste utilise : lecture avec filtre, jointure par relation devinée, compte et somme ; insertion, mise à jour, suppression de lignes. Tout le reste est refusé « hors du sous-ensemble » (§8.3, décision 23) — jamais en silence. | L'exécution d'un SQL large. | **1** refus « hors du sous-ensemble » rencontré par une développeuse sur une requête nécessaire à son propre test (le refus nomme la règle, §10 ; compté au journal des retours). |
| Pas de rechargement (§8.9) : réinitialiser = restaurer le fichier puis créer une nouvelle instance (§3). | La fonctionnalité §8.9. | Voir §3 : 1 demande explicite au journal des retours. |
| Les paliers 1 à 3 se démontrent sur une seule machine ; les familles de systèmes retenues par l'architecture n'entrent qu'au palier 4 (§17.1). | Reproduire chaque palier sur chaque famille. | **1** écart entre deux familles sur la batterie du geste (relevé de parité) — le seuil de route de §17.1, §8. |
| La « charge réelle » de §17.1 est la batterie du geste répétée par le banc, pas une vraie suite d'équipe. | Un banc réaliste. | **1** écart entre langages constaté par une développeuse sur sa propre suite (journal des retours). |
| Le fichier d'épreuve est celui du §8.1 augmenté d'un second portefeuille (deux tables, quatre lignes), grossi artificiellement pour le banc — pas de vraies fixtures avant le §7. | Collecter des fixtures avant d'avoir un moteur. | **1** fichier réel refusé au chargement par fausse détection, ou **1** relation manquée constatée après coup (pluriel irrégulier, §17.3 b) — journal des retours. |
| Le banc §17.2 est joué à la main, une fois à chacun des paliers 3 et 4, pas en continu. | L'outillage de mesure d'une exécution continue. | **1** régression de durée découverte au palier 4 alors qu'elle était déjà apparue au palier 3 (un jeu à la main l'aurait vue plus tôt) — relevé du banc. |

*Le seuil qui dit à partir de quelle taille la réécriture devient lente est, lui, une mesure et non un
chiffre inventé : il vit au §8, ligne §17.2.*

## 5. Ce qui ne se coupe jamais

**Circuit fermé** pour les paliers 1 à 4 (le fondateur, le fichier de démonstration, la personne à
côté) ; **vrais utilisateurs** — **trois** développeuses, sur leurs fixtures déjà suivies par Git — pour
le §7. Trois est le nombre décidé pour ce MVP : assez pour faire remonter l'essentiel des retours
d'usage, assez peu pour être réuni ; il fixe le critère de fin (§7) et une route (§8). Pas d'argent, pas
de donnée personnelle d'un tiers : les fixtures sont écrites par la développeuse elle-même (§3.4 du
brief), Memless n'écoute aucun port et n'envoie rien nulle part (§3.4, décision 6). Les garanties
ci-dessous tiennent dans les deux cas, recopiées sans changement du brief — la réécriture d'un fichier
est le seul geste irréversible du produit.

**Contrainte §5.1 du brief.** Toute règle du métier — une contrainte de structure, un refus, ce qui se
passe dans une transaction — vit dans le moteur lui-même, jamais dans un des trois traducteurs : sinon,
un des trois langages finit par se comporter différemment des deux autres, sans que personne ne l'ait
décidé. Cela
vaut aussi pour la lecture et l'écriture du fichier YAML et pour les trois gestes qui devinent sa
structure (§8.1). Ce que le texte SQL sait faire (§8) ne peut donc pas dépendre d'une bibliothèque
propre à un seul des trois écosystèmes.

**Règles §10 touchant l'intégrité, l'irréversible et la parité, recopiées sans changement.**

- Toute donnée qui entre dans l'état — au chargement (§8.2) ou par une écriture (§8.4) — respecte la
  structure devinée (§8.1). Une violation refuse l'entrée entière ; il n'existe pas d'entrée « partielle ».
- La structure devinée n'est jamais figée à un instant : elle se redérive, depuis les mêmes règles
  (§8.1), à chaque fois qu'un état doit être vérifié — au chargement, à la validation d'une transaction
  (contre l'état que cette transaction propose, jamais un état abandonné ou dont la validation a échoué),
  à un rechargement (§8.9). Une transaction qui rendrait l'état invalide une fois la structure ainsi
  redérivée échoue à sa validation, exactement comme toute autre violation (§8.4) : il est donc
  impossible d'écrire, par une transaction acceptée, un fichier que Memless refuserait ensuite de
  charger. Une table existe tant que son nom porte une clé dans le fichier — au chargement comme après
  un rechargement, sans dépendre d'un historique — y compris une table apparue puis vidée à l'intérieur
  de la transaction qui vient de se valider : elle garde sa place exactement comme une table plus
  ancienne réduite à une liste vide, puisque rien ici ne distingue une table selon la date à laquelle
  elle a commencé d'exister, et toute colonne qui la référence reste reconnue comme une relation. Une
  colonne, elle, n'existe que tant qu'au moins une ligne de sa table la porte : elle disparaît de ce qui
  est deviné dès que plus aucune ligne ne la renseigne, sans garder de trace de ce qu'elle a été.
- Une transaction est tout ou rien. À sa validation, soit son état final entre entièrement dans le
  fichier, soit rien n'entre. Une instruction refusée en cours de route n'a jamais eu lieu et laisse la
  transaction ouverte (§8.5, §15 décision 29) ; c'est la validation qui peut échouer — état final
  invalide, ou écriture disque impossible — et referme alors la transaction sans rien laisser au fichier
  (§8.6). Une écriture isolée, non regroupée explicitement avec d'autres, est elle-même une transaction
  d'une seule instruction : §8.4 et §8.5 ne sont pas deux mécanismes séparés, le second regroupe
  simplement plusieurs occurrences du premier.
- Le fichier sur le disque n'est jamais lisible, ni laissé, dans un état intermédiaire. Memless écrit
  d'abord à part, puis substitue d'un coup (§8.6) : un programme qui lirait le fichier pendant l'écriture
  voit soit l'état d'avant, soit l'état d'après, jamais un mélange des deux ; et une interruption pendant
  l'écriture — coupure, plantage — laisse le fichier d'origine intact, jamais à moitié écrit.
- Les trois langages exécutent le même cœur. Une même requête, sur le même état, rend le même résultat
  et, en cas de refus, le même message d'erreur (règle, table, ligne), quel que soit le langage
  appelant — §5.1, §15 décision 25. Un écart de comportement entre deux langages est un défaut, jamais
  une variante ; puisqu'un seul cœur partagé écrit le fichier, les trois produisent aussi le même fichier
  à l'octet près pour un même état — §8.8.
- Toute erreur nomme la règle précise qu'elle a empêché de violer — quelle contrainte, quelle table,
  quelle ligne — jamais un message générique qui obligerait à deviner.

**Interdits §13, recopiés sans changement.**

*Chargement et structure devinée*
- Un chemin qui ne désigne aucun fichier, un fichier illisible, un fichier vide, ou un YAML valide sans
  aucune table, ne produisent jamais un moteur démarré — §8.1.
- Un fichier dont la syntaxe YAML est invalide n'est jamais partiellement chargé — §8.1.
- Une valeur qui est elle-même une liste ou un ensemble de champs n'est jamais chargée : elle n'entre
  dans aucun des types reconnus — §8.1.
- Une clé de premier niveau dont la valeur n'est pas une liste de lignes, ou un élément de cette liste
  qui n'est pas lui-même un ensemble de champs, n'est jamais chargé — §8.1.
- Une ligne qui ne respecte pas la structure devinée n'est jamais chargée en mémoire, même seule — §8.2.
- Une ligne sans colonne `id` n'est jamais chargée — §8.1, §8.2.
- Un `id` qui n'est ni un texte ni un nombre entier n'est jamais chargé — §8.1, §15 décision 30.
- Deux lignes portant le même `id` ne coexistent jamais dans une même table — §8.2, §15 décision 34.
- Une relation devinée qui pointe vers une ligne absente de la table visée n'est jamais chargée — §8.2.

*Requêtes et modifications*
- Un texte qui ne s'analyse pas comme du SQL valide n'est jamais exécuté, même partiellement — §8.3.
- Une requête **de lecture** qui désigne une table ou une colonne absente de ce qui a été deviné n'est
  jamais exécutée — §8.3.
- Une écriture (`UPDATE`, `DELETE`) qui désigne une table absente n'est jamais un changement silencieux
  sans effet : elle est refusée, comme la lecture — §8.4.
- Une requête syntaxiquement valide mais hors du sous-ensemble SQL supporté n'est jamais exécutée à
  moitié, ni ignorée en silence — §8.3.
- Une transaction dont l'état final violerait la structure devinée (deux lignes de même `id`, une ligne
  sans `id`, ou une relation cassée) n'est jamais validée — §8.4.
- Aucune instruction ne déclare la forme d'une table (`CREATE TABLE`, `ALTER TABLE`), et aucune ne fait
  disparaître entièrement une table (`DROP TABLE` ou équivalent) — §8.4, §15 décision 24.
- Une transaction partiellement appliquée n'atteint jamais le fichier sur le disque — §8.5, §8.6.
- Une deuxième transaction ouverte sur une instance qui en a déjà une n'est jamais acceptée, et valider
  ou abandonner sans qu'aucune ne soit ouverte n'a jamais lieu non plus — §8.5, §15 décision 29.
- Un changement qui n'a pas atteint le disque ne reste jamais retenu en mémoire — §8.6, §15 décision 18.
- Une ligne encore référencée par une relation devinée, à la fin d'une transaction validée, n'est jamais
  supprimée ni laissée orpheline par une modification — §8.7.
- Le fichier réécrit sur le disque n'est jamais lisible par un tiers dans un état intermédiaire, et une
  interruption pendant l'écriture ne le laisse jamais à moitié écrit : c'est le fichier d'origine,
  intact, ou le nouveau, complet — jamais un mélange. Un arrêt brutal du processus entre l'écriture du
  fichier à part et sa substitution peut laisser ce fichier à part derrière lui, sans qu'il ne devienne
  jamais officiel ni ne soit confondu avec le fichier réel au chargement suivant — §10.

*Comportement entre langages*
- Deux langages qui exécutent la même requête sur le même état ne rendent jamais deux résultats
  différents — §10.
- Deux langages qui devinent, depuis les mêmes données, deux structures différentes (deux relations
  différentes, par exemple) ne coexistent jamais sans que ce soit un défaut — §8.8.

*L'interdit du §8.9 (un rechargement pendant une transaction ouverte, un rechargement raté qui viderait
l'état) est satisfait par l'absence de §8.9 dans ce MVP ; il redevient actif le jour où §8.9 rentre (§3).*
