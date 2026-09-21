<!-- charpente-navigation -->
**Index** : [brief — Moteur de base de données en mémoire pilotée par YAML](00-index.md)  
**Suivant** : [5. Le cadre imposé](02-cadre-et-acteurs.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 1. Le projet en une page

Memless ne remplace pas une base de données. Ce que Memless remplace, ce sont les trois pièces
séparées — un schéma (le plan qui dit quelles tables existent et ce que chacune contient) dans des
migrations (des scripts qui construisent la base pas à pas, comme une notice de montage) ou des
modèles d'ORM (un traducteur qui fait correspondre les classes du code aux tables d'une base), un
script qui génère des données, et souvent un troisième fichier de fixtures (un jeu de données de
départ tout prêt, comme un plateau-repas préparé avant une démonstration) — qu'une développeuse doit
aujourd'hui garder d'accord à la main avant de lancer ses tests.

Un seul fichier YAML porte les données, et seulement les données : aucune section séparée ne décrit
la forme des tables. Memless devine cette forme lui-même en lisant ce qui est écrit — le type de
chaque valeur et quelles colonnes pointent vers une autre table, la colonne qui identifie une ligne
étant toujours `id` — le charge en mémoire, laisse le lire et le modifier avec du texte SQL, et réécrit ce même fichier à
chaque changement validé : après chaque transaction validée, le fichier redevient la vérité, pour
l'acteur qui l'écrit — pas une copie qu'on resynchronise après coup. Entre deux validations, c'est la
mémoire de l'instance qui fait foi, et Memless la réaligne sur le fichier à la demande (§8.9).

Ce que Memless promet, en une phrase : **un seul fichier, uniquement des données, que trois langages
peuvent interroger et modifier de façon identique — sans jamais avoir à décrire la forme des tables à
part.**

---

## 2. À qui ça s'adresse, et sur quel terrain

### 2.1 Le problème de la développeuse qui écrit des tests

Avant de lancer une suite de tests, une développeuse doit préparer les données que ces tests liront
et modifieront. Aujourd'hui, ça demande plusieurs pièces séparées, qui doivent rester d'accord entre
elles :

