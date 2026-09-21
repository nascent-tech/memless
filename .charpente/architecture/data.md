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
- **L'état en mémoire** est la **copie de travail** : GlueSQL la lit et la modifie pour exécuter le
  SQL, l'inférence la lit pour deviner la structure, l'écrivain YAML la sérialise vers le disque. Il
  ne s'agit pas d'une seconde source de vérité — toute transaction validée la reverse aussitôt au
  fichier. Cette copie **réutilise `gluesql-memory-storage::MemoryStorage`** (décision tranchée,
  ex-question 1 — recherche confirmée sur GlueSQL 0.20 : ses champs sont publics, l'ordre des lignes y
  est conservé, un Store sur mesure n'apporterait rien de plus) derrière un *newtype* Memless qui
  intercepte les écritures pour tenir l'ordre des tables et des colonnes (§8.6) et fournir le
  tout-ou-rien que `MemoryStorage` n'offre pas — le snapshot d'une transaction est un `clone()`.

Aucune isolation de schémas ni requête inter-schémas ne se pose : il n'y a qu'un seul espace de
données par instance.

### 8.2 Cache

**Aucun.** Un cache introduirait une seconde copie à invalider, exactement ce que le produit refuse.
La copie de travail en mémoire n'est pas un cache du fichier : c'est l'état vivant que toute
transaction validée reverse aussitôt au fichier.

### 8.3 Identifiants

La clé primaire est **toujours la colonne `id`** (décision 30 du brief), au **type libre** : texte,
entier, décimal ou vrai/faux, deviné valeur par valeur. Memless ne **génère jamais** d'identifiant —
les données sont écrites à la main (§3.4 du brief). Le risque classique d'un auto-incrément exposé est
donc sans objet : Memless n'attribue aucun identifiant séquentiel. L'unicité et l'appariement des
`id` obéissent à la règle de comparaison (`"5"` ≠ `5`, décision 34 du brief).

### 8.4 Cohérence

**ACID à l'échelle du fichier entier**, sans multi-agrégat. Une transaction validée doit laisser un
état final cohérent : chaque ligne a un `id`, aucun `id` dupliqué dans une table, aucune relation
devinée cassée (§8.4 du brief). Un état intermédiaire de transaction ouverte peut sembler
transitoirement invalide ; seul l'état au commit compte (décision 15 du brief), et les deux ordres de
suppression y passent (§3.5). Comme la frontière de cohérence est l'état entier — et non plusieurs
agrégats répartis — il n'y a **ni saga, ni outbox, ni cohérence éventuelle** : tout est tranché
localement, d'un coup, à la validation.

La durabilité tient à l'écriture atomique complète et à l'annulation en mémoire si le disque refuse
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
brief). Ce qui **n'est pas** garanti de survivre : commentaires, mise en forme libre, `null` écrit à
la main, forme textuelle exacte d'un décimal (décision 26 du brief), et l'égalité octet pour octet
entre les trois langages (décision 25 du brief). Ces exceptions sont connues et assumées (§9.1 du
brief).

**Point tranché par la recherche, à conséquence directe.** GlueSQL sans schéma range les colonnes
d'une ligne dans un `BTreeMap` **trié alphabétiquement** : l'ordre d'écriture des colonnes est perdu
à l'intérieur de GlueSQL. Memless doit donc porter l'ordre des colonnes **hors** de GlueSQL, dans son
*newtype* de stockage (§8.1), alimenté depuis le fichier YAML au chargement et complété par la règle
« colonne nouvelle en fin de ligne ». Reste une sous-décision (§12.3, question 1-bis) : capter cet
ordre par table ou par ligne, et à quel moment (au chargement + règle, ou en analysant le littéral de
l'`INSERT` dans la garde SQL). Un test de round-trip « lire → réécrire → diff vide » verrouille les
décisions 13/27 du brief (§11.4).
