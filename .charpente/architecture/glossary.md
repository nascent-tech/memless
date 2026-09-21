<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§12. Vision et décisions](roadmap.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §13. Glossaire

Termes propres au projet, expliqués simplement ; pas un dictionnaire de standards.

- **Structure devinée** — la forme des tables (types, identifiant, relations) déduite des seules
  données, jamais déclarée à part ; redérivée à chaque vérification (§10 du brief, décision 37 du
  brief).
- **Relation devinée** — une colonne dont le nom finit par `_id`, traitée comme pointant vers la table
  au pluriel (simple ajout d'un `s`) du nom qui précède, si elle existe (décision 31 du brief) ; le
  terme du brief pour une clé étrangère non déclarée.
- **Règle de comparaison** — deux valeurs de types différents ne sont jamais égales ni ordonnées
  (`"5"` ≠ `5`) ; elle gouverne l'unicité d'un `id`, la reconnaissance d'une relation et toute
  comparaison SQL (décision 34 du brief).
- **Invariant** — une condition qui doit rester vraie de tout état accepté (ici : chaque ligne a un
  `id`, aucun `id` dupliqué, aucune relation cassée) ; vérifiée à la validation, comme un contrôle
  qualité en bout de chaîne.
- **Transaction tout-ou-rien** — un ensemble d'écritures qui réussissent toutes ensemble ou pas du
  tout ; une seule ouverte à la fois par instance (§8.5 du brief).
- **Snapshot (instantané)** — une copie de l'état prise à l'ouverture d'une transaction, gardée pour
  revenir en arrière si la transaction est abandonnée ou refusée.
- **ACID** — les quatre garanties d'une transaction fiable ; ici, tout-ou-rien et durable à l'échelle
  du fichier entier.
- **Réécriture atomique** — écrire d'abord un fichier temporaire (dans le même répertoire), forcer son
  écriture sur le disque (`fsync`), puis le substituer d'un coup (`rename`), de sorte qu'une coupure
  laisse l'ancien fichier intact ou le nouveau complet, jamais un mélange (décision 17 du brief).
- **Cœur commun** — le moteur unique, en Rust, dont les trois langages partagent le comportement
  (décision 11 du brief).
- **Pont natif** — l'adaptateur par lequel un langage appelle le cœur sans réseau ni réécriture de la
  logique : `napi-rs` (Node), `FFI` (PHP), `purego` (Go).
- **Port / adaptateur** — un *port* est un contrat (en Rust, un `trait`) ; un *adaptateur entrant*
  traduit un appel du dehors vers un cas d'usage, un *adaptateur sortant* implémente un besoin du cœur
  (SQL, fichier). La prise et l'appareil qu'on y branche.
- **DDL** (*Data Definition Language*) — les ordres SQL qui changent la forme des tables
  (`CREATE`/`ALTER`/`DROP TABLE`) ; Memless les refuse, la forme n'étant jamais déclarée (décision 24
  du brief).
- **ACL** (*Anti-Corruption Layer*, couche anticorruption) — un sas qui traduit entre deux modèles
  pour qu'aucun ne contamine l'autre ; ici, entre le modèle de lignes du domaine et les types de
  valeurs de GlueSQL.
- **GlueSQL sans schéma** — le mode de GlueSQL qui accepte des types différents pour une même colonne
  d'une ligne à l'autre et ne connaît ni relation ni clé primaire ; Memless devine ces deux-là
  lui-même (§5.2 du brief).
- **Store GlueSQL** — l'entrepôt de lignes en mémoire que Memless fournit à GlueSQL pour qu'il exécute
  le SQL ; sur mesure ou dérivé du stockage mémoire de GlueSQL reste à trancher (§12.3, question 1).
- **Handle d'instance** — la poignée opaque qu'un pont reçoit à la création d'un moteur et repasse à
  chaque appel ; elle désigne une copie en mémoire et son unique transaction éventuelle.
- **cdylib** — bibliothèque compilée dynamique, le fichier `.so`/`.dylib`/`.dll` que presque tout
  langage sait charger et appeler.
- **FFI** (*Foreign Function Interface*) — le mécanisme par lequel un langage appelle directement une
  bibliothèque écrite dans un autre, sans réseau ni format intermédiaire ; la prise murale qui laisse
  se brancher un appareil d'une autre marque.
- **C ABI** (*Application Binary Interface*) — la convention d'appel bas niveau, comprise par presque
  tous les langages, qu'expose la bibliothèque Rust compilée ; ce que Go et PHP chargent.
- **C4** — une façon de dessiner une architecture à quatre niveaux de zoom (contexte, conteneur,
  composant, code), du plus large au plus fin.
- **Banc de parité** — la suite de tests qui rejoue le même SQL et le même état dans les trois langages
  et exige un résultat identique ; elle transforme la parité de promesse en fait vérifié.
