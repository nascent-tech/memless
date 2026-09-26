<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§10. Sécurité](security.md)  
**Suivant** : [§12. Vision et décisions](roadmap.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §11. Développement local

Cette section décrit l'environnement **constaté sur le code**. `.charpente.json` pose déjà
`"architecture": "hexagonal"` à la racine du dépôt, ce qui active les gardes de frontière (§6.1).

### 11.1 Prérequis

- **Rust** (édition 2021, toolchain stable) avec `cargo`, `rustfmt`, `clippy`.
- **Node.js** ≥ 18, pour bâtir et tester le pont Node (`koffi`).
- **Go** 1.21, pour le pont `purego`.
- **PHP** ≥ 8.1 (testé en CI : 8.3, et 8.5 sur macOS Intel où Homebrew ne fournit plus de PHP précompilé) avec l'**extension FFI activée**, pour le pont PHP — à activer
  explicitement sur les distributions où elle ne l'est pas par défaut (§17.1 du brief).
- Pas de `cbindgen` : l'en-tête C (`crates/memless-capi/include/memless.h`) est écrit à la main.

### 11.2 Variables d'environnement

Aucune n'est nécessaire au fonctionnement du moteur : Memless ne lit ni configuration, ni secret, ni
URL. Le seul paramètre d'exécution est le **chemin du fichier YAML**, passé en argument à la création
d'une instance, jamais par variable d'environnement. Les trois ponts, eux, cherchent la bibliothèque
native par `MEMLESS_LIB` si elle est posée, sinon dans `target/{debug,release}/` du workspace — un
paramètre de résolution du pont, pas du moteur.

### 11.3 Démarrage

```sh
cargo build --workspace                  # compile le domaine, le cœur et la bibliothèque native
cargo build -p memless-capi              # la seule cdylib chargée par les trois ponts
cargo test  --workspace                  # tests unitaires et d'intégration Rust
cargo clippy --workspace --all-targets   # analyse statique
cargo fmt --all -- --check               # format, sur tous les crates du workspace

composer install -d bindings/php && vendor/bin/phpunit    # pont PHP
(cd bindings/go && go test ./...)                          # pont Go
(cd bindings/node && npm install && npm test)               # pont Node

composer install -d harness/parity/php   # dépendances du pilote PHP du banc de parité
bash harness/parity/run.sh               # banc de parité, les trois pilotes contre le même corpus
bash harness/bench/run.sh                # banc de mesure (§17.2 du brief)
```

Chaque pont se construit et se teste avec l'outillage de son écosystème, contre `MEMLESS_LIB` ou, à
défaut, la bibliothèque compilée dans `target/` du workspace.

### 11.4 Qualité et tests

- **Portes** (au sens `/charpente:verify`) : `cargo fmt --all -- --check`, `cargo clippy`,
  `cargo test`, `cargo build` — chacune séparément, code de sortie contrôlé.
- **Tests unitaires** : dans les modules (`#[cfg(test)] mod tests`), au plus près du domaine, sans
  base ni double (`memless-domain` est pur).
- **Tests d'intégration Rust** : sous `crates/<crate>/tests/`, contre l'API publique du crate.
- **Test de round-trip « lire → réécrire → diff vide »** (`crates/memless-engine/tests/golden.rs`) :
  charger un fichier, le réécrire sans changement de donnée, et exiger un diff vide (hors exceptions
  connues de la décision 26 du brief) — verrouille l'ordre des colonnes porté par `RawDocument` (§8.6,
  décisions 13/27 du brief).
- **Banc de parité** (`harness/parity/`) — **le test décisif du projet** : rejouer un même corpus de
  fichiers, requêtes et transactions dans les trois langages et **exiger le même résultat et le même
  message d'erreur**, plus une empreinte `sha256` identique du fichier réécrit (§5.1, §8.8, §13 du
  brief, décision 25). Il tourne dans la CI GitHub Actions sur chacune des quatre familles ; il
  transforme la parité de promesse en fait vérifié.
- **Banc de mesure** (`examples/bench_write.rs`, `bench_transaction.rs`) : faire varier taille de
  fixtures, nombre d'écritures et de rechargements pour trancher l'hypothèse §17.2 du brief.
