# 04 — Décisions et hors périmètre

<!-- charpente-navigation -->
**Index** : [conception — Palier 2 — Interroger](00-index.md)  
**Précédent** : [03 — Ponts et parité (`bindings/php`, `bindings/go`, `harness/parity`)](03-ponts-et-parite.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Les **choix** (C-*) et **non-choix** (N-*) que la conception tranche au-delà du cadrage, et ce
qui reste dehors. Un renvoi « C-<n> » / « N-<n> » d'une autre partie pointe ici.

## Choix (C)

| # | Choix | Pourquoi |
|---|---|---|
| C-1 | **`sqlparser` reste hors du domaine**, dans l'adaptateur `sql/` de `memless-engine`. Le domaine ne voit qu'un `Select` déjà descendu. | La pureté du domaine (archi §2) : une AST tierce dans le domaine y ferait entrer une dépendance. La frontière est le `Select`. |
| C-2 | **Le modèle de requête est `pub`** dans `memless-domain`, pas `pub(crate)`. | L'adaptateur `sql/` d'un autre crate le construit ; sans `pub` il ne pourrait pas. Le devinage interne reste `pub(crate)` (N-9 du palier 1), mais `Select` est une frontière assumée. |
| C-3 | **Le port d'analyse est une fonction** `ParseSql = fn(&str) -> Result<Select, QueryRefusal>`, pas un `trait`. | Une seule implémentation (`sql::parse`) ; le patron `ReadSource` du palier 1 (C-5) tient. Un `trait` serait une abstraction sans second appelant. |
| C-4 | **Le résultat traverse le C ABI par accesseurs typés**, pas par un JSON sérialisé. | Un seul contrat, testé une fois, sans dépendance de sérialisation dans le cœur ni de parseur JSON dans chaque pont (cadrage D7). Le `Rows` porte des types natifs jusqu'au bout. |
| C-5 | **Un objet résultat opaque** détenu par une table de handles jumelle de `instances.rs`, converti une fois en forme prête à lire. | Symétrie avec l'instance ; la propriété mémoire (dette FFI, archi §12.2) est tenue au même endroit et de la même façon. |
| C-6 | **`scalar/scalar_order.rs`** ajoute l'ordre **au sein d'un type** (jamais entre types). | Les opérateurs `< > <= >=` du `WHERE` l'exigent ; la règle de comparaison (décision 34) l'autorise au sein d'un type et l'interdit entre types — un `None` d'ordre entre types, une comparaison qui vaut faux. Le tri (`ORDER BY`) étant hors sous-ensemble (cadrage D1), aucun refus d'ordre n'existe. |
| C-7 | **La table de correspondance des libellés `construct`** (Q2) est un fichier de `sql/lower/`, jamais un fragment du texte utilisateur. | Un message stable et sûr (cadrage Q2) : le texte utilisateur pourrait contenir n'importe quoi ; le libellé est fermé. |

## Non-choix (N) — ce qu'on choisit de NE PAS créer

| # | Non-choix | Pourquoi |
|---|---|---|
| N-1 | **Aucun type `Literal`** distinct : un littéral SQL est un `Scalar`. | La grammaire des types d'un littéral est celle du devinage YAML (cadrage D1) ; un second type dédoublerait `Scalar`. |
| N-2 | **Aucun `Scalar::Null`** : l'absence d'une cellule est un `Option::None`. | `null` = colonne absente (décision 35) ; un cinquième scalaire romprait la règle de comparaison unique et la symétrie avec le chargement. |
| N-3 | **Aucun moteur SQL tiers exécutant** (GlueSQL, `MemoryStorage`). | Ils convertissent les types (cadrage D3) ; l'exécution est du produit, elle vit dans le domaine. |
| N-4 | **Aucun `GROUP BY` interne** pour les agrégats. | Le sous-ensemble n'a pas de `GROUP BY` (cadrage D1) : un agrégat porte sur tout le résultat filtré, une seule ligne rendue. |
| N-5 | **Aucun cache de structure devinée** entre requêtes. | La structure est redérivée depuis l'état à chaque vérification (décision 37, redérivation) ; au palier 2 l'état ne change pas, mais le principe tient pour le palier 3. |

## Hors périmètre de la conception

| Hors palier 2 | Où |
|---|---|
| L'écriture (`INSERT`/`UPDATE`/`DELETE`), la création de table à la première écriture, la réécriture atomique | Palier 3 — l'exécuteur `base/select/` sera rejoint par un `base/write/` sur le même `Base` |
| Les transactions, le snapshot par `clone`, la lecture de ses propres écritures | Palier 4 |
| Le pont Node/napi-rs | Hors MVP |
| Le banc de durée sur les requêtes | Paliers 3–4 |
| La révision de l'`ARCHITECTURE.md` (GlueSQL sorti, `sqlparser` entré ; §1, §2, §4, §7, §8, §9, §11.4) | Chantier d'architecture — désync consignée (cadrage D3) |
| Le repli `serde_yaml_ng`, le repli `cgo` du pont Go | Dettes (archi §12.2), hors palier |

## Découpage en pull requests (rappel, détaillé au plan)

`PR1` (domaine) → `PR2` (moteur `sql/` + cas d'usage) → `PR3` (C ABI + accesseurs) →
`{PR4` (PHP) ∥ `PR5` (Go)`}` → `PR6` (parité). Ordre imposé
`PR1→PR2→PR3→{PR4∥PR5}→PR6` ; PR4 et PR5 d'ordre libre (fichiers et chaînes d'outils
disjoints). Le plan fixe les phases, les critères vérifiables et la liste des tests.
