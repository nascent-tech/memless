# 05 — PR6, découpage et hors périmètre

<!-- charpente-navigation -->
**Index** : [plan — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [04 — PR3, PR4, PR5 : la surface native et les ponts](04-capi-et-ponts.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

---

## Phase 6 — PR6 : `harness/parity`, le lanceur de parité

**Statut : À faire**

Dernière PR. Dépend de **PR4 et PR5** fusionnées (elle rejoue les deux ponts). Elle vit hors
de Cargo. Fichiers figés par la conception §05, **plus les pilotes et leur câblage** ajoutés
par le plan (voir [partie 01](01-fichiers-figes.md), « Fichiers ajoutés ») :

- `harness/parity/go/go.mod` (module du pilote Go, `require` + `replace` vers `../../../bindings/go`)
- `harness/parity/go/main.go` — **pilote Go** : reçoit un chemin, charge via le pont, imprime
  l'issue (accepté / refusé + message), libère
- `harness/parity/php/composer.json` (dépendance `path` vers `../../../bindings/php`, autoload)
- `harness/parity/php/load.php` — **pilote PHP** : idem via le pont PHP
- `harness/parity/run.sh` (le lanceur)
- `harness/parity/README.md` (comment lancer, ce qu'est le relevé de parité)

**Câblage des pilotes** (ce qui rend les critères exécutables) : le pilote Go atteint
`bindings/go` par un `replace` dans son `go.mod` (ou un `go.work` à la racine du harness) ;
le pilote PHP atteint `bindings/php` par un dépôt Composer de type `path` et charge
`vendor/autoload.php`. Sans ce câblage, `go vet` et `php -l` n'auraient pas de pont à
résoudre.

**Ordre d'écriture** : les deux pilotes et leur câblage (`go.mod` → `main.go`,
`composer.json` → `load.php`) **avant** `run.sh` (qui les orchestre), et `README.md` en
dernier.

**Geste du lanceur** (conception §05) : pour chaque fixture, exécuter le pilote PHP,
**libérer**, exécuter le pilote Go, **libérer** (jamais deux instances vivantes sur le même
fichier) ; comparer les deux issues (accepté/refusé + texte) ; vérifier que le résidu voisin
est **intact** après un chargement réussi (D16) ; sortir 0 si toutes les fixtures produisent
la même issue des deux côtés, non-zéro sinon avec la liste des écarts.

**Tests, avant le code** (le lanceur a son propre test de non-régression) :
- runs every fixture through both bridges and exits 0 when all issues and messages match
- exits non-zero naming the fixture when the two bridges disagree on issue or text
- leaves the neighbouring residue byte-for-byte intact after loading `start.yaml` (D16)

**Critères vérifiables** (chaîne `sh`, les deux pilotes bâtis) :
- `bash harness/parity/run.sh` sort **0** sur les 18 cas de la batterie.
- `cd harness/parity/go && go vet ./...` sort 0 ; `php -l harness/parity/php/load.php` sort 0.
- Le test de non-régression du lanceur (un pilote factice qui diverge, monté par le test)
  fait sortir `run.sh` en non-zéro : `bash harness/parity/tests/detects-divergence.sh` sort 0.

**Ne fait pas** : ne prononce aucun refus (il **compare** ceux des ponts) ; ne mesure aucune
durée (paliers 3–4) ; ne rejoue pas Node ; ne teste qu'une seule famille de systèmes ×
deux langages.

**Ce que « zéro écart » ouvre sans clore** (README) : l'hypothèse §17.1 (brief, parité réelle
de PHP) est **ouverte**, pas tranchée — elle se tranchera au palier 4 sur toutes les familles
de systèmes.

---

## Le découpage, consolidé

### Ordre de fusion et dépendances

```
PR1 ──> PR2 ──> PR3 ──> PR4 ─┐
                      └─> PR5 ─┴─> PR6
```

- **PR2** importe `memless_domain` (`RawDocument`, `Refusal`, `Base`).
- **PR3** importe `memless_engine` (réexports `load`, `read`, `Base`, `Refusal`).
- **PR4** et **PR5** chargent la cdylib de PR3 et lisent son en-tête.
- **PR6** exécute les deux ponts et vérifie D16 sur la fixture résidu de PR2.

Chaque PR est **indépendamment fusionnable dans cet ordre** : elle ajoute un crate ou un
paquet sans toucher aux lignes déjà posées des précédents (la racine `Cargo.toml` grandit sur
des lignes disjointes, PR après PR). Aucune PR n'est **verte** sans sa dépendance bâtie
localement — enchaîner séquentiellement, sans anticiper (une seule développeuse, une PR = un
commit).

### Parallélisme

**PR4 ∥ PR5** : ordre libre, jamais un rebase entre elles. Fichiers (`bindings/php/**` vs
`bindings/go/**`) et chaînes d'outils (composer/php vs go) disjoints ; artefacts partagés en
**lecture seule** (cdylib, en-tête, fixtures) ; `.gitignore` posé en PR3 évite le seul
conflit trivial possible. « Parallèle » vaut ici « ordre libre », pas travail simultané :
PHP d'abord (hypothèse §17.1 du brief, la plus risquée).

### Sessions (reprise à froid, `/clear` entre chaque)

| Session | Phases | Contexte de reprise à froid |
|---|---|---|
| **S1** | PR1 (phases 1a, 1b) | ce plan (parties 01, 02) ; conception §01, §02, §05 ; décisions C-2, C-12, cadrage D1 |
| **S2** | PR2 (phases 2a, 2b) | ce plan (parties 01, 03) ; conception §03, §05 ; décisions C-5, C-8, C-11 ; **point d'entrée `serde-saphyr` et `Options` saturées, tranchés en partie 01** |
| **S3** | PR3 (phase 3) + PR4 (phase 4) | ce plan (parties 01, 04) ; conception §04 ; décisions C-6, C-7, C-9 ; **outillage de test PHP, tranché en partie 01** |
| **S4** | PR5 (phase 5) + PR6 (phase 6) | ce plan (parties 01, 04, 05) ; conception §04, §05 |

Le « 3 sessions » de la conception supposait PR3→PR6 (14 fichiers, 3 chaînes, débogage FFI
sur deux langages) en une seule session : trop optimiste. Chaque phase finit **verte**
(quatre portes de sa chaîne) et par un commit ; PR1 et PR2 restent **une seule PR** malgré
leurs deux phases.

---

## Hors périmètre (du cadrage et de la conception)

Ce plan **n'ajoute aucun périmètre**. Restent dehors, chacun avec son palier (cadrage §8,
conception §06) :

