# 02 — Moteur et surface native (`memless-engine`, `memless-capi`)

<!-- charpente-navigation -->
**Index** : [conception — Palier 2 — Interroger](00-index.md)  
**Précédent** : [01 — Domaine : modèle, exécution, refus (`memless-domain`)](01-domaine.md)  
**Suivant** : [03 — Ponts et parité (`bindings/php`, `bindings/go`, `harness/parity`)](03-ponts-et-parite.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

Deux couches : l'adaptateur `sql/` qui **analyse** un texte en `Select` du domaine (prononçant
Q1 et Q2), le cas d'usage `query` qui les relie ; puis la surface C ABI `memless_query` et ses
accesseurs de résultat. Porte **PR2** (moteur) et **PR3** (C ABI).

---

## `memless-engine` — l'adaptateur `sql/` et le cas d'usage

### `crates/memless-engine/Cargo.toml`

Gagne une dépendance : `sqlparser = "=0.54.0"` (épinglée `=`, comme `serde-saphyr` ; vérifiée
comme se résolvant et compilant, cadrage D3). Aucune autre.

### `crates/memless-engine/src/sql/` — texte → `Select`

Le seul endroit qui connaît `sqlparser`. Un port fonction, câblé par `memless-capi` :

```rust
// sql/mod.rs
pub type ParseSql = fn(&str) -> Result<Select, QueryRefusal>;

pub fn parse(text: &str) -> Result<Select, QueryRefusal>;   // l'implémentation par défaut
```

`parse` :

1. **`analyse.rs`** — `Parser::parse_sql(&GenericDialect, text)` ; une erreur → `Q1`
   (`InvalidSql { detail }`, `detail` = message de `sqlparser` nettoyé de son préfixe, position
   conservée). Exige **exactement une** instruction, sinon Q2 (`multiple statements`).
2. **`lower/`** — descend l'AST `Statement::Query` vers `Select`, **prononçant Q2** à chaque
   construction hors sous-ensemble via une **table de correspondance** de libellés fixes
   (`lower/construct.rs` : `ORDER BY`, `GROUP BY`, `HAVING`, `LIMIT`, `OFFSET`, `DISTINCT`,
   `LEFT JOIN`, self-join, `SELECT` sans `FROM`, `INSERT`/`UPDATE`/`DELETE`/`CREATE` →
   `<construct> is outside the supported SQL subset`). Un fichier par morceau descendu :
   `lower/projection.rs` (→ `Items`, refuse colonnes mêlées à agrégats), `lower/from.rs` (refuse
   l'absence de `FROM`), `lower/join.rs` (inner seul ; `ON relation = id` ; refuse la self-join,
   la deuxième jointure), `lower/filter.rs` (`Compare`/`IsNull`/`And`/`Or`, refuse
   `NOT` hors `IS NOT NULL`, `IN`/`LIKE`/`BETWEEN`/arithmétique/`NULL` littéral),
   `lower/column_ref.rs` (identifiant simple ou qualifié ; guillemets doubles → nom tel quel,
   cadrage D2 ; refuse la colonne non qualifiée quand il y a une jointure), `lower/literal.rs`
   (littéral SQL → `Scalar` ; nombre signé = littéral ; entier hors capacité → Q1).

Le libellé d'un `construct` ne cite **jamais** un fragment du texte utilisateur (cadrage Q2) :
il vient de la table. `ORDER BY`, une valeur en position de colonne, une fonction autre que
`COUNT`/`SUM`, une sous-requête, `UNION`, `CASE`, `CAST`, plus d'une instruction → Q2 avec leur
libellé. Le tri est refusé ici (hors sous-ensemble, cadrage D1), jamais exécuté.

**Refus typés** : `QueryRefusal` (Q1, Q2). Q3–Q8 appartiennent au domaine.

**Ne fait pas** : n'exécute rien ; ne touche pas `Base` ; ne connaît aucune règle de
comparaison (elle vit dans le domaine).

### `crates/memless-engine/src/application/query.rs` — le cas d'usage

```rust
pub fn query(parse: ParseSql, base: &Base, text: &str) -> Result<Rows, Refusal> {
    let select = parse(text)?;        // Q1, Q2  -> Refusal::Query
    let rows = base.select(&select)?; // Q3..Q8  -> Refusal::Query
    Ok(rows)
}
```

