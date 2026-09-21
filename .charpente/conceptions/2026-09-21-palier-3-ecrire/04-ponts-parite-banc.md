# 04 — Ponts, parité, banc et décisions

<!-- charpente-navigation -->
**Index** : [conception — Palier 3 — Écrire](00-index.md)  
**Précédent** : [03 — Moteur SQL, cas d'usage et surface native](03-moteur-capi.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Chaque pont gagne **un geste** — `execute` — qui rend un compte de lignes ; la parité exige la
même issue **et** le même fichier réécrit **à l'octet** ; le banc §17.2 tient son premier relevé.
Porte **PR5** (PHP), **PR6** (Go), **PR7** (parité + banc).

## Pont PHP — `bindings/php` (PR5)

`Instance::execute(string $sql): int` (dans `src/Execute.php`, jumelle de `Query.php`) : appelle
`memless_execute`, rend `out_affected` (Ok), lève `MemlessRefusal` (Refused, y compris l'échec
disque) ou la faute de frontière (LogicException). `Library::ABI_VERSION` → `3`. `Instance` gagne
`execute` (façade mince, comme `query`).

## Pont Go — `bindings/go` (PR6)

`func (i *Instance) Execute(sql string) (uint64, error)` (`execute.go`) : appelle
`memlessExecute`, rend le compte (Ok) ou un `*RefusalError`/`*FaultError`. `ffi.go` enregistre
`memless_execute`, `abiVersion` → `3`.

## Parité et banc — `harness/parity`, `harness/bench` (PR7)

- **`writes.txt`** — une écriture par ligne (`<fixture><TAB><sql>`, `!disk` en tête pour les cas
  d'échec disque). Couvre : `INSERT` (ligne, table nouvelle, colonne nouvelle), `UPDATE` (filtre,
  `SET NULL`, `SET id`), `DELETE` (filtre, table vidée), une écriture qui **ne change rien**, et
  chaque violation W4–W9.
- **Pilotes** — `go/main.go` et `php/load.php` gagnent un **mode écriture** : reçoivent
  `fixture` et `sql`, **copient la fixture dans un répertoire temporaire**, chargent, exécutent,
  impriment `accepted:<affected>` ou `refused:<message>`, **puis** le **SHA-256 du fichier réécrit**
  et la présence du résidu, libèrent. Le format d'impression est le contrat de comparaison.
- **`run.sh`** — une boucle d'écritures : chaque ligne exécutée par les deux pilotes, sorties
  comparées octet à octet **et** les deux fichiers réécrits comparés (`cmp`).
- **Comparaison du message `!disk` (W10).** Le message `cannot write file {path:?}: {kind}` porte le
  `path` **absolu** du répertoire temporaire propre à chaque pilote : il diffère par construction, la
  comparaison à l'octet du message brut est donc impossible. Règle fixée : les deux pilotes copient
  la fixture sous le **même nom relatif** dans leur tmp ; `run.sh` **normalise** le message en
  remplaçant le préfixe de répertoire par un jeton (`<dir>/`) avant de comparer, puis exige les deux
  messages **égaux après normalisation** — ce qui compare bien le nom du fichier **et** le `kind`. Le
  `kind` provient d'une **liste fermée** dérivée de `io::ErrorKind` (`PermissionDenied`, `NotFound`,
  `StorageFull`, `AlreadyExists`, sinon `other`), énumérée au domaine (partie 01) et testée. Les
  lignes `!disk` s'exécutent dans une copie `chmod 0555` et exigent le préfixe `refused:cannot write
  file `, empreinte inchangée, aucun résidu.
- **`tests/detects-write-divergence.sh`** — un pilote trafiqué (fichier réécrit d'un octet
  différent) doit faire échouer le lanceur.
- **Banc §17.2** — `crates/memless-engine/examples/bench_write.rs` + `harness/bench/run.sh` :
  génère `start.yaml × k` (k = 4, 100, 1 000, 10 000), mesure chargement, une écriture isolée
  (réécriture comprise) et une suite de 100 écritures, en **percentiles** (p50/p95) sur 20
  répétitions ; le **premier relevé** est écrit à la main dans `.charpente/releves/banc.md` (MVP §8).

## Ce que la parité garantit

Sur les fixtures, pour chaque écriture de `writes.txt`, PHP et Go rendent le **même** compte ou le
**même** refus, **et** produisent le **même fichier à l'octet** (décision 25). L'échec disque
laisse le fichier intact et aucun résidu, dans les deux langages. C'est la preuve que « le fichier
est la vérité » tient identiquement à travers les ponts, et le premier apport au relevé de parité
§17.1 côté écriture.

## Choix (C)

| # | Choix | Pourquoi |
|---|---|---|
| C-1 | **Exécution par copie** (`Base::write` clone, vérifie, rend), mémoire avancée après le disque. | Tient « la mémoire ne garde jamais ce que le disque n'a pas reçu » (§8.6) sans journal d'annulation ; le coût O(n) est celui de la réécriture qui suit ; la copie que réutilisera le palier 4 pour l'état d'avant. |
| C-2 | **`Base::verify` scindé de `load`**, partagé avec `write`. | La redérivation des contraintes (décision 37) est le code du chargement ; le partager rend l'orphelin par apparition de table gratuit et nomme deux tables sans code dédié. |
| C-3 | **Rendu maison** (`Base::document` domaine + `yaml/render` texte), pas le sérialiseur serde-saphyr. | « Identique à l'octet » et « diff lisible » sont des exigences produit ; elles tiennent dans un code que Memless possède et verrouille par un aller-retour. |
| C-4 | **Port `ReplaceFile` fonction**, écrivain concret dans `yaml/writer`. | Même patron que `ReadSource`/`ParseSql` (palier 2 C-3) ; une seule implémentation, inversion des dépendances préservée. |
| C-5 | **`memless_execute` unique** (pas un par verbe), compte de lignes, échec disque en `Refused`. | Verbe de PDO/`database/sql` ; les ponts n'ont qu'une bascule refus/faute, pas de cinquième statut (cadrage D6). |
| C-6 | **Évaluation de filtre montée dans `base/filter/`**, partagée select/write. | Une seule sémantique de `WHERE` pour le produit ; en dupliquer une ferait diverger lecture et écriture. |

## Non-choix (N)

| # | Non-choix | Pourquoi |
|---|---|---|
| N-1 | **Pas de `&mut self` + annulation manuelle** (opérations inverses appliquées après coup). | La copie est plus simple et sûre, et sert déjà le palier 4. |
| N-2 | **Pas de conservation** des commentaires/mise en forme d'origine. | Décision 26 : non garanti ; conséquences connues à la première réécriture. |
| N-3 | **Pas de sérialiseur `serde-saphyr`** en écriture. | Aucune maîtrise de la citation/indentation ; forme instable entre versions. |
| N-4 | **Pas de cinquième statut C ABI** pour l'échec disque. | Le brief le range parmi les refus de validation. |
| N-5 | **Pas de `VALUES` multi-lignes ni d'`INSERT` sans colonnes.** | Une ligne = une écriture isolée ; sans forme déclarée, aucune position pour une valeur sans nom. |

## Hors périmètre de la conception

Transactions multi-instructions et état d'avant explicite (palier 4) ; pont Node ; banc en continu ;
révision de `ARCHITECTURE.md` (cadrage D8) ; cascade/mise à `NULL` automatique (§14).

## Découpage (rappel, détaillé au plan)

`PR1` domaine → `PR2` écrivain par substitution → `PR3` moteur SQL → `PR4` C ABI → `{PR5` PHP ∥ `PR6` Go`}`
→ `PR7` parité + banc. Ordre imposé ; PR2 séparée de PR3 (la plus risquée, revue seule).
