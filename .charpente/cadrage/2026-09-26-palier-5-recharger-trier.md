---
type: cadrage
titre: Palier 5 — Recharger, trier et tenir trois ponts
slug: palier-5-recharger-trier
cree_le: 2026-09-26T08:45:22+0000
mis_a_jour_le: 2026-09-26T09:04:58+0000
branche: docs/deliver-all
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : [brief — Moteur de base de données en mémoire pilotée par YAML](../brief/00-index.md) › [mvp — Prouver la promesse de Memless](../mvp/00-index.md) › **cadrage — Palier 5 — Recharger, trier et tenir trois ponts**  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Depuis : .charpente/mvp/00-index.md

Cadrage du **palier 5 du MVP** — « Recharger, trier et tenir trois ponts » (MVP §6, ligne palier 5).
Il couvre le **rechargement** de l'état depuis le fichier à la demande (§8.9, décision 14), l'entrée du
**tri** (`ORDER BY`) dans le sous-ensemble SQL (§8.3, décision 34), et la **parité à trois ponts** —
Node rejoint PHP et Go (§8.8, §5.1) — portée par la CI et la distribution. Références `§`/`décision` au
brief validé, sauf mention du MVP. Les décisions techniques ont été revues et validées par le chef de
projet le 2026-09-26 ; elles sont reprises telles quelles dans `ARCHITECTURE.md` et ses parties
(notamment §12.3 questions 3, 5, 6, 7) et dans le brief et le MVP déjà mis à jour dans ce même palier —
en particulier la décision du 2026-09-22 sur le chemin de Node (`.charpente/architecture/roadmap.md`
§12.3 question 3).

## 1. Acteur et déclencheur

**Acteur.** Le **code appelant** (§6 du brief) — le test automatisé, désormais en PHP, Go **et
Node** ; et le **fondateur**, qui rejoue le banc de parité par la CI plutôt qu'à la main.

**Déclencheur.** Trois gestes distincts, réunis dans ce seul palier parce qu'ils clôturent ensemble le
MVP : **(a)** le code appelant demande un `reload` sur une instance vivante, après avoir restauré le
fichier par un moyen externe (Git) ; **(b)** il écrit une requête `SELECT … ORDER BY …` ; **(c)** la CI
rejoue le banc de parité sur les trois ponts et les quatre familles de systèmes à chaque push, PR et
tag ; le banc de mesure reste joué à la main (§7).

**Ce que l'acteur observe.** Un rechargement réussi rend l'état du fichier restauré, sans recréer
d'instance. Un rechargement refusé (transaction ouverte) ou raté (fichier de nouveau incohérent, ou
disparu) laisse l'état précédent intact ; l'instance répond encore, comme avant la tentative. Une
requête triée rend les mêmes lignes, dans le même ordre, dans les trois langages ; un tri sur une
colonne à types mêlés est refusé, nommant les deux lignes en cause. Un tag poussé produit une ligne
verte (ou rouge) dans le relevé de parité, sans que personne n'ait rejoué la batterie à la main.

## 2. Besoin, séparé de la solution

| Formulé en solution | Le besoin dessous |
|---|---|
| « Ajouter un verbe `reload` » | Qu'une suite de tests qui restaure son fichier entre deux scénarios puisse aligner la mémoire d'une instance déjà vivante sur ce fichier restauré, sans payer le coût de recréer un moteur (§8.9). |
| « Supporter `ORDER BY` » | Que la promesse du brief (§8.3 : « filtrer, trier, croiser… agréger ») soit tenue par le code, pas seulement par la prose — le MVP avait réduit le sous-ensemble avant que la promesse ne le rattrape. |
| « Ajouter un pilote Node » | Qu'une équipe qui travaille aussi en JavaScript/TypeScript n'ait pas à réécrire le mécanisme une troisième fois (§7 du brief) — et que l'hypothèse §17.1 (parité) soit éprouvée sur les trois langages promis dès la demande de départ, pas sur deux. |
| « Automatiser la CI » | Que le relevé de parité et le relevé du banc cessent de dépendre d'une main humaine qui rejoue la batterie sur chaque machine — condition de livraison du MVP §7 (b). |