Deux gardes, une transcription : le `?` remonte `QueryRefusal` en `Refusal::Query` par le `From`
de la partie 01. `application/mod.rs` déclare `pub mod query;`. `lib.rs` réexporte
`pub use application::query::query;`, `pub use sql::{parse, ParseSql};`, et réexporte du domaine
`Rows`, `Select`, `QueryRefusal`.

**Ne fait pas** : ne décide aucune règle ; ne garde aucun état (le `Base` lui est prêté).

---

## `memless-capi` — `memless_query` et les accesseurs de résultat

`ABI_VERSION` passe à **2**. L'en-tête `include/memless.h` est étendu à la main (le projet le
tient à la main, palier 1). Le résultat traverse la frontière par des **accesseurs typés**, pas
un JSON (cadrage D7) : un objet résultat opaque, détenu par une table comme les instances, lu
cellule par cellule, puis libéré.

### Le résultat opaque — `src/results.rs`

Une table d'objets `Rows` derrière un `MemlessResult = u64`, jumelle de `instances.rs`
(`store`/`discard`, `LazyLock<Mutex<…>>`). Le `Rows` du domaine y est **converti une fois** en
une forme prête à lire (colonnes en `CString`, cellules en un tag + valeur), pour qu'aucun
accesseur n'alloue.

### Les entrées C — `src/lib.rs` (ajouts)

```c
/* 0 Absent, 1 Text, 2 Integer, 3 Decimal, 4 Boolean */
typedef int32_t MemlessKind;
typedef uint64_t MemlessResult;

/* Runs `sql` against `handle`. On Ok, `*out_result` receives a result handle >= 1
 * the caller must release with memless_result_release. On Refused, `*out_message`
 * carries the domain message. */
MemlessStatus memless_query(MemlessHandle handle, const char *sql,
                            MemlessResult *out_result, char **out_message);

uint64_t memless_result_column_count(MemlessResult result);
uint64_t memless_result_row_count(MemlessResult result);
/* Borrowed, valid until memless_result_release; never freed by the caller. */
const char *memless_result_column(MemlessResult result, uint64_t index);
/* Writes the cell kind and, for a present scalar, one of the out_* payloads.
 * Text is borrowed until release. Absent writes kind only. */
MemlessKind memless_result_cell(MemlessResult result, uint64_t row, uint64_t column,
                                int64_t *out_integer, double *out_decimal,
                                int32_t *out_boolean, const char **out_text);
void memless_result_release(MemlessResult result);
```

Chaque entrée est enveloppée dans `guard` (catch_unwind, palier 1) ; un handle inconnu rend un
compte nul / un `kind` Absent / une chaîne vide, jamais un comportement indéfini. `memless_query`
retrouve l'instance (table de handles), appelle `application::query(parse, base, sql)`, range le
`Rows` en résultat opaque (Ok) ou rend le message (Refused). Fichiers, comme au palier 1 :
`query_compute.rs` (l'équivalent de `compute.rs`), `results.rs`, `cell.rs` (la conversion
`Option<Scalar>` → tag + payload), `result_write.rs`.

**Refus typés** : aucun neuf — `Refusal` est rendu en `MemlessStatus::Refused` + message,
`InvalidArgument`/`Internal` couvrent la frontière (palier 1).

**Ne fait pas** : n'analyse ni n'exécute (délègue à `memless-engine`) ; ne garde aucune règle ;
n'expose jamais un pointeur qui survivrait à `memless_result_release`.

---

## Tests (avant le code)

- **Domaine (PR1)** : `tests/select.rs` — un `Select` construit à la main contre une `Base`
  chargée depuis les fixtures ; filtre (types égaux/différents, absence, `<`/`>` au sein d'un
  type), jointure `wallets→users`, la jointure du fichier piège qui rejoint `id: 5` et jamais
  `id: "5"`, `COUNT(*)`/`COUNT(col)`/`SUM`, et Q3–Q5/Q7/Q8 chacun nommé. `tests/query_refusal.rs`
  — chaque gabarit de message.
- **Moteur (PR2)** : `tests/parse.rs` étendu ou `tests/sql.rs` — chaque construction exclue rend
  Q2 avec son libellé ; un SQL cassé rend Q1 ; un `SELECT` valide rend le `Select` attendu.
  `tests/query.rs` — `application::query(parse, &base, text)` bout en bout sur `start.yaml` et
  `trap.yaml` ; golden des messages.
- **C ABI (PR3)** : `tests/contract.rs` étendu — `memless_query` sur une instance chargée, lecture
  des cellules, `memless_result_release` ; un handle inconnu ; un résultat libéré deux fois ;
  `abi_version == 2`.
