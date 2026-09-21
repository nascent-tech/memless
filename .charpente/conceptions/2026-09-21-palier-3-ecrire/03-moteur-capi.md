# 03 — Moteur SQL, cas d'usage et surface native

<!-- charpente-navigation -->
**Index** : [conception — Palier 3 — Écrire](00-index.md)  
**Précédent** : [02 — L'écrivain YAML par substitution (`memless-engine`)](02-ecrivain.md)  
**Suivant** : [04 — Ponts, parité, banc et décisions](04-ponts-parite-banc.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

L'adaptateur `sql/` **descend** un `INSERT`/`UPDATE`/`DELETE` vers le `Write` du domaine ; le cas
d'usage `write` relie analyse, exécution par copie, réécriture et avancée mémoire dans l'ordre qui
tient la contrainte mémoire/disque ; la surface C ABI expose `memless_execute`. Porte **PR3** et **PR4**.

## `memless-engine` — la descente `sql/` et le cas d'usage

### `sql/` — texte → `Statement`

`parse` rend désormais `Statement` (lecture **ou** écriture) :

```rust
// sql/mod.rs — la signature évolue
pub type ParseSql = fn(&str) -> Result<Statement, QueryRefusal>;
pub fn parse(text: &str) -> Result<Statement, QueryRefusal>;
```

`sql/lower/` gagne `statement.rs` (aiguille selon le `Statement` de `sqlparser` : `Query` →
`Select`, `Insert`/`Update`/`Delete` → `Write`, le reste → Q2), et un descendeur par verbe :

- `lower/insert.rs` — `INSERT INTO t (cols) VALUES (row)` : exige la liste de colonnes (sinon Q2
  `INSERT without a column list`), **une seule** ligne de `VALUES` (sinon Q2 `multi-row VALUES`),
  chaque valeur un littéral `Scalar` **ou** `NULL` (réutilise `lower/literal` + `NULL`) ; refuse
  `ON CONFLICT`, `RETURNING`, `INSERT … SELECT` (Q2, libellés fermés).
- `lower/update.rs` — `UPDATE t SET c = v[, …] [WHERE …]` : `assignments` (valeur littérale ou
  `NULL`), `WHERE` descendu par `lower/filter` (réutilisé du palier 2) ; refuse `FROM`, alias,
  arithmétique en valeur (Q2).
- `lower/delete.rs` — `DELETE FROM t [WHERE …]` ; refuse `USING`, alias (Q2).

Le libellé d'un `construct` vient de la table fermée (palier 2), jamais du texte utilisateur. Un
`SELECT` reste une lecture ; c'est le **cas d'usage** qui refuse un `SELECT` passé à `execute` et
une écriture passée à `query` (Q2 `a SELECT in execute` / `a write in query`).

**Refus typés** : `QueryRefusal` (Q1, Q2). Q3/Q4/W4/W5/B appartiennent au domaine.

### `application::write` — le cas d'usage

```rust
// application/write.rs
pub fn write(parse: ParseSql, replace: ReplaceFile, instance: &mut Instance, text: &str)
    -> Result<u64, Refusal>;
```

Ordre **strict** (c'est lui qui tient la contrainte mémoire/disque, §8.6) :

1. `parse(text)?` → `Statement` ; un `Select` → `Refusal::Query(Q2 "a SELECT in execute")`.
2. `instance.base.write(&write)?` → `Applied { base, affected }` (Q3/Q4/W4/W5/B ici).
3. Si `applied.base == instance.base` → `Ok(applied.affected)` **sans toucher le disque** (§8.6).
4. Sinon `render(applied.base.document())` → texte ; `replace(&instance.path, &text)` — un `Err(io)`
   devient `Refusal::Write(DiskWriteFailed { path, kind })` (W10), `instance.base` **inchangé**.
5. **Puis seulement** `instance.base = applied.base` ; `Ok(applied.affected)`.

`application::query` (palier 2) est adapté à la nouvelle signature de `parse` : il refuse un
`Statement::Write` passé à la lecture (Q2 `a write in query`). `application::load` rend un
`Instance`. `lib.rs` réexporte `write`, `Instance`, `ReplaceFile`, `Statement`.

**Ne fait pas** : n'avance **jamais** la mémoire avant le succès disque ; ne garde aucun état ; ne
connaît pas les transactions (palier 4).

## `memless-capi` — `memless_execute` et l'instance

`ABI_VERSION` → **3**. `instances.rs` détient désormais un `Instance` (chemin + base) ;
`with_base` devient `with_instance` (`&`) et `with_instance_mut` (`&mut`). L'en-tête est étendu à
la main.

**Portée du verrou (cadrage point 8).** Le verrou global déjà tenu par `instances.rs` est conservé,
et `with_instance_mut` le tient **jusqu'au retour** de `application::write` — donc jusqu'à l'avance
mémoire. Une écriture est ainsi sérialisée de bout en bout : une lecture concurrente voit l'état
d'avant ou d'après, jamais entre ; deux écritures ne se perdent pas. Pas de verrou par instance au
palier 3 (le global suffit et reste le plus simple ; le raffiner est un chantier distinct).

```c
/* Runs a write `sql` against `handle`. On Ok, `*out_affected` receives the row count.
 * A SELECT, an unknown handle, or a null sql/out_affected yields InvalidArgument or Refused
 * per the contract. A disk failure is Refused with a message. */
MemlessStatus memless_execute(MemlessHandle handle, const char *sql,
                              uint64_t *out_affected, char **out_message);
```

Fichiers, comme au palier 2 : `execute_compute.rs` (retrouve l'instance par `with_instance_mut`,
appelle `application::write`, câble `parse` + `yaml::writer::replace_file`), réutilise
`write_outcome`/`Outcome` (le compte `affected` remplace le handle de résultat dans l'`Outcome`,
ou un `Outcome` jumeau `WriteOutcome { status, affected, message }`). Chaque entrée sous `guard` ;
`out_affected`/`out_message` nuls jamais déréférencés. `memless_query` **inchangé**.

**Ne fait pas** : n'analyse ni n'exécute (délègue) ; échec disque en `Refused`, pas un statut neuf.

## Tests (avant le code)

- **PR3** : `tests/sql.rs` étendu — chaque forme d'écriture valide → le `Write` attendu ; chaque
  exclusion (INSERT sans colonnes, VALUES multi-lignes, arithmétique, RETURNING, `SELECT` en
  execute, write en query) → Q2 avec son libellé. `tests/write.rs` d'intégration — batterie du MVP
  (insérer/modifier/supprimer isolément, puis les violations, puis le fichier attendu **à l'octet**
  après chaque écriture validée) via un `replace` de test capturant le texte.
- **PR4** : `tests/contract.rs` étendu — `abi_version == 3` ; `memless_execute` compte les lignes ;
  un `SELECT` → `Refused`/`InvalidArgument` ; un handle inconnu ; l'échec disque (un `replace` qui
  échoue) → `Refused` ; `out_affected` nul toléré.