Ce que le palier 5 prouve en plus : que les dix fonctionnalités du brief (et non neuf) tiennent
ensemble, sur trois ponts (et non deux), vérifiées automatiquement (et non à la main). Ce qu'il ne
prouve pas : les séances des trois développeuses (§7 bis du MVP, étape d'apprentissage postérieure à la
livraison).

## 3. Vocabulaire

Termes **[neuf]** à inscrire au glossaire, en plus des paliers 1–4.

| Terme | Définition | Voisin à ne pas confondre |
|---|---|---|
| **Rechargement** [neuf] | Relire le fichier d'origine et remplacer l'état d'une instance vivante, en entier ou pas du tout ; refusé pendant une transaction ouverte ; un échec laisse l'état intact (§8.9, décision 14). | Le **chargement** (palier 1), qui crée une instance ; le rechargement en réutilise une. |
| **Tri** [neuf] | Ordonner les lignes d'un résultat selon une ou plusieurs colonnes, croissant par défaut ou décroissant si demandé, stable entre égaux, colonne absente placée en dernier ; refusé si deux lignes du résultat portent, pour la colonne triée, des types différents (§8.3, décision 34). | Le **filtre** (palier 2), qui retient ou écarte des lignes sans les réordonner. |
| **Pilote Node** [neuf] | Le troisième pilote du banc de parité (`harness/parity/node/main.js`), aux mêmes conventions de sortie que les pilotes PHP et Go (`refused:`/`refused: `). | Le **pont Node** (`bindings/node/`), la bibliothèque que ce pilote consomme. |
| **Entier exact** [neuf] | Une valeur entière qui garde sa valeur au bit près dans les trois langages, y compris au-delà de `Number.MAX_SAFE_INTEGER` côté Node — rendue en `BigInt`, jamais arrondie en `number` (§8.8, décision Node du §6). | Le **décimal**, jamais garanti exact dans aucun des trois langages (décision 20 du brief). |
| **Famille de systèmes** [neuf, formalisé] | Un couple OS × architecture de processeur retenu pour la publication et la CI : `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (`ARCHITECTURE.md` §12.3 Q7). | La **machine** unique des paliers 1 à 3, où une seule famille était en jeu. |

**Termes exclus** (§6.1) : « ordre » seul pour désigner le tri sans dire ce qui est trié — dire **tri
sur** une colonne ; « pagination » — la fonctionnalité (`LIMIT`/`OFFSET`) reste hors sous-ensemble, ne
pas laisser croire qu'elle entre avec le tri.

## 4. Événements au passé sur une chronologie

| # | Événement | Origine | Observable par |
|---|---|---|---|
| 0 | **Rechargement demandé** | Commande — appel natif `reload()` sur une instance sans transaction ouverte. | Le geste ; l'état de l'instance peut changer. |
| 0b | **Rechargement refusé, transaction ouverte** | Politique — *dès qu'*une transaction est ouverte sur l'instance, *alors* le rechargement est refusé, l'état et la transaction restent inchangés. | Le message `cannot reload while a transaction is open` ; l'instance répond encore, transaction comprise. |
| 1a | **Rechargement réussi** | Politique — *dès que* le fichier relu passe les vérifications du §8.1/§8.2 (comme au premier chargement), *alors* `base` est remplacée d'un coup ; aucune écriture disque ; le résidu voisin n'est pas touché. | Une lecture suivante rend les valeurs du fichier restauré. |
| 1b | **Rechargement refusé, fichier de nouveau invalide** (dit familièrement « raté » : la tentative a eu lieu, contrairement à 0b où elle est bloquée avant) | Politique/Système — *dès que* le fichier relu échoue une vérification du §8.1/§8.2 (introuvable, illisible, vide, YAML invalide, sans table, incohérent), *alors* le rechargement est refusé, **le même message que `memless_load`** rendrait sur ce fichier ; `base` reste l'ancienne — celle d'avant la tentative, pas celle du fichier fautif. | Le message de refus ; une lecture suivante rend encore les anciennes valeurs. Une **écriture** suivante, elle, réécrit le fichier depuis cette même ancienne mémoire (§8.6 s'applique à l'identique après un rechargement refusé) : si le fichier avait été mal restauré par erreur, cette écriture efface la restauration sans avertissement — même piège que §8.6 décrit déjà pour une transaction validée après une restauration externe. |
| 2a | **Requête triée exécutée** | Commande — `SELECT … ORDER BY col [ASC\|DESC] [, …]` sur des colonnes de la projection ou des tables lues. | Les lignes rendues, dans l'ordre demandé, stable entre égaux, colonne absente en dernier. |
| 2b | **Tri refusé, types mêlés** | Politique — *dès que* deux lignes du résultat portent, pour une colonne triée, des types différents, *alors* la requête entière est refusée, nommant la colonne, la table, et les deux premières lignes en cause (par leur `id`, jamais par leur position). | Le message `cannot order by "<col>" of "<table>": row <id1> and row <id2> differ in type`. |
| 2c | **Tri refusé, hors sous-ensemble** | Politique — *dès qu'*un `ORDER BY` porte sur un agrégat, une position, une expression, ou porte `NULLS FIRST`/`NULLS LAST`, ou qu'une colonne de tri n'est pas qualifiée dans une jointure, *alors* la requête est refusée « outside the supported SQL subset ». | Le message nommant le cas précis (voir §6). |
| 3 | **Pilote Node exécuté** | Commande — le banc de parité lance `harness/parity/node/main.js` sur une fixture, une requête ou une transaction. | Le même bloc de sortie que PHP/Go ; comparé octet à octet. |
| 4 | **CI verte sur un tag** | Système — `ci.yml` compile, teste, rejoue le banc de parité et les tests de non-régression sur les quatre familles. | Le run vert de `ci.yml` sur chaque famille est la preuve ; sa ligne (date, famille, révision, lien du run, verdict) est recopiée dans `.charpente/releves/parite.md` par le fondateur au moment du tag — pas par un step de CI. |

**Concurrence.** Comme au palier 4 : le rechargement, comme la transaction, est un état de
l'**instance** — deux fils qui la partagent partagent son verrou d'accès. Un rechargement tient le
même verrou process-wide que `memless_query`/`memless_execute` pour sa durée.

## 5. Contexte(s) et sorte

**Contexte borné : le moteur** (MVP §6). Le palier 5 exerce §8.8, §8.9. **Sorte : `cœur de métier`** —
le rechargement porte une règle produit (refus pendant une transaction, jamais d'état vide sur échec) ;
le tri porte la règle de comparaison unique (§15 décision 34) jusque dans l'ordre des lignes. Le pilote
Node et la CI sont **support** : ils vérifient le cœur, ils n'en portent aucune règle.

## 6. Refus — tous les états impossibles, avant le nominal

| # | État impossible | Type / message |
|---|---|---|
| R1 | `reload` alors qu'une transaction est ouverte. | `TransactionRefusal::OpenDuringReload` **[neuf]** — `cannot reload while a transaction is open`. |
| R2 | `reload` refusé — sur un fichier devenu introuvable, illisible, vide, invalide, sans table, ou incohérent (événement 1b). | Le **même** refus, le **même** message que `memless_load` rendrait sur ce fichier — aucun texte neuf. |
| O1 | `ORDER BY` sur une colonne dont deux lignes du résultat portent des types différents. | `QueryRefusal::OrderMixedTypes { table, column, first, second }` — `cannot order by "<col>" of "<table>": row <id1> and row <id2> differ in type` ; les deux lignes citées sont, dans l'ordre du fichier de la table portant la colonne, la première qui porte une valeur et la première dont le type diffère — les lignes déjà filtrées ne comptent pas ; un `id` texte s'affiche entre guillemets (`row "5"`), un `id` entier sans (`row 5`). |
| O2 | `ORDER BY` sur le résultat d'un agrégat (par exemple `ORDER BY COUNT(*)`) — distinct du refus du palier 2 sur une projection qui mêle une colonne ordinaire et un agrégat sans le `GROUP BY` que le sous-ensemble ne supporte pas : ce dernier cas est déjà refusé, avant même d'atteindre le tri, par le refus existant du palier 2 (colonne mêlée à un agrégat) ; O2 ne s'applique qu'à un `ORDER BY` qui porte lui-même sur un agrégat, dans une requête par ailleurs valide. | `OutsideSubset` — `ORDER BY with an aggregate is outside the supported SQL subset`. |
| O3 | `ORDER BY` par position (`ORDER BY 1`). | `OutsideSubset` — `ORDER BY position is outside the supported SQL subset`. |
| O4 | `ORDER BY` sur une expression (pas une simple colonne). | `OutsideSubset` — `ORDER BY expression is outside the supported SQL subset`. |
| O5 | `ORDER BY … NULLS FIRST`. | `OutsideSubset` — `NULLS FIRST is outside the supported SQL subset`. |
| O6 | `ORDER BY … NULLS LAST`. | `OutsideSubset` — `NULLS LAST is outside the supported SQL subset`. |
| O7 | Une colonne de tri non qualifiée dans une requête jointe. | `OutsideSubset` — `unqualified column in a join is outside the supported SQL subset`. |
| N1 | Un entier au-delà de `Number.MAX_SAFE_INTEGER` rendu par erreur en `number` JavaScript plutôt qu'en `BigInt`. | Défaut de parité, pas un refus produit — couvert par un test dédié (`9007199254740993`) et par le rendu canonique du pilote de parité (la valeur imprimée en texte, identique aux deux autres ponts). |

**Ce qui n'est PAS un refus** :
- Un `ORDER BY` sur une colonne absente d'une ligne : cette ligne se place en dernier, elle n'est pas
  refusée (§8.3).
- Un `ORDER BY` sans agrégat, position, expression, ni `NULLS FIRST`/`LAST`, sur une colonne qualifiée
  quand la requête joint : nominal.
- Un rechargement réussi qui ne change rien à l'état (fichier restauré identique à l'état déjà en
  mémoire) : nominal, `base` est simplement remplacée par une valeur égale.

**Parité.** Même issue et même message depuis PHP, Go et Node pour chaque geste du tri et du
rechargement, et même empreinte `sha256` du fichier après un rechargement réussi qui ne réécrit rien
(aucune réécriture n'a lieu : un rechargement ne touche jamais le disque).

## 7. Limites / seuils sourcés

**Aucun seuil chiffré ne conditionne un refus.** Le banc §17.2 se complète ici : durée d'un
rechargement, en faisant varier la taille du fichier, en plus de la transaction, de l'écriture isolée et
du chargement des paliers précédents — un rechargement paie le même coût de lecture complète qu'un
chargement (§8.9 : « exactement comme au premier chargement »), aucun nombre n'est fixé d'avance ; comme
aux paliers 3 et 4, ce banc de rechargement est **joué à la main** par le fondateur (MVP §4 : « le banc
§17.2 est joué à la main, une fois à chacun des paliers »), pas par la CI — il ne figure pas dans
`ci.yml`, dont la liste au paragraphe suivant est exhaustive.

**Distribution et CI, décidées, pas mesurées à seuil.** Quatre familles de systèmes retenues
(`aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`,
`aarch64-unknown-linux-gnu`) ; CI **GitHub Actions**, un `ci.yml` sur push/PR (matrice des quatre
familles : `cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, `cargo test`,
`cargo build --release -p memless-capi`, `go test ./...`, `composer install && vendor/bin/phpunit`,
`npm ci && npm test`, puis `bash harness/parity/run.sh` et les `tests/detects-*.sh`) et un
`release.yml` sur tag `v*` qui reproduit ces builds et publie sur **GitHub Releases** : quatre archives
`memless-capi-<version>-<cible>.tar.gz` (la cdylib + `memless.h`), un fichier `SHA256SUMS` couvrant
tous les assets, plus `nascent-tech-memless-<version>.tgz` (`npm pack`), `memless-php-<version>.zip`
(repository Composer `artifact`), et le tag `bindings/go/vX.Y.Z`, posé à la main sur le même commit que
`vX.Y.Z` (aucun workflow ne le crée). Aucun registre externe (pas de publication npm/Packagist à
proprement parler, pas de module Go publié sur un registre) — licence **MIT** sur l'ensemble du dépôt.