- **GlueSQL et le Store, la garde SQL** (N-1, N-2) — palier 2.
- **L'écrivain YAML, la transaction, l'instantané, le rechargement** (N-4) — paliers 3–4.
- **Le pont Node** (`memless-node`, `bindings/node`, N-5) — hors MVP ; **non membre** du
  workspace.
- **Le module `comparison`, un `trait` de port, un `LoadCommand`, un `composition.rs`, un
  port d'horloge, un type public « structure devinée », `cbindgen`/`build.rs`, `indexmap`,
  l'enveloppe `Instance`** (N-6 à N-12) — choisis de ne pas être créés.
- **Requêtes, écritures, transactions, banc de mesure, familles de systèmes multiples** —
  paliers 2–4 et au-delà.

Restent aussi hors de ce plan, renvoyés à leur propriétaire par la conception §06 :
`/charpente:init` (modèle de branches work/prod/staging, serveurs MCP) ; l'amendement de
`D10` du cadrage (`Options` saturées + dette assumée — **décideur : le fondateur**) ; les
désyncs d'`ARCHITECTURE.md` (glossaire « invariant »/« ACID »/« atomique », nettoyage du
résidu au chargement, crate nommé `memless-core`) et l'imprécision « budget » de la conception
§03/C-8 — **chantier d'architecture et amendement de la conception**.
