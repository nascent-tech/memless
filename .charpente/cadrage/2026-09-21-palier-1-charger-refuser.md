---
type: cadrage
titre: Palier 1 — Charger et refuser
slug: palier-1-charger-refuser
cree_le: 2026-09-21T15:02:41+0000
mis_a_jour_le: 2026-09-21T15:20:52+0000
branche: main
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../mvp/00-index.md) › **cadrage — Palier 1 — Charger et refuser** › [conception — Palier 1 — Charger et refuser](../conceptions/2026-09-21-palier-1-charger-refuser/00-index.md) › [plan — Palier 1 — Charger et refuser](../plans/2026-09-21-palier-1-charger-refuser/00-index.md)  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/mvp/00-index.md

Cadrage du **palier 1 du MVP** — « Charger et refuser ». Il ne couvre que le
**chargement** d'un fichier YAML et son **refus**, depuis PHP puis Go. Les requêtes (§8.3),
les écritures (§8.4), les transactions (§8.5) et la réécriture (§8.6) sont hors périmètre
(point 8). Les références `§`/`décision` renvoient au brief validé, sauf mention du MVP.

## 1. Acteur et déclencheur

**Acteur.** Le **code appelant** (§6) — le test automatisé écrit en PHP, puis en Go, par
l'auteure du fichier ; au palier 1, une seule personne tient les deux rôles (MVP §1).
L'auteure agit en amont : elle écrit le fichier YAML, uniquement des données, et décide
seule de sa forme sans la déclarer (§6, §8.1). Aucun « utilisateur final » n'existe (§6).

**Déclencheur.** Le code appelant **crée une instance** du moteur en lui désignant un chemin
de fichier ; c'est ce geste unique qui déclenche le chargement. Il n'y a pas de geste séparé
« charger » après « créer » : une instance n'existe que démarrée sur un fichier accepté, ou
n'existe pas (§13 : un refus « ne produit jamais un moteur démarré »). Le rechargement à la
demande (§8.9) n'est pas dans ce MVP (MVP §3) : le seul chargement est celui de la création
d'instance.

**Ce que l'acteur observe.** Soit une instance démarrée, soit un refus qui nomme la règle,
la table et la ligne (§10). Rien d'autre : le brief exclut tout inventaire consultable de ce
qui a été deviné (§14) — le refus est la seule preuve du devinage (MVP §6).

## 2. Besoin, séparé de la solution

| Formulé en solution | Le besoin dessous |
|---|---|
| « Charger un fichier YAML » | Qu'un fichier de données écrit à la main, sans aucune description de sa forme, devienne un état interrogeable — sans payer le coût d'un schéma, d'un script de données et de fixtures tenus d'accord à trois endroits (§7). |
| « Refuser un fichier incohérent » | Qu'une erreur d'écriture manuelle — une relation qui vise une ligne absente, deux lignes qui se prennent pour la même, une ligne qu'on ne peut pas désigner — soit découverte **au chargement**, en nommant exactement ce qui cloche, plutôt qu'à l'usage quand un test échoue sans dire pourquoi (décision 22). C'est la condition pour que « le fichier est la vérité » tienne : un fichier accepté est un fichier dont toute relation est fiable (§8.2, §10). |
| « Ignorer le résidu » | Qu'un arrêt brutal survenu lors d'une écriture antérieure ne mette jamais en doute quel fichier est le vrai (décision 17) — l'auteure n'a pas à nettoyer son répertoire pour que son test reparte. |
| « Charger depuis PHP puis Go » | Qu'une équipe multi-langages n'ait jamais deux lectures du même fichier : même acceptation, même refus, même message (§5.1, §8.8). Le palier 1 est la **première épreuve** de cette parité (§17.1), pas sa tranche. |

Ce que le palier 1 ne prouve pas : que l'état chargé est *utilisable* (paliers 2–4). Il
prouve qu'on sait dire **oui** au bon fichier et **non**, de la même voix, à tous les mauvais.

## 3. Vocabulaire

Les termes marqués **[neuf]** sont à inscrire au glossaire (point 9, D14). Chaque terme prend
le nom le plus court qui le désigne.

