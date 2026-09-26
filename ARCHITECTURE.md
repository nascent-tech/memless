# Memless — Architecture

<!-- charpente-navigation -->
**Parties** : [§3. Diagrammes](.charpente/architecture/diagrams.md) · [§4. Composants](.charpente/architecture/components.md) · [§5. Solidité du système](.charpente/architecture/system-design.md) · [§6. Fondations logicielles](.charpente/architecture/software-foundations.md) · [§7. Stack](.charpente/architecture/stack.md) · [§8. Données](.charpente/architecture/data.md) · [§9. Dépendances externes](.charpente/architecture/dependencies.md) · [§10. Sécurité](.charpente/architecture/security.md) · [§11. Développement local](.charpente/architecture/dev-environment.md) · [§12. Vision et décisions](.charpente/architecture/roadmap.md) · [§13. Glossaire](.charpente/architecture/glossary.md)  
**Maillage** : [maillage.md](.charpente/maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

> **Statut : constaté sur le code.** **Les paliers 1 à 6 sont livrés (0.3.0 ; les révisions « paquet npm unique » et « miroir Node » partent en 0.4.0)** ; toute divergence entre
> ce document et le code est un défaut du document, à corriger dans le document. **Convention de renvoi :** « §X du brief » / « décision N du brief »
> pointent vers le brief ; un « §X » nu, vers ce document.

## §1. Le produit

Memless est un **moteur de base de données en mémoire piloté par un seul fichier YAML** : le fichier
ne porte que des données — chaque table est une liste de lignes — et Memless devine tout seul leur
forme (le type de chaque valeur, la colonne `id` qui identifie une ligne, les relations entre tables
lues dans le nom des colonnes). Il charge cet état en mémoire, le laisse interroger et modifier en
texte SQL, et réécrit le fichier à chaque transaction validée : le fichier reste la seule source de
vérité. Imagine un classeur où chaque fiche se range toute seule dès qu'on l'écrit, sans qu'on ait à
dessiner d'abord la forme des cases.

Le public est **une développeuse qui prépare les données de ses tests automatisés et de ses
démonstrations** — jamais une application en production (§14 du brief). Ce choix commande toute
l'architecture : pas de serveur, pas de réseau, pas de coordination entre écritures concurrentes, un
moteur pensé pour la taille d'un test. La contrainte fondatrice (§5.1 du brief) est qu'**un seul
cœur** serve **trois langages** — JavaScript/TypeScript, Go, PHP — à l'identique : toute règle du
métier vit dans le cœur, jamais dans un pont, sous peine de voir un langage diverger des deux autres.

Le brief renvoie le pari technique à ce document (§5.2, décision 12 du brief) ; il est tranché ainsi :
cœur en **Rust** ; l'analyse SQL est
confiée à `sqlparser` (0.54), l'exécution est écrite dans le domaine ; chaque langage appelant le cœur
par la voie la plus directe qu'il offre (`koffi`, FFI dynamique, pour Node, `FFI` intégré pour PHP,
`purego` pour Go). Memless est **gratuit et ouvert** sous licence MIT (décision 1 du brief) ; il est
publié sur GitHub (`nascent-tech/memless`), et, depuis le palier 6, sur les registres natifs de chaque
langage — **npm** (contenu montré dans un dépôt miroir `nascent-tech/memless-node`), **Packagist** (par un dépôt miroir), **le proxy Go** (module `github.com/nascent-tech/memless-go`,
publié par un dépôt miroir `nascent-tech/memless-go`) — en plus de GitHub Releases, pour que chaque pont
s'installe par l'outil natif de son langage sans bibliothèque à télécharger à la main. Les
contributions externes (issues, pull requests) sont reçues sur le dépôt principal, jamais sur les
miroirs, que seule la CI écrit (`CONTRIBUTING.md`).

## §2. Structure du projet

L'arbre ci-dessous est **constaté sur le code**, en architecture hexagonale (§6.1), sous la forme d'un
espace de travail Cargo — un dépôt qui réunit plusieurs paquets Rust (des *crates*), tel un casier à
tiroirs où chaque tiroir se ferme sur ses propres dépendances.

```text
memless/
├── Cargo.toml                  # manifeste d'espace de travail (workspace, manifeste virtuel)
├── crates/
│   ├── memless-domain/         # domaine pur : inférence, règle de comparaison, contraintes — AUCUNE dépendance
│   ├── memless-engine/         # application (cas d'usage + ports) + adaptateurs sortants
│   │   └── src/
│   │       ├── application/    # cas d'usage + ports (types de fonctions) : charger, interroger, écrire, transiger, recharger
│   │       ├── sql/            # adaptateur sortant : analyse par sqlparser + abaissement vers Statement + refus hors sous-ensemble
│   │       └── yaml/           # adaptateur sortant : lecteur, analyseur, rendu et écrivain par substitution du fichier
│   └── memless-capi/           # surface native C ABI (cdylib + staticlib + rlib) — chargée par les trois ponts
├── bindings/
│   ├── php/                    # paquet Composer (FFI) — charge memless-capi
│   ├── go/                     # module Go (purego) — charge memless-capi
│   └── node/                   # paquet npm (koffi, FFI dynamique) — charge memless-capi
└── harness/parity/             # banc de parité multi-langage (hors Cargo) : même SQL + même état → même résultat
```

