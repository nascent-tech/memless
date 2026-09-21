# 01 — Phases, critères et tests par PR

<!-- charpente-navigation -->
**Index** : [plan — Palier 3 — Écrire](00-index.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Une phase = une pull request. Chaque phase donne son périmètre exact (fichiers figés par la
conception), ses **tests d'abord**, ses **critères vérifiables**, ses **portes**. Statut par phase
tenu à jour à la fin du lot (`Fait`/`En cours`/`À faire`).

## PR1 — domaine : modèle, refus, filtre partagé, exécution par copie, rendu

**Statut : À faire.** Depuis conception §01. Crate `memless-domain` (pur). **Dépend de** : —.

Périmètre : `query/{write,insert,update,delete,statement}.rs` (`pub`) ; `refusal/write/` (`WriteRefusal`
W4/W5/W10, `Refusal::Write`, `From`, `Display`) ; `#[derive(Clone, PartialEq)]` sur `Base`/`Table`/`Row` ;
`base/filter/` (déplacement à l'identique de l'évaluation de `Filter` depuis `base/select/`,
`filter::keeps(row, filter) -> bool`) ; `base/write/` (`Applied`, `Base::write` : `apply` insert/update/delete,
`verify`, `unchanged`) ; `base/verify.rs` (scindé de `load`, partagé load/write) ; `base/render/`
(`Base::document() -> RawDocument`) ; réexports `lib.rs`.

Tests d'abord :
- `tests/write.rs` — `INSERT` (table neuve **en fin**, ligne **en fin**, colonnes dans l'ordre de la liste, `None`=absent) ; `UPDATE` (filtre, `SET NULL` retire, `SET id`, colonne neuve **en fin de ligne**, sans `WHERE`=toutes les lignes) ; `DELETE` (avec/sans `WHERE`=`t: []`) ; « ne change rien » (état égal) ; refus W4, W5, W6, W7, W8, W9 (dont orphelin par apparition de table nommant deux tables ; `UPDATE` de la relation vers une valeur absente).
- `tests/render.rs` — `Base::document` : ordre préservé, colonne absente omise, texte ambigu (`"5"`) reçoit `ExplicitText` ; aller-retour `load(document(base)) == base` sur les fixtures.
- `tests/write_refusal.rs` — gabarits W4/W5/W10.

Critères vérifiables : la batterie de lecture du palier 2 reste verte (déplacement de filtre sans changement de comportement) ; une `Base` égale à elle-même après une écriture sans effet ; W3 (table vidée `t: []` filtrée) rendu `UnknownColumn` comme au palier 2.

Portes : `cargo check -p memless-domain --all-targets` ; `cargo clippy -p memless-domain --all-targets -- -D warnings` ; `cargo test -p memless-domain` ; `cargo build -p memless-domain --release`.

## PR2 — écrivain YAML par substitution

**Statut : À faire.** Depuis conception §02. Crate `memless-engine`. **Dépend de** : PR1.

Périmètre : `yaml/render/` (rendu maison depuis `RawDocument` → `String` ; `quote.rs` : **liste
fermée** des cas de citation) ; `yaml/writer/` (`residue.rs` chemin `.<nom>.memless-tmp`,
`write_temp.rs` crée/tronque+permissions+`write_all`+`sync_all`, `swap.rs` substitution+fsync
répertoire, `cleanup.rs` supprime le résidu sur échec avant substitution) ; `application/replace_file.rs`
(`type ReplaceFile`) ; `application/instance.rs` (`Instance { path, base }`) ; `application::load`
élargi à `load(read, path) -> Result<Instance, Refusal>`. **Adaptation des appelants existants de
`load` dans CETTE PR** (constat plan-splitting) : `application::query` lit `instance.base`, le
câblage `capi` de `load` reçoit l'`Instance` — condition pour que `memless-engine` et `memless-capi`
restent verts sur `main` et que PR2 soit fusionnable seule.

Tests d'abord :
- `tests/render.rs` — golden par fixture ; **fixture déjà canonique → rendu identique (diff vide)** ; corpus limite (vide, espaces, indicateurs, mots nuls, décimal non représentable, **clés introduites par SQL** `"5"`/`"true"`/`"a: b"`) en aller-retour `render(document(load)) → load → égalité`.
- `tests/writer.rs` — `replace_file` écrit puis relit à l'octet ; répertoire `chmod 0555` → `Err(io)` **et aucun résidu** **et original intact** ; résidu préexistant écrasé ; permissions de l'original conservées.

Critères vérifiables : un fichier déjà canonique traverse `render(document(load))` sans un octet de différence ; après un échec d'écriture, `ls -a` du répertoire ne montre aucun `.<nom>.memless-tmp` ; `memless-engine` **et** `memless-capi` compilent (les appelants de `load` sont adaptés ici).

Limite assumée (constat user-flow) : la branche « échec **après** la substitution » (fsync du répertoire, non bloquant → `swap.rs` rend `Ok`, la mémoire avance) n'est **pas testée directement** — l'échec d'un fsync de répertoire n'est pas simulable de façon portable. L'issue retenue est fixée par la conception §02 et l'ordre strict du cas d'usage §03.

Portes : les quatre, `-p memless-engine`.

## PR3 — moteur SQL : descente d'écriture et cas d'usage

**Statut : À faire.** Depuis conception §03 (partie moteur). Crate `memless-engine`. **Dépend de** : PR2.

Périmètre : `sql/lower/{statement,insert,update,delete}.rs` (aiguillage `Statement` sqlparser →
`Select`/`Write`/Q2 ; exclusions W2 à libellé fermé ; `WHERE` via `lower/filter` réutilisé) ;
`sql::parse -> Result<Statement, QueryRefusal>` ; `application/write.rs` (ordre strict : parse →
`base.write` → si inchangé retour sans disque → `render`+`replace` → **puis** avance mémoire) ;
`application::query` adapté à la **nouvelle signature de `parse`** (refuse un `Statement::Write`
passé à la lecture, Q2) ; réexports `write`, `Instance`, `ReplaceFile`, `Statement`. *(L'adaptation
de `load → Instance` et de ses appelants a déjà été faite en PR2 ; PR3 ne porte que `parse → Statement`.)*

Tests d'abord :
- `tests/sql.rs` étendu — chaque écriture valide → le `Write` attendu ; chaque exclusion (INSERT sans colonnes, VALUES multi-lignes, arithmétique, RETURNING, `SELECT` en execute, write en query) → Q2 avec son libellé.
- `tests/write.rs` d'intégration — batterie MVP (insérer/modifier/supprimer isolément, violations, **fichier attendu à l'octet** après chaque écriture validée) via un `replace` de test capturant le texte, et l'ordre mémoire-après-disque (un `replace` qui échoue laisse `instance.base` inchangé).

Critères vérifiables : un `replace` de test qui renvoie `Err` prouve que `instance.base` n'a pas bougé ; une écriture « ne change rien » n'appelle jamais `replace`.

Portes : les quatre, `-p memless-engine`.

## PR4 — surface C ABI : `memless_execute`

**Statut : À faire.** Depuis conception §03 (partie capi). Crate `memless-capi` (cdylib). **Dépend de** : PR3.

Périmètre : `ABI_VERSION` → 3 ; `instances.rs` détient `Instance`, `with_instance`/`with_instance_mut`
(verrou tenu jusqu'au retour de `write`) ; `execute_compute.rs` (retrouve l'instance, câble `parse` +
`yaml::writer::replace_file`, `application::write`) ; `write_outcome`/`Outcome` avec `affected` ;
`memless_execute(handle, sql, out_affected, out_message)` sous `guard`, nuls jamais déréférencés ;
en-tête C étendu à la main ; `memless_query` inchangé.

Tests d'abord :
- `tests/contract.rs` étendu — `abi_version == 3` ; `memless_execute` compte les lignes ; `SELECT` → `Refused`/`InvalidArgument` ; handle inconnu ; échec disque (un `replace` qui échoue) → `Refused` ; `out_affected` nul toléré.

Critères vérifiables : `abi_version()` rend 3 ; un `SELECT` passé à `memless_execute` ne réécrit rien et rend `Refused`/`InvalidArgument`.

Portes : les quatre, `-p memless-capi` (advisory `avoidUnsafeBlocks` sur `unsafe fn`, toléré).

## PR5 — pont PHP : `Instance::execute`

**Statut : À faire.** Depuis conception §04 (PHP). `bindings/php`. **Dépend de** : PR4. **Parallèle à PR6.**

Périmètre : `src/Execute.php` (`Instance::execute(string $sql): int` ; appelle `memless_execute`,
rend `out_affected` (Ok), lève `MemlessRefusal` (Refused, échec disque compris) ou la faute de
frontière) ; `Library::ABI_VERSION` → 3 ; façade `execute` sur `Instance`.

Tests d'abord : `tests/ExecuteTest.php` — écriture acceptée (compte), refus (message), échec disque (répertoire non inscriptible → `MemlessRefusal`).

Critères vérifiables : `composer test` vert ; une écriture acceptée réécrit la fixture temporaire, un refus la laisse intacte.

Portes : `composer test` ; `php -l` sur les fichiers touchés.

## PR6 — pont Go : `Instance.Execute`

**Statut : À faire.** Depuis conception §04 (Go). `bindings/go`. **Dépend de** : PR4. **Parallèle à PR5.**

Périmètre : `execute.go` (`func (i *Instance) Execute(sql string) (uint64, error)` ; appelle
`memlessExecute`, rend le compte (Ok) ou `*RefusalError`/`*FaultError`) ; `ffi.go` enregistre
`memless_execute`, `abiVersion` → 3.

Tests d'abord : `execute_test.go` — écriture acceptée (compte), refus, échec disque (répertoire non inscriptible).

Critères vérifiables : `go test ./...` vert ; `go vet ./...` propre.

Portes : `go test ./...` ; `go vet ./...`.

## PR7 — parité et banc

**Statut : À faire.** Depuis conception §04 (parité/banc). `harness/parity`, `harness/bench`. **Dépend de** : PR5 **et** PR6.

Périmètre : `parity/writes.txt` (une écriture par ligne `<fixture><TAB><sql>`, `!disk` en tête pour
l'échec disque ; couvre INSERT ligne/table neuve/colonne neuve, UPDATE filtre/`SET NULL`/`SET id`,
DELETE filtre/table vidée, écriture sans changement, violations W4–W9) ; **mode écriture** des
pilotes `go/main.go` et `php/load.php` (copie la fixture en tmp **sous le même nom relatif**, charge,
exécute, imprime `accepted:<affected>` ou `refused:<message>`, **puis SHA-256** du fichier réécrit et
présence du résidu) ; `run.sh` (chaque ligne par les deux pilotes, sorties comparées octet à octet
**et** les deux fichiers réécrits comparés par `cmp` ; les lignes `!disk` s'exécutent dans une copie
`chmod 0555`, message **normalisé** — préfixe de répertoire tmp remplacé par un jeton avant
comparaison, puis les deux messages exigés **égaux** (compare nom de fichier **et** `kind` de la
liste fermée) —, empreinte inchangée, aucun résidu) ;
`tests/detects-write-divergence.sh` (pilote trafiqué → échec du lanceur) ;
`examples/bench_write.rs` + `harness/bench/run.sh` (k = 4/100/1 000/10 000 ; chargement, écriture
isolée, suite de 100 ; p50/p95 sur 20 répétitions) ; **premier relevé** à la main dans
`.charpente/releves/banc.md`.

Tests d'abord : `tests/detects-write-divergence.sh` échoue avec un pilote trafiqué ; la boucle `writes.txt` passe sur les deux ponts.

Critères vérifiables : `sh run.sh` imprime l'accord sur toutes les fixtures et toutes les écritures (issue **et** octets) ; `banc.md` porte un relevé daté avec percentiles. Le statut du document passe à `livre` (posé par build à la dernière phase, inclus dans la PR7).

Portes : `sh harness/parity/run.sh` (exit 0) ; `sh harness/bench/run.sh` + relevé écrit ; les quatre portes Cargo sur `memless-engine` (l'exemple de banc compile).

## Ordre et parallélisme

PR1 → PR2 → PR3 → PR4 → {PR5 ∥ PR6} → PR7. PR5 et PR6 ne dépendent que de PR4 et sont
indépendantes l'une de l'autre (langages disjoints). PR7 exige les deux ponts fusionnés. Chaque PR
part de `main` rafraîchi ; une PR dépendante attend la fusion de sa précédente (goal §« Par lot »).
