---
type: cadrage
titre: Palier 4 — Transiger
slug: palier-4-transiger
cree_le: 2026-09-22T01:00:00+0000
mis_a_jour_le: 2026-09-26T11:36:49+0000
branche: main
statut: perime
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../mvp/00-index.md) › **cadrage — Palier 4 — Transiger** › [conception — Palier 4 — Transiger](../conceptions/2026-09-22-palier-4-transiger/00-index.md) › [plan — Palier 4 — Transiger](../plans/2026-09-22-palier-4-transiger/00-index.md)  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/mvp/00-index.md

Cadrage du **palier 4 du MVP** — « Transiger », le **virement tout-ou-rien** (MVP §palier 4). Il
couvre le **regroupement** de plusieurs écritures en une **transaction** (§8.5, décision 9), la
**lecture de ses propres écritures** avant validation, la **validation** (une seule réécriture par
substitution de l'état final, §8.6), l'**abandon** volontaire (décision 16), et toutes les issues :
instruction refusée qui **laisse la transaction ouverte** (décision 29), validation qui **échoue**
(état final invalide ou disque), deuxième ouverture, valider/abandonner **sans** transaction.
Références `§`/`décision` au brief validé, sauf mention du MVP. La stratégie technique est tranchée
par la **décision Fable** du 2026-09-22 (voir la conception, partie 04).

## 1. Acteur et déclencheur

**Acteur.** Le **code appelant** (§6) — le test automatisé en PHP puis en Go. Il tient une
**instance vivante** (palier 1) qu'il sait interroger (palier 2) et modifier (palier 3) ; ici il
**groupe** plusieurs modifications en un geste unique.

**Déclencheur.** Le code appelant **ouvre une transaction**, soumet une ou plusieurs écritures, puis
**valide** ou **abandonne**. Une validation acceptée mute l'état en mémoire **et réécrit le fichier
une seule fois**, d'un coup (§8.6). Un abandon ne touche ni la mémoire ni le fichier.

**Ce que l'acteur observe.** Pendant la transaction, ses écritures sont **visibles à ses propres
lectures** mais **pas sur le disque** (§8.5). À la validation : soit **tout** entre (un fichier
réécrit, identique à l'octet quel que soit le langage, §8.8), soit **rien** (la validation échoue,
la mémoire revient à l'état d'avant, le fichier reste intact). À l'abandon : **rien**, aucun diff.
Une écriture isolée (palier 3) est une **transaction d'une seule instruction** (§8.5) : les deux
mécanismes ne sont pas séparés.

## 2. Besoin, séparé de la solution

| Formulé en solution | Le besoin dessous |
|---|---|
| « Grouper des écritures » | Qu'un **virement** (retirer d'une tirelire, ajouter à une autre) ne laisse **jamais**, sur le disque, l'état où l'argent a quitté la première sans arriver dans la seconde (§5.1, MVP §1). |
| « Valider / abandonner » | Qu'un geste métier composé soit **tout ou rien** : un `git diff` montre **un** changement cohérent, jamais deux moitiés (§7). |
| « Lire ses propres écritures » | Qu'un test puisse **vérifier dans la transaction** l'effet de ses écritures avant de valider, comme toute base transactionnelle. |
| « État d'avant à l'échec » | Qu'une validation impossible (état final incohérent, disque) **referme** la transaction sans rien laisser, la mémoire revenue exactement à l'état d'avant l'ouverture (§8.5, §8.6). |

Ce que le palier 4 prouve en plus : que plusieurs écritures sont **tout ou rien ensemble**, et que
la **parité** PHP/Go tient sur tout le geste (ouvrir/valider/abandonner) — le relevé §17.1 se
complète ici. Ce qu'il ne prouve pas : les familles de systèmes multiples (tenues par le fondateur,
§17.1) ; le rechargement à la demande (§8.9, hors MVP).

## 3. Vocabulaire

Termes **[neuf]** à inscrire au glossaire, en plus des paliers 1–3.