- **Point d'entrée réel :** chaque langage entre par son pont ; les trois ponts chargent la même
  surface native `memless-capi` (C ABI) — aucun addon compilé séparément pour Node, `koffi` liant la
  même `cdylib` par FFI dynamique. Aucun pont ne touche le domaine directement.
- **Sens des dépendances :** `bindings/*` → `memless-capi` → `memless-engine::application` →
  `memless-domain`. Le crate `memless-domain` ne dépend d'**aucun** crate externe, et **Cargo
  l'impose** — un cycle entre crates ne compile pas.
- **Point de composition :** `memless-capi` seul, qui passe `read`, `parse` et `replace_file` (des
  types de fonctions, pas des traits) aux cas d'usage de `memless-engine::application`.

## §3. Diagrammes

Vue C4 (contexte, conteneur, composant) et séquences des flux critiques : l'écriture validée avec
réécriture par substitution du fichier, le refus d'une transaction qui casserait une relation devinée, et le
chargement qui valide ou refuse en bloc.

→ [Lire le détail](.charpente/architecture/diagrams.md)

## §4. Composants

Le handle d'instance, le lecteur YAML, l'inférence de structure, la règle de comparaison, la garde
du sous-ensemble SQL, la `Base` et son exécuteur, la vérification des contraintes, le gestionnaire de
transaction, l'écrivain YAML par substitution, et les trois ponts — chacun avec sa responsabilité et sa
technologie.

→ [Lire le détail](.charpente/architecture/components.md)

## §5. Solidité du système

Memless n'est pas un service : pas d'échelle horizontale, pas de disponibilité réseau. Sa solidité
tient à la sûreté du fichier (réécriture par substitution avec `fsync`), au tout-ou-rien
transactionnel, et au coût de la réécriture complète — mesuré au banc sur les fixtures du MVP.

→ [Lire le détail](.charpente/architecture/system-design.md)

## §6. Fondations logicielles

Paradigme hexagonal justifié pour une bibliothèque embarquée à trois ponts ; un contexte borné
unique — le moteur — dont le cœur de métier est l'inférence de structure, l'intégrité et l'exécution
SQL, l'analyse SQL (`sqlparser`) étant générique et la persistance YAML un support.

→ [Lire le détail](.charpente/architecture/software-foundations.md)

## §7. Stack

Rust, `sqlparser`, `serde-saphyr`, `purego`, FFI PHP, `koffi` — chaque outil avec sa version, tirée du
code, et sa justification tirée du brief, jamais de sa popularité.

→ [Lire le détail](.charpente/architecture/stack.md)

## §8. Données

L'état en mémoire détenu par `memless-domain::Base`, le fichier YAML comme seule source de vérité, les
identifiants texte ou entier, la cohérence tout-ou-rien à l'échelle du fichier entier, l'absence de
migrations.

→ [Lire le détail](.charpente/architecture/data.md)

## §9. Dépendances externes

Aucun service réseau. Les seules dépendances sont des bibliothèques liées à la compilation
(`sqlparser`, `serde-saphyr`) et les mécanismes de pont de chaque langage.

→ [Lire le détail](.charpente/architecture/dependencies.md)

## §10. Sécurité

Pas de réseau, pas d'authentification, pas de secret. La surface sensible est la **frontière FFI** —
pointeurs, chaînes, propriété des tampons et paniques venus du langage hôte — et la robustesse du
chargement YAML face à un fichier hostile.

→ [Lire le détail](.charpente/architecture/security.md)

## §11. Développement local

Chaîne Rust (cargo), plus les chaînes Node, Go et PHP pour les ponts ; l'en-tête C écrit à la main ; et
le banc de parité comme test de première classe.

→ [Lire le détail](.charpente/architecture/dev-environment.md)

## §12. Vision et décisions

Feuille de route (serveur, cascade, API native, verrou inter-processus, production), dettes assumées
(concurrence, sûreté FFI, budget YAML saturé, coût de réécriture, parité PHP, bibliothèque
embarquée non signée, miroir PHP qui grossit, glibc minimale non figée) et les huit questions
d'architecture, toutes tranchées (§12.3) — la question 6 (distribution) révisée au palier 6 : GitHub Releases conservé, les
registres natifs de chaque langage s'y ajoutent.

→ [Lire le détail](.charpente/architecture/roadmap.md)

## §13. Glossaire

Les termes propres au projet : structure devinée, relation devinée, règle de comparaison, contrainte,
transaction tout-ou-rien, état de travail, réécriture par substitution, rechargement, résidu,
sous-ensemble SQL, cœur commun, pont natif, port/adaptateur, DDL, handle d'instance, cdylib, FFI, C ABI,
C4, banc de parité.

→ [Lire le détail](.charpente/architecture/glossary.md)
