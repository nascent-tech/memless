# 01 — Phases, critères et tests par PR

<!-- charpente-navigation -->
**Index** : [plan — Palier 2 — Interroger](00-index.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Chaque phase laisse les quatre portes vertes et tient dans une session. Les tests sont écrits
**avant** le code de leur phase (un refus par ligne).

---

## PR1 — Domaine (`memless-domain`)


### Phase 1a — modèle, résultat, refus

Fichiers (conception §01) : `query/{mod,select,items,column_ref,aggregate,join,filter,compare,op}.rs`, `rows.rs`, `refusal/query/{mod,syntax,name,execution}.rs`, la variante `Refusal::Query` + `From` + branche `Display`, `scalar/scalar_order.rs`. `lib.rs` réexporte `Rows`, `QueryRefusal`, `query::*`. Pas de `order`/`direction`/`sort` : le tri est hors sous-ensemble (cadrage D1).

Tests avant code — `tests/query_refusal.rs` : un test par gabarit (Q1, Q2, Q3, Q4, Q5, Q7, Q8 — pas de Q6, le tri est coupé). `tests/scalar_order.rs` : ordre au sein d'un type (Integer, Decimal, Text octet à octet), `None` entre deux types différents, égalité inchangée.

Critères : les sept messages sortent au mot près (conception §01) ; `scalar_order` n'ordonne jamais deux types ; portes vertes.

### Phase 1b — exécution

Fichiers : `base/select/{mod,resolve,columns,combine,keep,project}.rs` ; `Base.tables` perd `#[allow(dead_code)]`. Pas de `sort.rs` (tri hors sous-ensemble).

Tests avant code — `tests/select.rs`, un `Select` construit à la main contre une `Base` chargée depuis les fixtures existantes :
1. filtre `=` types égaux → ligne retenue ; types différents (`id = '5'` sur `id: 5`) → exclue, sans erreur ; colonne absente → exclue ; `IS NULL`/`IS NOT NULL` ; `<`/`>` au sein d'un type.
2. jointure `wallets→users` : chaque `wallets` rejoint son `users` ; ordre = ordre de `wallets` (ordre du fichier).
3. fichier piège : la jointure de `wallets.user_id: 5` rejoint `users` `id: 5` et **jamais** `id: "5"` ; une relation absente/`null` sort de la jointure.
4. `COUNT(*)` (lignes retenues), `COUNT(col)` (ignore absentes), `SUM(col)` entier et décimal ; `SUM` sur vide → cellule absente ; `COUNT` sur vide → 0.
5. refus nommés : Q3 (table absente), Q4 (colonne absente, table vide), Q5 (jointure hors `relation = id`), Q7 (`SUM` d'un texte, nomme la première ligne par `id`), Q8 (débordement 64 bits).

Critères : chaque test vert ; aucune conversion de type ; ordre du fichier respecté ; portes vertes ; garde Charpente verte (≤10 instructions, un type/fichier).

---

## PR2 — Moteur (`memless-engine`)


### Phase 2a — analyse

Fichiers (conception §02) : `Cargo.toml` gagne `sqlparser = "=0.54.0"` ; `sql/{mod,analyse}.rs`, `sql/lower/{mod,construct,projection,from,join,filter,column_ref,literal}.rs`. Pas de `lower/order.rs`.

Tests avant code — `tests/sql.rs` : un `SELECT` valide → le `Select` attendu (projection, from, join, filter) ; chaque construction exclue (cadrage Q2 : `ORDER BY`, `GROUP BY`, `HAVING`, `LIMIT`, `DISTINCT`, `LEFT JOIN`, deux jointures, self-join, `SELECT` sans `FROM`, alias, `IN`, `LIKE`, `NOT`, arithmétique, `NULL` littéral, `AVG`, sous-requête, `UNION`, `CASE`, `CAST`, colonne mêlée à agrégat, colonne non qualifiée en jointure, `INSERT`/`UPDATE`/`DELETE`/`CREATE`) → Q2 avec son libellé fixe ; un SQL cassé (backtick, tronqué), texte vide, entier hors capacité → Q1 ; deux instructions → Q2 ; nom entre guillemets doubles → nom tel quel, sensible à la casse.

Critères : chaque libellé `construct` vient de la table, jamais du texte utilisateur ; portes vertes.

### Phase 2b — cas d'usage et batterie

Fichiers : `application/query.rs`, `application/mod.rs`, réexports de `lib.rs` ; `fixtures/trap.yaml` ajoutée à la batterie (via `tests/common`).

Tests avant code — `tests/query.rs` : `application::query(parse, &base, text)` bout en bout sur `start.yaml` et `trap.yaml` — un cas nominal par famille (filtre, jointure, `COUNT`, `SUM`) et un refus par famille (Q1, Q2, Q3, Q4, Q5, Q7, Q8) ; golden des messages.

Critères : le `?` remonte chaque `QueryRefusal` en `Refusal::Query` ; portes vertes.

---

## PR3 — C ABI (`memless-capi`)


Fichiers (conception §02) : `ABI_VERSION=2` ; `lib.rs` gagne `memless_query`, `memless_result_column_count`/`_row_count`/`_column`/`_cell`/`_release` ; `query_compute.rs`, `results.rs`, `cell.rs`, `result_write.rs` ; `include/memless.h` étendu (`MemlessKind`, `MemlessResult`, les six entrées).

Tests avant code — `tests/contract.rs` étendu : `abi_version == 2` ; `memless_query` sur une instance chargée → lecture `column_count`/`row_count`/`column`/`cell` conformes ; un refus → statut `Refused` + message ; un handle inconnu → compte nul / `kind` Absent / chaîne vide, jamais un plantage ; `memless_result_release` libère ; libérer deux fois est sans effet ; `tests/internals.rs` : la table de résultats.

Critères : chaque entrée sous `guard` ; aucun pointeur ne survit à `release` ; portes vertes.

---

## PR4 — Pont PHP (`bindings/php`)


Fichiers (conception §03) : `src/Query.php`, `src/Instance.php` (gagne `query`), `src/Library.php` (`ABI_VERSION=2`).

Tests avant code — `tests/InstanceTest.php` : `query` nominal (lignes = tableaux associatifs, `null` pour absent, `int`/`float`/`string`/`bool` selon le `kind`) ; un refus lève `MemlessRefusal` avec le message exact ; le fichier piège rend la non-jointure ; `LibraryTest` : `ABI_VERSION=2`.

Critères : `composer test` vert ; le résultat est libéré avant de rendre ; aucun message re-analysé.

---

## PR5 — Pont Go (`bindings/go`)


Fichiers (conception §03) : `query.go`, `cell.go`, `ffi.go` (cinq symboles, `abiVersion=2`), `memless.go`/`rows.go` (type `Rows`).

Tests avant code — `memless_test.go` : `Query` nominal (`[][]any` avec `int64`/`float64`/`string`/`bool`/`nil`) ; un refus rend `*RefusalError` avec le message exact ; le fichier piège rend la non-jointure ; `abiVersion=2`.

Critères : `go test` et `go vet` verts ; aucun pointeur du résultat conservé après `Release`.

---

## PR6 — Parité (`harness/parity`)


Fichiers (conception §03) : `fixtures/trap.yaml`, `queries.txt`, `go/main.go` et `php/load.php` (mode `query`, format d'impression stable), `run.sh` (boucle de requêtes), `tests/detects-divergence.sh` (régression requête).

Critères : `sh run.sh` imprime « parity: all fixtures agree on both bridges » sur chargements **et** requêtes ; la régression échoue quand un pilote de requête est trafiqué ; le fichier piège prouve la non-jointure identique dans les deux langages. **Le statut `livre` de la conception et du plan est posé dans cette PR** (dernier lot).

---

## Ce qui reste hors du plan

Écriture et transactions (paliers 3–4), pont Node, banc de durée, révision de l'`ARCHITECTURE.md`
(GlueSQL sorti). Ces points sont nommés comme tâches distinctes, jamais glissés dans une PR du
palier 2.
