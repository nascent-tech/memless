# 03 — Ponts et parité (`bindings/php`, `bindings/go`, `harness/parity`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 2 — Interroger](00-index.md)  
**Précédent** : [02 — Moteur et surface native (`memless-engine`, `memless-capi`)](02-moteur-et-capi.md)  
**Suivant** : [04 — Décisions et hors périmètre](04-decisions-et-hors-perimetre.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Chaque pont ajoute **un seul geste** — `query` — au-dessus de l'instance du palier 1, dans la
forme naturelle de son langage. Un pont **traduit sans décider** (§5.1) : il lit le résultat
cellule par cellule par les accesseurs (partie 02) et le rend en objet natif ; il ne connaît
aucune règle SQL. Porte **PR4** (PHP), **PR5** (Go), **PR6** (parité).

---

## Pont PHP — `bindings/php` (PR4)

`Instance::query(string $sql): array` : appelle `memless_query`, et selon le statut rend les
lignes (Ok) ou lève une `MemlessRefusal` (Refused) / `LogicException` (faute de frontière) —
exactement le triage de `Instance::load`.

- **`src/Query.php`** — jumelle de `Call.php` : un appel `memless_query`, récupère le
  `MemlessResult`, lit `column_count`/`row_count`, boucle les cellules par
  `memless_result_cell` (traduit le `MemlessKind` : Absent → `null`, Text → `string`, Integer →
  `int`, Decimal → `float`, Boolean → `bool`), libère le résultat par `memless_result_release`
  **avant de rendre**, puis rend un tableau de lignes, chaque ligne un tableau associatif
  `nom de colonne → valeur`.
- **`src/Instance.php`** — gagne `query`, réutilise `STATUS_OK`/`STATUS_REFUSED`.
- **`src/Library.php`** — `ABI_VERSION` passe à `2` ; l'en-tête étendu est lu tel quel.

Une valeur absente devient `null` ; deux colonnes de même nom après jointure sont impossibles
car les en-têtes sont **qualifiés** `table.column` (cadrage D6), donc les clés du tableau
associatif restent uniques.

**Ne fait pas** : ne re-analyse aucun message ; ne garde aucun `MemlessResult` au-delà de
l'appel ; ne devine aucun type (le `kind` le donne).

---

## Pont Go — `bindings/go` (PR5)

`(*Instance).Query(sql string) (Rows, error)` avec un type de résultat idiomatique :

```go
type Rows struct {
    Columns []string
    Rows    [][]any   // int64, float64, string, bool, ou nil pour une cellule absente
}
```

- **`query.go`** — appelle `memlessQuery`, lit le résultat par les accesseurs, remplit `Rows`,
  libère par `memlessResultRelease`, rend `Rows` (Ok) ou un `*RefusalError` / `*FaultError`
  (même triage que `interpret`).
- **`ffi.go`** — enregistre les cinq nouveaux symboles (`memless_query`,
  `memless_result_column_count`, `_row_count`, `_column`, `_cell`, `_release`) ; `abiVersion`
  passe à `2`.
- **`cell.go`** — traduit un `MemlessKind` + payload en `any` (`nil` pour Absent).

**Ne fait pas** : ne conserve aucun pointeur du résultat après `Release` ; `errors.As` distingue
refus et faute, inchangé.

---

## Parité — `harness/parity` (PR6)

Le lanceur du palier 1 ne rejoue que le **chargement**. Le palier 2 y ajoute une **batterie de
requêtes** rejouée sur deux fixtures chargées, comparée résultat **et** message entre PHP et Go.

- **Fixtures** : `fixtures/trap.yaml` — `users` portant `id: 5` **et** `id: "5"`, plus une
  `wallets` avec `user_id: 5` — pour prouver la non-jointure (MVP §6). `start.yaml` (palier 1)
  sert aux requêtes nominales.
- **`queries.txt`** — la batterie : une ligne = `fixture | sql`. Couvre filtre, jointure
  `wallets→users`, `COUNT`, `SUM`, tri, résultat vide, la non-jointure du piège, et chaque refus
  Q1–Q8. C'est le golden partagé.
- **Pilotes** : `go/main.go` et `php/load.php` gagnent un mode `query` — reçoivent `fixture` et
  `sql`, chargent, interrogent, **impriment chaque cellule dans un format stable** fixé par le
  lanceur (`kind=valeur`, ligne par ligne, colonnes en en-tête), libèrent, ou impriment
  `refused: <message>` / `fault: <…>`. Le format d'impression est le **contrat de comparaison**,
  identique aux deux pilotes.
- **`run.sh`** — après la boucle de chargement, une boucle de requêtes : pour chaque ligne de
  `queries.txt`, exécute les deux pilotes, compare octet à octet leurs sorties ; toute
  divergence est listée comme au chargement.
- **`tests/detects-divergence.sh`** — étendu : un pilote de requête trafiqué doit faire échouer
  le lanceur (la régression prouve que le lanceur voit bien une divergence de requête).

**Ne fait pas** : n'introduit aucune règle ; le lanceur ne juge pas le contenu, il **compare**
les deux sorties et exige leur identité (§5.1).

---

## Ce que le contrat de parité garantit

Sur `start.yaml` et `trap.yaml`, pour chaque requête de `queries.txt`, PHP et Go rendent le
**même** jeu de lignes (mêmes colonnes, mêmes cellules, même ordre) ou le **même** refus (même
message). Le fichier piège est la preuve vivante de la règle de comparaison unique en lecture :
`user_id: 5` ne rejoint jamais `id: "5"`, dans les deux langages, sans erreur. C'est la
deuxième épreuve de §17.1, toujours pas sa tranche (palier 4).
