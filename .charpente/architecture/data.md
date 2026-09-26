<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§7. Stack](stack.md)  
**Suivant** : [§9. Dépendances externes](dependencies.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §8. Données

Le coffre de Memless a une particularité : sa vraie serrure est un fichier texte que n'importe qui
peut lire et corriger à la main, et dont le moteur ne garde qu'une copie de travail en mémoire.

### 8.1 Bases

Il n'y a **aucune base de données au sens classique** — pas de moteur externe, pas de serveur.

- **Le fichier YAML** est la **seule source de vérité** (§1, décision 3 du brief). Chaque instance en
  a sa propre copie ; rien n'est partagé entre instances (décision 6 du brief).
- **L'état en mémoire** est la **copie de travail** : `memless-domain::Base`. L'exécuteur SQL la lit et
  la modifie pour exécuter le texte SQL, l'inférence la lit pour deviner la structure, l'écrivain YAML
  la sérialise vers le disque. Il ne s'agit pas d'une seconde source de vérité — toute transaction
  validée la reverse aussitôt au fichier. `Instance { path, base: Base, transaction: Option<Base> }` :
  l'état de travail d'une transaction ouverte est un `base.clone()` ; valider vérifie l'état proposé,
  l'écrit, puis avance `base` dessus.

Aucune isolation de schémas ni requête inter-schémas ne se pose : il n'y a qu'un seul espace de
données par instance.

### 8.2 Cache

**Aucun.** Un cache introduirait une seconde copie à invalider, exactement ce que le produit refuse.
La copie de travail en mémoire n'est pas un cache du fichier : c'est l'état vivant que toute
transaction validée reverse aussitôt au fichier.

### 8.3 Identifiants

La clé primaire est **toujours la colonne `id`** (décision 30 du brief), au type **texte ou entier**
uniquement — un `id` décimal ou vrai/faux n'est jamais chargé. Memless ne **génère jamais** d'identifiant —
les données sont écrites à la main (§3.4 du brief). Le risque classique d'un auto-incrément exposé est
donc sans objet : Memless n'attribue aucun identifiant séquentiel. L'unicité et l'appariement des
`id` obéissent à la règle de comparaison (`"5"` ≠ `5`, décision 34 du brief).

### 8.4 Cohérence

**Tout-ou-rien à l'échelle du fichier entier**, sans multi-agrégat. Une transaction validée doit laisser
un état final cohérent : chaque ligne a un `id`, aucun `id` dupliqué dans une table, aucune relation
devinée cassée (§8.4 du brief). Un état intermédiaire de transaction ouverte peut sembler
transitoirement invalide ; seul l'état à la validation compte (décision 15 du brief), et les deux
ordres de suppression y passent (§3.5). Comme la frontière de cohérence est l'état entier — et non
plusieurs agrégats répartis — il n'y a **ni saga, ni outbox, ni cohérence éventuelle** : tout est
tranché localement, d'un coup, à la validation.

La durabilité tient à l'écriture par substitution complète et à l'annulation en mémoire si le disque refuse
(décisions 17, 18 du brief ; mécanisme détaillé en §5.3).

### 8.5 Migrations

**Aucune, et c'est structurel.** Sans schéma déclaré, il n'existe ni `CREATE TABLE`, ni `ALTER
TABLE`, ni `DROP TABLE` : la garde du sous-ensemble SQL (§4) les refuse (décision 24 du brief). Une
table ou une colonne apparaît en étant simplement utilisée par une ligne — écrire dans une table
absente la crée, geste porté par cette même garde (§8.4 du brief) — et disparaît quand plus aucune
ligne ne la porte (une colonne) ou par édition à la main du fichier suivie d'un rechargement (une
table). Il n'y a donc aucun outil de migration, aucun ordre à tenir, aucun rollback de schéma.

La seule question voisine d'une migration est la **compatibilité entre versions de fichier** écrites
par des ponts différents : hors périmètre au lancement, où une seule version existe (§14 du brief) ;
une marque de version dans le format est une évolution de la feuille de route (§12.1).

### 8.6 Écriture stable pour un diff lisible

Exigence de données, pas option : la réécriture conserve l'ordre des tables, des lignes et des
colonnes déjà présentes ; une ligne nouvelle se place à la fin de sa table (décisions 13, 27 du
brief ; `memless-domain::base::write::next_position`). Ce qui **n'est pas** garanti de survivre :
commentaires, mise en forme libre, `null` écrit à la main, forme textuelle exacte d'un décimal
(décision 26 du brief). L'**égalité octet pour octet entre les trois langages, elle, est garantie et
vérifiée** (décision 25 du brief) — le banc de parité en fait la preuve par une empreinte `sha256` du
fichier réécrit, sur chaque fixture, requête et transaction.

**Point tranché par la recherche.** L'ordre des colonnes n'est porté par aucune bibliothèque
générique : `RawDocument` (`memless-domain::document`) le porte lui-même, alimenté depuis le fichier
YAML au chargement et complété par la règle « colonne nouvelle en fin de ligne » — capté par ligne, au
chargement. Un test de round-trip « lire → réécrire → diff vide » (`crates/memless-engine/tests/
golden.rs`) verrouille les décisions 13/27 du brief (§11.4).
