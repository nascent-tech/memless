# 01 — Modèle et ports

<!-- charpente-navigation -->
**Index** : [conception — Palier 1 — Charger et refuser](00-index.md)  
**Suivant** : [02 — Domaine (`memless-domain`)](02-domaine.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## Le modèle

Le cadrage donne une chronologie : le code appelant demande un chargement (0), le
résidu est ignoré (1), puis le chargement **refuse** (2a) ou **démarre une instance**
(2b). On ne modélise pas cette chronologie ; on regroupe par responsabilité.

Un seul concept porte une décision métier au palier 1 : **vérifier en bloc la cohérence
d'un fichier lu et refuser au premier défaut**. Tout le reste — lire l'octet, traduire
l'appel d'un langage — est du transport. Le modèle tient donc en **un agrégat** et
quelques objets-valeur.

### L'agrégat : `Base`

`Base` est l'état deviné et cohérent — toutes les tables d'un fichier accepté (terme
inscrit au glossaire du contexte). Les trois questions de frontière
(`charpente:domain-modeling`) :

1. **Contrainte d'ensemble.** Quelle condition doit tenir d'un coup, sur un tout ? Une
   **relation devinée** d'une table vise une ligne d'une **autre** table (C6/C7). Aucune
   sous-partie ne peut se vérifier seule sans voir les autres. La frontière de cohérence
   est donc **l'état entier**, pas une table.
2. **Transaction.** Que touche une commande ? Le chargement vérifie **tout le fichier
   d'un coup, ou rien** (§10, §13). Une commande, un agrégat : `Base`.
3. **Contention.** Concurrence ? Chaque instance a sa copie, rien n'est partagé
   (décision 6) ; le palier ne pose aucun verrou (cadrage §8). Un agrégat large ne crée
   ici aucune contention.

`Base` est donc **la** racine d'agrégat, et la seule. Sa naissance est une fabrique
faillible — `Base::load` — qui applique le devinage et les contraintes, puis démarre ou
refuse. Au palier 1 elle ne porte **aucune transition** (ni insertion, ni suppression :
paliers 3–4) ; elle naît cohérente et ne change plus.

### Les contraintes (ex-« invariants », terme exclu §6.1)

Trois conditions que toute `Base` acceptée respecte, vérifiées à la fabrique :

- **Chaque ligne porte un `id`**, texte ou entier (C1, C2, C3).
- **Aucun `id` dupliqué** dans une même table, au sens de la règle de comparaison (C5).
- **Toute relation devinée vise une ligne existante** de la table visée (C6, C7).

Elles ne tiennent qu'après que le fichier a **la forme** d'une base (refus B) et qu'il
est **lisible** (refus A). L'ordre dans lequel ces trois familles s'établissent est
figé par D1 et porté par la fabrique (partie 2).

### Les objets-valeur

Tous immuables, égalité structurelle, construits à la vérification, jamais mutés — au
palier 1 rien ne se modifie en place :

- **`Scalar`** — une valeur d'un des quatre types (texte, entier, décimal, vrai/faux).
  Son **égalité structurelle est la règle de comparaison** : deux variantes différentes
  ne sont jamais égales (`Text("5") ≠ Integer(5)`, `Integer(5) ≠ Decimal(5.0)`). C'est le
  cœur de métier, imposé par Memless (décision 34).
- **`Id`** — un `Scalar` restreint à texte ou entier (C2/C3 refusent le reste), porté par
  une énumération à deux variantes pour gagner l'égalité et le hachage qu'exige la
  recherche de doublon C5.
- **`Table`, `Row`** — internes à `Base`, tenus **objets-valeur** et non entités : au
  palier 1 ils ne sont jamais modifiés individuellement, seulement construits puis
  remplacés en bloc à un futur rechargement. Ils portent une identité de *donnée* (nom de
  table, `id` de ligne) que les contraintes lisent, mais aucun cycle de vie propre.
  Alternative écartée : en faire des entités internes dès maintenant — machinerie
  d'identité sans transition à protéger (partie 6, décision C-4).
- **La représentation d'entrée** (`RawDocument` et ses nœuds) — une **donnée d'entrée
  immuable** (non un objet-valeur métier) qui décrit ce que l'analyseur YAML a lu **sans
  encore juger** : elle préserve l'ordre, les doublons de clés, les valeurs imbriquées et
  les clés non textuelles, pour que le domaine — et lui seul — prononce les refus B.
  Détail en partie 2.

La relation devinée n'est pas un type mais une **règle de nom** — une fonction pure
(`relation::guessed_target`) que la fabrique appelle ; il n'y a aucun objet à retenir.

## Les ports : un seul, et c'est une fonction

Un port n'existe que s'il y a quelque chose à substituer. Le palier n'a qu'un besoin
sortant — **lire le fichier** — et le cœur ne doit pas dépendre de l'adaptateur qui le
satisfait. Ce port est donc posé de la façon la plus légère qui préserve l'inversion des
dépendances : **une fonction reçue en paramètre**, pas un `trait`.

- `application::load(read, path)` déclare le type de la lecture (`ReadSource = fn(&str) ->
  Result<RawDocument, SourceRefusal>`) et dépend de **cette signature**, jamais du module
  `yaml`. La surface d'entrée (`memless-capi`) lui passe `yaml::read` — c'est la racine de
  composition, réduite à un argument. L'application n'importe donc pas l'infrastructure.
- Aucun `trait` : un seul producteur (`yaml::read`), un seul appelant. Un `trait` +
  `Arc<dyn>` n'ajouterait qu'une indirection ; le seul port à **deux** implémentations
  réelles (production/mémoire) est l'exécution SQL, qui arrive au palier 2 et justifiera
  alors un contrat en bonne et due forme. Testabilité : `load` prend un `read` factice dans
  les tests, sans disque.

C'est une lecture **minimale** de l'hexagonal, fidèle à son cœur : les couches (domaine ←
application ← infrastructure ← interface) **et** le sens des dépendances tiennent (partie 3).
Décision et alternatives écartées en partie 6 (C-5).

**Ports choisis de ne pas créer** (partie 6) : aucun port d'exécution SQL, aucun port
d'écriture de fichier, aucun port d'horloge — le palier ne lit rien du temps, n'écrit rien,
n'interroge rien ; et le port de lecture reste une fonction, sans `trait`.