| Terme | Définition | Voisin à ne pas confondre |
|---|---|---|
| **Transaction** [neuf] | Un groupe d'écritures **tout ou rien** : ouverte, puis validée (tout entre) ou abandonnée (rien) (§8.5, décision 9). Une écriture isolée (palier 3) en est une, d'une seule instruction. | L'**écriture** isolée (palier 3), cas particulier. |
| **Ouvrir** [neuf] | Commencer une transaction sur une instance. Une seule à la fois (décision 29). | Le **chargement** d'une instance (palier 1). |
| **Valider** [neuf] | Clore une transaction en faisant **entrer** son état final : vérification de structure (décision 15), puis réécriture par substitution, puis avancée mémoire (§8.6). Peut **échouer**. | La **réécriture** (palier 3), ici pilotée par la validation. |
| **Abandonner** [neuf] | Clore une transaction en **jetant** son état de travail : ni mémoire ni fichier ne bougent (décision 16). | La **validation qui échoue** (involontaire) ; l'abandon est volontaire. |
| **État de travail** [neuf] | La copie de l'état qu'une transaction ouverte porte, sur laquelle s'appliquent ses écritures et **ses propres lectures**, jamais écrite sur le disque avant validation. | L'**état d'avant** (celui de l'instance à l'ouverture, restauré à l'abandon/échec). |
| **Lecture de ses propres écritures** [neuf] | Une requête dans une transaction ouverte voit l'**état de travail** (ses écritures non encore validées), pas l'état d'avant. Une **autre** instance sur le même fichier ne voit rien, ni pendant ni après (pas de rechargement, §8.9 hors MVP). | Une requête **hors** transaction, qui voit l'état validé. |

**Termes exclus** (§6.1) : « commit »/« rollback » (hors mots-clés SQL) — dire **valider** /
**abandonner** ; « snapshot »/« savepoint » — dire **état de travail** / **état d'avant** ;
« atomique » — dire **tout ou rien** / **par substitution** ; « stager » — dire **appliquer à
l'état de travail**.

## 4. Événements au passé sur une chronologie

| # | Événement | Origine | Observable par |
|---|---|---|---|
| 0 | **Transaction ouverte** | Commande — `BEGIN` sur une instance sans transaction ; l'état d'avant est cloné en état de travail. | Le geste ; les écritures suivantes s'y appliquent. |
| 0b | **Deuxième ouverture refusée** | Politique — *dès qu'*une transaction est déjà ouverte, *alors* `BEGIN` est refusé, l'ouverte est inchangée (décision 29). | Une erreur nommant la règle ; la transaction continue. |
| 1 | **Écriture appliquée à l'état de travail** | Politique — *dès qu'*une écriture valide (forme, noms, id) est soumise dans la transaction, *alors* elle s'applique à l'état de travail, **sans vérifier la structure d'ensemble** (unicité, relations — décision 15). | Le compte ; une lecture ultérieure la voit. |
| 1b | **Instruction refusée, transaction ouverte** | Politique — *dès qu'*une instruction (écriture **ou** lecture) est refusée (syntaxe, sous-ensemble, table/colonne, forme, id), *alors* rien n'est appliqué, **la transaction reste ouverte inchangée** (décision 29). | Une erreur nommant la règle ; la transaction continue. |
| 2a | **Validation acceptée** | Politique — *dès que* l'état final passe la vérification de structure d'ensemble (décision 15) **et** diffère de l'état d'avant, *alors* le fichier est réécrit par substitution, **puis** la mémoire avance, la transaction se referme. | Le fichier changé d'un coup ; la transaction close. |
| 2b | **Validation sans changement** | Politique — *dès que* l'état final égale l'état d'avant, *alors* la transaction se referme **sans** toucher le disque (§8.6). Une table apparue puis vidée (`t: []`) **est** un changement (décision 37) et réécrit. | La transaction close ; le fichier inchangé. |
| 2c | **Validation échouée** | Politique/Système — *dès que* l'état final viole la structure d'ensemble (W8/W9) **ou** que le disque échoue **avant la substitution** (W10), *alors* la validation **échoue**, la transaction se **referme**, la mémoire revient à l'**état d'avant**, le fichier reste intact, le résidu créé est supprimé (décision 17). | Un échec nommant la règle ; la transaction close. |
| 3 | **Transaction abandonnée** | Commande — `ROLLBACK` : l'état de travail est jeté, la mémoire et le fichier restent à l'état d'avant (décision 16). | La transaction close ; aucun diff. |
| 3b | **Valider/abandonner sans transaction refusé** | Politique — *dès qu'*aucune transaction n'est ouverte, *alors* `COMMIT`/`ROLLBACK` est refusé (décision 29). | Une erreur nommant la règle. |
| 3c | **Instance relâchée, transaction ouverte** | Système — *dès que* l'instance est relâchée sans valider ni abandonner, *alors* l'état de travail est jeté avec elle : **abandon implicite**, disque à l'état d'avant, aucun résidu (aucun fichier à part n'est créé avant la validation, décision 17). | Rien ; le fichier n'a jamais bougé. |

**La différence clé avec le palier 3.** La vérification de la **structure d'ensemble** (W8 unicité,
W9 relations) est **reportée à la validation** — un état intermédiaire peut être transitoirement
invalide (décision 15 : supprimer deux lignes qui se référencent, insérer avant le parent). En
revanche W6/W7 (id présent/typé) restent prononcés **à l'écriture** : voir le choix inscrit au
point 6. La mémoire n'est **jamais** avancée avant le point de substitution (§8.6). Le point de
substitution est le point d'acquisition (palier 3) : un échec **après** lui n'est pas un échec de
validation (la transaction est acceptée, la mémoire avance).