**Politique de publication d'un tag — tout-ou-rien, comme le produit lui-même.** `release.yml` ne
publie **aucune** archive si un seul job de la matrice (build, tests, banc de parité) échoue sur une
seule des quatre familles : une publication n'est jamais partielle, exactement comme une transaction
n'est jamais à moitié validée (§8.5 du brief). Le run vert de `ci.yml` sur chaque famille est la
preuve ; sa ligne (date, famille, révision, lien du run, verdict) est recopiée dans
`.charpente/releves/parite.md` par le fondateur au moment du tag — pas par un step de CI. Rejouer le
même tag après correction exige de supprimer d'abord la release et le tag, puis de les recréer :
`gh release create` refuse une release déjà existante — il n'y a donc jamais deux jeux d'assets côte à
côte.

## 8. Hors périmètre explicite

| Hors du palier 5 | Où |
|---|---|
| `LIMIT`, `OFFSET`, `GROUP BY`, `DISTINCT` | Restent hors sous-ensemble (§8.3 du brief) ; jamais promis, le tri seul entre |
| Windows (`.dll`) et Linux musl | Exclus au lancement (`ARCHITECTURE.md` §12.3 Q7) ; les ponts ne cherchent que `.dylib`/`.so` |
| Publication sur des registres externes (npm, Packagist, pkg.go.dev) | Distribution par GitHub Releases seule (§7 ci-dessus) ; `MEMLESS_LIB`/`target/` reste la résolution du binaire dans les trois ponts |
| Les séances des trois développeuses (MVP §7 bis) | Étape d'apprentissage **postérieure** à la livraison du palier 5, ne conditionne pas « fini » (MVP §7) |
| Transactions imbriquées, points de sauvegarde | N'existent toujours pas (décision 29 du brief, inchangé depuis le palier 4) |
| Verrouillage inter-processus, cascade/`NULL` automatique | Hors lancement (§12.2, §14 du brief), inchangés |

