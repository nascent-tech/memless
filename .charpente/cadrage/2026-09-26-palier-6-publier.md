---
type: cadrage
titre: Palier 6 — Publier sur les registres
slug: palier-6-publier
cree_le: 2026-09-26T12:11:30+0000
mis_a_jour_le: 2026-09-26T12:50:37+0000
branche: feat/publish-registries
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../mvp/00-index.md) › **cadrage — Palier 6 — Publier sur les registres**  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/mvp/00-index.md

Cadrage du **palier 6**, postérieur au MVP livré en 0.1.1 : « Publier sur les registres ». Il révise
`ARCHITECTURE.md` §12.3 question 6 (« Distribution du binaire natif — TRANCHÉ » : GitHub Releases
seul) et la dette §12.2 sur la résolution `MEMLESS_LIB`/`target/` seule. Décisions tranchées par
l'arbitrage Fable du 2026-09-26, sur demande du propriétaire ; le code correspondant est livré par
`feat/publish-registries` (0.2.0). Ce document décrit l'état livré en **0.2.0**.

## 1. Acteur et déclencheur

**Acteur.** La **développeuse** qui installe un pont Memless (§6 du brief) — jusqu'ici en récupérant
une archive GitHub Releases et en posant `MEMLESS_LIB` à la main ; et le **propriétaire**, qui publie
une version.

**Déclencheur.** La développeuse tape la commande d'installation native de son langage — `npm install
@nascent-tech/memless`, `composer require nascent-tech/memless`, `go get
github.com/nascent-tech/memless/bindings/go` — sans télécharger de bibliothèque à la main ni poser
`MEMLESS_LIB`. Le propriétaire pousse un tag `vX.Y.Z` ; la CI publie sur les registres en plus de
GitHub Releases.

**Ce que l'acteur observe.** L'installation par l'outil natif de son langage suffit : le pont trouve
sa bibliothèque native tout seul, à la bonne plateforme, sans étape supplémentaire. `MEMLESS_LIB` reste
disponible pour qui veut imposer un autre fichier. Un tag sur une famille de systèmes qui échoue ne
publie aucun registre — la publication reste tout-ou-rien, comme au palier 5.

## 2. Besoin, séparé de la solution

| Formulé en solution | Le besoin dessous |
|---|---|
| « Publier sur npm » | Qu'une développeuse Node n'ait pas à chercher une archive GitHub, la décompresser et poser une variable d'environnement avant de pouvoir importer Memless — `npm install` doit suffire, comme pour n'importe quel paquet npm. |
| « Publier sur Packagist » | Que `composer require` fonctionne pour PHP, alors que Packagist exige un `composer.json` à la racine d'un dépôt — ce que le monorepo Memless n'offre pas tel quel : d'où un dépôt miroir, alimenté par la CI, jamais tenu à la main. |
| « Passer par le proxy Go » | Que `go get` fonctionne pour Go sans registre à créer : le proxy Go module lit directement un module public ; le seul geste est d'embarquer la bibliothèque dans le module, plutôt que de la faire chercher ailleurs. |
| « Garder GitHub Releases » | Que rien ne casse pour qui installait déjà par archive — les quatre archives et `SHA256SUMS` restent, la nouveauté s'ajoute, elle ne remplace rien. |

Ce que le palier 6 prouve : que les trois ponts s'installent par l'outil natif de leur langage, à
l'identique du reste de l'écosystème que chaque développeuse connaît déjà, sans changer une seule règle
métier des ponts. Ce qu'il ne prouve pas : un nouveau langage, une nouvelle plateforme, ou un
changement du sous-ensemble SQL.

## 3. Vocabulaire

Termes **[neuf]** à inscrire au glossaire.

| Terme | Définition | Voisin à ne pas confondre |
|---|---|---|
| **Paquet de plateforme** [neuf] | Un paquet npm dédié à une seule plateforme (`@nascent-tech/memless-darwin-arm64`, etc.), qui ne porte que la bibliothèque native pour cette plateforme ; posé en `optionalDependencies` du paquet principal, jamais installé seul par la développeuse. | Le **paquet principal** (`@nascent-tech/memless`), qui porte le code du pont et dépend des quatre paquets de plateforme. |
| **Bibliothèque embarquée** [neuf] | La bibliothèque native (`libmemless_capi.<ext>`) portée à l'intérieur du paquet publié (npm, miroir PHP) ou du module (Go, par `//go:embed`), trouvée par le pont sans variable d'environnement ni chemin `target/`. | La bibliothèque compilée localement dans `target/release`/`target/debug`, seule voie avant ce palier. |
| **Miroir** [neuf] | Un dépôt GitHub distinct (`nascent-tech/memless-php`) qui ne porte, à chaque tag, que le contenu publiable de `bindings/php` plus les bibliothèques natives des quatre plateformes ; alimenté par la CI, jamais édité à la main ; existe parce que Packagist exige un `composer.json` à la racine d'un dépôt, que le monorepo Memless n'offre pas. | Le **dépôt principal** (`nascent-tech/memless`), qui porte tout le reste et ne reçoit jamais de binaire sur `main`. |
| **Publication** [neuf] | Le geste, réservé à la CI sur un tag `v*`, qui pousse une version vers un registre externe (npm, Packagist via le miroir) ou vers le proxy Go (par un tag de module) ; distinct de la **Release GitHub**, qui existait déjà depuis le palier 5 et n'est pas remplacée. | La **Release GitHub**, l'archive/zip/tgz déjà en place ; la publication registre s'ajoute, elle ne s'y substitue pas. |

