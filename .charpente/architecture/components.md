<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§3. Diagrammes](diagrams.md)  
**Suivant** : [§5. Solidité du système](system-design.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §4. Composants

Chaque composant est une pièce de l'usine à requêtes ; ensemble ils transforment un texte SQL et un
fichier de données en un état cohérent, réécrit sans risque. Tous vivent dans le même processus que
le code appelant — aucun n'est déployé à part.

| Composant | Responsabilité (métaphore) | Technologie | Déploiement |
|---|---|---|---|
| Pont natif | le traducteur qui porte l'appel d'un langage jusqu'au cœur, sans jamais décider à sa place | Composer + FFI · module Go + purego · npm + koffi (FFI dynamique) | paquet par langage, chargé dans le processus hôte |
| Adaptateur entrant | le comptoir d'accueil : décode l'appel, retrouve l'instance, appelle le cas d'usage, rend la réponse et sa propriété mémoire | Rust — `memless-capi` (unique `cdylib`, C ABI) | lié à la compilation dans la bibliothèque native |
| Handle d'instance | la poignée numérotée du vestiaire : désigne une copie en mémoire et son unique transaction | Rust — table d'instances opaques, protégée entre fils (§10.5) | en mémoire, dans le processus |
| Cas d'usage | le chef d'atelier qui orchestre chargement, requête, écriture, transaction, rechargement | Rust — `memless-engine::application` (charger, interroger, écrire, ouvrir/valider/abandonner, recharger) | bibliothèque native |
| Lecteur + analyseur YAML | l'ouvreur de boîte : lit le fichier, refuse un YAML invalide ou une valeur imbriquée | Rust — `memless-engine::yaml` (`reader`, `parse`) | bibliothèque native |
| Inférence de structure | le trieur qui reconnaît le contenu sans étiquette : type de chaque valeur, colonne `id`, relations par le nom | Rust pur — `memless-domain` | bibliothèque native |
| Règle de comparaison | l'arbitre qui refuse d'égaler `"5"` et `5` | Rust pur — `memless-domain` seule, y compris pour les comparaisons d'une requête SQL (l'exécuteur vit dans le domaine, voir 4.3) | bibliothèque native |
| Garde du sous-ensemble SQL | le portier : refuse le DDL (`CREATE`/`ALTER`/`DROP`) et le hors-sous-ensemble (dont `LIMIT`/`OFFSET`/`GROUP BY`), crée la table à la première écriture | Rust — `memless-engine::sql` (analyse par `sqlparser` 0.54, `GenericDialect`, abaissement vers `Statement`, refus par les `reject_*`, type `OutsideSubset`) | bibliothèque native |
| Base | l'entrepôt : détient les tables et leurs lignes, dans l'ordre du fichier ; l'état de travail d'une transaction en est un clone | Rust pur — `memless-domain::Base` (`RawDocument` porte l'ordre des colonnes) | bibliothèque native |
| Exécuteur SQL | le juriste du SQL : exécute le `Statement` abaissé (lire, trier, joindre, agréger, écrire), sans rien deviner ni persister | Rust pur — `memless-domain::base::{select, write}` | bibliothèque native |
| Vérification des contraintes | le contrôleur qualité de fin de chaîne : à la validation, `id` présent et unique, relations intactes | Rust pur — `memless-domain` | bibliothèque native |
| Gestionnaire de transaction | le gardien du tout-ou-rien : une seule transaction ouverte, lecture de ses propres écritures, annulation par clone | Rust — `Instance { path, base: Base, transaction: Option<Base> }`, clone de `base` à l'ouverture | bibliothèque native |
| Écrivain YAML par substitution | le copiste prudent : temporaire + `fsync` + `rename` + `fsync` du répertoire ; ordre stable pour un diff lisible | Rust — `memless-engine::yaml::writer` (temporaire `.<nom>.memless-tmp`) | bibliothèque native |

### 4.1 Bordure, passerelle, BFF

Ces notions supposent un réseau ; Memless n'en a aucun. **Aucun Edge/CDN, aucune API Gateway, aucun
BFF** (*Backend For Frontend*, une couche qui adapte un service aux besoins d'un client) : le seul
« client » est le code appelant dans le même processus, et il n'y a rien à router, mettre en cache ou
protéger en bordure. Le pont natif joue un rôle comparable à un adaptateur de bordure — il traduit
l'appel d'un langage vers le contrat du cœur — mais sans distribution ni cache, dans la même mémoire.

### 4.2 Le chemin de Node — tranché

Node charge la même `memless-capi` (C ABI) que Go et PHP, par `koffi` ≥ 2.16 (FFI dynamique) plutôt que
par un addon compilé (napi-rs) — décision humaine du 2026-09-22. Un seul binaire natif à empaqueter
pour les trois ponts.

### 4.3 La règle de comparaison, à un seul endroit

Il n'y a plus deux moteurs : une seule règle de comparaison, un seul endroit. L'exécuteur SQL
(`memless-domain::base::select`, `base::write`) applique la même règle de comparaison
(`memless_domain::scalar::scalar_order`, fonction `compare_same_type`) que l'unicité d'`id` et les relations —
exigence de la décision 34 du brief, désormais garantie par construction : il n'existe qu'un seul
moteur de comparaison dans tout le crate, pas deux à tenir d'accord.
