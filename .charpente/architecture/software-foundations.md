<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§5. Solidité du système](system-design.md)  
**Suivant** : [§7. Stack](stack.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §6. Fondations logicielles

Une usine bien pensée sépare l'atelier où l'on décide (le métier) des quais où entrent et sortent les
marchandises (les transports et le stockage) : on peut changer un quai sans toucher à l'atelier.
C'est ce que cherche l'architecture de Memless.

### 6.1 Paradigme — hexagonal (ports et adaptateurs)

**Recommandation : architecture hexagonale.** Le brief impose (§5.1 du brief) que *toute* règle du
métier — inférence de structure, invariants, comparaison, transaction, lecture/écriture YAML — vive
dans un **cœur unique**, et que trois langages l'appellent sans jamais décider à sa place. C'est
exactement la promesse de l'hexagonal : un cœur pur, entouré de ports (des contrats) qu'implémentent
des adaptateurs interchangeables — la prise et l'appareil qu'on y branche.

- **Domaine** (`memless-domain`, crate pur, déterministe, sans I/O) : l'inférence, la règle de
  comparaison, les invariants, le modèle de lignes.
- **Application** (`memless-core::application`) : les cas d'usage (charger, interroger, écrire,
  transaction, recharger) qui orchestrent, et les ports (des `trait`) qu'ils appellent.
- **Adaptateurs entrants (primaires)** : la surface C ABI de `memless-capi` et les trois ponts. Ils
  traduisent un appel étranger en appel de cas d'usage — aucun invariant métier chez eux.
- **Adaptateurs sortants (secondaires)** : le Store GlueSQL (exécution SQL sur l'état) et l'écrivain
  YAML (lecture + écriture atomique), tous deux derrière un port. La **garde du sous-ensemble SQL**
  (§4) n'est pas un module libre : elle a besoin de l'analyseur de GlueSQL, elle vit donc dans
  l'adaptateur `gluesql/`, derrière un port `application::SqlGate` que les cas d'usage appellent.

Ce que ce choix **interdit** : qu'un pont porte une règle, qu'un adaptateur décide du métier, ou que
le domaine dépende de GlueSQL, de la bibliothèque YAML ou du mécanisme FFI. En Rust, la frontière
compilée est le crate : `memless-domain` ne dépendant d'aucun crate externe, sa pureté est imposée
par Cargo (un cycle ne compile pas). Une nuance essentielle : l'adaptateur `gluesql` **traduit** (une
ACL — couche anticorruption) entre le modèle de lignes du domaine et les types de valeurs de GlueSQL,
pour que ces derniers ne remontent jamais dans le domaine.

Comme aucun `.charpente.json` n'existe encore (`/charpente:init` n'a pas tourné), aucune clé
`architecture` n'est posée : à l'initialisation, la fixer à **`hexagonal`**, sinon les gardes de
frontière ne contrôleront pas l'arbre (voir §11).

### 6.2 Métier — un seul contexte, un cœur qui devine

Memless tient dans **un seul contexte borné** — *le moteur* — ; le brief ne nomme aucune frontière
interne entre plusieurs métiers. À l'intérieur, trois sous-domaines se distinguent par leur nature :

| Sous-domaine | Sorte | Pourquoi |
|---|---|---|
| Inférence de structure + intégrité | **cœur de métier** | c'est le différenciateur du produit (§3.3 du brief) : deviner types, `id` et relations, et les protéger — ce qu'aucun concurrent ne réunit |
| Exécution SQL | **générique** | achetée/réutilisée : GlueSQL fait l'analyse et l'exécution, Memless ne la réécrit pas (§5.2 du brief) |
| Persistance YAML (lecture + écriture atomique) | **support** | nécessaire mais non différenciante ; sert le cœur |

L'« agrégat » a ici une forme inhabituelle : la **frontière de cohérence transactionnelle est l'état
entier** (toutes les tables), parce qu'une transaction validée réécrit le fichier entier d'un coup.
Il n'y a donc pas plusieurs agrégats à coordonner, mais un seul ensemble cohérent — ce qui écarte
d'emblée toute saga ou tout outbox (§8.4).

### 6.3 Dépendances — sens autorisé

Sens unique, vers l'intérieur : `bindings/*` → `memless-capi` → `application` → `memless-domain` ; et
`application` → ports ← adaptateurs sortants. **Interdits** : `memless-domain` → GlueSQL / YAML /
FFI ; un pont → le domaine directement ; un `trait` « port » déclaré dans un adaptateur et importé par
le cœur. Ces règles seront vérifiées par `guard-boundaries` / `guard-coupling` une fois
`.charpente.json` posé.

### 6.4 CQRS

**Pertinent au sens léger, sans machinerie.** Le brief sépare déjà lire (§8.3 du brief) et modifier
(§8.4 du brief). En pratique : une **requête** de lecture traverse GlueSQL sans ouvrir de transaction
ni muter l'état ; une **commande** d'écriture passe par la garde SQL, la transaction et, au commit,
par les invariants puis l'écriture atomique. Nul besoin de bus, de bases séparées ni de modèles de
lecture distincts — la portée reste « deux chemins dans les cas d'usage », son coût est nul et son
bénéfice est la clarté du flux. Un CQRS lourd (bus, projections) serait une sur-ingénierie ici, et
est écarté.