**Termes exclus** : « registre » seul sans dire lequel — nommer **npm**, **Packagist** ou **le proxy
Go** ; « package manager » (anglicisme) — dire **outil natif du langage** ou nommer l'outil
(`npm`, `composer`, `go get`).

## 4. Événements au passé sur une chronologie

| # | Événement | Origine | Observable par |
|---|---|---|---|
| 1 | **Bibliothèque de plateforme trouvée** | Politique — *dès que* `MEMLESS_LIB` n'est pas posée, *alors* chaque pont cherche la bibliothèque embarquée par son paquet pour la plateforme courante, avant `target/release` puis `target/debug`. | Le pont s'ouvre sans configuration supplémentaire. |
| 2 | **Plateforme non embarquée (musl, Windows)** | Politique — *dès qu'*aucune bibliothèque embarquée n'existe pour la plateforme courante, *alors* le pont retombe sur `target/release`/`target/debug`, puis refuse avec un message clair qui cite `MEMLESS_LIB`. | Le message de refus, qui nomme `MEMLESS_LIB` comme échappatoire. |
| 3 | **npm publié** | Système — la CI, sur un tag `v*` et la variable de dépôt `NPM_PUBLISH == 'true'`, publie les quatre paquets de plateforme puis le paquet principal. | Le paquet visible sur npm à la version du tag ; `npm install @nascent-tech/memless` fonctionne. |
| 4 | **Miroir PHP publié** | Système — la CI pousse un commit orphelin (contenu de `bindings/php` sans `vendor`, `tests/`, `phpunit.xml.dist` ni `composer.lock`, plus `LICENSE`, les bibliothèques des quatre plateformes, `lib/memless.h` et `lib/SHA256SUMS`) sur `nascent-tech/memless-php`, tag `vX.Y.Z`, `main` du miroir poussé en force dessus. | Le hook GitHub met Packagist à jour ; `composer require nascent-tech/memless` fonctionne. |
| 5 | **Module Go embarqué publié** | Système — la CI crée, hors de `main`, un commit qui ajoute `bindings/go/lib/<plateforme>/libmemless_capi.<ext>` pour les quatre plateformes, et pose le tag `bindings/go/vX.Y.Z` sur ce commit. | `go get github.com/nascent-tech/memless/bindings/go@vX.Y.Z` résout un module qui embarque sa bibliothèque. |
| 6 | **Extraction Go réussie** | Système — au premier appel, si l'embarqué contient la bibliothèque de la plateforme courante, le pont l'extrait dans `os.UserCacheDir()/memless/<version>-<sha256 court>/`, vérifie son SHA-256 complet contre les octets embarqués avant tout `dlopen`, réécrit en cas d'écart. | Le fichier extrait dans le cache utilisateur ; un appel suivant réutilise le fichier déjà vérifié. |
| 7 | **Publication conditionnelle sautée** | Système — *tant que* le propriétaire n'a pas créé ses comptes (organisation npm, compte Packagist, clé de déploiement), les jobs `publish-npm`/`publish-php` sont sautés sans faire échouer la CI. | La CI reste verte ; aucune tentative de publication sur un registre non prêt. |

