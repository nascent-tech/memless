---
type: cadrage
titre: Palier 3 — Écrire
slug: palier-3-ecrire
cree_le: 2026-09-21T21:40:00+0000
mis_a_jour_le: 2026-09-21T22:10:00+0000
branche: main
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../mvp/00-index.md) › **cadrage — Palier 3 — Écrire** › [conception — Palier 3 — Écrire](../conceptions/2026-09-21-palier-3-ecrire/00-index.md) › [plan — Palier 3 — Écrire](../plans/2026-09-21-palier-3-ecrire/00-index.md)  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/mvp/00-index.md

Cadrage du **palier 3 du MVP** — « Écrire ». Il couvre la **modification** de l'état par du SQL
(`INSERT`/`UPDATE`/`DELETE`, chacun **isolément** — chaque écriture est sa propre transaction,
§10), la **réécriture par substitution** du fichier YAML qui s'ensuit (§8.6), le **refus** de casser
une relation devinée (§8.7), et l'**échec disque** sans perte mémoire. Les transactions
multi-instructions (`BEGIN`/`COMMIT`/`ROLLBACK`, §8.5) sont le palier 4. Références `§`/`décision`
au brief validé, sauf mention du MVP. La stratégie technique est tranchée par la **décision Fable**
du 2026-09-21 (voir la conception, partie 04).

## 1. Acteur et déclencheur

**Acteur.** Le **code appelant** (§6) — le test automatisé en PHP puis en Go. Il tient une
**instance démarrée** (palier 1) qu'il sait interroger (palier 2) ; ici il la **modifie**.

**Déclencheur.** Le code appelant **soumet un texte SQL d'écriture** à une instance vivante. Une
écriture acceptée mute l'état en mémoire **et réécrit aussitôt le fichier**, d'un coup (§8.6). Une
écriture refusée ne touche ni la mémoire ni le fichier.

**Ce que l'acteur observe.** Soit un **compte de lignes affectées** (et un fichier réécrit), soit
un **refus** qui nomme la règle. Rien entre les deux : une écriture n'est jamais appliquée à
moitié (§13). Le fichier réécrit est **identique à l'octet** quel que soit le langage appelant
(§8.8, décision 25) : c'est la promesse « le fichier est la vérité » (§1) et un `git diff` lisible.

## 2. Besoin, séparé de la solution

| Formulé en solution | Le besoin dessous |
|---|---|
| « Modifier l'état en SQL » | Tester une vraie règle métier (retirer de l'argent, changer un rôle) sans installer ni réinitialiser une base entre deux tests (§7). |
| « Réécrire le fichier » | Que chaque changement de donnée devienne **une ligne d'historique Git lisible** (§7) : le fichier reste la seule source de vérité (§1). |
| « À l'octet près entre langages » | Qu'une équipe qui alterne un test Go et un test PHP contre le même fichier ne voie jamais, dans l'historique Git, un changement de donnée qui n'a pas eu lieu (§8.8, décision 25). |
| « Ne jamais laisser un fichier à moitié écrit » | Qu'une coupure ou un plantage laisse **soit l'ancien fichier intact, soit le nouveau complet** — jamais un mélange (§8.6) ; et qu'un échec disque ne laisse jamais la mémoire en avance sur le fichier (§8.6). |

Ce que le palier 3 prouve en plus : que l'état est **modifiable** et que la modification est
**durable et sans état intermédiaire visible**. Ce qu'il ne prouve pas : le tout-ou-rien de
plusieurs écritures groupées (palier 4).

## 3. Vocabulaire

Termes **[neuf]** à inscrire au glossaire, en plus des paliers 1–2.

