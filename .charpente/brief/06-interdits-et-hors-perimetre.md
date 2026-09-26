<!-- charpente-navigation -->
**Index** : [brief — Moteur de base de données en mémoire pilotée par YAML](00-index.md)  
**Précédent** : [10. Les règles transverses](05-regles-et-limites.md)  
**Suivant** : [15. Les décisions, et ce que chacune écarte](07-decisions.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 13. Ce qui est interdit

Chaque ligne est un état qui doit être impossible, pas un message d'erreur à afficher.

**Chargement et structure devinée**

- Un chemin qui ne désigne aucun fichier, un fichier illisible, un fichier vide, ou un YAML valide
  sans aucune table, ne produisent jamais un moteur démarré — §8.1.
- Un fichier dont la syntaxe YAML est invalide n'est jamais partiellement chargé — §8.1.
- Une valeur qui est elle-même une liste ou un ensemble de champs n'est jamais chargée : elle n'entre
  dans aucun des types reconnus — §8.1.
- Une clé de premier niveau dont la valeur n'est pas une liste de lignes, ou un élément de cette liste
  qui n'est pas lui-même un ensemble de champs, n'est jamais chargé — §8.1.
- Une ligne qui ne respecte pas la structure devinée n'est jamais chargée en mémoire, même seule —
  §8.2.
- Une ligne sans colonne `id` n'est jamais chargée — §8.1, §8.2.
- Un `id` qui n'est ni un texte ni un nombre entier n'est jamais chargé — §8.1, §15 décision 30.
- Deux lignes portant le même `id` ne coexistent jamais dans une même table — §8.2, §15 décision 34.
- Une relation devinée qui pointe vers une ligne absente de la table visée n'est jamais chargée —
  §8.2.

**Requêtes et modifications**

- Un texte qui ne s'analyse pas comme du SQL valide n'est jamais exécuté, même partiellement — §8.3.
- Une requête **de lecture** qui désigne une table ou une colonne absente de ce qui a été deviné n'est
  jamais exécutée — §8.3.
- Une écriture (`UPDATE`, `DELETE`) qui désigne une table absente n'est jamais un changement silencieux
  sans effet : elle est refusée, comme la lecture — §8.4.
- Une requête syntaxiquement valide mais hors du sous-ensemble SQL supporté n'est jamais exécutée à
  moitié, ni ignorée en silence — §8.3.
- Une transaction dont l'état final violerait la structure devinée (deux lignes de même `id`, une
  ligne sans `id`, ou une relation cassée) n'est jamais validée — §8.4.
- Aucune instruction ne déclare la forme d'une table (`CREATE TABLE`, `ALTER TABLE`), et aucune ne fait
  disparaître entièrement une table (`DROP TABLE` ou équivalent) — §8.4, §15 décision 24.
- Une transaction partiellement appliquée n'atteint jamais le fichier sur le disque — §8.5, §8.6.
- Une deuxième transaction ouverte sur une instance qui en a déjà une n'est jamais acceptée, et valider
  ou abandonner sans qu'aucune ne soit ouverte n'a jamais lieu non plus — §8.5, §15 décision 29.
- Un rechargement demandé pendant qu'une transaction est ouverte n'a jamais lieu, et un rechargement
  qui échoue ne vide jamais l'état déjà chargé — §8.9.
- Une transaction ouverte au moment où l'instance est libérée n'atteint jamais le disque — §8.5.
- Un changement qui n'a pas atteint le disque ne reste jamais retenu en mémoire — §8.6, §15 décision
  18.
- Une ligne encore référencée par une relation devinée, à la fin d'une transaction validée, n'est
  jamais supprimée ni laissée orpheline par une modification — §8.7.
- Le fichier réécrit sur le disque n'est jamais lisible par un tiers dans un état intermédiaire, et une
  interruption pendant l'écriture ne le laisse jamais à moitié écrit : c'est le fichier d'origine,
  intact, ou le nouveau, complet — jamais un mélange. Un arrêt brutal du processus entre l'écriture du
  fichier à part et sa substitution peut laisser ce fichier à part derrière lui, sans qu'il ne devienne
  jamais officiel ni ne soit confondu avec le fichier réel au chargement suivant — §10.

