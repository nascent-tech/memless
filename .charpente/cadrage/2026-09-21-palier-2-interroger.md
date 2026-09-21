---
type: cadrage
titre: Palier 2 — Interroger
slug: palier-2-interroger
cree_le: 2026-09-21T19:47:00+0000
mis_a_jour_le: 2026-09-21T19:47:00+0000
branche: main
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../mvp/00-index.md) › **cadrage — Palier 2 — Interroger** › [conception — Palier 2 — Interroger](../conceptions/2026-09-21-palier-2-interroger/00-index.md) › [plan — Palier 2 — Interroger](../plans/2026-09-21-palier-2-interroger/00-index.md)  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/mvp/00-index.md

Cadrage du **palier 2 du MVP** — « Interroger ». Il ne couvre que la **lecture** d'un état
déjà chargé par du texte SQL, et le **refus** d'une requête qui n'est pas lisible, désigne
l'absent, ou dépasse le sous-ensemble — depuis PHP puis Go. Les écritures (§8.4), les
transactions (§8.5) et la réécriture (§8.6) restent hors périmètre (point 8). Les références
`§`/`décision` renvoient au brief validé, sauf mention du MVP.

## 1. Acteur et déclencheur

**Acteur.** Le **code appelant** (§6) — le test automatisé en PHP puis en Go. Il tient déjà
une **instance démarrée** (palier 1) ; ici il l'interroge. Aucun « utilisateur final ».

**Déclencheur.** Le code appelant **soumet un texte SQL** à une instance vivante. C'est ce
geste qui déclenche l'analyse puis, si elle passe, l'exécution en lecture contre l'état en
mémoire. Aucune écriture, aucun accès disque : interroger ne réécrit jamais le fichier (§8.3,
§8.6).

**Ce que l'acteur observe.** Soit un **jeu de lignes** — colonnes ordonnées, cellules typées,
lignes dans l'ordre du fichier sauf `ORDER BY` —, soit un **refus** qui nomme la règle et ce
qu'elle peut nommer (table, colonne, position, construction). Rien entre les deux : une
requête n'est jamais exécutée à moitié (§8.3).

## 2. Besoin, séparé de la solution

| Formulé en solution | Le besoin dessous |
|---|---|
| « Interroger façon SQL » | Écrire des scénarios de test réalistes — filtrer, croiser des tables selon leurs relations devinées, compter, sommer — sans écrire à la main du code d'accès aux données, et sans que ce code diffère d'un langage à l'autre : le **texte de la requête est portable** (§7, §5.1). |
| « Croiser deux tables » | Suivre une relation **devinée** (§8.1) sans l'avoir déclarée : `wallets.user_id` rejoint `users.id` parce que le nom le dit, pas un schéma. |
| « Même résultat partout » | Qu'une équipe multi-langages n'ait jamais deux lectures du même état : mêmes lignes, même ordre, mêmes messages de refus depuis PHP et Go (§5.1, §8.8). |
| « Refuser une requête fausse » | Qu'une faute de frappe sur un nom de table ou de colonne, un SQL invalide, ou une requête hors du sous-ensemble soit **dite**, jamais exécutée à moitié ni ignorée en silence (§8.3). |

Ce que le palier 2 prouve en plus du palier 1 : que l'état chargé est **lisible** et que la
**règle de comparaison unique** (décision 34) tient en lecture — le fichier piège `id: 5` /
`id: "5"` ne rejoint jamais (MVP §6). Ce qu'il ne prouve pas : qu'on peut le **modifier**
(paliers 3–4).

## 3. Vocabulaire

Les termes marqués **[neuf]** sont à inscrire au glossaire. Ils s'ajoutent à ceux du palier 1.