| Terme | Définition | Voisin à ne pas confondre |
|---|---|---|
| **Écriture** [neuf] | Un texte SQL de **modification** : `INSERT`, `UPDATE` ou `DELETE`, une seule instruction. Sa propre transaction (§10). | La **requête** (palier 2, lecture). |
| **Lignes affectées** [neuf] | Le compte rendu par une écriture acceptée : insérées (1), retenues par le `WHERE` d'un `UPDATE` (même sans changement de valeur, convention SQL), supprimées. | Le **jeu de lignes** (palier 2). |
| **Réécriture** [neuf] | La production d'un nouveau fichier YAML reflétant l'état après une écriture validée qui a **changé** l'état (§8.6). Une écriture sans changement ne réécrit pas. | La **lecture/chargement**. |
| **Réécriture par substitution** [neuf] | Écrire un **résidu** (fichier à part), puis le **substituer** d'un coup à l'original (point de substitution) — jamais par-dessus l'original morceau par morceau (§8.6). Le glossaire l'a fixée en remplacement de « réécriture atomique ». | Une écriture « en place ». |
| **Résidu** | Le fichier à part `.<nom>.memless-tmp`, dans le même répertoire. Au **chargement**, Memless n'y touche jamais (décision 17). En **écriture**, Memless gère le sien : il le crée, et si l'écriture échoue avant la substitution, **il le supprime lui-même** ; un résidu hérité d'un plantage antérieur est écrasé par la création du prochain résidu. Le seul résidu que Memless touche est celui de l'écriture en cours. | Le **fichier**. |
| **Forme canonique** [neuf] | La forme que produit la **réécriture de normalisation** (MVP §1 ; première réécriture au palier 3) : ordre des tables/lignes/colonnes de l'état, scalaires rendus par une règle fermée, colonne absente omise. Un fichier déjà canonique se réécrit **sans diff**. | La **forme d'origine** (commentaires, mise en forme), non conservée (décision 26). |
| **Relation cassée** | Une ligne qui, à la fin d'une écriture validée, pointe vers une ligne inexistante — quelle qu'en soit la cause (§8.7). Refusée en nommant **les deux tables** pour le cas orphelin. | Une relation **absente/`null`** (décision 35), qui ne pointe vers rien et ne casse rien. |
| **Échec disque** [neuf] | L'écriture sur le disque échoue (chemin inaccessible, espace insuffisant) : l'écriture-transaction **échoue** (§6.1), la mémoire reste **inchangée** (§8.6). | Un **refus de structure** (donnée incohérente). |

**Termes exclus** (§6.1) : « atomique », « commit » (hors Git), « transaction » au sens
multi-instructions (palier 4), « invariants » (dire **contraintes**), « snapshot »/« rollback »
(dire « état d'avant » / « abandonner » — le palier 4 n'en hérite pas).

## 4. Événements au passé sur une chronologie

| # | Événement | Origine | Observable par |
|---|---|---|---|
| 0 | **Écriture soumise** | Commande — le code appelant passe un texte SQL d'écriture à une instance vivante. | Le geste. |
| 1a | **Écriture refusée** | Politique — *dès qu'*une raison de refus (point 6) est établie, *alors* rien n'est appliqué, la mémoire et le fichier restent inchangés (§13). | Une erreur nommant la règle. |
| 1b | **Écriture appliquée sans changement** | Politique — *dès que* l'état final égale l'état de départ, *alors* aucune réécriture, le compte est rendu (§8.6). | Le compte ; le fichier inchangé. |
| 1c | **Écriture validée et réécrite** | Politique — *dès que* l'état final est cohérent **et** différent, *alors* le fichier est réécrit par substitution, **et le point de substitution acquis, la mémoire est avancée**. Seules les valeurs touchées bougent, à l'octet (après normalisation). | Le compte ; le fichier changé d'un coup. |
| 1d | **Échec disque avant la substitution** | Système de fichiers — *dès que* la réécriture échoue **avant** le point de substitution, *alors* l'écriture-transaction échoue, la mémoire reste inchangée, **Memless supprime le résidu qu'il vient de créer**. | Un échec d'écriture disque (statut `Refused`). |

**Le point de substitution est le point d'acquisition.** Un échec **après** lui (par exemple le
fsync du répertoire, durabilité de l'entrée sous coupure) **n'est pas un refus** : le fichier est
déjà remplacé, la mémoire avance, seule la durabilité en cas de coupure immédiate est moindre
(limite consignée). La mémoire n'est donc jamais en retard sur le fichier.