**Comportement entre langages**

- Deux langages qui exécutent la même requête sur le même état ne rendent jamais deux résultats
  différents — §10.
- Deux langages qui devinent, depuis les mêmes données, deux structures différentes (deux relations
  différentes, par exemple) ne coexistent jamais sans que ce soit un défaut — §8.8.

---

## 14. Hors périmètre

| Ce qui est laissé dehors | Pourquoi | À quelle condition ça reviendrait |
|---|---|---|
| Utilisation en production pour faire tourner une application réelle | décision produit (§15 décision 2) : le premier usage retenu est les données de test et de démonstration, jamais une charge de production | Exclusion durable : « données de test » est le premier usage retenu, pas une étape vers la production ; celle-ci serait une décision de produit distincte, prise à part, pas une condition déjà engagée ici |
| Un serveur ou processus central partagé entre plusieurs langages en même temps | décision produit (§15 décision 6) : chaque instance garde sa propre copie en mémoire, plus simple à construire et suffisante pour l'usage retenu | qu'un besoin réel de tests croisant plusieurs services en même temps soit mesuré, pas supposé — §16 |
| Une protection contre plusieurs écritures concurrentes sur le même fichier — deux processus, deux instances du moteur dans un même processus, une même instance appelée par deux exécutions concurrentes, ou une personne qui édite le fichier à la main pendant qu'un moteur l'a déjà chargé | décision produit (§12.2) : sans coordination, la dernière écriture gagne en silence, et qui a perdu la sienne ne le sait pas. C'est un risque assumé pour l'usage retenu, où un fichier n'est normalement manipulé que par un seul acteur à la fois | qu'un usage réel où plusieurs acteurs touchent le même fichier en même temps soit constaté — §16 |
| Une API « façon SQL » en code natif, propre à chaque langage, en plus du texte SQL | décision produit (§15 décision 8) : un seul texte SQL partagé garantit que les trois langages se comportent à l'identique (§5.1) ; une API par langage romprait cette garantie tant qu'elle n'est pas éprouvée | que le texte SQL partagé soit éprouvé et stable dans les trois langages d'abord — §16 |
| Suppression en cascade et mise à `NULL` automatique des références | décision produit (§8.7, §15 décision 10) : seul le refus protège contre une perte de donnée non voulue au lancement | qu'un usage réel bute concrètement sur le refus systématique — §16 |
| Une façon de déclarer explicitement une relation quand la convention de nom ne suffit pas (une colonne qui n'a pas le suffixe `_id`, ou qui pointe vers une table au nom irrégulier) | décision produit (§15 décision 31) : la convention de nom est le seul mécanisme au lancement, pour rester aussi simple que possible à écrire — le risque que ça fait courir est nommé en hypothèse, §17.3 | qu'un usage réel bute sur des relations que la convention ne détecte pas — §16 |
| Un système d'audit applicatif — qui a déclenché un changement, avec quelle autorisation | l'historique Git (§9.1) montre ce qui a changé, pas qui a lancé le test qui l'a changé ; ce sont deux besoins différents | Exclusion durable : tant que le premier usage est les tests, cet audit reste hors périmètre ; il ne « reviendra » que par une décision de produit distincte |
| La compatibilité entre fichiers écrits par des versions différentes des trois ponts | au lancement, une seule version existe : la question ne se pose pas encore, et les fichiers déjà écrits par cette version ne porteront jamais de marque a posteriori | qu'une deuxième version des ponts soit publiée — §16 |
| Un inventaire consultable de ce que Memless a deviné (types, identifiants, relations) | décision produit : au lancement, un refus ou une relation qui tient déjà dit ce qu'il faut savoir, sans registre séparé à maintenir | qu'une fausse détection ou une relation manquée soit constatée sur de vraies fixtures (§17.3 b) — §16 |
