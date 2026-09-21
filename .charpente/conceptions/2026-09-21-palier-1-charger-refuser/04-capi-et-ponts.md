# 04 — Surface native et ponts

<!-- charpente-navigation -->
**Index** : [conception — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [03 — Application et lecture YAML (`memless-engine`)](03-application-et-yaml.md)  
**Suivant** : [05 — La batterie, le lanceur, l'espace de travail](05-banc-de-parite-et-workspace.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Les adaptateurs entrants : la surface C ABI que Go et PHP chargent, puis les deux ponts.
Un pont **traduit sans décider** (§5.1) — il rend le message tel quel dans la forme
d'erreur naturelle de son langage (D13), sans le reformuler.

Deux sigles reviennent ici. Un **handle** est un ticket de vestiaire : un nombre opaque
que l'appelant garde et rend pour désigner son instance, sans en connaître le contenu.
Le **C ABI** (*Application Binary Interface*) est la façon bas niveau, comprise par presque
tout langage, dont une bibliothèque compilée expose ses fonctions ; une **cdylib** est le
fichier compilé (`.so`/`.dylib`/`.dll`) qui la porte.

---

## `crates/memless-capi` — la surface C ABI

Crate `cdylib` + `staticlib`. C'est le **seul contrat publié** du palier (consommé par deux
ponts) ; sa sûreté est le point sensible (archi §10.5, sécurité archi §10).

### `crates/memless-capi/Cargo.toml`

```toml
[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
memless-engine = { path = "../memless-engine" } # réexporte Base, Refusal, load, read
```

**Ne porte pas** : `cbindgen`, ni `build.rs` (en-tête écrit à la main, C-9).

### `crates/memless-capi/include/memless.h`

En-tête C **écrit à la main** et committé (quatre fonctions, un statut, un type de handle :
trop peu pour justifier `cbindgen` et sa dépendance de build — C-9). Il est **chargeable par
PHP FFI**, qui refuse les directives du préprocesseur : donc **aucun `#include`**, des
types que PHP FFI connaît (`uint64_t`, `int32_t`), et la seule directive tolérée,
`#define FFI_SCOPE "memless"`. Contenu figé :

```c
#define FFI_SCOPE "memless"

typedef uint64_t MemlessHandle;

// 0 = Ok, 1 = Refused, 2 = InvalidArgument, 3 = Internal
typedef int32_t MemlessStatus;

uint32_t memless_abi_version(void);
MemlessStatus memless_load(const char *path, MemlessHandle *out_handle, char **out_message);
void memless_release(MemlessHandle handle);
void memless_free_string(char *message);
```

**Ne fait pas** : n'émet aucune directive `#include` ni bloc `__cplusplus` ; ne nomme pas
la bibliothèque (`FFI_LIB` reste au pont : l'extension dépend de la plateforme).

### `crates/memless-capi/src/lib.rs`

Les fonctions du contrat. La sortie sépare **trois catégories** pour qu'un défaut de
Memless ne se présente jamais comme un refus du fichier (le relevé de parité les
distingue) : un refus
produit (`Refused`), une faute d'usage de la frontière (`InvalidArgument`), un défaut
interne (`Internal`) — le relevé de parité les distingue. Le verbe est **charger**
(`load`), non « ouvrir » — « ouvrir » est réservé aux transactions du palier 4 (cadrage §8).

```rust
#[repr(C)]
pub enum MemlessStatus { Ok = 0, Refused = 1, InvalidArgument = 2, Internal = 3 }

pub type MemlessHandle = u64;

/// Charge une instance sur `path`. Contrat, sans ambiguïté sur `out_message` :
/// - Quand `out_message` est non nul, la fonction l'**écrit toujours** : `NULL` sur `Ok`,
///   sinon un message possédé (à libérer par `memless_free_string`). Les ponts ne lisent
///   `*out_message` que lorsqu'il est non nul, quel que soit le statut.
/// - `Ok` : `*out_handle` reçoit un handle >= 1 ; `*out_message` reçoit `NULL`.
/// - `Refused` : `*out_message` reçoit le message du refus ; `*out_handle` inchangé,
///   aucune instance créée.
/// - `InvalidArgument` : un pointeur d'entrée est nul, le chemin n'est pas UTF-8, **ou**
///   `out_handle` est nul ; `*out_message` reçoit un message de frontière ; aucune
///   instance créée. Si c'est `out_message` lui-même qui est nul, le statut est rendu seul
///   (aucune écriture n'est tentée à travers un pointeur nul).
/// - `Internal` : une panique a été rattrapée ; `*out_message` (s'il est non nul) reçoit
///   « internal error », déposé **après** `guard` par un `CString` qui ne peut pas paniquer.
/// La validation (chargement, chemin, `out_handle`) est enveloppée par `guard` : aucune
/// panique ne franchit la frontière ; le dépôt de « internal error » reste hors garde.
#[no_mangle]
pub extern "C" fn memless_load(
    path: *const c_char,
    out_handle: *mut MemlessHandle,
    out_message: *mut *mut c_char,
) -> MemlessStatus;

/// Libère l'instance désignée (geste terminal exigé par la batterie entre l'exécution
/// PHP et l'exécution Go, cadrage §8). Un handle inconnu ou `0` est ignoré. Enveloppé
/// par `guard`.
#[no_mangle]
pub extern "C" fn memless_release(handle: MemlessHandle);

/// Libère une chaîne rendue dans `out_message`. `NULL` est ignoré. Enveloppé par `guard`.
#[no_mangle]
pub extern "C" fn memless_free_string(message: *mut c_char);

/// Version de l'ABI (1 au palier 1) : chaque pont l'appelle au chargement et refuse un
/// écart avec la version qu'il attend (la constante vit dans le code du pont).
#[no_mangle]
pub extern "C" fn memless_abi_version() -> u32;
```

**Refus typés** : le `Refusal` du cœur, rendu en message via son `Display` (texte inchangé
— la parité vit là) ; les fautes de frontière, via `FfiError` (ci-dessous), texte hors
parité.

**Ne fait pas** : ne formule aucun message de refus (il rend celui du domaine) ; n'expose
aucune requête ni écriture (paliers 2–3) ; ne partage aucune instance entre appelants
(décision 6).

### `crates/memless-capi/src/instances.rs`

La table des handles, protégée entre fils, **tolérante à l'empoisonnement** (une panique
rattrapée pendant le verrou ne doit pas faire abandonner le processus au prochain appel).

```rust
/// `Mutex<HashMap<MemlessHandle, Base>>` ; handles alloués en séquence **à partir de 1**
/// (le `0` reste « aucune instance »). Le verrou se prend par
/// `lock().unwrap_or_else(PoisonError::into_inner)` — la table interrompue entre deux
/// appels reste cohérente.
pub(crate) fn register(base: Base) -> MemlessHandle;
pub(crate) fn release(handle: MemlessHandle);
```

Au palier 1 la table range directement une `Base` ; aucune enveloppe `Instance` (elle
renaîtra au palier 4 quand elle devra tenir une transaction — C-7).

**Ne fait pas** : ne tient aucune transaction (palier 4) ; ne réutilise pas un handle
libéré.

### `crates/memless-capi/src/ffi.rs`

Les gardes de frontière, réunies dans un seul fichier (dont la garde de panique — trop
petite pour un fichier à elle seule, C-9).

```rust
/// Fautes de frontière FFI — propres à `memless-capi`, jamais dans le domaine (C-6).
pub(crate) enum FfiError { NullPointer, InvalidUtf8, NullOutHandle }
impl FfiError { pub(crate) fn message(&self) -> &'static str; } // texte hors parité

/// Copie le chemin en `String` valide, ou rend une faute (nul, UTF-8 invalide). Rendu en
/// `String` possédé : aucun emprunt ne survit au pointeur hôte.
pub(crate) fn own_path(path: *const c_char) -> Result<String, FfiError>;

/// Valide un pointeur de sortie non nul avant tout chargement.
pub(crate) fn out_slot<'a, T>(out: *mut T) -> Result<&'a mut T, FfiError>;

/// Alloue un message possédé en `*mut c_char`. Un U+0000 intérieur (un `"\0"` venu du
/// fichier) est remplacé par U+FFFD avant `CString::new`, jamais une panique : le refus
/// nommé survit et les deux ponts lisent le même texte.
pub(crate) fn own_message(text: &str) -> *mut c_char;

/// Exécute `body` sous `catch_unwind` ; sur panique rattrapée, rend `fallback` plutôt que
/// de laisser l'unwind franchir la frontière — depuis Rust 1.81 un unwind non rattrapé sur
/// une ABI « C » **abandonne le processus** hôte (archi §10.5).
pub(crate) fn guard<R>(fallback: R, body: impl FnOnce() -> R + std::panic::UnwindSafe) -> R;
```

Les quatre points d'entrée (`load`, `release`, `free_string`, `abi_version`) passent tous
par `guard`. Le profil de compilation du scaffold **ne doit pas** poser `panic = "abort"`
pour ce crate, sans quoi `guard` serait inopérant (consigne portée en partie 5, à
répercuter au scaffold).

**Ne fait pas** : ne suppose jamais un pointeur valide (validation systématique) ; ne
propage aucun type du domaine pour une faute de transport.

---

## `bindings/php` — le pont FFI

Paquet Composer. Charge la cdylib via l'extension **FFI** (§18 : PHP ≥ 7.4) en lisant
l'en-tête committé : `FFI::cdef(file_get_contents($header), $libPath)`.

### `bindings/php/composer.json`

Nom du paquet, autoload PSR-4 vers `src/`, contrainte PHP `>=7.4` avec `ext-ffi`.

### `bindings/php/src/MemlessRefusal.php`

```php
final class MemlessRefusal extends \RuntimeException {}
```

Porte le message **tel quel** (D13). Une faute d'usage (`InvalidArgument`) ou interne
(`Internal`) lève au contraire une `\LogicException` — un bug n'est pas un refus du fichier.

### `bindings/php/src/Instance.php`

```php
final class Instance {
    public static function load(string $path): self; // memless_load
    public function release(): void;                  // memless_release (idempotent)
    public function __destruct();                     // release si non déjà fait
}
```

`load` appelle `memless_abi_version` au premier usage (écart => `\LogicException`), puis
`memless_load`. Sur `Refused` : **copie** le message (`FFI::string`), appelle
`memless_free_string`, **puis** lève `MemlessRefusal` — jamais un `throw` avant la
libération (sinon fuite). Ne conserve jamais le pointeur C.

**Ne fait pas** : ne décide rien du refus (il relaie) ; ne garde pas deux instances vivantes
sur le même fichier (la batterie libère avant Go).

---

## `bindings/go` — le pont purego

Module Go. Charge la cdylib via **purego** (`Dlopen` + `RegisterLibFunc`), sans cgo au
palier 1 (repli cgo documenté, non écrit — stack §7).

### `bindings/go/go.mod`

Module `.../memless`, version Go figée au plan, dépendance `purego`.

### `bindings/go/memless.go`

```go
type Instance struct { /* handle opaque */ }

// RefusalError porte le message tel quel (D13) ; errors.As permet de le distinguer d'une
// faute d'usage de la frontière.
type RefusalError struct{ Message string }
func (e *RefusalError) Error() string { return e.Message }

// Load charge le fichier. Un Refused rend un *RefusalError ; InvalidArgument/Internal, une
// error distincte. Aucun Instance n'est rendu sur erreur.
func Load(path string) (*Instance, error)

// Release libère l'instance. Aucun retour : memless_release rend void et ignore un handle
// inconnu, aucune erreur n'est possible — symétrique de release() côté PHP.
func (i *Instance) Release()
```

`Load` passe un `*byte` en `out_message` (un `**byte` côté C ; jamais un retour `string`,
que purego copierait en perdant le pointeur à libérer) ; `memless_load` rend le statut en
`int32`. Sur une erreur, `Load` **copie** le message jusqu'au `\0`, appelle
`memless_free_string`, **puis** rend l'erreur. Il vérifie `memless_abi_version` au
chargement du module.

**Ne fait pas** : ne reformule aucun message ; n'utilise pas cgo ; ne partage pas un
`Instance` entre goroutines sur le même fichier (hors périmètre au lancement, cadrage §8).
