<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§11. Développement local](dev-environment.md)  
**Suivant** : [§13. Glossaire](glossary.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §12. Vision et décisions

### 12.1 Évolutions décidées (feuille de route)

Repoussées sciemment (§16 du brief), chacune conditionnée à un besoin réel constaté — pas encore
implémentées. La colonne de droite dit ce que chacune changerait dans **cette** architecture.

| Évolution (voir §16 du brief) | Ce que ça changerait ici |
|---|---|
| Serveur partagé entre processus/langages | un nouvel adaptateur entrant **réseau** et un état partagé — rompt le « chaque instance sa copie » (décision 6 du brief) |
| Cascade / mise à `NULL` automatique (en option) | une option sur le cas d'usage d'écriture et la vérification des contraintes (§8.7 du brief) |
| API « façon SQL » native par langage | une surface supplémentaire par pont, à tenir en parité |
| Verrou inter-processus sur un même fichier | un nouveau **port** de verrou de fichier et son adaptateur |
| Base utilisable en production | remet en cause les hypothèses de taille (§17.2 du brief) et la durabilité |
| Marque de version dans le format de fichier | un champ de version lu/écrit par l'adaptateur YAML |
| Déclaration explicite d'une relation | une entrée de configuration lue par l'inférence (§17.3 du brief) |
| Inventaire de ce que Memless a deviné | un cas d'usage de lecture exposant la structure devinée |

### 12.2 Dettes

| Dette | Risque | Conséquence |
|---|---|---|
| Aucune coordination entre écritures concurrentes (décision 7 du brief) | **moyen** | deux écrivains simultanés : le dernier gagne en silence, le changement perdu ne laisse aucune trace, même dans Git (§9.1 du brief) ; deux écritures simultanées de deux instances sur le même fichier partagent aussi le même nom de temporaire `.<nom>.memless-tmp` : la seconde écrase le résidu de la première, dont le `rename` peut alors échouer (§5.3) |
| Protections de la frontière FFI (§10.5) : validation des entrées, propriété/libération des sorties, `catch_unwind`, verrou sur la table de handles | **élevé** | l'une manquante = corruption mémoire, fuite, double libération ou comportement indéfini — seul endroit hors de la garantie Rust |
| Budget YAML saturé — le « YAML bomb » n'est pas borné (§10.5, `options.rs`) | **moyen** | un fichier démesuré ou à alias profonds peut épuiser la mémoire au chargement ; assumé parce que le fichier vient de la développeuse elle-même, jamais d'une source non fiable |
| Coût de la réécriture complète du fichier, non mesuré au-delà des fixtures du MVP (§17.2 du brief) | **moyen** | si les fixtures grossissent, la réécriture peut devenir le geste le plus lent de la suite ; à trancher par le banc de mesure |
| Parité réelle du pont PHP sur toutes les plateformes (§17.1 du brief) | **moyen** | PHP pourrait rester citoyen de seconde zone (FFI non activé, comportement divergent) ; prouvé par le banc de parité multi-OS, en CI |
| `serde-saphyr` à mainteneur unique (§7) | faible | une dépendance critique repose sur une seule personne |
| Bibliothèque embarquée non signée (npm, miroir PHP, miroir Go) | **faible** | aucune signature cryptographique sur la bibliothèque native distribuée ; atténué par le SHA-256 complet vérifié côté Go avant tout `dlopen`, par l'intégrité du paquet garantie par npm à l'installation, et par le recoupement possible avec `SHA256SUMS` déjà publié en Release GitHub |
| Miroir PHP qui grossit d'environ 20 Mo par version | **faible** | chaque tag ajoute un commit orphelin portant les quatre bibliothèques natives sur `nascent-tech/memless-php` ; le dépôt miroir grossit sans purge, aucune limite fixée par ce palier |
| Miroir Go qui grossit d'environ 20 Mo par version | **faible** | chaque tag ajoute un commit orphelin portant les quatre bibliothèques natives sur `nascent-tech/memless-go` ; le dépôt miroir grossit sans purge, aucune limite fixée par cette révision |
| Binaires Go des versions ≤ 0.2.1 retenus dans le dépôt principal par les tags `bindings/go/v0.2.x` | **faible, figée** | les tags sont immuables, plus rien ne s'y ajoute : un `git clone` de `nascent-tech/memless` télécharge encore ces binaires-là, mais aucune version postérieure ne les rejoint — le poids déjà accumulé reste, sans grossir davantage |
| Paquet npm d'environ 22 Mo décompressés (7,5 Mo compressés) | **faible** | chaque installation Node télécharge les quatre bibliothèques, comme Go et PHP ; acceptable pour quatre plateformes, à rouvrir dès qu'une cinquième entre (question 6, révision « paquet npm unique ») |
| glibc minimale non figée | **moyen** | les bibliothèques Linux sont construites sur ubuntu-24.04 (glibc 2.39) ; sur une distribution plus ancienne le chargement peut échouer — correctif ultérieur : construire dans un conteneur à glibc ancienne ou par `cargo zigbuild` |

### 12.3 Questions ouvertes

Les huit questions de la roadmap sont désormais **toutes tranchées**, par le code ou par une décision
humaine datée :

*Question 6 : révisé (miroir Go, puis paquet npm unique).*

1. **Store — TRANCHÉ (caduque).** GlueSQL est sorti au palier 2 : l'état est
   `memless_domain::Base` (tables ordonnées, ordre des colonnes porté par `RawDocument`). L'ordre est
   capté au chargement, par ligne, colonne nouvelle en fin (`base/write/next_position.rs`) — la
   sous-question 1-bis est tranchée dans le même geste.
2. **Format d'échange à la frontière — TRANCHÉ.** Pas de JSON, pas de format binaire tagué : accès
   cellule par cellule par le C ABI (`memless_result_column_count`, `memless_result_cell`, kinds
   Absent/Text/Integer/Decimal/Boolean, décimaux en `double`, texte emprunté). Le banc de parité
   compare un rendu canonique (décimaux en bits IEEE-754).
3. **Chemin de Node — TRANCHÉ (humain, 2026-09-22).** FFI dynamique `koffi` ≥ 2.16 sur la même
   `memless-capi`, pas d'addon `napi-rs` : un seul binaire natif pour les trois ponts.
4. **Bibliothèque YAML — TRANCHÉ.** `serde-saphyr` `=1.3.0`, sans `IndexMap` : l'ordre vit dans
   `RawDocument` (memless-domain). Le budget est **saturé** (`usize::MAX`), pas resserré : le
   « YAML bomb » n'est pas borné — dette assumée (§12.2), pas une sous-question restante.
5. **CI/CD — TRANCHÉ.** **GitHub Actions** (`nascent-tech/memless`). Un `ci.yml` sur push/PR, matrice
   sur les quatre cibles de la question 7 : `cargo check --all-targets`, `cargo clippy --all-targets -- -D
   warnings`, `cargo test`, `cargo build --release -p memless-capi`, `go test ./...` (Go 1.21),
   `composer install && vendor/bin/phpunit` (PHP 8.3 + `ext-ffi`, 8.5 préinstallé sur macOS Intel), `npm ci && npm test` (Node 24 LTS), puis
   `bash harness/parity/run.sh` et les tests `tests/detects-*.sh`. Un `release.yml` sur tag `v*`
   reproduit ces builds et produit les artefacts de la question 6. Le run vert de `ci.yml` sur chaque
   famille est la preuve ; sa ligne (date, famille, révision, lien du run, verdict) est recopiée dans
   `.charpente/releves/parite.md` par le fondateur au moment du tag — pas par un step de CI.
6. **Distribution du binaire natif — TRANCHÉ, révisé au palier 6 (2026-09-26), puis révisé une
   deuxième fois (miroir Go, 2026-09-26), puis une troisième (paquet npm unique, 2026-09-26).** **GitHub Releases est conservé** — quatre archives
   `memless-capi-<version>-<cible>.tar.gz` (la cdylib + `memless.h`),
   `nascent-tech-memless-<version>.tgz` (le paquet npm publié), `memless-php-<version>.zip`,
   `memless-lib-SHA256SUMS` (les bibliothèques embarquées) et un fichier `SHA256SUMS` couvrant tous
   les assets — **et les registres natifs de chaque langage s'y ajoutent** : **npm**, un paquet
   unique `@nascent-tech/memless` qui embarque les quatre bibliothèques sous `lib/<plateforme>/` ; **Packagist**, par un dépôt **miroir**
   `nascent-tech/memless-php` (Packagist exige un `composer.json` à la racine d'un dépôt, que le
   monorepo n'offre pas) alimenté par la CI à chaque tag ; **le proxy Go**, par un module
   `github.com/nascent-tech/memless-go` qui embarque sa bibliothèque native (`//go:embed`), publié par
   un dépôt **miroir** `nascent-tech/memless-go` — sur le même patron que le miroir PHP : commit
   orphelin par version, tag `vX.Y.Z`, `main` du miroir poussé en force — il n'embarque que la
   plateforme compilée et l'extrait au cache utilisateur, vérifié par SHA-256 avant chaque chargement.
   La résolution de la bibliothèque dans les trois ponts devient :
   `MEMLESS_LIB` (prioritaire, erreur claire s'il désigne un fichier absent), puis la **bibliothèque embarquée par le paquet** de la plateforme courante,
   puis `target/release` / `target/debug` du workspace (développement) — identique dans les trois
   ponts, aucune règle métier n'entre dans un pont. Linux musl ou à libc indéterminable (ni
   `/usr/bin/ldd` ni chargeur dynamique reconnu sous `/lib`) et Windows n'ont pas de bibliothèque
   embarquée : ils retombent à `target/`, puis à l'erreur claire citant `MEMLESS_LIB`. Cadrage :
   `.charpente/cadrage/2026-09-26-palier-6-publier.md` (révisions « miroir Go » et « paquet npm unique » en fin de document). La
   publication npm, Packagist et le miroir Go attendent les comptes du propriétaire (variables
   `NPM_PUBLISH`, `PHP_MIRROR_PUBLISH`, `GO_MIRROR_PUBLISH`) ; les étapes sont dans
   `docs/PUBLISHING.md`.

   *Révision « miroir Go » (2026-09-26).* Le module Go **change de chemin** :
   `github.com/nascent-tech/memless-go`, à la place de
   `github.com/nascent-tech/memless/bindings/go` — rupture pour tout code Go qui importait l'ancien
   chemin, publiée en **version 0.3.0** (`BREAKING CHANGE`). Le job `publish-go` qui posait un commit
   hors `main` du dépôt principal et le tag `bindings/go/vX.Y.Z` **disparaît** : ce commit hors branche
   restait retenu par le tag dans chaque clone du dépôt principal, et le chemin d'import était lourd.
   Les versions déjà publiées à l'ancien chemin (`bindings/go/v0.2.0`, `v0.2.1`) restent en place,
   jamais supprimées ni déplacées ; plus aucun tag `bindings/go/*` n'est créé après cette révision
   (dette §12.2 : « binaires Go des versions ≤ 0.2.1 retenus dans le dépôt principal »). Clé de
   déploiement SSH en écriture, secret **`GO_MIRROR_DEPLOY_KEY`**, limitée au seul miroir
   `nascent-tech/memless-go` (§10.6) ; `publish-go` n'a plus besoin de `contents: write` sur le dépôt
   principal, permission ramenée à `contents: read`.

   *Révision « paquet npm unique » (2026-09-26).* Les quatre **paquets de plateforme**
   `@nascent-tech/memless-<plateforme>`, posés jusqu'en 0.3.0 en `optionalDependencies` injectées par
   la CI, **ne sont plus publiés** : le paquet principal embarque les quatre bibliothèques sous
   `lib/<plateforme>/`, sur le patron du miroir PHP, et le pont les lit dans son propre répertoire. Le
   découpage n'économisait qu'environ 5,5 Mo compressés par installation, pour un mode de panne que le
   paquet unique supprime — le lockfile écrit sur une plateforme qui omet le paquet d'une autre, puis
   `npm ci` qui échoue en conteneur ou en CI (npm/cli#4828, fermé en 2025 mais encore signalé en
   2025 et 2026 jusqu'à npm 11) — et pour cinq paquets et cinq éditeurs de confiance npm à tenir. Ordre de recherche et
   détection glibc/musl inchangés. Aucune rupture pour qui installe `@nascent-tech/memless` : publié en
   **version 0.4.0** (`feat(node)`). Les paquets de plateforme déjà publiés restent installables et
   sont dépréciés (`npm deprecate`, jamais retirés). Le paquet unique vaut pour les quatre plateformes
   de la question 7 : toute plateforme supplémentaire rouvre cette décision (dette §12.2).

   *Ce que cela remplace* — décision du palier 5 : « Aucun registre externe : GitHub Releases sur tag
   `vX.Y.Z` — quatre archives `memless-capi-<version>-<cible>.tar.gz` (la cdylib + `memless.h`), un
   fichier `SHA256SUMS` couvrant tous les assets, plus `nascent-tech-memless-<version>.tgz` (`npm
   pack`), `memless-php-<version>.zip` (repository Composer `artifact`), et le tag
   `bindings/go/vX.Y.Z`, posé à la main sur le même commit que `vX.Y.Z` (aucun workflow ne le crée). La
   résolution de la bibliothèque dans les trois ponts (`MEMLESS_LIB`, sinon `target/` du workspace) ne
   change pas : la variable est le mode de livraison documenté, pas un paquet autonome par plateforme
   (dette, §12.2). » Le tag `bindings/go/vX.Y.Z` posé à la main sur le même commit que `vX.Y.Z`
   (paliers 0.1.0/0.1.1) reste sur `main` ; la voie du commit hors branche, prévue au palier 6 initial
   à partir de `bindings/go/v0.2.0`, est remplacée par le miroir dès la révision ci-dessus.
7. **Cibles à publier — TRANCHÉ.** Quatre familles : `aarch64-apple-darwin`, `x86_64-apple-darwin`,
   `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`. Exclus au lancement : Windows (les ponts ne
   cherchent que `.dylib`/`.so`) et Linux musl. Sous-ensemble SQL : `SELECT`/`WHERE`/`JOIN` par relation
   devinée/`COUNT`/`SUM`/`ORDER BY` + `INSERT`/`UPDATE`/`DELETE` + `BEGIN`/`COMMIT`/`ROLLBACK`,
   identifiants entre guillemets doubles ; `LIMIT`/`OFFSET`/`GROUP BY`/`DISTINCT` restent hors
   sous-ensemble.
8. **Snapshot/rollback de l'état en mémoire — TRANCHÉ (palier 4).** Copie complète (`Base::clone()`) à
   l'ouverture d'une transaction, pas de journal d'annulation. Mesuré au banc
   (`.charpente/releves/banc.md`) : une transaction de 1 000 écritures coûte 15 ms, un seul `fsync`.