- **le schéma** — le plan des tables, un peu comme le plan d'un meuble en kit dit combien d'étagères
  il y a avant même qu'on y pose un livre — porté par des migrations SQL ou des modèles d'ORM (un
  traducteur qui fait correspondre les classes du code aux tables d'une base) ;
- **un script qui génère les données de départ** ;
- souvent, **un troisième fichier de fixtures**, au format JSON ou YAML, qui double une partie du
  schéma sans le dire.

Ces trois pièces vivent dans des fichiers différents, écrits à des moments différents, et rien ne
prévient quand l'une a changé sans que les autres suivent. Un champ ajouté au schéma sans être ajouté
aux fixtures ne se découvre qu'au moment où un test échoue — pour une raison qui n'a rien à voir avec
ce qu'il teste vraiment.

### 2.2 Ce qui coûte, et que personne ne voit

Une équipe qui écrit des services en JavaScript, en Go et en PHP refait ce travail trois fois : trois
moteurs de test, trois façons de préparer des données, aucune garantie que les mêmes fixtures
produisent le même comportement d'un langage à l'autre. Un scénario de test écrit et validé côté Go
n'apprend rien à qui doit reproduire le même scénario côté PHP — il recommence de zéro.

### 2.3 Le terrain

Memless est un outil gratuit et ouvert, publié sous une licence permissive (§15, décision 1) : aucune
société ne le vend, personne ne paie pour l'utiliser, et une entreprise peut l'intégrer dans son
propre logiciel, y compris fermé, sans rien devoir en retour.

Les deux autres acteurs (§6) n'ont pas de problème propre distinct : le code appelant hérite du
problème de l'auteure — préparer des données fiables pour ses tests — et la mainteneuse du moteur
existe pour que ce problème soit résolu une fois pour les trois langages, plutôt que trois fois.

---

## 3. Ce que le produit fait, et ce qu'il n'est pas

### 3.1 Ce qu'il fait

Memless lit un fichier YAML qui ne contient que des données : chaque table est directement une liste
de lignes, sans section à part pour en décrire la forme. Memless devine cette forme lui-même, comme
on reconnaît le contenu d'une boîte non étiquetée rien qu'en la soupesant et en l'ouvrant :

- **le type de chaque valeur** se lit dans son écriture — un mot ou une phrase est du texte, un
  nombre sans virgule est un nombre entier, un nombre à virgule est un nombre décimal, `true`
  ou `false` est un vrai/faux ; des guillemets forcent en texte ce qui, sans eux, se lirait comme un
  nombre ou un vrai/faux — `"5"` reste du texte là où `5` est un nombre ;
- **la clé primaire** — la colonne qui identifie chaque ligne sans ambiguïté, comme un numéro de
  sécurité sociale identifie une personne — est toujours la colonne appelée `id` ;
- **les clés étrangères** — des colonnes qui pointent vers la ligne d'une autre table, comme le
  numéro de commande imprimé sur un bon de livraison pointe vers la commande dont il parle — sont
  devinées par leur nom : une colonne `user_id` pointe vers la table `users`, si cette table existe.

Memless charge cet état en mémoire, vérifie qu'il est cohérent avec ce qu'il en a deviné, puis laisse
une développeuse l'interroger et le modifier avec du texte SQL — depuis du code JavaScript,
TypeScript, Go ou PHP. Chaque changement validé est aussitôt réécrit dans le fichier YAML d'origine.

### 3.2 Ce que ça donne, concrètement

Une développeuse qui veut tester « un virement refuse de s'exécuter si le solde est insuffisant »
écrit une seule **requête** SQL — une instruction en texte qui dit ce qu'on veut lire ou changer,
comme une phrase de commande passée au comptoir — de mise à jour, dans le langage de son service,
contre un fichier qu'elle peut lire, modifier à la main et relire dans son éditeur de texte habituel
— sans installer, démarrer ni réinitialiser une vraie base de données entre deux tests, et sans avoir
à décrire la forme de ses tables avant de pouvoir écrire ses données.

### 3.3 Pourquoi ce n'est pas copiable

Ce que Memless réunit tient en quatre traits, et nulle part ailleurs les quatre ne sont réunis dans un
seul outil : **(1)** aucune section de schéma séparée — la forme des tables est devinée depuis les
données elles-mêmes, à la seule exception de l'identifiant, toujours la colonne `id` par convention,
**(2)** y compris les relations entre tables, devinées par le nom des colonnes, **(3)** un vrai moteur SQL en mémoire, capable de lire et d'écrire, dont chaque
écriture est aussitôt reportée sur le disque, et **(4)** le même moteur utilisable nativement dans
trois langages.

Chaque trait pris seul est proche de quelque chose qui existe déjà, mais jamais réuni aux trois
autres. Deviner le **type** d'une donnée depuis un fichier existe (YamlQL, trdsql — §18), mais ces
outils devinent en lecture seule, sans écrire, et surtout **sans deviner les relations entre tables** —
YamlQL le dit explicitement : pas de clé primaire, pas de clé étrangère détectées. Un schéma
explicitement déclaré, avec ses relations, existe aussi (GroundDB, Pyrseas — §18), mais alors il
**faut** l'écrire à part, exactement ce que Memless évite. Un moteur SQL qui vit en mémoire existe
(pg-mem, sql.js, AlaSQL — §18), sans jamais rien deviner ni réécrire sur le disque. Des ponts natifs
vers trois langages à partir d'un seul cœur existent pour d'autres moteurs (SQLite, DuckDB — §18),
sans rien deviner non plus.

Deviner les relations entre tables sans qu'elles soient jamais déclarées, c'est le trait le plus
risqué des quatre (§17.3) — et c'est aussi celui qu'aucun concurrent trouvé en recherche ne tente.

### 3.4 Ce que Memless n'est pas

- **Ce n'est pas une base de données de production.** Aucune application ne doit dépendre de Memless
  pour rester en ligne — §14.
- **Ce n'est pas un serveur.** Il n'écoute aucun port, ne répond à aucune connexion réseau, et ne
  partage son état en temps réel avec personne : chaque instance du moteur détient sa propre copie
  en mémoire, quel que soit son langage — §15 décision 6.
- **Ce n'est pas un générateur de données aléatoires.** Les données sont écrites à la main par la
  développeuse ; Memless ne les invente pas — il devine seulement leur forme.
- **Ce n'est pas un ORM.** Il ne fabrique aucune classe, aucun modèle : il exécute du texte SQL et
  rend des résultats bruts.
- **Ce n'est pas un système qui impose qu'une colonne soit toujours renseignée** — à une exception
  près. Sans schéma déclaré, rien ne force une colonne à être présente sur toutes les lignes d'une
  table (§15 décision 32), **sauf `id`** : elle seule est requise sur chaque ligne, puisque c'est elle
  qui identifie la ligne (§8.1, §8.2, §15 décision 30).

---

## 4. Pourquoi le pari technique tient

Le pari : réécrire le fichier YAML entier à chaque transaction validée coûte un temps qui grandit
avec la taille totale du fichier, pas avec la taille du changement — comme redevoir recopier toute
une page d'un carnet à chaque fois qu'on corrige un seul mot, plutôt que de gommer juste ce mot. Pour
un jeu de données de test — plus proche de quelques lignes que de plusieurs millions, sans qu'aucun
seuil précis ne soit fixé (§12.3) — recopier la page entière reste, en théorie, un geste
rapide pour le cœur que l'architecture retiendra.

**Ce que ça suppose n'est pas mesuré.** Combien de temps exactement, à partir de quelle taille ce
recopiage cesse d'être négligeable, et ce que ça donne quand une suite de tests écrit des centaines de
fois de suite plutôt qu'une seule fois : ce n'est pas un fait acquis, c'est une hypothèse risquée —
§17.2, qui dit aussi ce qui la tranchera.

Ce qui casserait le pari : des fixtures dont la taille grandit sans borne avec le temps, au point que
réécrire le fichier entier devienne le geste le plus lent de la suite de tests elle-même.
