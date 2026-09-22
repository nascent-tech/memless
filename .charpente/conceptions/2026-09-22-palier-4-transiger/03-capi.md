# 03 — Surface C ABI

<!-- charpente-navigation -->
**Index** : [conception — Palier 4 — Transiger](00-index.md)  
**Précédent** : [02 — État de travail et cas d'usage (`memless-engine`)](02-moteur.md)  
**Suivant** : [04 — Ponts, parité, banc et décisions](04-ponts-parite-banc.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

**Aucune fonction ABI neuve** (cadrage D5, décision 8 : un seul texte SQL). Le **recâblage** de
`run_execute`/`run_query` vers `application::execute`/`query(instance)` a lieu en **PR2** (frontière
de compilation) ; PR3 ne porte que la **version**, l'**en-tête** et les **tests de contrat**. Porte
**PR3**.

## Ce que PR2 a déjà câblé (rappel)

`memless_execute` (signature inchangée) délègue à `application::execute` par `with_instance_mut` — le
**verrou global reste tenu jusqu'au retour**, donc pendant tout un `COMMIT` (vérification + réécriture
+ avance mémoire), sans ré-entrance. `out_affected` reçoit **0** pour `BEGIN`/`COMMIT`/`ROLLBACK`, le
compte pour une écriture ; T1/T2 en `Refused` avec message. `memless_query` (signature inchangée)
délègue à `application::query(instance)` par `with_instance` : une transaction ouverte fait voir
l'**état de travail** (read-your-writes) ; une écriture passée à `query` reste refusée sans toucher
la transaction.

## Ce que PR3 ajoute : version, en-tête, contrat

`ABI_VERSION` → **4**. L'en-tête `include/memless.h` est étendu **à la main** : le commentaire de
`memless_execute` mentionne que `BEGIN`/`COMMIT`/`ROLLBACK` sont acceptés (`out_affected = 0`), que
`COMMIT`/`ROLLBACK` sans transaction et un deuxième `BEGIN` rendent `Refused` ; celui de
`memless_query` mentionne qu'une requête dans une transaction ouverte voit les écritures non encore
validées (lecture de ses propres écritures). `memless_load`/`memless_release` inchangés.

**Conséquence de la version (comme au palier 3).** `ABI_VERSION = 4` fait **refuser** le chargement
par les ponts PHP/Go tant qu'ils sont à `3` : `bindings/php/src/Library.php`, `bindings/go/ffi.go`
et `harness/parity/run.sh` **restent rouges** entre PR3 et {PR4 ∥ PR5}. C'est le précédent du palier
3 (l'ABI monte avant les ponts) ; PR3 ne prétend qu'aux **quatre portes Cargo** (au workspace), pas
à `phpunit`/`go test`/parité, qui reviennent au vert en PR4/PR5.

**Ne fait pas** : n'analyse ni n'exécute (délègue) ; n'ajoute aucun statut (T1/T2 = `Refused`) ni
aucune fonction ; ne connaît pas l'état ouvert/fermé (il vit dans l'`Instance`).

## Tests (avant le code)

- `tests/contract.rs` étendu — `abi_version == 4`. Une **transaction complète par le handle** :
  `memless_execute(h, "BEGIN")` → Ok, `out_affected = 0` ; deux `UPDATE` → Ok ; `memless_query(h,
  "SELECT …")` **voit** les écritures non validées (read-your-writes) ; `memless_execute(h,
  "COMMIT")` → Ok ; le fichier temporaire réécrit **une fois**. **Abandon** : `BEGIN`, `UPDATE`,
  `ROLLBACK` → fichier inchangé, `memless_query` après revient à l'état d'avant. **Gardes** : deuxième
  `BEGIN` → `Refused` (`a transaction is already open`) ; `COMMIT` sans transaction → `Refused` (`no
  open transaction`). **Validation échouée** : un état final incohérent → `COMMIT` `Refused`,
  transaction fermée. Les écritures se font sur une **copie** de la fixture (jamais la partagée).