**Frontières.** 1a–1d sont exclusifs. La mémoire n'est **jamais** avancée avant le point de
substitution (§8.6). Aucun réseau, aucune échéance (MVP §5). Le seul système externe est le système
de fichiers, écrit par l'adaptateur de substitution.

**Concurrence.** Sur une instance, les écritures sont **sérialisées** par le pont (sûreté mémoire,
décision 7) ; le verrou est tenu **jusqu'à l'avance mémoire**, si bien qu'une lecture concurrente
voit l'état d'avant **ou** d'après une écriture, jamais entre (conséquence de la copie, D2), et deux
écritures ne se perdent pas. L'ordre entre deux appels concurrents n'est pas garanti. Deux
instances ou processus sur le **même fichier** : dernière écriture gagne (§14). La **portée** du
verrou (global, comme `instances.rs` aujourd'hui, ou par instance) est un choix de conception.

## 5. Contexte(s) et sorte

**Contexte borné : le moteur** (MVP §6). Le palier 3 exerce §8.4, §8.6, §8.7. **Sorte : `cœur de
métier`** — l'écriture porte les règles produit : la structure redérivée à chaque vérification
(décision 37), la relation cassée nommant deux tables (§8.7), la réécriture par substitution et son
ordre (§8.6), la forme canonique octet-près (§8.8). L'analyse déléguée à `sqlparser` et l'`io` de
fichier sont génériques ; le reste est le cœur.

## 6. Refus — tous les états impossibles, avant le nominal

Ordre d'établissement : **syntaxe → sous-ensemble → noms → structure (contraintes) → disque**. La
partie « structure » réutilise l'ordre du palier 1 (D1 : id présent, id typé, id unique, relations) ;
W4/W5 sont des fautes de **forme de l'instruction** et se prononcent **avant** les noms. Un refus
n'applique rien, ne réécrit rien, la mémoire et le fichier restent intacts (§13). La plupart des
refus **réutilisent** les types et messages des paliers 1–2 : une écriture est refusée *parce que le
fichier qui en sortirait ne se chargerait pas*.

| # | État impossible | Type / message |
|---|---|---|
| W1 | Le texte ne s'analyse pas comme du SQL. | `InvalidSql` (Q1) — inchangé. |
| W2 | Construction hors sous-ensemble d'écriture : `INSERT` sans liste de colonnes, `VALUES` multi-lignes, expression/arithmétique en valeur, `INSERT … SELECT`, `RETURNING`, `ON CONFLICT`, `UPDATE/DELETE` avec jointure/alias/`ORDER BY`/`LIMIT`, DDL, `BEGIN`/`COMMIT`, plusieurs instructions ; un `SELECT` passé à l'écriture ; une écriture passée à la lecture. | `OutsideSubset` (Q2) — libellé fermé. |
| W3 | `UPDATE`/`DELETE` sur une **table absente** ; colonne absente dans un `WHERE`. Une table **vidée** (`t: []`) n'a **aucune colonne, pas même `id`** (palier 2 Q4) : `UPDATE`/`DELETE … WHERE` sur elle est refusé `UnknownColumn`, pas « 0 ligne », jusqu'au prochain `INSERT`. | `UnknownTable`/`UnknownColumn` (Q3/Q4) — inchangés. Une faute de frappe sur un nom de table ne passe jamais pour un changement sans effet (§8.4). |
| W4 | Liste de colonnes et liste de valeurs de **longueurs différentes** dans un `INSERT`. | `WriteRefusal::ColumnCountMismatch` **[neuf]**. |
| W5 | Même colonne **deux fois** dans la liste d'`INSERT` ou dans un `SET`. | `WriteRefusal::ColumnRepeated` **[neuf]**. |
| W6 | Ligne sans `id` (`INSERT` sans `id`, `INSERT (id) VALUES (NULL)`, `SET id = NULL`). | `MissingId` (B) — inchangé. |
| W7 | `id` décimal ou vrai/faux (`UPDATE` d'un `id` vers un décimal). | `IdNotTextOrInteger` (B) — inchangé. |
| W8 | `id` en doublon après l'écriture (règle de comparaison unique : `5` et `"5"` restent deux `id`). | `DuplicateId` (B) — inchangé. |
| W9 | **Relation cassée**, quelle qu'en soit la cause : `DELETE`/`UPDATE` de la ligne visée, `UPDATE` de la relation vers une valeur absente, `INSERT` orphelin, `INSERT` qui fait apparaître la table transformant une colonne ordinaire en relation orpheline (nomme **les deux tables**). | `BrokenRelation` (B) — inchangé. |
| W10 | **Échec disque** : résidu impossible à créer, espace insuffisant, substitution impossible. | `WriteRefusal::DiskWriteFailed { path, kind }` **[neuf]** — vocabulaire fermé, sans texte d'OS localisé ; rendu en statut `Refused` côté C ABI. |

**Sort au palier 4 (pour ne pas le retrofitter).** Une écriture isolée est sa propre transaction :
W1–W5 sont des fautes de l'**instruction** (la laisseraient ouverte) ; W6–W9 (structure, vérifiée à
la validation, décision 15) et W10 (disque) **font échouer** l'écriture-transaction. Pour une
écriture isolée les effets coïncident (§8.4) ; le marquage prépare l'héritage des types.

**Ce qui n'est PAS un refus** :
- Une écriture qui **ne change rien** (état final = état de départ) : acceptée, compte rendu, **pas** de réécriture (§8.6). La « première réécriture de normalisation » (MVP §1) exige donc une écriture qui **change réellement** l'état.
- `UPDATE` **sans `WHERE`** : toutes les lignes de la table sont retenues (`affected` = leur nombre).
- `SET col = NULL` : la colonne devient **absente** pour la ligne (décision 35) ; si c'était sa dernière valeur, la colonne disparaît de la structure devinée (§8.4).
- `SET` sur une colonne **absente** : elle est **ajoutée** à la ligne (§8.4, comme l'`INSERT` d'une colonne neuve).
- `DELETE FROM t` sans `WHERE` : la table est **vidée**, sa clé reste (`t: []`) — pas supprimée (décision 37).
- `INSERT` d'une colonne qu'aucune autre ligne ne porte : l'ajoute pour cette ligne (§8.4).
- Littéraux admis en `VALUES`/`SET` : texte, entier, décimal, vrai/faux, nombre signé, `NULL` (= absence, décision 35).
- Deux lignes qui se référencent l'une l'autre, supprimées **dans la même** instruction ? — non : une écriture isolée est une seule instruction ; ce cas est le palier 4.

**Placement de ce qui apparaît (§8.6, décision 27).** Une **table** créée par `INSERT` s'écrit **en
fin** de fichier ; une **ligne** insérée, **en fin** de table ; ses **colonnes**, dans l'ordre de la
liste de l'`INSERT` ; une **colonne** neuve (par `INSERT` ou `SET`), **en fin** de ligne ; une
colonne remise après un `SET col = NULL` antérieur revient **en fin** de ligne (sans trace de son
ancienne place, décision 37).

**Parité sur le refus et sur le fichier.** Même message depuis PHP et Go (décision 25) ; **et** le
fichier réécrit est identique à l'octet (décision 25). Un écart est un défaut consigné au relevé.
Seul le message W10 porte un `path` propre au répertoire temporaire de chaque pilote : la parité s'y
vérifie **après normalisation** du préfixe de répertoire, le `kind` étant une **liste fermée** testée
(règle de comparaison fixée en conception §04).

## 7. Limites / seuils sourcés

**Aucun seuil chiffré ne conditionne un refus d'écriture.** Le seul refus « mesuré » est l'échec
disque (W10), une propriété du système, pas une limite de produit. Le **banc de durée (§17.2)**
entre ici : le palier 3 tient le **premier relevé** — durée d'un chargement, d'une écriture isolée
(réécriture comprise) et d'une suite d'écritures, en faisant varier la taille du fichier ; aucun
nombre n'est fixé d'avance, c'est la mesure qui informe le pari du §4 du brief (la réécriture
complète reste-t-elle imperceptible ?).

