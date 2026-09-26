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
| Résolution du binaire natif par `MEMLESS_LIB`/`target/` seulement | **faible** | pas de paquet autonome par plateforme : chaque pont doit trouver ou recevoir le chemin de la `cdylib` lui-même, plutôt que la recevoir embarquée dans son paquet |
| `serde-saphyr` à mainteneur unique (§7) | faible | une dépendance critique repose sur une seule personne |

### 12.3 Questions ouvertes

Les huit questions de la roadmap sont désormais **toutes tranchées**, par le code ou par une décision
humaine datée :

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
   `composer install && vendor/bin/phpunit` (PHP 8.3 + `ext-ffi`), `npm ci && npm test` (Node 24 LTS), puis
   `bash harness/parity/run.sh` et les tests `tests/detects-*.sh`. Un `release.yml` sur tag `v*`
   reproduit ces builds et produit les artefacts de la question 6. Le run vert de `ci.yml` sur chaque
   famille est la preuve ; sa ligne (date, famille, révision, lien du run, verdict) est recopiée dans
   `.charpente/releves/parite.md` par le fondateur au moment du tag — pas par un step de CI.
6. **Distribution du binaire natif — TRANCHÉ.** Aucun registre externe : **GitHub Releases** sur tag
   `vX.Y.Z` — quatre archives `memless-capi-<version>-<cible>.tar.gz` (la cdylib + `memless.h`), un
   fichier `SHA256SUMS` couvrant tous les assets, plus `nascent-tech-memless-<version>.tgz` (`npm
   pack`), `memless-php-<version>.zip` (repository Composer `artifact`), et le tag
   `bindings/go/vX.Y.Z`, posé à la main sur le même commit que `vX.Y.Z` (aucun workflow ne le crée). La
   résolution de la bibliothèque
   dans les trois ponts (`MEMLESS_LIB`, sinon `target/` du workspace) ne change pas : la variable est
   le mode de livraison documenté, pas un paquet autonome par plateforme (dette, §12.2).
7. **Cibles à publier — TRANCHÉ.** Quatre familles : `aarch64-apple-darwin`, `x86_64-apple-darwin`,
   `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`. Exclus au lancement : Windows (les ponts ne
   cherchent que `.dylib`/`.so`) et Linux musl. Sous-ensemble SQL : `SELECT`/`WHERE`/`JOIN` par relation
   devinée/`COUNT`/`SUM`/`ORDER BY` + `INSERT`/`UPDATE`/`DELETE` + `BEGIN`/`COMMIT`/`ROLLBACK`,
   identifiants entre guillemets doubles ; `LIMIT`/`OFFSET`/`GROUP BY`/`DISTINCT` restent hors
   sous-ensemble.
8. **Snapshot/rollback de l'état en mémoire — TRANCHÉ (palier 4).** Copie complète (`Base::clone()`) à
   l'ouverture d'une transaction, pas de journal d'annulation. Mesuré au banc
   (`.charpente/releves/banc.md`) : une transaction de 1 000 écritures coûte 15 ms, un seul `fsync`.