| Terme | Définition | Voisin à ne pas confondre |
|---|---|---|
| **Fichier** | Le fichier YAML désigné par le chemin, seule source de vérité des données (§7). | Le **résidu**, qui n'est jamais le fichier. |
| **Table** | Une clé de premier niveau dont la valeur est une liste de lignes (§6.1) ; existe dès que sa clé est présente, même avec une liste vide (décision 21, 37). | Une **table vide** est une table ; un **fichier sans table** est un refus. |
| **Ligne** | Un élément de la liste d'une table ; un ensemble de champs (§6.1). | La **colonne**, qui est un champ d'une ligne. |
| **Colonne** | Un champ d'une ligne ; n'existe que tant qu'au moins une ligne de sa table la porte (décision 37). | Le **type**, qui est celui d'une valeur, pas de la colonne (décision 36). |
| **Scalaire** [neuf] | Une valeur d'un des quatre types reconnus : texte, entier, décimal, vrai/faux (§8.1). Une date sans guillemets est du texte. | La **valeur imbriquée**. |
| **Valeur imbriquée** [neuf] | Une valeur qui est elle-même une liste ou un ensemble de champs là où un scalaire est attendu (§8.1, décision 35). Refusée. | Une **ligne** (ensemble de champs légitime, mais au niveau d'une table). |
| **Valeur absente** | Une colonne qu'une ligne ne porte pas, **ou** qu'elle porte avec `null` : indiscernables (décision 35). | Une valeur vide mais présente (`""` est un texte présent). |
| **Structure devinée** | La forme déduite des seules données — type de chaque valeur, colonne `id`, relations devinées — redérivée à chaque vérification (§6.1, décision 37). | Le **schéma**, mot interdit (§6.1). |
| **`id`** | L'unique colonne requise sur chaque ligne ; texte ou entier (décision 30) ; identifie la ligne sans ambiguïté dans sa table. | Une colonne **`xxx_id`**, qui est une relation devinée, pas un identifiant. |
| **Relation devinée** | Une colonne dont le nom finit par `_id`, pointant vers la table nommée par ce qui précède, au pluriel par simple ajout d'un `s`, **si cette table existe** (décision 31). Peut viser sa propre table. | La **clé étrangère** déclarée, qui n'existe pas ici. Un `_id` sans table cible = colonne ordinaire. |
| **Règle de comparaison** | Deux valeurs de types différents ne sont jamais égales ni ordonnées : `"5"` ≠ `5`, `5` ≠ `5.0` (décision 34). Gouverne l'unicité d'un `id` et la reconnaissance d'une relation ; règle produit imposée par Memless, jamais héritée d'un moteur (§5.2). | La **ressemblance** d'écriture, qui ne compte jamais. |
| **Contrainte** | Une condition que la structure devinée impose : un `id` par ligne, unique dans sa table ; une relation devinée qui vise une ligne existante (§6.1). | La **règle de comparaison**, qui dit *comment* on décide qu'une contrainte tient. |
| **Structure cohérente / incohérente** [neuf] | Cohérente : la structure devinée ne se contredit pas. Incohérente : au moins une contrainte est violée (§8.2, décision 22). | Un **fichier irrecevable** (mal formé) : la cohérence ne se pose pas encore. |
| **Fichier irrecevable** [neuf] | Refusé avant toute vérification de cohérence : introuvable, illisible, vide, YAML invalide, sans table, forme de table/ligne invalide, valeur imbriquée, clé dupliquée (décision 21). | La **structure incohérente** (forme correcte, données qui se contredisent). |
| **Chargement** [neuf] | Le geste unique par lequel une instance lit le fichier, devine sa structure, vérifie sa cohérence et démarre si tout tient — en entier ou pas du tout (§8.1, §8.2, §10). | Le **rechargement** (§8.9), même lecture sur une instance déjà vivante — hors palier. |
| **Refus (de chargement)** [neuf] | L'issue où aucune ligne n'entre en mémoire et aucune instance ne démarre, avec une erreur qui nomme la règle, la table et la ligne quand elles existent (§10, décision 21). | Une **instruction refusée** et une **validation qui échoue** (§8.4, §8.5) — réservés aux transactions, hors palier. |
| **Message de refus** [neuf] | Le texte d'un refus : règle, table, ligne. Fait partie de la parité : identique d'un langage à l'autre (décision 25). | Un **message générique** (« fichier invalide »), interdit (§10). |
| **Résidu** [MVP §6 emploie déjà le terme ; le brief dit « fichier à part »] | Le fichier à part qu'une écriture antérieure a commencé et qu'un arrêt brutal a laissé sans substitution ; nom reconnaissable ; jamais officiel, jamais confondu avec le fichier, jamais touché par Memless (§8.6, décision 17). | Le **fichier**. |
| **Fichier de départ** [neuf] | Le fichier du §8.1 augmenté d'un second portefeuille : `users` (`01H7B2`, `01H7B3`), `wallets` (`w_123` → `01H7B3`, `w_124` → `01H7B2`), montants entiers (MVP §1). Le fichier valide du palier 1. | Une **fixture réelle** (§7 du MVP). |
| **Batterie** [neuf] | Les fichiers que le palier 1 doit refuser, un par cas du MVP §6, plus le fichier de départ accompagné d'un résidu. | Le **banc** (§17.2, mesure de durée, paliers 3–4). |
| **Instance** | Une copie en mémoire indépendante détenue par un appelant (§6.1, §12.2) ; au palier 1, démarre ou pas. | Le **cœur**. |
| **Cœur commun** | Le moteur unique dont les langages partagent le comportement (§5.1) ; c'est lui qui lit le YAML et devine, jamais un pont. | Le **pont**. |
| **Pont** | L'accès natif d'un langage au cœur ; traduit sans décider (§5.1). Au palier 1 : PHP et Go (MVP §4). | Le **cœur** — un pont qui « décide » est un défaut. |
| **Parité** [neuf] | Sur le même fichier, deux ponts produisent la même issue : même acceptation, ou même refus avec le même message (§10, décision 25). Un écart est un défaut, jamais une variante. | Le **relevé de parité**, la trace qui la consigne. |
| **Relevé de parité** [neuf] | La trace tenue par le fondateur, ouverte au palier 1 : un geste de la batterie × langage × famille de systèmes, avec l'issue observée (MVP §6, §8). | Le **journal des retours**. |

**Termes exclus** (§6.1) : « schéma », « invariant », « ACID », « atomique », « commit »
(hors Git), « librairie ».

## 4. Événements au passé sur une chronologie

Les faits observables par le code appelant, dans l'ordre. Les étapes internes (« structure
devinée », « cohérence vérifiée ») ne sont **pas** des événements : le brief exclut tout
inventaire de ce qui a été deviné (§14) — seule l'issue est observable.

