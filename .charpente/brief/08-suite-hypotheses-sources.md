<!-- charpente-navigation -->
**Index** : [brief — Moteur de base de données en mémoire pilotée par YAML](00-index.md)  
**Précédent** : [15. Les décisions, et ce que chacune écarte](07-decisions.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 16. Ce qui vient après le lancement

Ce qui est repoussé sciemment, avec la condition qu'une fonctionnalité doit franchir pour entrer :
qu'un besoin réel et constaté la réclame, plutôt qu'un besoin supposé.

| La fonctionnalité | Ce qu'elle apporte | Ce qu'elle attend |
|---|---|---|
| Un serveur partagé entre plusieurs processus ou langages | un état vraiment commun, vu en temps réel par plusieurs services à la fois | qu'un besoin réel de tests croisant plusieurs services en même temps soit mesuré, pas supposé — §14 |
| Suppression en cascade et mise à `NULL` automatique, chacune en option, en plus du refus | évite de devoir supprimer ou corriger à la main les lignes liées avant de supprimer la ligne principale | qu'un usage réel bute concrètement sur le refus systématique du §8.7 — §14 |
| Une API « façon SQL » en code natif, par langage, en plus du texte SQL | plus confortable à écrire, avec l'aide de l'auto-complétion du langage | que le texte SQL partagé soit éprouvé et stable dans les trois langages d'abord — §14 |
| Un verrou entre processus qui protège un même fichier ouvert deux fois en même temps | protège contre deux processus qui écrivent en même temps dans le même fichier | qu'un usage réel où deux processus ouvrent le même fichier en même temps soit constaté — §14 |
| Une marque de version dans le format de fichier, pour la compatibilité entre ponts | permet de détecter et de gérer un fichier écrit par une version différente des trois ponts | qu'une deuxième version des trois ponts soit publiée — §14 |
| Une façon de déclarer explicitement une relation que la convention de nom ne repère pas | couvre les cas où le nom d'une colonne ou d'une table ne suit pas la convention `_id` / pluriel | qu'un usage réel bute sur des relations que la convention ne détecte pas (§17.3) — §14 |
| Un inventaire de ce que Memless a deviné | dit quelles relations et quels types ont été reconnus, pour comprendre un refus ou une relation manquée | qu'un retour du §17.3 (b) le réclame — §14 |

---

## 17. Les hypothèses risquées

### 17.1 Le troisième langage (PHP) peut atteindre une parité réelle

**Ce qu'on tient pour vrai.** Le langage historiquement le plus difficile à servir nativement — PHP —
peut recevoir un accès aussi complet et confortable que les deux autres, avec un effort d'ingénierie
raisonnable (§5.1). **Un premier prototype existe déjà et va dans ce sens** : le même cœur, appelé
depuis PHP et depuis Go, a rendu exactement le même résultat sur des requêtes qui aboutissent et le
même message d'erreur sur une requête refusée — mais sur une seule machine, une seule famille de
systèmes.

**Ce qui casse si c'est faux.** PHP devient un citoyen de seconde zone — fonctionnalités partielles,
couche réseau déguisée en « natif », ou version bêta qui dure — ce qui viole l'exigence des trois
langages à parité posée dès la demande de départ.

**Ce qui le trancherait.** Ce que le premier prototype ne couvrait pas encore : le même comportement sur
les familles de systèmes retenues par l'architecture (`ARCHITECTURE.md` §12.3 Q7), et non une seule
machine, et sous une charge réelle de requêtes plutôt qu'un appel isolé — ce que le banc de parité,
rejoué par l'architecture sur chaque famille retenue, vérifie à chaque changement (relevé de parité).
Comment ce pont est construit relève de l'architecture ; l'hypothèse, elle,
est de produit. Elle s'appuyait déjà sur un fait observé : sur les moteurs comparables trouvés en
recherche (DuckDB, libSQL/Turso, SurrealDB), le pont PHP est systématiquement le plus fragile des
trois — §18.

### 17.2 Réécrire le fichier entier reste imperceptible aux tailles visées

**Ce qu'on tient pour vrai.** Réécrire le fichier YAML entier à chaque transaction validée reste
imperceptible pour les tailles de fixtures que vise Memless — des jeux de test, pas des volumes de
production (§15, décision 2) — §4.

**Ce qui casse si c'est faux.** La promesse d'une manipulation rapide en mémoire s'effondre dès que
les suites de tests grossissent, et la réécriture devient elle-même le geste le plus lent de la
suite — au point de décourager l'usage que Memless vise en premier. Le risque n'est pas seulement la
taille du fichier : une suite qui écrit des centaines de fois, chaque écriture isolée réécrivant le
fichier entier (§10), multiplie ce coût par le nombre d'écritures, pas seulement par leur taille — et
une suite qui recharge l'état entre chaque test (§8.9) paie le même coût de lecture complète à chaque
rechargement.

**Ce qui le trancherait.** Un banc de mesure qui fait varier la taille des fixtures, le nombre
d'écritures, le nombre de transactions ouvertes et le nombre de rechargements d'une même suite, sur le
cœur effectivement retenu par l'architecture — c'est cette combinaison précise, mesurée, et non une
intuition générique, qui doit trancher.

### 17.3 Les développeuses acceptent les deux conventions imposées

**Ce qu'on tient pour vrai.** Deux conventions que Memless impose sont acceptées par des développeuses
sur de vraies fixtures : (a) qu'un test modifie durablement le fichier qu'elles ont pointé, tant
qu'elles contrôlent elles-mêmes quel fichier c'est — plutôt qu'un mode « lecture seule » protégé par
défaut (§15, décision 3) ; et (b) que les relations soient devinées par le seul nom des colonnes
(`_id`, puis le nom qui précède au pluriel en `s`), sans jamais être déclarées (§8.1, §15 décision 31).

**Ce qui casse si c'est faux.** Pour (a) : la réécriture automatique, pourtant fondation du produit et
de sa fonctionnalité décisive (§9.1), devient la première raison de ne pas l'adopter — la crainte de
voir ses fixtures modifiées par erreur. Pour (b), deux façons opposées, et la première est la plus
dure : une colonne nommée par coïncidence comme une relation — un identifiant externe tel
que `stripe_id`, si une table `stripes` apparaît par ailleurs pour une tout autre raison — serait
protégée à tort comme une vraie relation interne : au mieux, cela bloque des suppressions légitimes
(§8.7) ; au pire, si des valeurs `stripe_id` déjà écrites ne correspondent à aucune ligne de `stripes`,
**le fichier entier refuse de charger** (§8.2) — un fichier qui se chargeait la veille cesse de charger
le jour où une table sans rapport apparaît. À l'inverse, une vraie relation mal nommée, ou vers une
table au pluriel irrégulier, ne serait jamais reconnue ni protégée, laissant une ligne orpheline passer
inaperçue — exactement ce que le produit existe pour empêcher (§2.1).

**Ce qui le trancherait.** Le même signal pour les deux conventions : le retour des premières
développeuses qui l'essaient sur un vrai projet — une réécriture vécue comme une perte, une suppression
refusée de façon inattendue (fausse détection), ou une ligne orpheline découverte après coup (relation
manquée). Le produit ne rend aujourd'hui aucun inventaire de ce qu'il a deviné ; si ces retours
reviennent, un tel inventaire (§16) devient une candidate pour la feuille de route.

---

## 18. Sources

- **YamlQL** — transforme n'importe quel YAML en tables relationnelles interrogeables en SQL (via
  DuckDB) ; le schéma est **inféré** par heuristiques, mais explicitement **sans détection de clé
  primaire ni de clé étrangère**, et en lecture seule.
  [github.com/AKSarav/YamlQL](https://github.com/AKSarav/YamlQL) — consulté le 31/08/2026.
- **trdsql** — outil en ligne de commande qui exécute du SQL sur des fichiers CSV, JSON, YAML et
  autres ; aucun schéma déclaré, aucune détection de relation, aucune écriture dans le fichier
  source. [github.com/noborus/trdsql](https://github.com/noborus/trdsql) — consulté le 31/08/2026.
- **GroundDB** — schéma déclaratif YAML avec types et clés étrangères, vues SQL, argument affiché de
  lisibilité git ; données séparées du YAML (fichiers Markdown), cœur Rust seul, aucun pont JS/Go/PHP.
  [github.com/JustMaier/groundDb](https://github.com/JustMaier/groundDb) — consulté le 31/08/2026.
- **yamdl** — fixtures YAML chargées dans un SQLite en mémoire, interrogeables par ORM ; schéma
  défini dans les modèles Django, pas dans le YAML ; lecture seule, mono-langage.
  [github.com/andrewgodwin/yamdl](https://github.com/andrewgodwin/yamdl) — consulté le 31/08/2026.
- **Frictionless Data — Table Schema** — spécification normative d'un descripteur unique portant
  types, clé primaire, clés étrangères déclarées par référence, et données en ligne possibles ;
  format JSON, pas YAML, aucune sémantique SQL.
  [datapackage.org/standard/table-schema](https://datapackage.org/standard/table-schema/) — consulté
  le 31/08/2026.
- **Dolt** — base MySQL-compatible au versionnement façon git (branch, commit, diff, merge) ;
  stockage en arbre « Prolly » adressé par contenu, format binaire — pas un texte que `git diff`
  rendrait lisible en revue de code.
  [dolthub.com/docs/architecture/storage-engine/prolly-tree](https://www.dolthub.com/docs/architecture/storage-engine/prolly-tree)
  — consulté le 31/08/2026.
- **pg-mem, sql.js, AlaSQL** — moteurs SQL en mémoire, aucun ne pilote son état par un fichier YAML
  faisant à la fois schéma et données. Confirme l'existence du « SQL en mémoire », pas de la
  combinaison recherchée. Consulté le 31/08/2026.
- **DuckDB — clients officiels** — le pont PHP est décrit comme un client tertiaire, maintenu par un
  tiers, passant par FFI, « sans garantie de fonctionnalité ni de support ».
  [duckdb.org/docs/lts/clients/php](https://duckdb.org/docs/lts/clients/php) — consulté le
  31/08/2026.
- **libSQL / Turso — SDK PHP** — annoncé en « préversion technique » (« technical preview ») ; absent
  de la page officielle des SDK clients de Turso.
  [github.com/tursodatabase/libsql-php](https://github.com/tursodatabase/libsql-php) — consulté le
  31/08/2026.
- **SurrealDB — SDK PHP** — marqué bêta, connexion distante uniquement (HTTP/WebSocket), aucun mode
  embarqué contrairement aux SDK JavaScript et Go.
  [github.com/surrealdb/surrealdb.php](https://github.com/surrealdb/surrealdb.php) — consulté le
  31/08/2026.
- **Rails fixtures** — antériorité des fixtures YAML avec résolution des clés étrangères par label,
  chargées en base avant chaque test ; schéma porté par les migrations, pas par le YAML des fixtures.
  [guides.rubyonrails.org/testing.html](https://guides.rubyonrails.org/testing.html) — consulté le
  31/08/2026.
- **Pyrseas (`dbtoyaml`/`yamltodb`)** — schéma PostgreSQL complet exporté en YAML, avec
  `primary_key:`, `foreign_keys:` et `check_constraints:` ; aucune donnée n'y vit — l'outil est
  décrit comme « analogous to using `pg_dump --schema-only` », un outil de migration, pas un moteur.
  [pyrseas.readthedocs.io/en/latest/dbtoyaml.html](https://pyrseas.readthedocs.io/en/latest/dbtoyaml.html)
  — consulté le 31/08/2026.
- **SQLite** — le moteur embarqué de référence pour les liaisons multi-langages, à des degrés très
  inégaux selon le langage : natif en PHP (les extensions `SQLite3` et `PDO_SQLITE` sont activées par
  défaut), en release candidate dans Node.js (`node:sqlite`) ou via `better-sqlite3`, et sans driver
  officiel en Go — `mattn/go-sqlite3` et `modernc.org/sqlite` sont tous deux tiers.
  [nodejs.org/api/sqlite.html](https://nodejs.org/api/sqlite.html),
  [php.net/manual/en/sqlite3.installation.php](https://www.php.net/manual/en/sqlite3.installation.php)
  — consultés le 31/08/2026.
**Ce qui n'a pas été vérifié.** La disponibilité du nom « Memless » comme marque, comme paquet npm,
Packagist ou module Go n'a pas été recherchée : elle est hors du périmètre de ce document. La
couverture de l'écosystème Go dans la recherche ci-dessus est moins exhaustive que celle de
JavaScript et PHP — un projet Go récent et peu visible a pu échapper à la recherche. Le prototype qui
soutenait l'hypothèse §17.1 n'avait tourné que sur une seule machine ; son comportement sur les
familles de systèmes retenues par l'architecture (`ARCHITECTURE.md` §12.3 Q7) se vérifie par le banc de
parité rejoué sur chaque famille — §17.1.