## 5. Contexte(s) et sorte

**Contexte borné : le moteur** (MVP §6). Le palier 6 ne touche **aucune règle métier des ponts** :
l'invariant posé par l'arbitrage est que seule la *recherche de la bibliothèque* change, à l'identique
dans les trois ponts — la parité (banc à trois voix) ne bouge pas. **Sorte : support.** La publication
et l'ordre de recherche de la bibliothèque ne portent aucune règle produit ; ils vérifient et livrent
le cœur, ils n'en décident rien. Le cœur, l'exécution SQL et la règle de comparaison unique restent
`cœur de métier`, inchangés.

## 6. Refus — tous les états impossibles, avant le nominal

| # | État impossible | Type / message |
|---|---|---|
| R1 | `MEMLESS_LIB` posée mais désignant un fichier absent. | Erreur claire, citant `MEMLESS_LIB` — inchangé depuis les paliers précédents ; cette étape reste prioritaire sur toutes les autres. |
| R2 | Plateforme sans bibliothèque embarquée (Linux musl, Windows), aucune de `target/release`/`target/debug` présente. | Erreur claire qui cite `MEMLESS_LIB` comme seule voie restante — aucun message neuf, le refus existant s'applique. |
| R3 | Go, cache utilisateur indisponible à l'extraction. | Retombe sur `target/release`/`target/debug`, puis le refus R2 s'applique. |
| R4 | Go, fichier déjà présent dans le cache mais dont le SHA-256 complet diffère des octets embarqués. | Le fichier est réécrit avant tout `dlopen` — jamais chargé sans que son intégrité soit revérifiée. |
| R5 | Paquet npm déjà publié à la version du tag (rejeu de la CI). | La publication passe (idempotence), aucune erreur. |
| R6 | Variable de dépôt `NPM_PUBLISH`/`PHP_MIRROR_PUBLISH` absente ou fausse. | Le job correspondant est sauté ; la CI reste verte. |
| R7 | Un seul job de la matrice CI échoue sur une seule des quatre familles (comme au palier 5). | Aucune publication, ni GitHub Releases ni registre — la politique tout-ou-rien du palier 5 s'étend aux registres. |

**Ce qui n'est PAS un refus** :
- Une plateforme sans bibliothèque embarquée qui retombe sur `target/` en développement : nominal,
  c'était déjà le seul chemin avant ce palier.
- Un paquet npm de plateforme absent côté développeuse parce que `optionalDependencies` ne l'a pas
  installé pour sa plateforme : nominal, c'est le mécanisme même d'`optionalDependencies` — le pont
  détecte l'absence et retombe à l'étape suivante.

**Parité.** L'ordre de recherche de la bibliothèque est identique dans les trois ponts (§ci-dessus,
invariant de l'arbitrage) ; aucune divergence de comportement entre langages n'est introduite par la
publication elle-même.

## 7. Limites / seuils sourcés

**Plateformes embarquées.** Les mêmes quatre familles que le palier 5 (`aarch64-apple-darwin`,
`x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`) ; aucune cinquième
plateforme n'entre avec ce palier.

**Croissance du miroir PHP.** Chaque version du miroir PHP grossit d'environ 20 Mo (quatre bibliothèques
natives par version, commit orphelin) — dette assumée (§12.2), pas resserrée par ce palier.

**Bibliothèque embarquée non signée.** Aucune signature cryptographique sur la bibliothèque distribuée
par npm ou par le miroir PHP — dette assumée, faible : le SHA-256 complet est vérifié côté Go avant
tout `dlopen`, npm garantit l'intégrité de son propre paquet à l'installation, et `SHA256SUMS` (déjà
publié en Release GitHub depuis le palier 5) permet de recouper à la main.

**Aucun seuil chiffré ne conditionne un refus** dans ce palier — comme au palier 5, la publication est
tout-ou-rien, jamais partielle.

## 8. Hors périmètre explicite

