<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§12. Vision et décisions](roadmap.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §13. Glossaire

Termes propres au projet, expliqués simplement ; pas un dictionnaire de standards. Le glossaire de
référence du cœur de métier est `.charpente/glossaire/le-moteur.md` ; ce qui suit s'y aligne et peut y
renvoyer.

- **Structure devinée** — la forme des tables (types, identifiant, relations) déduite des seules
  données, jamais déclarée à part ; redérivée à chaque vérification (§10 du brief, décision 37 du
  brief).
- **Relation devinée** — une colonne dont le nom finit par `_id`, traitée comme pointant vers la table
  au pluriel (simple ajout d'un `s`) du nom qui précède, si elle existe (décision 31 du brief) ; le
  terme du brief pour une clé étrangère non déclarée.
- **Règle de comparaison** — deux valeurs de types différents ne sont jamais égales ni ordonnées
  (`"5"` ≠ `5`) ; elle gouverne l'unicité d'un `id`, la reconnaissance d'une relation et toute
  comparaison SQL (décision 34 du brief), à un seul endroit du code (§4.3).
- **Contrainte** — une condition qui doit rester vraie de tout état accepté (ici : chaque ligne a un
  `id`, aucun `id` dupliqué, aucune relation cassée) ; vérifiée à la validation, comme un contrôle
  qualité en bout de chaîne (remplace « invariant », exclu §6.1 du brief).
- **Transaction tout-ou-rien** — un ensemble d'écritures qui réussissent toutes ensemble ou pas du
  tout ; une seule ouverte à la fois par instance (§8.5 du brief).
- **État de travail** — la copie de l'état (`Base::clone()`) qu'une transaction ouverte porte, sur
  laquelle s'appliquent ses écritures et ses propres lectures, jamais écrite sur le disque avant
  validation (remplace « snapshot », exclu §6.1 du brief).
- **Réécriture par substitution** — écrire d'abord un fichier temporaire (dans le même répertoire),
  forcer son écriture sur le disque (`fsync`), puis le substituer d'un coup (`rename`), de sorte qu'une
  coupure laisse l'ancien fichier intact ou le nouveau complet, jamais un mélange (décision 17 du
  brief ; remplace « réécriture atomique », « atomique » exclu §6.1 du brief).
- **Rechargement** — relire le fichier d'origine et remplacer l'état d'une instance vivante, en entier
  ou pas du tout, sans passer par le texte SQL ; refusé pendant une transaction ouverte ; un échec
  laisse l'état précédent intact (§8.9 du brief, `memless_reload`, ABI 5).
- **Résidu** — le fichier à part qu'une écriture a commencé et qu'un arrêt brutal a laissé sans
  substitution ; nom reconnaissable (`.<nom>.memless-tmp`), jamais officiel ; jamais touché au
  chargement, remplacé par l'écriture suivante (décision 17 du brief).
- **Sous-ensemble SQL** — ce que l'analyseur (`sqlparser`) et l'exécuteur du domaine savent traiter :
  lecture avec filtre, tri, jointure par relation devinée, compte et somme ; insertion, mise à jour,
  suppression ; `BEGIN`/`COMMIT`/`ROLLBACK` ; tout le reste (`LIMIT`, `OFFSET`, `GROUP BY`, `DISTINCT`,
  DDL) est refusé « outside the supported SQL subset » (§8.3 du brief).
- **Cœur commun** — le moteur unique, en Rust, dont les trois langages partagent le comportement
  (décision 11 du brief).
- **Pont natif** — l'adaptateur par lequel un langage appelle le cœur sans réseau ni réécriture de la
  logique : `FFI` (PHP), `purego` (Go), `koffi` — FFI dynamique (Node).
- **Port / adaptateur** — un *port* est un contrat (ici, un type de fonction : `ReadSource`, `ParseSql`,
  `ReplaceFile`) ; un *adaptateur entrant* traduit un appel du dehors vers un cas d'usage, un
  *adaptateur sortant* implémente un besoin du cœur (SQL, fichier). La prise et l'appareil qu'on y
  branche.
- **DDL** (*Data Definition Language*) — les ordres SQL qui changent la forme des tables
  (`CREATE`/`ALTER`/`DROP TABLE`) ; Memless les refuse, la forme n'étant jamais déclarée (décision 24
  du brief).
- **Handle d'instance** — la poignée opaque qu'un pont reçoit à la création d'un moteur et repasse à
  chaque appel ; elle désigne une copie en mémoire et son unique transaction éventuelle.
- **cdylib** — bibliothèque compilée dynamique, le fichier `.so`/`.dylib` (jamais `.dll`, Windows étant
  hors périmètre) que presque tout langage sait charger et appeler.
- **FFI** (*Foreign Function Interface*) — le mécanisme par lequel un langage appelle directement une
  bibliothèque écrite dans un autre, sans réseau ni format intermédiaire ; la prise murale qui laisse
  se brancher un appareil d'une autre marque.
- **C ABI** (*Application Binary Interface*) — la convention d'appel bas niveau, comprise par presque
  tous les langages, qu'expose la bibliothèque Rust compilée ; ce que les trois ponts chargent.
- **C4** — une façon de dessiner une architecture à quatre niveaux de zoom (contexte, conteneur,
  composant, code), du plus large au plus fin.
- **Banc de parité** — la suite de tests qui rejoue le même SQL et le même état dans les trois langages
  et exige un résultat identique, jusqu'à l'empreinte `sha256` du fichier réécrit ; elle transforme la
  parité de promesse en fait vérifié.