## 9. Décisions restantes et qui tranche

Toutes tranchées par l'arbitrage Fable du 2026-09-26 et par la décision humaine du 2026-09-22 (chemin
de Node), selon le principe le plus prudent aligné sur le brief. Les points saillants :

| # | Décision | Tranché par défaut |
|---|---|---|
| D1 | **Rechargement — verbe natif, pas SQL.** | `memless_reload(handle, out_message)`, ABI 5 ; `sqlparser` ne porte aucun verbe « RELOAD » ; un `SELECT`/`execute` contenant ce mot est refusé comme SQL invalide, comme aujourd'hui. |
| D2 | **Tri — entre dans le sous-ensemble, règle de comparaison unique appliquée à l'ordre.** | `ORDER BY col [ASC\|DESC] [, …]`, tri stable, colonne absente en dernier ; refus `OrderMixedTypes` si deux types se rencontrent dans la colonne triée, message nommant les deux lignes par leur `id`. |
| D3 | **Node — troisième pont, par koffi (FFI dynamique), pas par addon compilé.** | Décision humaine du 2026-09-22 ; un seul binaire natif (`memless-capi`) pour les trois ponts. |
| D4 | **Node — entiers exacts au-delà de `Number.MAX_SAFE_INTEGER`.** | Rendu en `BigInt` plutôt qu'en `number` arrondi ; le pilote de parité compare un rendu canonique textuel, identique aux deux autres ponts. |
| D5 | **CI et distribution.** | GitHub Actions (`ci.yml`/`release.yml`), quatre familles de systèmes, GitHub Releases, licence MIT. |
| D6 | **Découpage en PR.** | domaine (règle du tri, refus `OrderMixedTypes`, `reload` du domaine) → moteur (grammaire, cas d'usage `reload`) → C ABI (ABI 5) → {PHP ∥ Go ∥ Node} → pilote Node + parité généralisée à trois pilotes → CI. Tranché au plan. |
