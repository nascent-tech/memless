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
| Pont natif | le traducteur qui porte l'appel d'un langage jusqu'au cœur, sans jamais décider à sa place | npm + napi-rs · Composer + FFI · module Go + purego | paquet par langage, chargé dans le processus hôte |
| Adaptateur entrant | le comptoir d'accueil : décode l'appel, retrouve l'instance, appelle le cas d'usage, rend la réponse et sa propriété mémoire | Rust — `memless-capi` (unique `cdylib`, C ABI) | lié à la compilation dans la bibliothèque native |
| Handle d'instance | la poignée numérotée du vestiaire : désigne une copie en mémoire et son unique transaction | Rust — table d'instances opaques, protégée entre fils (§10.5) | en mémoire, dans le processus |
| Cas d'usage | le chef d'atelier qui orchestre chargement, requête, écriture, transaction, rechargement | Rust — `memless-core::application` | bibliothèque native |
| Chargeur + parseur YAML | l'ouvreur de boîte : lit le fichier, refuse un YAML invalide ou une valeur imbriquée | Rust — adaptateur `yaml` (bibliothèque YAML, §7) | bibliothèque native |
| Inférence de structure | le trieur qui reconnaît le contenu sans étiquette : type de chaque valeur, colonne `id`, relations par le nom | Rust pur — `memless-domain` | bibliothèque native |
| Règle de comparaison | l'arbitre qui refuse d'égaler `"5"` et `5` | Rust pur — `memless-domain` pour `id` et relations ; GlueSQL pour les comparaisons SQL (voir 4.3) | bibliothèque native |
| Garde du sous-ensemble SQL | le portier : refuse le DDL (`CREATE`/`ALTER`/`DROP`) et le hors-sous-ensemble, crée la table à la première écriture | Rust — port `application::SqlGate`, implémenté dans l'adaptateur `gluesql/` (il s'appuie sur l'analyseur de GlueSQL) | bibliothèque native |
| Store GlueSQL | l'entrepôt : détient les lignes en mémoire et les sert à GlueSQL ; porte aussi l'ordre des tables/colonnes que GlueSQL perd (§8.6) | Rust — *newtype* autour de `gluesql-memory-storage::MemoryStorage` | bibliothèque native |
| Moteur GlueSQL | le juriste du SQL : analyse et exécute le texte, sans rien deviner ni persister | crate `gluesql` (Apache-2.0) | lié à la compilation |
| Validateur d'invariants | le contrôleur qualité de fin de chaîne : à la validation, `id` présent et unique, relations intactes | Rust pur — `memless-domain` | bibliothèque native |
| Gestionnaire de transaction | le gardien du tout-ou-rien : une seule transaction ouverte, lecture de ses propres écritures, annulation par snapshot — `MemoryStorage` n'offre pas de transaction native | Rust — `application` + snapshot par `clone()` de l'état | bibliothèque native |
| Écrivain YAML atomique | le copiste prudent : temporaire + `fsync` + `rename` ; ordre stable pour un diff lisible | Rust — adaptateur `yaml` | bibliothèque native |

### 4.1 Bordure, passerelle, BFF

Ces notions supposent un réseau ; Memless n'en a aucun. **Aucun Edge/CDN, aucune API Gateway, aucun
BFF** (*Backend For Frontend*, une couche qui adapte un service aux besoins d'un client) : le seul
« client » est le code appelant dans le même processus, et il n'y a rien à router, mettre en cache ou
protéger en bordure. Le pont natif joue un rôle comparable à un adaptateur de bordure — il traduit
l'appel d'un langage vers le contrat du cœur — mais sans distribution ni cache, dans la même mémoire.

### 4.2 Un point à trancher — le chemin de Node

Node passe par un addon napi-rs — un mince enrobage compilé du **même cœur**, sans logique propre.
Reste à décider s'il dépend de `memless-core` en Rust ou lie `memless-capi` par C ABI, et s'il expose
des types plus riches ou colle au contrat partagé — ce qui touche la vérification de parité (§5.1 du
brief) et le nombre de binaires à empaqueter. Le détail vit en §12.3, question 3.

### 4.3 Un point à surveiller — la règle de comparaison à deux endroits

La décision 34 du brief veut **une seule** règle de comparaison. En pratique deux moteurs
l'appliquent : le domaine pour l'unicité d'`id` et les relations, GlueSQL pour les comparaisons dans
les requêtes SQL. Le brief a vérifié une fois que GlueSQL sans schéma ne convertit jamais un type dans
un autre (§5.2 du brief). Pour que les deux restent d'accord à chaque montée de version de GlueSQL, le
banc de parité inclut des paires témoins (`"5"` vs `5`, décimaux — décision 20 du brief) ; le risque
de divergence est inscrit en dette (§12.2).