| # | Événement | Origine | Ce qui le rend observable |
|---|---|---|---|
| 0 | **Chargement demandé** | Commande — le code appelant crée une instance sur un chemin. | Le geste lui-même. |
| 1 | **Résidu ignoré** | Politique — *dès qu'*un fichier au nom reconnaissable du résidu accompagne le fichier désigné, *alors* Memless ne le lit pas, ne le confond pas avec le fichier, n'y touche pas (décision 17). | Le résidu est toujours là, inchangé, après le chargement ; le chargement a porté sur le fichier réel. |
| 2a | **Chargement refusé** | Politique — *dès qu'*une raison de refus (point 6) est établie, *alors* le chargement s'arrête, aucune ligne n'entre, aucune instance ne démarre (§13, §10). | Une erreur nommant la règle, la table et la ligne quand elles existent. |
| 2b | **Fichier chargé** (instance démarrée) | Politique — *dès que* le fichier est recevable **et** la structure devinée cohérente, *alors* l'instance démarre sur l'état entier. | Une instance utilisable ; au palier 1, l'absence de refus est la seule preuve (aucune requête ne la confirme — palier 2). |

**Frontières.** 2a et 2b sont exclusifs et l'un survient toujours : pas de troisième issue
(chargement « partiel », « avec avertissement ») (§10, §13). Aucun système externe ni
échéance n'intervient : pas de réseau, pas de délai, pas de port (MVP §5). Le seul système
externe est le système de fichiers, qui n'émet rien : c'est Memless qui le lit. **Aucune
écriture ne se produit au chargement** — charger ne réécrit jamais le fichier (seule une
transaction validée le réécrit — §8.6, décision 5), même pour le normaliser (la normalisation
du MVP §1 est la conséquence d'une *transaction validée*, palier 3).

## 5. Contexte(s) et sorte

**Contexte borné : le moteur.** Tout le MVP vit dans ce seul contexte (MVP §6) ; le palier 1
en exerce §8.1 et §8.2. Il n'y a pas de second contexte : le fichier n'est pas un contexte
mais la source de vérité que le moteur lit ; les ponts ne sont pas des contextes mais des
traducteurs sans décision (§5.1). La seule communication est **entrante** — le code
appelant, via un pont, demande un chargement — et la seule réponse est l'issue du point 4.

**Sorte : `cœur de métier`.** Le chargement porte tout ce qui distingue Memless d'un simple
lecteur YAML : deviner une structure depuis les seules données (§3.1, §8.1), reconnaître une
relation par convention de nom (décision 31), appliquer la règle de comparaison unique
(décision 34), refuser en nommant la règle (§10). Ce sont des règles produit imposées au
moteur (§5.2), qui doivent vivre dans le cœur et nulle part ailleurs (§5.1) — la définition
d'un cœur de métier. `générique` aurait convenu à « lire du YAML » seul ; mais ce que le
palier cadre est le **devinage et le refus**, qui n'existent dans aucun composant du marché.

## 6. Refus — tous les états impossibles, avant le nominal

Cœur du palier. Chaque refus : aucune ligne n'entre en mémoire, aucune instance ne démarre,
le fichier n'est pas touché, l'erreur nomme la règle et ce qu'elle peut nommer (§10, §13,
décision 21). Un fichier n'est jamais chargé à moitié. Les refus sont énumérés dans **l'ordre
où ils s'établissent** — cet ordre décide quel message sort quand un fichier cumule plusieurs
fautes (point 9, D1).

