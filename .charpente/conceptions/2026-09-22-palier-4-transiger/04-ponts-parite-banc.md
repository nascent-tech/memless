# 04 — Ponts, parité, banc et décisions

<!-- charpente-navigation -->
**Index** : [conception — Palier 4 — Transiger](00-index.md)  
**Précédent** : [03 — Surface C ABI](03-capi.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Chaque pont gagne **trois gestes minces** — `begin`/`commit`/`rollback` — qui passent le verbe à
`memless_execute` ; la parité exige la même issue sur tout le geste **et** le même fichier réécrit à
l'octet à la validation ; le banc mesure une transaction validée. Porte **PR4** (PHP), **PR5** (Go),
**PR6** (parité + banc).

## Pont PHP — `bindings/php` (PR4)

`Instance` gagne trois façades minces (jumelles d'`execute`, dans `src/Transaction.php` ou par
délégation directe) :

```php
public function begin(): void    { Execute::run($this->handle, 'BEGIN'); }
public function commit(): void    { Execute::run($this->handle, 'COMMIT'); }
public function rollback(): void  { Execute::run($this->handle, 'ROLLBACK'); }
```

`Execute::run` (palier 3) rend déjà `int` (0 pour ces verbes) et lève `MemlessRefusal` sur T1/T2. Les
façades rendent `void` (le compte est 0). `Library::ABI_VERSION` → `4`. `query`/`execute` inchangés
(la lecture de ses propres écritures vient du cœur).

## Pont Go — `bindings/go` (PR5)

`Instance` gagne trois méthodes minces :

```go
func (i *Instance) Begin() error    { _, err := i.Execute("BEGIN"); return err }
func (i *Instance) Commit() error    { _, err := i.Execute("COMMIT"); return err }
func (i *Instance) Rollback() error  { _, err := i.Execute("ROLLBACK"); return err }
```

`Execute` (palier 3) rend `(uint64, error)` ; les méthodes jettent le compte (0) et rendent l'erreur
(`*RefusalError` sur T1/T2). `abiVersion` → `4`.

## Parité et banc — `harness/parity`, `harness/bench` (PR6)

- **`transactions.txt`** — une **transaction par ligne** : `<fixture><TAB><sql1>;;<sql2>;;…` (les
  instructions séparées par `;;`, exécutées dans l'ordre sur une même instance). Couvre : le
  **virement** (`BEGIN`;;`UPDATE`;;`UPDATE`;;`COMMIT`), l'**abandon** (`…`;;`ROLLBACK`), une
  **lecture de ses écritures** (`BEGIN`;;`UPDATE`;;`SELECT`;;`COMMIT` — la sortie du `SELECT` est
  comparée), l'**état intermédiaire invalide toléré** (`BEGIN`;;deux `DELETE` croisés;;`COMMIT`), la
  **validation échouée** (`BEGIN`;;`DELETE` référencé;;`COMMIT` → échec), les **gardes** (`BEGIN`;;
  `BEGIN` → T1 ; `COMMIT` seul → T2). `!disk` en tête pour l'échec disque au `COMMIT`.
- **Pilotes** — `go/main.go` et `php/load.php` gagnent un **mode transaction** : reçoivent `fixture`
  et la suite d'instructions, **copient la fixture en tmp** (même nom), ouvrent, exécutent chaque
  instruction dans l'ordre en imprimant son issue (`accepted:<n>` / `ok`+lignes pour un `SELECT` /
  `refused:<message>`), **puis** le **SHA-256 du fichier** et la présence du résidu. Le format est le
  contrat de comparaison.
- **`run.sh`** — une boucle de transactions : chaque ligne par les deux pilotes, sorties comparées
  octet à octet **et** le fichier réécrit comparé (par le SHA imprimé) ; `!disk` par `chmod 0555`
  (comme au palier 3), saut sous root.
- **`tests/detects-transaction-divergence.sh`** — un pilote trafiqué (un `COMMIT` qui réécrit un
  octet différent) fait échouer le lanceur.
- **Banc** — `examples/bench_transaction.rs` : durée d'une **transaction de k écritures validée**
  (une seule réécriture) vs k écritures isolées (k réécritures), en percentiles ; relevé ajouté à
  `.charpente/releves/banc.md` (montre qu'une transaction paie **un** `fsync`, non k).

## Ce que la parité garantit

Sur les fixtures, pour chaque transaction, PHP et Go rendent la **même** suite d'issues et le **même
fichier à l'octet** à la validation (décision 25). L'abandon et la validation échouée laissent le
fichier intact des deux côtés. C'est la preuve que « la transaction est tout ou rien » et « le
fichier est la vérité » tiennent identiquement à travers les ponts — le dernier apport au relevé
§17.1, **rejouable** par le fondateur sur chaque famille.

## Choix (C)

| # | Choix | Pourquoi |
|---|---|---|
| C-1 | **État de travail = un `Base` cloné dans l'`Instance`** (`Option<Base>`), pas un journal d'opérations. | Réutilise l'exécution par copie du palier 3 ; la lecture de ses écritures est un `select` sur ce `Base` sans code neuf ; l'abandon est un `drop`, la validation un `verify`+réécriture — le snapshot du palier 4 était déjà la copie du palier 3 (palier 3 C-1). |
| C-2 | **Scission `apply_write`/`verify_state`** exposée au domaine (le nom évite « stager », exclu). | La vérification reportée (décision 15) est exactement « appliquer sans vérifier, vérifier à la fin » ; scinder rend l'écriture isolée (`apply_write`+`verify_state`) et la transaction (`apply_write`×n puis `verify_state`) un seul mécanisme (§8.5). |
| C-3 | **Un seul chemin ABI, le texte SQL** (`memless_execute` reconnaît les verbes). | Décision 8 : un texte SQL identique quel que soit le langage ; pas de fonctions ABI par langage, pas de cinquième surface où diverger (§5.1). |
| C-4 | **`take` de la transaction en tête de `validate`.** | Garantit que la transaction est **fermée** quelle que soit l'issue (accepté, échec structure, échec disque) — décision 29, la validation referme toujours ; aucun état ouvert résiduel après un échec. |
| C-5 | **`query` lit `transaction.unwrap_or(base)`.** | Lecture de ses propres écritures sans dupliquer `select` ; hors transaction, comportement du palier 2 inchangé. |

## Non-choix (N)

| # | Non-choix | Pourquoi |
|---|---|---|
| N-1 | **Pas de journal d'annulation** (undo log / opérations inverses). | La copie est plus simple et sûre ; l'abandon est un `drop`, pas un rejeu inverse. |
| N-2 | **Pas de réécriture par écriture dans une transaction.** | Le tout-ou-rien exige **une** réécriture à la validation (§8.6) ; réécrire à chaque `stage` laisserait un état intermédiaire sur le disque. |
| N-3 | **Pas de fonctions ABI `memless_begin`/…** ni de niveau d'isolation. | Décision 8 (un texte SQL) ; décision 29 (une seule transaction, pas de `SAVEPOINT`). |
| N-4 | **Pas de coordination inter-fils de l'état de travail.** | Décision 7 : sérialiser est à l'appelant ; le moteur ne garantit que la sûreté mémoire. |

## Hors périmètre de la conception

Familles de systèmes multiples et relevé §17.1 complet (fondateur) ; transactions imbriquées /
`SAVEPOINT` ; rechargement §8.9 ; pont Node ; verrouillage inter-processus.

## Découpage (rappel, détaillé au plan)

`PR1` domaine (scission + refus) → `PR2` moteur (grammaire + état de travail + cas d'usage) → `PR3`
C ABI → `{PR4` PHP ∥ `PR5` Go`}` → `PR6` parité + banc. Ordre imposé ; PR4/PR5 parallèles.