**Concurrence.** La transaction est un état de l'**instance** (décision 29), pas du fil appelant :
deux fils/tâches qui partagent une instance partagent **sa** transaction et son état de travail.
Memless **ne coordonne pas** ces accès (§12.2, décision 7) — l'ordre et le résultat de deux appels
concurrents sur une même instance ne sont pas garantis, c'est la responsabilité de l'appelant ; le
moteur garantit seulement la **sûreté mémoire** (il ne corrompt pas sa mémoire, ne plante pas). Deux
instances/processus sur le même fichier : dernière validation gagne (§14).

## 5. Contexte(s) et sorte

**Contexte borné : le moteur** (MVP §6). Le palier 4 exerce §8.5, §8.6, §9.1. **Sorte : `cœur de
métier`** — la transaction porte les règles produit : le tout-ou-rien (décision 9), la vérification
de structure d'ensemble reportée à la validation (décision 15), la transaction unique et les refus
de garde (décision 29), la lecture de ses propres écritures. L'analyse SQL (`sqlparser`) et
l'écrivain par substitution (palier 3) sont réutilisés tels quels.

## 6. Refus — tous les états impossibles, avant le nominal

Ordre d'établissement : **(par instruction) syntaxe → sous-ensemble → garde de transaction (T1/T2)
→ noms → forme → id ; (à la validation) structure d'ensemble → disque**. La colonne **issue** dit ce
que devient la transaction : un **refus d'instruction** la laisse **ouverte inchangée** (décision
29) ; un **échec de validation** la **referme**.

