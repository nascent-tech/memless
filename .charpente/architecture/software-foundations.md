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
métier — inférence de structure, contraintes, comparaison, transaction, lecture/écriture YAML — vive
dans un **cœur unique**, et que trois langages l'appellent sans jamais décider à sa place. C'est
exactement la promesse de l'hexagonal : un cœur pur, entouré de ports (des contrats) qu'implémentent
des adaptateurs interchangeables — la prise et l'appareil qu'on y branche.

- **Domaine** (`memless-domain`, crate pur, déterministe, sans I/O) : l'inférence, la règle de
  comparaison, les contraintes, le modèle de lignes, et l'**exécution** SQL (`base::select`,
  `base::write`).
- **Application** (`memless-engine::application`) : les cas d'usage (charger, interroger, écrire,
  transiger, recharger) qui orchestrent, et les ports qu'ils appellent — des **types de fonctions**
  (`ReadSource`, `ParseSql`, `ReplaceFile`), pas des `trait`, injectés par `memless-capi`.
- **Adaptateurs entrants (primaires)** : la surface C ABI de `memless-capi` et les trois ponts. Ils
  traduisent un appel étranger en appel de cas d'usage — aucune règle métier chez eux.
- **Adaptateurs sortants (secondaires)** : `memless-engine::sql` (analyse par `sqlparser`, abaissement
  vers `Statement`, refus hors sous-ensemble) et `memless-engine::yaml` (lecture, analyse, rendu et
  écriture par substitution), tous deux derrière un port.

Ce que ce choix **interdit** : qu'un pont porte une règle, qu'un adaptateur décide du métier, ou que
le domaine dépende de `sqlparser`, de la bibliothèque YAML ou du mécanisme FFI. En Rust, la frontière
compilée est le crate : `memless-domain` ne dépendant d'aucun crate externe, sa pureté est imposée
par Cargo (un cycle ne compile pas).

`.charpente.json` pose `"architecture": "hexagonal"` à la racine du dépôt : les gardes de frontière
(`guard-boundaries`, `guard-coupling`) contrôlent l'arbre sur cette base.

### 6.2 Métier — un seul contexte, un cœur qui devine

Memless tient dans **un seul contexte borné** — *le moteur* — ; le brief ne nomme aucune frontière
interne entre plusieurs métiers. À l'intérieur, quatre sous-domaines se distinguent par leur nature :

| Sous-domaine | Sorte | Pourquoi |
|---|---|---|
| Inférence de structure + intégrité | **cœur de métier** | c'est le différenciateur du produit (§3.3 du brief) : deviner types, `id` et relations, et les protéger — ce qu'aucun concurrent ne réunit |
| Analyse SQL | **générique** | achetée/réutilisée : `sqlparser` fait l'analyse syntaxique et l'abaissement vers `Statement`, Memless ne réécrit pas un parseur SQL (§5.2 du brief) |
| Exécution SQL (lire, trier, joindre, agréger, écrire) | **cœur de métier** | c'est ici que vit la règle de comparaison unique et la sémantique de l'absence (§8.1 du brief) : Memless ne peut pas l'acheter, une bibliothèque générique ne connaît pas cette règle |
| Persistance YAML (lecture + écriture par substitution) | **support** | nécessaire mais non différenciante ; sert le cœur |

L'« agrégat » a ici une forme inhabituelle : la **frontière de cohérence transactionnelle est l'état
entier** (toutes les tables), parce qu'une transaction validée réécrit le fichier entier d'un coup.
Il n'y a donc pas plusieurs agrégats à coordonner, mais un seul ensemble cohérent — ce qui écarte
d'emblée toute saga ou tout outbox (§8.4).

### 6.3 Dépendances — sens autorisé

Sens unique, vers l'intérieur : `bindings/*` → `memless-capi` → `application` → `memless-domain` ; et
`application` → ports ← adaptateurs sortants. **Interdits** : `memless-domain` → `sqlparser` / YAML /
FFI ; un pont → le domaine directement. Ces règles sont vérifiées par `guard-boundaries` /
`guard-coupling` (`.charpente.json`, `architecture: hexagonal`).

### 6.4 CQRS

**Pertinent au sens léger, sans machinerie.** Le brief sépare déjà lire (§8.3 du brief) et modifier
(§8.4 du brief). En pratique : une **requête** de lecture traverse l'exécuteur SQL du domaine sans
ouvrir de transaction ni muter l'état ; une **commande** d'écriture passe par la garde SQL, la
transaction et, à la validation, par la vérification des contraintes puis l'écriture par substitution. Nul besoin de
bus, de bases séparées ni de modèles de lecture distincts — la portée reste « deux chemins dans les cas
d'usage », son coût est nul et son bénéfice est la clarté du flux. Un CQRS lourd (bus, projections)
serait une sur-ingénierie ici, et est écarté.