**A. Le fichier n'est pas lisible** (§8.1, §13, décision 21)

| # | État impossible | Règle violée (ce que le message nomme) | Nomme aussi |
|---|---|---|---|
| A1 | Le chemin ne désigne aucun fichier ordinaire — rien à ce chemin, ou un répertoire, ou un autre non-fichier. | « Le chemin ne désigne aucun fichier. » | Le chemin ; pas de table ni de ligne. |
| A2 | Le fichier ordinaire existe mais n'est pas lisible (droits). | « Le fichier n'est pas lisible. » | Le chemin. |
| A3 | Le fichier est vide (aucun contenu, ou uniquement des blancs). | « Le fichier est vide. » | Le chemin. |
| A4 | Le contenu ne se décode pas (encodage invalide, BOM inattendu) ou ne s'analyse pas comme un YAML syntaxiquement valide. | « Le YAML est invalide. » | Le chemin ; la position dans le texte si le dialecte retenu la fournit (§8.1 : dialecte = conception, D9). |
| A5 | Seul un résidu est présent, le fichier réel manque. | = A1. Le résidu ne remplace **jamais** le fichier (décision 17). | Le chemin du fichier réel, pas celui du résidu. |

**B. Le fichier est lisible mais n'a pas la forme d'une base de données** (§8.1, §13, décision 21)

| # | État impossible | Règle violée | Nomme aussi |
|---|---|---|---|
| B1 | YAML valide qui ne déclare aucune table (document vide, uniquement des commentaires, un scalaire à la racine, ou une liste à la racine). | « Aucune table n'est déclarée. » Memless devine depuis des données, pas depuis rien : il faut au moins `users: []`. | Le chemin. |
| B2 | Une clé de premier niveau dont la valeur n'est pas une liste de lignes (scalaire, ensemble de champs, ou nulle — `users:` sans rien). | « La table n'est pas une liste de lignes. » | La table. |
| B3 | Un élément d'une liste de table qui n'est pas un ensemble de champs (scalaire, liste). | « La ligne n'est pas un ensemble de champs. » | La table ; la ligne par sa position (pas d'`id` lisible). |
| B4 | Une valeur de champ qui est une liste ou un ensemble de champs, là où un scalaire est attendu. | « La valeur est imbriquée : aucun type reconnu. » (décision 35) | La table, la ligne, la colonne. |
| B5 | Une clé de table écrite deux fois au premier niveau. | « La clé de table est dupliquée. » Laisser la dernière gagner serait deviner (§8.1). | La table. |
| B6 | Une clé de colonne écrite deux fois dans une même ligne. | « La clé de colonne est dupliquée. » | La table, la ligne, la colonne. |
| B7 | Une clé de table ou de colonne qui n'est pas un texte (entier, vrai/faux, `null`, ou structure — `5: []`, `true: x`, `? [a]`). | « La clé n'est pas un texte. » Un nom de table ou de colonne vient d'une clé YAML (§8.3), qui doit se lire comme un texte. | La table quand elle est nommable ; sinon la position. |