| Hors du palier 6 | Où |
|---|---|
| Linux musl, Windows | Aucune bibliothèque embarquée pour ces plateformes ; on retombe à `target/`, puis erreur claire citant `MEMLESS_LIB` — inchangé depuis le palier 5 |
| crates.io | Le cœur Rust n'est publié sur aucun registre de crates ; seuls les trois ponts (npm, Packagist, proxy Go) et GitHub Releases sont concernés |
| PECL | Le pont PHP reste installé par Composer/Packagist, jamais par une extension PECL compilée |
| Le nom de marque | N'est vérifié sur aucun registre par ce palier |
| Transactions imbriquées, points de sauvegarde, verrouillage inter-processus, cascade/`NULL` automatique | Toujours hors périmètre, inchangé depuis les paliers précédents |

## 9. Ce que seul le propriétaire peut faire

- Créer l'organisation npm `nascent-tech` et un jeton granulaire (`NPM_TOKEN`, portée publish sur
  `@nascent-tech/*`, Bypass 2FA, ≤ 90 jours) pour la toute première publication, puis créer les cinq
  Trusted Publishers (OIDC) et supprimer le jeton une fois en place.
- Générer la clé de déploiement SSH en écriture limitée au miroir `nascent-tech/memless-php` (partie
  publique posée sur le miroir, partie privée dans le secret `PHP_MIRROR_DEPLOY_KEY`).
- Créer le compte Packagist, le lier à GitHub, y soumettre `nascent-tech/memless-php`, activer le hook
  qui met Packagist à jour à chaque poussée du miroir.
- Poser les variables de dépôt `NPM_PUBLISH=true` et `PHP_MIRROR_PUBLISH=true` une fois les comptes
  prêts — tant qu'elles sont absentes ou fausses, la CI reste verte sans tenter de publication (R6).

## 10. Décisions restantes et qui tranche

Toutes tranchées par l'arbitrage Fable du 2026-09-26 (« Révisions de l'arbitrage Fable », qui fait
autorité sur toute conception antérieure) :

| # | Décision | Tranché par défaut |
|---|---|---|
| D1 | **Invariant.** Seule la recherche de la bibliothèque change, à l'identique dans les trois ponts ; aucune règle métier n'entre dans un pont. | Ordre : `MEMLESS_LIB` → bibliothèque embarquée par le paquet de la plateforme courante → `target/release` puis `target/debug`. |
| D2 | **Node — npm.** Paquet principal inchangé (`@nascent-tech/memless`), quatre paquets de plateforme en `optionalDependencies`, même version exacte, `os`/`cpu`/`libc` posés dans chaque gabarit, `repository`/`license` renseignés. | `library-path.js` résout par `require.resolve` du paquet de plateforme, sinon étape suivante ; détection glibc/musl sans dépendance. |
| D3 | **PHP — Packagist par miroir.** Nom `nascent-tech/memless`, dépôt miroir `nascent-tech/memless-php` alimenté par commit orphelin à chaque tag, authentifié par clé de déploiement SSH limitée au miroir. | `LibraryPath.php` résout par un chemin relatif au miroir, plateforme déduite de `PHP_OS_FAMILY`/`php_uname('m')`. |
| D4 | **Go — module proxy avec bibliothèque embarquée.** Module inchangé (`github.com/nascent-tech/memless/bindings/go`), quatre fichiers `embed_<os>_<arch>.go` sous contrainte de build, plus un `embed_other.go` vide ; commit hors `main` au tag, jamais de binaire sur `main`. | Extraction unique dans `os.UserCacheDir()`, vérifiée par SHA-256 complet avant `dlopen`, écriture atomique (fichier temporaire puis `rename`). |
| D5 | **GitHub Releases conservé.** Rien ne remplace la Release existante (archives, zip PHP, tgz npm, `SHA256SUMS`) ; les registres s'ajoutent. | `release.yml` garde son job `publish` existant tel quel. |
| D6 | **CI de publication.** `publish-go` toujours ; `publish-npm`/`publish-php` conditionnels aux variables de dépôt, sautés sans échec tant que le propriétaire n'a pas créé ses comptes. | Permissions minimales par job (`publish-go` : `contents: write` ; `publish-npm` : `id-token: write`, `contents: read` ; `publish-php` : `contents: read`), secrets exposés au seul job qui les utilise. |

**Points de la conception non transcrits ici** : aucun — le contenu technique détaillé (contenu exact
des workflows, code des ponts) revient au code, livré par `feat/publish-registries` (0.2.0) ; ce cadrage
n'en garde que ce qui définit le besoin, le vocabulaire et les décisions au niveau produit, conformément
à `§5.2` du brief (le choix technique appartient à l'architecture, pas au cadrage).
