<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§10. Sécurité](security.md)  
**Suivant** : [§12. Vision et décisions](roadmap.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §11. Développement local

Comme rien n'est encore construit, cette section décrit l'environnement **cible**, pas un état
constaté. Elle sera vérifiée et figée à `/charpente:scaffold`. Avant le premier fichier de code,
`/charpente:init` doit poser `.charpente.json` (à la racine du dépôt) avec `architecture: hexagonal`,
sans quoi les gardes de frontière ne contrôlent pas l'arbre (§6.1).

### 11.1 Prérequis

- **Rust** (toolchain stable, édition à figer au plan) avec `cargo`, `rustfmt`, `clippy`.
- **Node.js** (version à figer au plan) et son gestionnaire de paquets, pour bâtir et tester le pont
  Node (napi-rs).
- **Go** (version à figer au plan), pour le pont purego.
- **PHP** (version à figer au plan) avec l'**extension FFI activée**, pour le pont PHP — à activer
  explicitement sur les distributions où elle ne l'est pas par défaut (§17.1 du brief).
- **cbindgen**, pour générer l'en-tête C consommé par Go et PHP.

### 11.2 Variables d'environnement

Aucune n'est nécessaire au fonctionnement du moteur : Memless ne lit ni configuration, ni secret, ni
URL. Le seul paramètre d'exécution est le **chemin du fichier YAML**, passé en argument à la création
d'une instance, jamais par variable d'environnement.

### 11.3 Démarrage

```sh
cargo build --workspace              # compile le domaine, le cœur et la bibliothèque native
cargo test  --workspace              # tests unitaires et d'intégration Rust
cargo clippy --workspace --all-targets   # analyse statique
cargo fmt --all -- --check           # format, sur tous les crates du workspace
```

Les ponts se construisent et se testent avec l'outillage de leur écosystème (npm pour Node, `go test`
pour Go, Composer/PHPUnit pour PHP), chacun contre la bibliothèque native compilée. Le banc de parité
vit hors de Cargo (`harness/parity/`) : un lanceur (script ou tâche CI) enchaîne les trois suites
contre le même corpus.

### 11.4 Qualité et tests

- **Portes** (au sens `/charpente:verify`) : `cargo fmt --all -- --check`, `cargo clippy`,
  `cargo test`, `cargo build` — chacune séparément, code de sortie contrôlé.
- **Tests unitaires** : dans les modules (`#[cfg(test)] mod tests`), au plus près du domaine, sans
  base ni double (`memless-domain` est pur).
- **Tests d'intégration Rust** : sous `crates/<crate>/tests/`, contre l'API publique du crate.
- **Test de coïncidence de la règle de comparaison** : confronter la règle du domaine et le
  comportement de GlueSQL sur un corpus de paires (`"5"` vs `5`, décimaux) — garde-fou contre une
  divergence après une montée de version de GlueSQL (§4.3, §12.2).
- **Test de round-trip « lire → réécrire → diff vide »** : charger un fichier, le réécrire sans
  changement de donnée, et exiger un diff vide (hors exceptions connues de la décision 26 du brief) —
  verrouille l'ordre des colonnes que Memless porte hors de GlueSQL (§8.6, décisions 13/27 du brief).
- **Banc de parité** (`harness/parity/`) — **le test décisif du projet** : rejouer un même corpus de
  fichiers, requêtes et transactions dans les trois langages et **exiger le même résultat et le même
  message d'erreur** (§5.1, §8.8, §13 du brief). Il tourne dans la CI sur chaque plateforme cible ; il
  transforme la parité de promesse en fait vérifié.
- **Banc de mesure** (`benches/` d'un crate, ou `harness/`) : faire varier taille de fixtures, nombre
  d'écritures et de rechargements pour trancher l'hypothèse §17.2 du brief.
- **pre-commit** : brancher `fmt`, `clippy` et les portes rapides avant chaque commit (à poser au
  scaffold).