| Terme | Définition | Voisin à ne pas confondre |
|---|---|---|
| **Requête** [neuf] | Un texte SQL de **lecture** soumis à une instance vivante ; une seule instruction `SELECT`, `;` final toléré (§8.3). | Une **instruction d'écriture** (§8.4, palier 3), refusée ici comme hors sous-ensemble. |
| **Sous-ensemble SQL** [neuf] | L'étendue exacte de SQL que Memless sait exécuter (point 6, D1) : un `SELECT`, une jointure par relation devinée, filtre, tri, `COUNT`, `SUM`. Fixé par ce cadrage, pas par le brief (§8.3). | Le **SQL complet**, dont tout le reste est « hors sous-ensemble ». |
| **Jeu de lignes** [neuf] | L'issue d'une requête acceptée : des colonnes ordonnées et des lignes de cellules ; un agrégat est un jeu de lignes d'une seule ligne (point 5). | Le **refus de requête**. |
| **Cellule** [neuf] | La valeur d'une colonne sur une ligne du résultat : un scalaire, ou **absente** (§8.1, décision 35). L'absence n'est jamais un scalaire. | Le **scalaire** présent (`""` est un texte présent, pas une absence). |
| **Relation de jointure** [neuf] | La condition `ON a.x = b.y` d'une jointure, dont **un côté est une relation devinée vers l'`id`** de l'autre table (§8.1) ; toute autre égalité est refusée (Q5). | Une **relation devinée**, la règle de nom ; la relation de jointure l'emploie. |
| **Agrégat** [neuf] | `COUNT(*)`, `COUNT(col)`, `SUM(col)` : une valeur unique calculée sur les lignes retenues (§8.3). `SUM`/`COUNT` ignorent les lignes sans la colonne (§8.1). | Une **colonne** rendue telle quelle. |
| **Refus de requête** [neuf] | L'issue où aucune ligne n'est rendue, avec une erreur qui nomme la règle : SQL invalide, hors sous-ensemble, table/colonne absente, jointure hors relation, somme non numérique (point 6). | Le **refus de chargement** (palier 1), qui porte sur le fichier, pas sur un texte SQL. |
| **Ordre du fichier** [neuf] | L'ordre dans lequel les lignes ont été lues du YAML ; le **seul** ordre d'un résultat, le tri étant hors sous-ensemble (MVP §4, §6). | Un **tri** demandé, qui n'existe pas dans ce palier. |

**Termes exclus** (§6.1) : « schéma », « index » (au sens SQL), « vue », « curseur »,
« plan d'exécution » — Memless n'expose aucun de ces objets.

## 4. Événements au passé sur une chronologie

Les faits observables par le code appelant. Les étapes internes (analyse, vérification des
noms, exécution) ne sont pas des événements : seule l'issue est observable.

| # | Événement | Origine | Ce qui le rend observable |
|---|---|---|---|
| 0 | **Requête soumise** | Commande — le code appelant passe un texte SQL à une instance vivante. | Le geste lui-même. |
| 1a | **Requête refusée** | Politique — *dès qu'*une raison de refus (point 6) est établie, *alors* l'analyse ou l'exécution s'arrête, aucun jeu de lignes n'est rendu. | Une erreur nommant la règle et ce qu'elle peut nommer. |
| 1b | **Jeu de lignes rendu** | Politique — *dès que* la requête est dans le sous-ensemble et ne désigne que de l'existant, *alors* l'exécution rend les lignes retenues, dans l'ordre du fichier ou de `ORDER BY`. | Le jeu de lignes lui-même. |

**Frontières.** 1a et 1b sont exclusifs et l'un survient toujours : pas de résultat
« partiel » ni « avec avertissement » (§8.3). **Aucune écriture ne se produit** : interroger
ne touche ni le fichier ni l'état en mémoire.

## 5. Contexte(s) et sorte

**Contexte borné : le moteur** (MVP §6). Le palier 2 exerce §8.3. **Sorte : `cœur de
métier`** — l'analyse déléguée à `sqlparser` est générique, mais l'**exécution** porte tout ce
qui distingue Memless : la règle de comparaison unique (décision 34), l'absence indiscernable
d'un `null` (décision 35), l'ordre du fichier, le refus de convertir un type en un autre. Ces
règles produit vivent dans le cœur et nulle part ailleurs (§5.1, §5.2). La communication est
entrante seule (le code appelant soumet un texte) ; la réponse est l'issue du point 4.

## 6. Refus — tous les états impossibles, avant le nominal

Cœur du palier. Chaque refus : aucun jeu de lignes rendu, l'état inchangé, l'erreur nomme la
règle. Énumérés dans **l'ordre où ils s'établissent** — cet ordre décide quel message sort
quand une requête cumule plusieurs fautes (D1). L'ordre : **syntaxe → sous-ensemble → noms
(toutes les tables d'abord, dans l'ordre `FROM` puis `JOIN` ; puis les colonnes, dans l'ordre
`ON`, `SELECT`, `WHERE`) → jointure → exécution**. Le tri (`ORDER BY`) n'existe pas dans ce
sous-ensemble (MVP §4, §6 : « faute de tri ») — hors périmètre (point 8), donc pas de refus
dédié.