| # | État impossible | Type / message | Issue |
|---|---|---|---|
| T1 | `BEGIN` alors qu'une transaction est déjà ouverte. | `TransactionRefusal::AlreadyOpen` **[neuf]**. | reste ouverte |
| T2 | `COMMIT`/`ROLLBACK` alors qu'aucune transaction n'est ouverte. | `TransactionRefusal::NoOpenTransaction` **[neuf]**. | s.o. (aucune) |
| T3 | `BEGIN`/`COMMIT`/`ROLLBACK` avec options, imbriqués, ou plusieurs instructions ; toute forme hors le mot-clé nu. | `OutsideSubset` (Q2). | comme tout Q2 : laisse ouverte une transaction en cours |
| W1–W3 (par instruction) | Syntaxe (Q1) ; hors sous-ensemble, y compris un `SELECT` passé à l'écriture ou une écriture passée à la lecture (Q2) ; table/colonne absente **en lecture comme en écriture** (Q3/Q4, dont une table vidée sans colonnes). | `InvalidSql`/`OutsideSubset`/`UnknownTable`/`UnknownColumn` — inchangés. | laisse ouverte |
| W4/W5 (par écriture) | Longueurs de colonnes/valeurs différentes (W4) ; colonne répétée (W5). | `WriteRefusal` (palier 3) — inchangés. | laisse ouverte |
| W6/W7 (par écriture) | Ligne sans `id` ; `id` décimal ou vrai/faux. | `MissingId`/`IdNotTextOrInteger` (B) — inchangés. | laisse ouverte |
| W8/W9 (à la validation) | État final : `id` en doublon (W8), relation cassée (W9). | `DuplicateId`/`BrokenRelation` (B) — inchangés. | **referme**, mémoire à l'état d'avant |
| W10 (à la validation) | Échec disque pendant la réécriture de validation, **avant** la substitution. | `WriteRefusal::DiskWriteFailed` (palier 3). | **referme**, mémoire et fichier intacts |

**Choix inscrit — W6/W7 à l'écriture, pas à la validation.** Le brief §13 range « une ligne sans
`id` » parmi les violations de l'état final. On le prononce pourtant **à l'écriture** (comme W4/W5),
pour trois raisons, sans contredire §13 : (1) l'**état de travail est un état interrogeable**
(lecture de ses propres écritures) — une ligne se construit avec son `id`, on ne peut pas porter une
ligne sans `id` dans un état qu'on interroge ; (2) une ligne malformée est une **faute
d'instruction**, mieux laissée **rejouable** avec la transaction ouverte (esprit de décision 29),
qu'un état à corriger plus tard ; (3) l'état final ne peut alors **jamais** violer §13 sur l'`id`
(l'application le garantit). Les exemples de décision 15 (« deux lignes de même `id` », « insérer
avant le parent ») portent sur l'**unicité** (W8) et les **relations** (W9), qui restent reportées.

**Ce qui n'est PAS un refus** :
- Une transaction dont l'état final **égale** l'état d'avant : validée, **pas** de réécriture (§8.6).
- Un état **intermédiaire** transitoirement invalide en unicité/relations tant que la transaction n'est pas validée (décision 15) : toléré ; c'est la validation qui tranche.
- Une lecture dans la transaction qui voit un état pas encore sur le disque : normal (état de travail).
- **Une écriture soumise alors qu'une transaction est ouverte** : elle s'**applique à l'état de travail** (elle n'ouvre pas une transaction implicite) ; l'auto-validation d'une écriture isolée (D4) ne joue qu'**en l'absence** de transaction ouverte.

**Sort de la transaction (récapitulatif, décision 29).** La **referment** : la validation acceptée
(2a/2b), la validation échouée (2c : W8/W9/W10), l'abandon (3), et l'abandon implicite par relâche
de l'instance (3c). La **laissent ouverte** : T1, T3, et toute W1–W7 d'une instruction refusée. T2
suppose qu'aucune n'est ouverte.

**Parité.** Même issue et même message depuis PHP et Go pour chaque geste (ouvrir/appliquer/lire/
valider/abandonner), **et** même fichier à l'octet à la validation (décision 25). Le palier fournit
une **batterie rejouable** (script) que le fondateur peut relancer telle quelle sur chaque famille
de systèmes pour compléter le relevé §17.1.

## 7. Limites / seuils sourcés

**Aucun seuil chiffré ne conditionne un refus.** Le **banc §17.2** se **complète** ici : durée d'une
**transaction validée** (réécriture comprise) en plus de l'écriture isolée et du chargement du
palier 3, en faisant varier la taille du fichier ; aucun nombre fixé d'avance. C'est la mesure qui
tranche le pari du §4 du brief à l'échelle des fixtures réelles (§7).

