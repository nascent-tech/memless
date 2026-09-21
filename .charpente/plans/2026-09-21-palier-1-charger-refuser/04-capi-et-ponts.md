# 04 — PR3, PR4, PR5 : la surface native et les ponts

<!-- charpente-navigation -->
**Index** : [plan — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [03 — PR2 : le moteur et la batterie (`memless-engine`)](03-pr2-moteur-et-batterie.md)  
**Suivant** : [05 — PR6, découpage et hors périmètre](05-pr6-parite-et-decoupage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Trois PR d'interface. **PR3** publie le seul contrat du palier (la surface C ABI) ; **PR4**
et **PR5** le consomment depuis PHP et Go, sur des chaînes d'outils disjointes, donc d'ordre
libre. PR4 et PR5 dépendent de PR3 fusionnée (elles chargent la cdylib et lisent l'en-tête).

---

## Phase 3 — PR3 : `memless-capi`, le contrat C ABI

**Statut : À faire**

Fichiers figés écrits (conception §04), dans l'**ordre d'écriture** `ffi.rs` → `instances.rs`
→ `lib.rs` → `memless.h` (les gardes et la table d'abord, les points d'entrée qui s'en
servent ensuite, l'en-tête transcrivant en dernier la surface écrite) :

- `crates/memless-capi/Cargo.toml` (`crate-type = ["cdylib","staticlib"]`, dépend de
  `memless-engine`)
- `crates/memless-capi/include/memless.h` (en-tête C **écrit à la main**, chargeable par
  PHP FFI : aucun `#include`, `#define FFI_SCOPE "memless"`, types `uint64_t`/`int32_t`)
- `crates/memless-capi/src/lib.rs` (les quatre fonctions du contrat)
- `crates/memless-capi/src/instances.rs` (la table des handles)
- `crates/memless-capi/src/ffi.rs` (les gardes de frontière et `guard`)

Cette PR touche aussi la **racine** (lignes disjointes) : ajoute `memless-capi` à
`members`, pose le **profil de compilation sans `panic = "abort"`** pour ce crate (sinon
`guard` est inopérant — conception §04, §05), et pose le `.gitignore` (dont `vendor/`,
`target/` ; `go.sum` **n'est pas** ignoré) pour éviter tout conflit trivial entre PR4 et PR5.

**Tests, avant le code** (intégration, sur la vraie cdylib et le vrai contrat) :
- returns abi version 1 from `memless_abi_version`
- loads a valid start file: status `Ok`, handle >= 1, message NULL
- refuses an incoherent file: status `Refused`, message = the domain text unchanged (parité)
- writes the message through a non-null `out_message` on every non-Ok status
- returns `InvalidArgument` on a null path pointer, with a boundary message
- returns `InvalidArgument` on non-UTF-8 path
- returns `InvalidArgument` when `out_handle` is null
- returns the status alone (no write attempted) when `out_message` itself is null
- returns its fallback from `ffi::guard` when the wrapped body panics (unit test on `guard`, no forced panic across the ABI), and `memless_load` maps that to `Internal` with "internal error"
- allocates handles from 1, never reusing a released handle (`register`/`release`)
- keeps the handle table coherent after a poisoned lock (`unwrap_or_else(into_inner)`)
- replaces an interior U+0000 in a message by U+FFFD instead of panicking (`own_message`)
- ignores an unknown or `0` handle on `memless_release`
- ignores NULL on `memless_free_string`

**Critères vérifiables** :
- `cargo build -p memless-capi` sort 0, et la cdylib **et** la staticlib sont produites :
  `test -f target/debug/libmemless_capi.a && ls target/debug/libmemless_capi.dylib target/debug/libmemless_capi.so 2>/dev/null | grep -q .`
- `cargo test -p memless-capi` sort 0.
- `cargo clippy -p memless-capi -- -D warnings` sort 0.
- L'en-tête ne contient **aucune** ligne `#include` ni bloc `__cplusplus`
  (`! grep -qE '#include|__cplusplus' crates/memless-capi/include/memless.h`).

**Ne fait pas** : n'émet aucune requête ni écriture ; ne nomme pas la bibliothèque
(`FFI_LIB` reste au pont) ; ne pose pas `cbindgen` ni `build.rs` (C-9) ; ne peuple aucun
Store (C-7).

---

## Phase 4 — PR4 : `bindings/php`, le pont FFI

**Statut : À faire**

Dépend de PR3 fusionnée. Fichiers figés écrits (conception §04), dans l'**ordre d'écriture**
`composer.json` → `MemlessRefusal.php` → `Instance.php` (le manifeste, puis le type d'erreur
que l'instance lève, puis l'instance) :

- `bindings/php/composer.json` (autoload PSR-4 vers `src/`, contrainte `php >=7.4`,
  `ext-ffi`)
- `bindings/php/src/MemlessRefusal.php` (`extends \RuntimeException`, message tel quel — D13)
- `bindings/php/src/Instance.php` (`load`, `release` idempotent, `__destruct`)

**Portes de la chaîne PHP** (figées par ce plan — voir [partie 01](01-fichiers-figes.md)) :
`php -l` sur chaque source (syntaxe), `composer validate --strict` (lint du manifeste),
**PHPUnit** (test), et le chargement effectif de la cdylib (build). L'en-tête committé de
PR3 est lu par `FFI::cdef(file_get_contents($header), $libPath)`.

**Tests, avant le code** (PHPUnit ; la cdylib de PR3 doit être bâtie) :
- loads the start fixture and returns an Instance
- throws `MemlessRefusal` carrying the exact domain message on an incoherent fixture
- throws `\LogicException` on `InvalidArgument`/`Internal` (a bug is not a file refusal)
- copies the message and calls `memless_free_string` before throwing (no leak, no dangling pointer)
- checks `memless_abi_version` on first use and throws `\LogicException` on mismatch
- releases idempotently, and `__destruct` releases when not already released

**Critères vérifiables** :
- `php -l` sur `src/*.php` sort 0.
- `composer validate --strict` sort 0.
- `vendor/bin/phpunit` (depuis `bindings/php`) sort 0, avec la cdylib bâtie et `ext-ffi` actif.

**Ne fait pas** : ne décide rien du refus (il relaie) ; ne garde pas deux instances vivantes
sur le même fichier ; ne conserve jamais le pointeur C.

---

## Phase 5 — PR5 : `bindings/go`, le pont purego

**Statut : À faire**

Dépend de PR3 fusionnée ; **indépendante de PR4** (chaîne et fichiers disjoints). Fichiers
figés écrits (conception §04), dans l'**ordre d'écriture** `go.mod` → `memless.go` :

- `bindings/go/go.mod` (module `.../memless`, `go 1.21`, dépendance
  `github.com/ebitengine/purego` 0.10.2 — [partie 01](01-fichiers-figes.md))
- `bindings/go/memless.go` (`Instance`, `RefusalError`, `Load`, `Release`)

**Portes de la chaîne Go** (natives) : `go vet ./...` (typecheck+lint), `go test ./...`
(test), `go build ./...` (build). `Load` passe un `*byte` en `out_message` (jamais un retour
`string` que purego copierait) ; sur erreur, **copie** le message jusqu'au `\0`, appelle
`memless_free_string`, **puis** rend l'erreur.

**Tests, avant le code** (`go test` ; la cdylib de PR3 doit être bâtie) :
- loads the start fixture and returns an Instance
- returns a `*RefusalError` with the exact domain message on an incoherent fixture
- returns a distinct error (not `*RefusalError`) on `InvalidArgument`/`Internal`, `errors.As` distinguishes it
- returns no Instance on any error
- checks `memless_abi_version` at module load
- releases via `Release` (no return, unknown handle ignored)

**Critères vérifiables** :
- `go vet ./...` (depuis `bindings/go`) sort 0.
- `go build ./...` sort 0.
- `go test ./...` sort 0, avec la cdylib bâtie et son chemin résolu par purego `Dlopen`.

**Ne fait pas** : ne reformule aucun message ; n'utilise pas cgo (repli documenté, non
écrit) ; ne partage pas un `Instance` entre goroutines sur le même fichier.

---

## Ce que PR4 et PR5 partagent, en lecture seule

La cdylib (`target/debug/libmemless_capi.*`), l'en-tête `crates/memless-capi/include/memless.h`
(lu par PHP FFI à l'exécution ; Go enregistre par nom via purego), et deux ou trois fixtures
de PR2 pour leurs tests. **Aucune des deux ne les modifie** : le parallélisme (ordre libre)
tient. Un point d'environnement propre à chaque chaîne — `ext-ffi` actif côté PHP, résolution
du chemin de la cdylib sur macOS côté Go — est un aléa de mise en route, pas un couplage.