## 8. Hors périmètre explicite

| Hors du palier 3 | Où |
|---|---|
| Transactions multi-instructions : `BEGIN`/`COMMIT`/`ROLLBACK`, lecture de ses propres écritures, état d'avant/abandon, deuxième ouverture, valider sans transaction (§8.5) | Palier 4 |
| `VALUES` multi-lignes, `INSERT … SELECT`, expression/arithmétique en valeur, `INSERT` sans colonnes | Hors sous-ensemble (W2) ; rentrent sur **1** demande au journal des retours |
| Conservation des commentaires et de la mise en forme du fichier d'origine (décision 26) | Jamais garanti ; conséquences connues à la première réécriture (forme canonique) |
| Cascade / mise à `NULL` automatique sur relation cassée (§8.7) | Hors lancement (§14) |
| Suppression d'une table par SQL ; `CREATE`/`ALTER` (§8.4) | N'existe pas (décision 24) |
| Le rechargement à la demande (§8.9) | Hors MVP |
| Le pont JavaScript/TypeScript | Hors MVP (MVP §4) |

## 9. Décisions restantes et qui tranche

Toutes tranchées par la **décision Fable** (voir la conception, partie 04) selon le principe le
plus prudent aligné sur le brief. Les points saillants :

| # | Décision | Tranché par défaut |
|---|---|---|
| D1 | **Grammaire d'écriture.** | `INSERT INTO t (cols) VALUES (…)` une ligne, colonnes obligatoires, `id` dedans ; `UPDATE t SET … [WHERE]` ; `DELETE FROM t [WHERE]` ; le `WHERE` réutilise le `Filter` du palier 2. Tout le reste → W2. |
| D2 | **Exécution par copie, pas de mutation en place.** | `Base::write` clone les tables, applique, **vérifie** (contraintes du chargement redérivées), rend la copie ; la mémoire n'est avancée qu'après le point de substitution. Garantit « la mémoire ne garde jamais ce que le disque n'a pas reçu » sans journal d'annulation ; sert de base au palier 4. |
| D3 | **Réécriture par substitution maison.** | `Base::document` (domaine) → `yaml/render` **écrit à la main** (canonique, déterministe à l'octet, verrouillé par un test aller-retour couvrant **scalaires et clés** — un nom introduit par SQL comme `"true"`, `"5"`, `"a: b"` doit se relire comme texte, §8.3) — **pas** le sérialiseur serde-saphyr ; résidu `.<nom>.memless-tmp` même répertoire, écrire → fsync fichier → substituer (point de substitution) → fsync répertoire ; port `ReplaceFile` fonction. |
| D4 | **Forme canonique et ses conséquences.** | Colonne absente omise, `NULL` non écrit, ordre porté par `Base` ; ce qui apparaît va **en fin** (voir point 6) ; un fichier déjà canonique se réécrit sans diff. Première réécriture d'un fichier commenté / `x: null` / CRLF / décimal non représentable : conséquences connues (décision 26), non des refus. |
| D5 | **Surface C ABI.** | `memless_execute(handle, sql, out_affected, out_message)`, `ABI_VERSION = 3`, `memless_query` inchangé ; échec disque en statut `Refused`. |
| D6 | **Échec disque = statut `Refused`, pas un cinquième statut ABI.** | Le brief range l'échec disque parmi les issues d'une validation ; les ponts n'ont qu'une bascule refus/faute. (Axe « statuts ABI » — distinct de l'axe « sortes de refus » de D8.) |
| D7 | **Banc §17.2 : premier relevé au palier 3.** | `examples/bench_write.rs` + `harness/bench/`, percentiles, relevé tenu à la main dans `.charpente/releves/banc.md`. |
| D8 | **Désyncs `ARCHITECTURE.md`** (Instance = chemin + Base ; point de substitution ; ABI 3 ; nouvelle **sorte de refus** `WriteRefusal` ; GlueSQL déjà sorti). | Chantier d'architecture distinct. |