| # | État impossible | Règle violée (ce que le message nomme) | Nomme aussi |
|---|---|---|---|
| Q1 | Le texte ne s'analyse pas comme du SQL (dialecte fixé, D2) : backtick, crochet, instruction tronquée, entier littéral hors capacité 64 bits. **Aucune instruction** (texte vide, blancs, `;` seul) est aussi Q1 (rien à analyser). | « Le SQL est invalide. » | La position, quand l'analyseur la donne. |
| Q2 | Le texte s'analyse mais emploie une construction **hors du sous-ensemble** : autre `JOIN` qu'inner, plus d'une jointure, **jointure d'une table avec elle-même** (indécidable sans alias), **`SELECT` sans `FROM`**, **plus d'une instruction**, alias, `DISTINCT`, `GROUP BY`, `HAVING`, `LIMIT`, `OFFSET`, `ORDER BY`, `NOT` (hors `IS NOT NULL`), `IN`, `BETWEEN`, `LIKE`, arithmétique, fonction autre que `COUNT`/`SUM`, sous-requête, `UNION`, `CASE`, `CAST`, comparaison colonne–colonne (hors `ON`), colonne mêlée à un agrégat, colonne non qualifiée dans une jointure, littéral en position de colonne, `NULL` en littéral de comparaison (seul `IS [NOT] NULL` parle de l'absence), toute instruction non-`SELECT` (`INSERT`/`UPDATE`/`DELETE`/DDL — les écritures entrent au palier 3). | « <Construction> est hors du sous-ensemble SQL supporté. » Le libellé vient d'une table de correspondance, jamais un fragment du texte utilisateur. | La construction. |
| Q3 | La requête désigne une **table absente** de ce qui a été deviné. | « Aucune table <nom>. » | La table. |
| Q4 | La requête désigne une **colonne absente** — aucune ligne de la table ne la porte (définition du palier 1). Une **table vide** (`users: []`) n'a aucune colonne, pas même `id` : toute colonne y est refusée. | « Aucune colonne <nom> dans la table <table>. » | La table, la colonne. |
| Q5 | Le `ON` d'une jointure est valide mais n'est pas `relation devinée = id de la table visée` : soit **aucun côté n'est une relation devinée** vers l'autre table (`wallets.balance = users.id`), soit l'autre côté **n'est pas l'`id`** de la table visée (`wallets.user_id = users.email`). | « <colonne> de <table> n'est pas une relation devinée vers l'`id` de <table visée>. » | La table porteuse, la colonne, la table visée. |
| Q7 | `SUM` rencontre un **texte, un vrai/faux, ou un mélange entier/décimal** parmi **les lignes retenues** (après filtre, dans l'ordre du fichier) — sommer deux types serait convertir (décision 34). Refus **ajouté par ce cadrage** (D8). | « Impossible de sommer <colonne> de <table> à la ligne <ligne>. » | La table, la colonne, la première ligne fautive (par `id`). |
| Q8 | Une somme d'entiers **déborde** l'entier 64 bits signé. | « La somme de <colonne> dans <table> déborde. » | La table, la colonne. |

**Ce qui n'est PAS un refus** — les états qu'on pourrait croire impossibles et qui sont
acceptés :

- Une comparaison `WHERE` sur une **colonne absente** d'une ligne : la ligne sort du résultat
  sans erreur (décision 35) ; seul `IS NULL`/`IS NOT NULL` reconnaît l'absence.
- Une comparaison `WHERE` entre **deux types différents** (`WHERE id = '5'` sur `id: 5`) : vaut
  faux, jamais une erreur (décision 34) — la ligne sort du résultat.
- Une **jointure** qui rejoint la ligne de **même type** et jamais l'autre : sur le fichier piège
  (`users` portant `id: 5` **et** `id: "5"`, `wallets.user_id: 5`), la jointure rejoint `id: 5`,
  **jamais** `id: "5"` (décision 34). C'est la preuve du fichier piège (MVP §6), aucun refus. Ce
  fichier **charge** (le palier 1 l'accepte : `user_id: 5` trouve bien `id: 5`).
- Une ligne dont la **relation devinée est absente ou `null`** sort de la jointure interne sans
  erreur (décision 35, palier 1 D5).
- Un résultat **vide** (aucune ligne ne passe le filtre) : un jeu de lignes à zéro ligne,
  jamais un refus.
- Un nom de table ou de colonne **hors norme** (espace, mot réservé) désigné entre guillemets
  doubles : lu tel quel, sensible à la casse (§8.3, D2). Une colonne qualifiée **sans** jointure
  (`SELECT users.id FROM users`) est acceptée, en-tête non qualifié (D6).
- Un **nombre signé** en littéral (`WHERE balance < -1`) est un littéral, pas de l'arithmétique.
- `COUNT(*)` sur zéro ligne : rend `0` ; `SUM(col)` sur zéro valeur présente : rend une cellule
  **absente** (comme SQL sur un ensemble vide), jamais `0` (D5).

**Parité sur le refus.** Pour chaque cas Q1–Q8, le message est le même texte depuis PHP et Go
(décision 25). Un écart est un défaut consigné au relevé de parité.

## 7. Limites / seuils sourcés

**Aucun seuil chiffré ne conditionne un refus de requête** : ni longueur du texte, ni nombre
de lignes rendues, ni profondeur du filtre. Le seul débordement refusé est arithmétique (Q8,
la somme d'entiers hors `i64`) — une propriété du type, pas une limite de produit. Le banc de
durée (§17.2) reste aux paliers 3–4. Le palier 2 se joue sur une seule machine, une famille ×
deux langages, comme le palier 1.

## 8. Hors périmètre explicite

| Hors du palier 2 | Où ça entre |
|---|---|
| Modifier l'état en SQL (`INSERT`/`UPDATE`/`DELETE`) et ses refus (§8.4) ; casser une relation (§8.7) ; toute réécriture du fichier (§8.6) | Palier 3 — refusés ici comme hors sous-ensemble (Q2) |
| Transactions : ouvrir, valider, abandonner (§8.5) | Palier 4 |
| Le rechargement à la demande (§8.9) | Hors MVP |
| `ORDER BY` (le tri), `GROUP BY`, `HAVING`, `LIMIT`, `OFFSET`, `DISTINCT`, `LEFT/RIGHT/FULL JOIN`, auto-jointure, alias, sous-requêtes, `IN`, `LIKE`, `BETWEEN`, `NOT`, arithmétique, `AVG`/`MIN`/`MAX`, `CASE`, `CAST`, `UNION` | Hors sous-ensemble au lancement (Q2) ; rentrent sur **1** demande au journal des retours. `ORDER BY` est coupé par le MVP §4 (« faute de tri »), pas par le brief §8.3 qui l'incluait. |
| Deux lectures simultanées sur une instance, ou une lecture après libération de l'instance | Sans objet au palier 2 : interroger ne change pas l'état (point 4) ; la coordination entre instances reste hors périmètre comme au palier 1 (§12.2, décision 7) |
| Le pont JavaScript/TypeScript | Hors MVP (MVP §4) |
| Le banc de durée (§17.2) sur les requêtes | Paliers 3 et 4 |
| GlueSQL et son store `MemoryStorage` | **Sortis de la pile** (point 9, D3) ; l'`ARCHITECTURE.md` est en désync à réviser |

## 9. Décisions restantes et qui tranche

| # | Décision | Tranché par défaut, et pourquoi | Qui peut réviser |
|---|---|---|---|
| D1 | **Étendue exacte du sous-ensemble SQL.** | Fixé par ce cadrage (produit) et l'architecture (faisabilité), comme le concilie le brief §8.3 : un `SELECT` avec `FROM` **obligatoire** ; au plus une jointure inner par relation devinée vers l'`id` de la table visée, `ON` obligatoire, **jamais** une table avec elle-même ; `WHERE` (col–littéral, `= <> < <= > >=`, `IS [NOT] NULL`, `AND`/`OR`, parenthèses, logique à deux valeurs) ; `COUNT(*)`/`COUNT(col)`/`SUM(col)`. Un **nombre signé** est un littéral ; un `NULL` en littéral de comparaison et un entier hors capacité sont refusés (Q2, Q1). **Pas de tri** : `ORDER BY` est hors sous-ensemble (MVP §4). Tout le reste refusé nommément (Q2). | Le fondateur (produit) ; l'architecture (faisabilité). |
| D2 | **Désignation d'un nom hors norme.** | Guillemets doubles ANSI, échappés par doublement ; nom nu pris tel quel, **sensible à la casse** (le nom vient de la clé YAML, §8.3, aucune normalisation). Backticks/crochets → SQL invalide (Q1). | L'architecture (dialecte). |
| D3 | **GlueSQL sort de la pile ; `sqlparser` analyse, l'exécuteur vit dans le domaine.** | GlueSQL 0.20 convertit les types de façon asymétrique (`i = s` vrai, `s = i` faux ; `5 = 5.0` vrai) et contredit la règle unique sur chaque point ; l'envelopper d'une ACL serait plus gros que l'exécuteur maison pour ce sous-ensemble. `sqlparser` 0.54.0 (celui qu'emploient GlueSQL/DataFusion) rend l'AST ; l'exécution s'écrit dans `memless-domain`. **Conséquence** : `ARCHITECTURE.md` (§1, §2, §4, §7, §8, §9, §11.4) en désync, à reprendre au chantier d'architecture. | Le fondateur (règle produit imposée) ; l'architecture (à répercuter). |
| D4 | **Ordre des lignes.** | L'ordre du fichier de la table `FROM`. Avec jointure : les lignes suivent l'ordre de `FROM` ; **quand la table jointe porte la relation** (un-à-plusieurs), les lignes jointes d'une même ligne source suivent l'ordre du fichier de la table jointe. Il n'y a **pas d'autre ordre possible** : le tri est hors sous-ensemble (MVP §4, §6). | Le fondateur (observable). |
| D5 | **`SUM`/`COUNT` sur ensemble vide.** | `COUNT(*)`/`COUNT(col)` → `0` ; `SUM(col)` sans valeur présente → cellule **absente** (comme SQL sur un agrégat vide), jamais `0`. | Le fondateur. |
| D6 | **En-têtes du résultat.** | Sans jointure : le nom de colonne tel qu'écrit dans le fichier (une colonne qualifiée `users.id` rend l'en-tête `id`) ; avec jointure : toujours qualifié `table.column` ; `*` rend **toutes les colonnes de la table `FROM` puis celles de la table jointe**, chacune dans l'ordre de première apparition dans le fichier ; agrégats : en-tête canonique `COUNT(*)`, `COUNT(col)`, `SUM(col)`. | Le fondateur (observable) ; l'architecture (forme exacte des ponts). |
| D7 | **Format d'échange et cycle de vie à la frontière C ABI.** | Accesseurs de résultat typés (nombre de colonnes/lignes, nom de colonne, cellule par `(ligne, colonne)` avec un discriminant de type), plutôt qu'un JSON à re-analyser dans chaque pont : un seul contrat, testé une fois, sans dépendance de sérialisation. Le jeu de lignes est un objet **détenu** que l'appelant **libère** par un geste dédié (comme il libère une instance, palier 1) ; il ne survit pas à sa libération. Un accès hors bornes `(ligne, colonne)` est une **erreur de programmation du pont**, jamais un refus de requête. La version d'ABI, le dialecte et le numéro exact relèvent du chantier d'architecture. | L'architecture (cœur/pont). |
| D8 | **Refus ajoutés hors §8.3 : `SUM` non numérique (Q7), et sur quelles lignes.** | Le brief §8.3 n'énumère que SQL invalide, nom absent, hors sous-ensemble ; §8.1 dit qu'une comparaison entre types « vaut faux, jamais une erreur ». Refuser la somme mêlée (Q7) est un **choix produit ajouté** (sommer convertirait — décision 34), marqué comme le palier 1 marquait B7 (D19). Q7 s'établit sur **les lignes retenues** (après `WHERE`), dans l'ordre du fichier ; la ligne nommée est la première où un second type apparaît. | Le fondateur (ajouter/retirer ce refus). |
| D9 | **Sens de la jointure et cardinalité.** | Les deux sens sont admis (`FROM wallets JOIN users` et `FROM users JOIN wallets`) tant que le `ON` est `relation devinée = id` (D1) ; la cardinalité un-à-plusieurs du second sens est ordonnée par D4. Le MVP ne joue que `wallets → users`, mais rien n'oblige à interdire l'autre sens, bien défini. | Le fondateur (observable). |
| D10 | **Glossaire à aligner.** | Inscrire les termes neufs du point 3 ; **généraliser « Refus »** (le glossaire le définit comme l'issue d'un chargement — le palier 2 en crée une seconde sorte, sur une requête) ; rappeler que la langue des messages est l'**anglais** (palier 1 D3) — les phrases françaises de Q1–Q8 nomment la règle, pas le texte littéral. | L'architecture. |