## 8. Hors périmètre explicite

| Hors du palier 4 | Où |
|---|---|
| Les **familles de systèmes multiples** et le relevé §17.1 complet | Tenus par le fondateur (MVP §17.1) ; le palier fournit la parité PHP/Go rejouable sur une machine |
| Transactions imbriquées, points de sauvegarde (`SAVEPOINT`) | N'existent pas (décision 29, une seule transaction) |
| Le rechargement à la demande (§8.9) et son interaction avec une transaction ouverte | Hors MVP |
| Le verrouillage / l'isolation inter-fils ou inter-processus | Hors lancement (§12.2, §14, décision 7) — responsabilité de l'appelant |
| Le pont JavaScript/TypeScript | Hors MVP (MVP §4) |

## 9. Décisions restantes et qui tranche

Toutes tranchées par la **décision Fable** (voir la conception, partie 04) selon le principe le plus
prudent aligné sur le brief. Les points saillants :

| # | Décision | Tranché par défaut |
|---|---|---|
| D1 | **Grammaire de transaction.** | `BEGIN`/`START TRANSACTION`, `COMMIT`, `ROLLBACK` **nus** (aucune option) ; tout le reste → T3 (Q2). `parse` gagne `Statement::Begin`/`Commit`/`Rollback`. |
| D2 | **État de travail = un `Base` de travail porté par l'`Instance`.** | Une transaction ouverte est un `Option<Base>` (l'état de travail) sur l'`Instance` ; ouvrir clone l'état courant ; appliquer une écriture applique **sans vérifier la structure d'ensemble** (décision 15) ; lire interroge l'état de travail ; abandonner le jette ; valider le vérifie puis le fait entrer. Réutilise l'exécution par copie du palier 3. |
| D3 | **Vérification de structure d'ensemble reportée à la validation.** | `Base::write` du palier 3 se **scinde** : `apply` (forme W4/W5, noms Q3/Q4, id W6/W7 via la construction de ligne) et `verify` (structure d'ensemble W8/W9). En transaction, appliquer n'appelle qu'`apply` ; valider appelle `verify` sur l'état final, puis l'écrivain du palier 3. |
| D4 | **Écriture isolée = transaction implicite, seulement hors transaction ouverte.** | Sans transaction ouverte, une écriture ouvre-applique-vérifie-valide en un geste (exactement le palier 3). Tant qu'une transaction est ouverte, l'écriture s'applique à l'état de travail. |
| D5 | **Surface C ABI : un seul chemin, le texte SQL.** | `memless_execute` **reconnaît** `BEGIN`/`COMMIT`/`ROLLBACK` (aucune fonction ABI neuve — décision 8, un seul texte SQL quel que soit le langage) ; `out_affected = 0` pour ces verbes ; `memless_query` consulte l'état de travail si une transaction est ouverte (lecture de ses propres écritures) ; refus T1/T2 en statut `Refused`. `ABI_VERSION = 4` (nouveaux `Statement`, comportement de `query`/`execute` étendu). |
| D6 | **Refus typés neufs.** | `TransactionRefusal { AlreadyOpen, NoOpenTransaction }`, rejoint `Refusal::Transaction` ; W8/W9/W10 réutilisés pour la validation échouée (le nom `WriteRefusal::DiskWriteFailed` porte « refusal » pour un échec — écart de nommage hérité du palier 3, consigné, non réparé ici). |
| D7 | **Banc §17.2 complété.** | Un banc de transaction (durée d'une transaction de k écritures validée) ; relevé ajouté à `.charpente/releves/banc.md`. |
| D8 | **Découpage en PR.** | domaine (état de travail + scission `apply`/`verify` + refus) → moteur (grammaire + cas d'usage `open`/`validate`/`abandon` + lecture de ses écritures) → C ABI → {PHP ∥ Go} → parité+banc. Tranché au plan. |
