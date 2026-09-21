# 06 — Décisions, non-créés, hors périmètre

<!-- charpente-navigation -->
**Index** : [conception — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [05 — La batterie, le lanceur, l'espace de travail](05-banc-de-parite-et-workspace.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## Décisions figées par cette conception

Chacune tranche un choix que le build ne rouvrira pas ; l'alternative écartée est dite.
Plusieurs sont issues de la relecture (sept agents), consolidée avant validation.

| # | Décision | Alternative écartée |
|---|---|---|
| C-1 | **La frontière domaine/adaptateur passe après la lecture brute.** L'adaptateur YAML ne prononce que les refus **A** (I/O, YAML invalide) et rend une représentation d'entrée fidèle (`RawDocument`) ; **B et C sont jugés dans le domaine**. | Que l'adaptateur vérifie la forme (B) et rende une structure déjà propre : ferait fuir des règles produit dans l'infrastructure, contre la sorte `cœur de métier`. |
| C-2 | **La règle de comparaison est l'égalité structurelle de `Scalar`** (variantes distinctes jamais égales), documentée comme telle. Pas de module `comparison`. | Un module/fonction `comparison` dédié : couche de simple réexport, sans règle propre. |
| C-3 | **Le refus est typé par couche mais rendu d'une seule voix** : `SourceRefusal` (A) et `StructureRefusal` (B/C) sous un `Refusal { Source, Structure }`, un **seul** `Display`. Chaque signature ne promet que ce qu'elle produit (`read → SourceRefusal`, `Base::load → StructureRefusal`), la parité reste sur un texte unique. | Un enum plat unique : la fabrique et la lecture (`read`) promettraient des variantes qu'elles ne peuvent produire (le domaine modéliserait un `open(2)`). |
| C-4 | **`Table` et `Row` sont des objets-valeur** au palier 1 (aucune mutation en place). | Entités internes dès maintenant : machinerie d'identité sans transition à protéger avant le palier 3. |
| C-5 | **Le port de lecture est une fonction, pas un `trait`.** `application::load(read, path)` reçoit la lecture en paramètre (`ReadSource = fn(&str) -> Result<RawDocument, SourceRefusal>`) ; `memless-capi` lui passe `yaml::read` (racine de composition = un argument). L'application ne dépend donc que d'une signature, pas du module `yaml` : l'inversion des dépendances tient sans `trait`. Pas de `LoadBase`, pas de `SourceReader`, pas de `LoadCommand`, pas de `composition.rs`. Le cas d'usage rend la `Base` à la surface native, qui la range derrière un handle. | (a) un `trait` de port + `Arc<dyn>` + racine de composition dédiée dès maintenant : indirection sans seconde implémentation (elle vient au palier 2 avec GlueSQL). (b) un appel direct `application::load` → `yaml::read` : casserait l'inversion des dépendances (l'application importerait l'infrastructure). |
| C-6 | **Transport FFI à trois catégories** : `memless_load(path, out_handle, out_message) -> MemlessStatus { Ok, Refused, InvalidArgument, Internal }`. Un refus produit, une faute d'usage et un défaut interne sont distincts ; les fautes de frontière vivent dans un `FfiError` **privé à `memless-capi`**, jamais dans le domaine. | Une seule sortie chaîne (`NULL` = succès) : un bug ou un pointeur nul se présenterait comme un refus du fichier, faussant le relevé de parité. |
| C-7 | **Une instance ne tient que la `Base`** ; le Store GlueSQL n'est pas peuplé, et aucune enveloppe `Instance` n'est créée (la table range une `Base`). | Peupler `MemoryStorage` dès le chargement, ou envelopper `Instance` maintenant : machinerie d'un palier sans requête ni transaction. Le Store entre au palier 2, l'enveloppe au palier 4. |
| C-8 | **Aucun refus chiffré au chargement (D10), assumé comme dette.** La bibliothèque retenue ne se désactive pas entièrement (recherche : `budget: None` laisse des limites de parseur et d'alias ; seul un budget **saturé** approche §12.3, laissant deux limites de directives et le risque de débordement de pile — qui n'est pas une panique rattrapable et abandonne le processus, d'où le budget saturé plutôt que `None`). Le réglage exact (budget saturé) se fige au plan ; le résidu non éliminable est une **dette moyenne assumée** (fixtures écrites par la développeuse, pas hostiles — MVP §5), révisable en refus **nommé** (D1). | « Budget désactivé » tout court : faux en pratique, la bibliothèque garde des limites chiffrées. |
| C-9 | **En-tête C écrit à la main**, PHP-FFI-compatible (aucun `#include`, `FFI_SCOPE`) ; `cbindgen`, `build.rs` et `cbindgen.toml` **non créés** ; la garde de panique est fondue dans `ffi.rs`. | `cbindgen` : sa sortie par défaut (`#include`) n'est pas chargeable par PHP FFI, et trois fonctions ne justifient pas une dépendance de build. `cbindgen` reviendra quand le contrat grossira. |
| C-10 | **Le crate d'application se nomme `memless-engine`.** | `memless-core` : `core` est un nom fourre-tout interdit, et le crate contient aussi de l'infrastructure (`yaml`). |
| C-11 | **L'analyse lit au niveau événements/arbre, jamais par désérialisation `serde`.** Seul ce niveau préserve les doublons de clés (B5/B6) et les clés non textuelles (B7). Le point d'entrée précis de la bibliothèque se fige au plan (stack §7). | La désérialisation `serde` vers une carte : fusionnerait les doublons et rejetterait les clés non textuelles — B5/B6/B7 deviendraient impossibles à prononcer. |
| C-12 | **La grammaire du devinage de type est figée dans le domaine** (`scalar.rs`), et le style du scalaire (`Plain`/`ExplicitText`) est porté par la représentation d'entrée. | Laisser le type au dialecte de la bibliothèque : `Scalar::guess` étant dans le domaine, la grammaire est une règle produit — elle doit être écrite, pas héritée. |

**Aucune question ouverte ne subsiste** pour le périmètre du palier 1 : chaque décision
est tranchée. Les points renvoyés plus bas relèvent d'autres paliers ou du chantier
d'architecture, pas de cette conception.

## Choisis de ne pas être créés

Le plus petit ensemble qui satisfait le cadrage exclut, sciemment :

- **N-1 — GlueSQL et le Store.** Aucune requête au palier 1 ; l'adaptateur `gluesql/` et
  le *newtype* (une struct qui enveloppe un type existant pour lui ajouter un comportement)
  autour de `MemoryStorage` entrent au palier 2.
- **N-2 — La garde du sous-ensemble SQL (`SqlGate`).** Rien à garder sans SQL.
- **N-3 — Toute détection du résidu.** Lire **exactement** le chemin donné satisfait par
  construction « ignorer le résidu, ne jamais y toucher » (décision 17, D16) et « un chemin
  pointant directement sur le résidu le charge » (D11). Aucun code ne connaît le nom réservé.
- **N-4 — L'écrivain YAML, la transaction, l'instantané.** Aucune écriture au chargement
  (cadrage §4) ; paliers 3–4.
- **N-5 — Le pont Node (`memless-node`, `bindings/node`).** Hors MVP (MVP §4).
- **N-6 — Un module `comparison`** (C-2).
- **N-7 — Un `trait` de port, un `LoadCommand`, un `composition.rs`** : le port de lecture
  est une fonction (C-5) ; le `trait` et une racine de composition dédiée arrivent au
  palier 2, à la première substitution réelle.
- **N-8 — Un port d'horloge.** Le domaine ne lit pas le temps.
- **N-9 — Un type public « structure devinée ».** Un inventaire consultable est exclu de
  façon durable (§14) ; le devinage reste interne à `Base::load`.
- **N-10 — `cbindgen`, `build.rs`, `cbindgen.toml`** (C-9) ; **N-11 — la dépendance
  `indexmap`** (aucun appelant au palier 1) ; **N-12 — l'enveloppe `Instance` côté capi** (C-7).

## Hors périmètre, renvoyé avec son propriétaire

- **Ordre des colonnes pour la réécriture** (sous-décision 1-bis de l'architecture) :
  `RawDocument` capte déjà l'ordre ; *où et quand* le porter pour la réécriture se tranche au
  **palier 3** (écriture). Sans objet au palier 1.
- **Forme exacte du nom réservé du résidu** (D11), **point d'entrée précis de la
  bibliothèque YAML et réglage du budget saturé** (stack §7, C-8, C-11) : renvoyés à
  `/charpente:plan`.
- **`/charpente:init`** : `.charpente.json` est posé en PR1 (partie 5) ; le modèle de
  branches (work/prod/staging) et les serveurs MCP restent à `init`.
- **C-8 amende D10 du cadrage** : l'option « budget désactivé » de D10 est infaisable (la
  bibliothèque garde des limites) ; la conception retient « budget saturé + dette assumée ».
  À répercuter au cadrage — **décideur : le fondateur** (D10 lui revient).
- **Trois désyncs d'`ARCHITECTURE.md`** à reprendre au **chantier d'architecture**, cette
  conception suivant le cadrage : (1) le glossaire d'archi porte encore « invariant »,
  « ACID », « réécriture atomique » (exclus §6.1, à dire « contrainte », « tout-ou-rien »,
  « réécriture par substitution », D14) ; (2) `system-design.md` propose de nettoyer le
  résidu au chargement (contredit D16) ; (3) `ARCHITECTURE.md` §2 nomme le crate
  `memless-core` (C-10). Et une prémisse à corriger : `security.md` §10.5 / `roadmap.md`
  §12.2 disent le chargement « borné par le `Budget` » — or le budget ne couvre pas le
  chemin d'analyse et ne se désactive pas entièrement (C-8) ; le palier 1 assume la dette.
- **Requêtes, écritures, transactions, rechargement, pont Node, banc de mesure, familles de
  systèmes multiples** : paliers 2–4 et au-delà (cadrage §8).

## La voie : un plan, pas la voie courte

Le découpage conclut, sur la liste figée (≈ 33 fichiers de code + 18 fixtures, 4 chaînes
d'outils : cargo, composer, go, sh) : **6 pull requests, 3 sessions — ni une seule PR, ni
une seule session**. Le travail **ne tient pas dans une session** : cette conception **ne
porte donc pas de phases**. `/charpente:plan` les répartira selon ce découpage :

1. **PR1** — workspace + `.charpente.json` + `memless-domain` (refus B/C, ordre D1, textes
   anglais).
2. **PR2** — `memless-engine` + la batterie de fixtures (le golden des textes de refus).
3. **PR3** — `memless-capi` (le contrat C ABI publié).
4. **PR4** — pont PHP (l'hypothèse §17.1 la plus risquée, traité avant Go).
5. **PR5** — pont Go (indépendant de PR4 : chaîne et fichiers distincts).
6. **PR6** — le lanceur de parité (compare PHP à Go ; ne peut arriver qu'après les deux ponts).

Ordre imposé : PR1 → PR2 → PR3 → {PR4 ∥ PR5} → PR6. Les fixtures atterrissent en **PR2**
(elles spécifient les textes), pas au lanceur. Le détail des critères de fusion autonomes et
des couplages est l'affaire de `/charpente:plan`.