**C. Le fichier a la bonne forme, mais la structure devinée se contredit** (§8.2, §13,
décision 22) — chaque règle est éprouvée avec la règle de comparaison (décision 34), l'ordre
de parcours et l'occurrence nommée étant fixés par D1

| # | État impossible | Règle violée | Nomme aussi |
|---|---|---|---|
| C1 | Une ligne sans colonne `id` — y compris `id: null`, indiscernable d'une absence (décision 35). | « Chaque ligne porte un `id`. » (décision 30) | La table ; la ligne par sa position. |
| C2 | Un `id` qui est un nombre décimal. | « Un `id` est un texte ou un nombre entier. » (décision 30 ; un décimal n'est pas garanti exact, décision 20) | La table, la ligne par position, la valeur fautive. |
| C3 | Un `id` qui est un vrai/faux. | Même règle que C2. | Idem. |
| C4 | Un `id` imbriqué (liste ou ensemble de champs). | = B4, établi avant C : la valeur imbriquée est refusée pour ce qu'elle est, avant qu'on regarde si c'est un `id`. | Table, ligne, colonne `id`. |
| C5 | Deux lignes d'une même table portant le même `id` — *même type et même valeur* (`5` et `5`, `"a"` et `"a"`). | « Deux lignes d'une même table n'ont jamais le même `id`. » (décision 34) | La table, l'`id` en cause, et les lignes en collision **par leur position** (elles partagent l'`id`, qui ne les distingue pas — D2). |
| C6 | Une relation devinée dont la valeur ne correspond à aucun `id` de la table visée — y compris quand une ligne « ressemble » (`user_id: 5` vers `id: "5"`) : types différents, jamais égaux (§8.1, décision 34). | « Une relation devinée pointe vers une ligne qui existe. » (décision 22) | La table porteuse, sa ligne (par `id`), la colonne, la table visée, la valeur orpheline. |
| C7 | Une relation devinée vers sa propre table dont la valeur ne trouve aucune ligne (`users.user_id` orphelin). | = C6 ; l'auto-référence n'est pas un cas à part (§8.1). | Idem. |

**Ce qui n'est PAS un refus** — les états qu'on pourrait croire impossibles et qui sont
acceptés, à dire pour que personne ne les ajoute :

- Une table dont la liste est vide (`users: []`) : table déclarée, rien à vérifier (décision 21, 22).
- `users: [{id: 5}, {id: "5"}]` : **deux** lignes, types différents jamais égaux (décision 30, 34). C'est le fichier piège du palier 2 — il **charge**.
- Une même colonne avec des types différents d'une ligne à l'autre (décision 36).
- Une ligne qui omet une colonne que d'autres lignes portent — aucune colonne obligatoire hors `id` (§12.2, décision 32).
- Une colonne `stripe_id` sans table `stripes` : colonne ordinaire, aucune vérification (§8.1) — le cas de fausse détection surveillé en §17.3 b, jamais un refus tant que la table n'existe pas.
- Une relation devinée absente ou `null` sur une ligne : valeur absente, aucun pointeur, rien à vérifier (décision 35 ; point 9, D5).
- Une valeur `null` sur une colonne ordinaire : colonne absente pour cette ligne (décision 35).
- Une date sans guillemets : du texte (§8.1). Un nom de table ou de colonne hors norme (espace, mot réservé SQL) : accepté tel quel (§8.3).
- Un résidu présent à côté du fichier : ignoré, jamais un refus, jamais supprimé (décision 17). Si le chemin donné pointe **directement** sur un fichier portant le nom réservé du résidu, il est lu comme n'importe quel YAML — le nom réservé n'écarte un fichier que lorsqu'il en accompagne un autre (point 9, D11).
- Un fichier commenté ou mis en forme à la main : lu sans réécriture ; la perte de mise en forme n'arrive qu'à la première réécriture (décision 26, palier 3).

**Parité sur le refus.** Pour chaque cas A–C, le message est le même texte depuis PHP et Go
(décision 25) ; un pont ne traduit ni ne reformule, il rend le message dans la forme d'erreur
naturelle de son langage (exception, valeur d'erreur) sans en changer le contenu (§8.8). Un
écart de texte est un défaut consigné au relevé de parité.

## 7. Limites / seuils sourcés

**Aucune valeur chiffrée ne conditionne un refus au chargement** (§12.3) : ni taille maximale
de fichier, ni nombre maximal de tables, de lignes ou de colonnes, ni longueur d'un `id`, ni
délai. Un fichier n'est jamais refusé parce qu'il est « trop gros » ; ce palier n'invente
aucun seuil.

Ce qui reste incertain est une **mesure**, pas une limite : la taille à partir de laquelle un
chargement complet ralentit de façon perceptible vit au banc §17.2, joué aux paliers 3 et 4,
jamais au palier 1 (MVP §4, §8).

**Anti-YAML-bomb (« Budget »).** Aucune règle produit — brief, MVP — ne pose de protection
chiffrée contre un YAML construit pour exploser en mémoire (ancres et alias répétés) ; §12.3
dit qu'aucun refus chiffré ne conditionne le chargement. Mais l'analyseur YAML retenu par
l'`ARCHITECTURE.md` porte un **budget par défaut** qui refuse au-delà d'une profondeur ou d'un
nombre d'alias : laissé tel quel, il refuserait un fichier légitime riche en ancres, par un
chemin hors A–C et hors parité, rendant §12.3 faux en pratique. Ce cadrage **ne pose aucun
seuil** ; il tranche que ce budget hérité doit être réconcilié avant le palier 1 (point 9,
D10) — soit désactivé pour tenir §12.3, soit assumé comme un refus **nommé** à sa place dans
l'ordre D1.

Le palier 1 se joue sur une seule machine ; les familles de systèmes retenues par
l'architecture n'entrent qu'au palier 4 (MVP §4). Le relevé de parité du palier 1 porte donc
une seule famille × deux langages.

## 8. Hors périmètre explicite

| Hors du palier 1 | Où ça entre |
|---|---|
| Interroger l'état en SQL — filtre, jointure, compte, somme, et leurs refus (§8.3) ; le fichier piège `id: 5` / `id: "5"` en lecture | Palier 2 |
| Modifier l'état en SQL et ses refus (§8.4) ; casser une relation devinée (§8.7) ; toute réécriture du fichier, écriture à part puis substitution (§8.6) ; l'échec disque ; la normalisation d'un fichier commenté (décision 26) | Palier 3 |
| Transactions : ouvrir, valider, abandonner, instruction refusée, validation qui échoue, deuxième ouverture, validation sans transaction (§8.5) ; l'historique Git comme journal d'audit (§9.1) ; le `git diff` | Palier 4 |
| Le rechargement à la demande (§8.9) et ses interdits | Hors MVP (MVP §3) ; **1** demande explicite au journal des retours pour rentrer |
| Trancher l'hypothèse §17.1 (parité réelle de PHP) : le palier 1 ouvre le relevé, ne le clôt pas | Palier 4, sur toutes les familles de systèmes |
| Le pont JavaScript/TypeScript | Hors MVP (MVP §4) ; **1** développeuse du troisième langage après 0 écart PHP/Go |
| Plusieurs instances, processus ou fils sur le même fichier ; édition à la main pendant qu'une instance est vivante ; tout verrou ou coordination (§12.2, §14) | Hors périmètre au lancement ; la sûreté mémoire tient (décision 7), l'ordre des données n'est pas garanti |
| La **libération d'une instance** — geste terminal après « instance démarrée » (§8.5, dernier alinéa) | Comportement du produit, mais pas un refus de chargement ; la batterie l'exige **entre** l'exécution PHP et l'exécution Go (jamais deux instances vivantes sur le même fichier — MVP §3, et la ligne « Plusieurs instances… » ci-dessus) |
| Un inventaire de ce qui a été deviné (§14) | Exclusion durable |
| Une déclaration explicite de relation quand la convention ne suffit pas (§14, décision 31) | Retour §17.3 b |
| Un mode lecture seule (décision 3) | Retour §17.3 a |
| Le nettoyage du résidu par Memless | Jamais (décision 17 : « sans jamais y toucher ») ; l'ignorer côté Git est une pratique d'équipe |
| Le banc de durée (§17.2) au chargement | Paliers 3 et 4 |
| Le dialecte YAML exact et la lecture des scalaires limites (§8.1) | Architecture, pas ce cadrage |

## 9. Décisions restantes et qui tranche

Chaque décision par défaut a été prise par ce cadrage selon le principe le plus prudent
aligné sur le brief. Elle tient tant que la personne nommée ne la révise pas.

| # | Décision | Tranché par défaut, et pourquoi | Qui peut réviser |
|---|---|---|---|
| D1 | **Ordre des refus quand un fichier cumule plusieurs fautes.** | Un refus nomme **une** règle et **une** occurrence, de façon déterministe (la parité, décision 25, l'exige). **Par règle d'abord** : A → B → C (fichier, forme, cohérence), et dans C l'ordre C1–C3 (`id` présent et bien typé), C5 (unicité), C6–C7 (relations) — chaque règle est éprouvée sur tout le fichier avant la suivante (on ne juge pas la cohérence d'une forme non encore acceptée). **À règle égale**, la première occurrence dans l'ordre du fichier : table, puis ligne, puis colonne. Un budget hérité (D10) prend place au niveau où il refuse. | Le fondateur, sur avis de l'architecture (l'ordre est observable, donc produit). |
| D2 | **Comment une ligne est nommée dans un message.** | Une ligne dont l'`id` est un texte ou un entier valide est nommée par son `id` ; une ligne sans `id` (C1) **ou dont l'`id` est présent mais refusé** (C2 décimal, C3 vrai/faux) par sa position dans la liste de sa table (première = ligne 1) et la valeur fautive ; **en cas de doublon d'`id` (C5), les lignes en collision par leur position**, l'`id` partagé ne les distinguant pas. §10 exige « quelle ligne » ; la position est la seule désignation qui ne devine rien. | Le fondateur. |
| D3 | **Langue et forme du texte des messages.** | Le cadrage fixe le **contenu** (règle, table, ligne, colonne, valeur fautive), pas la formulation. **Langue : anglais**, par la règle de langage du projet (`charpente:language` : erreurs et chaînes exposées par l'API en anglais) — les phrases françaises entre guillemets du point 6 nomment la règle, elles ne sont pas le texte littéral du message. Seule l'identité du texte entre ponts est produit. | L'architecture pour la forme native exacte. |
| D4 | **`id: null` = ligne sans `id`.** | Refusé par C1, pas C2 : décision 35 rend `null` indiscernable d'une absence. | Le fondateur. |
| D5 | **Relation devinée absente ou `null` = pas de relation à vérifier.** | Acceptée : décision 35 (`null` = absent) et §8.1 (une colonne absente n'a aucune valeur à comparer). Conséquence : une relation « optionnelle » est possible sans rien déclarer. | Le fondateur — choix produit visible. |
| D6 | **`users:` sans valeur (nulle) = B2, refusé.** | Décision 21 exige `users: []` pour une table vide ; une valeur nulle n'est pas une liste, et l'accepter serait deviner. | Le fondateur. |
| D7 | **Fichier « vide » vs « sans table ».** | Aucun contenu / uniquement des blancs = A3 ; uniquement des commentaires ou document YAML vide = B1. Le brief distingue les deux refus (décision 21) ; frontière = « y a-t-il du YAML à analyser ». | Le fondateur. |
| D8 | **Refus hors batterie du MVP.** | A1–A4, B2, B3 sont des refus **exigés** par la décision 21 mais absents de la batterie MVP §6 ; B7 (D19) est un refus **ajouté par ce cadrage**, hors décision 21. Leur démonstration au palier 1 est recommandée, pas requise par le MVP. | Le fondateur (ajouter à la batterie, ou non). |
| D9 | **Plusieurs documents YAML dans un même fichier (`---`).** | Non tranché par le brief ; relève du dialecte YAML (§8.1). Principe prudent : refusé comme A4, jamais chargé à moitié. | L'architecture. |
| D10 | **Budget de l'analyseur YAML (anti-bomb) hérité de l'architecture.** | **Défaut tranché : le budget par défaut de l'analyseur est désactivé au palier 1**, pour tenir §12.3 (aucun refus chiffré au chargement) — le premier usage vient de fixtures écrites par la développeuse, pas d'un fichier hostile (MVP §5, §3.4). Le fondateur peut réviser vers un refus **nommé** (alors placé dans l'ordre D1) le jour où un fichier hostile devient un risque réel ; sa forme non chiffrée resterait à définir. | Le fondateur (produit) ; l'architecture (faisabilité). |
| D11 | **Nom reconnaissable du résidu, et chemin pointant dessus.** | Décision 17 impose que le résidu existe ; sa forme est de conception. Tranché ici : le nom réservé n'écarte un fichier que lorsqu'il **accompagne** le fichier désigné ; un chemin pointant **directement** sur lui le charge comme n'importe quel YAML. | L'architecture (forme du nom) ; le fondateur (le comportement « pointé directement »). |
| D12 | **Colonne nommée exactement `_id`.** | Cas non prévu. Principe prudent : convention appliquée littéralement — relation vers une table `s` si elle existe, colonne ordinaire sinon. | Le fondateur, si un fichier réel le rencontre (journal des retours). |
| D13 | **Ce que le pont fait de l'erreur.** | Message rendu tel quel dans la forme d'erreur naturelle du langage, sans traduction ni enrichissement (§5.1, décision 25). La forme native exacte est de conception. | L'architecture (cœur/pont). |
| D14 | **Glossaire d'architecture à aligner.** | « Invariant », « ACID » et « Réécriture atomique » y figurent alors que §6.1 exclut « invariant », « ACID », « atomique » ; dire « contrainte », « tout-ou-rien », « substitution d'un coup ». Les termes neufs du point 3 sont à y inscrire, **et la sorte `cœur de métier`** en tête avec la date de ce cadrage (le glossaire est le domicile durable de la sorte). | L'architecture. |
| D15 | **Sorte du contexte : `cœur de métier`.** | Tranchée au point 5 ; révisable par l'architecture si le contexte venait à se scinder. | L'architecture. |
| D16 | **Le chargement ne touche jamais le résidu — contre la proposition d'architecture.** | Décision 17 (« sans jamais y toucher ») prime : le chargement ignore le résidu et le laisse en place. L'`ARCHITECTURE.md` propose au contraire de le nettoyer au chargement suivant — désync à reprendre au chantier architecture. La batterie du palier 1 vérifie que le résidu est **toujours là, intact**, après un chargement réussi. | Le fondateur (règle produit) ; à répercuter dans l'`ARCHITECTURE.md`. |
| D17 | **Encodage non décodable = A4.** | Un fichier lisible mais non décodable (encodage invalide, BOM inattendu) est rangé en A4 (« YAML invalide »), n'étant ni « pas lisible » (A2) ni un scalaire limite. | L'architecture. |
| D18 | **Batterie du MVP : quels fichiers pour « clé dupliquée ».** | Le MVP §6 nomme un seul cas « clé dupliquée » ; la batterie porte **un fichier par cas B5 (clé de table) et B6 (clé de colonne)**, les deux étant des refus distincts. | Le fondateur. |
| D19 | **Clé de table ou de colonne non textuelle = B7, refusée.** | Refus **ajouté par ce cadrage**, hors de l'énumération de la décision 21 : une clé entière, vrai/faux, `null` ou structure ne se lit pas comme un nom, et l'accepter reviendrait à deviner un nom (un nom vient tel quel d'une clé YAML — §8.3 — mais une clé qui n'est pas un texte n'est pas un nom). Placé en fin de bloc B (D1). | Le fondateur (ajouter/retirer ce refus) ; l'architecture pour ce qu'un dialecte YAML autorise comme clé. |
