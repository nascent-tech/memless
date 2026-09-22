---
type: releve
titre: Banc de durée — §17.2
cree_le: 2026-09-22T00:30:00+0000
statut: vivant
---

# Banc de durée (§17.2)

Premier relevé du palier 3 « Écrire ». Mesure, pour un fichier `rows` de **k** lignes (`id` entier
1..k, colonne `val`), trois durées : un **chargement**, une **écriture isolée** (`UPDATE … WHERE
id = 1`, réécriture par substitution comprise) et une **suite de 100 écritures**. Percentiles
**p50/p95** sur **20 répétitions**. Reproduire par `bash harness/bench/run.sh` (bâtit et lance
`crates/memless-engine/examples/bench_write.rs`) ; recopier les nombres ici à la main (MVP §8).

## Relevé — 2026-09-22

Machine : poste de développement macOS (Darwin 27), profil `release`. Durées en **microsecondes**.

| k | load p50 | load p95 | write p50 | write p95 | suite100 p50 | suite100 p95 |
|---|---|---|---|---|---|---|
| 4 | 25 | 33 | 8 697 | 9 793 | 843 197 | 954 807 |
| 100 | 373 | 487 | 8 385 | 9 435 | 957 255 | 983 235 |
| 1 000 | 1 910 | 2 634 | 10 786 | 11 018 | 1 070 247 | 1 125 108 |
| 10 000 | 13 616 | 18 048 | 17 268 | 18 247 | 1 704 965 | 1 743 763 |

## Lecture

- **Le chargement est O(n)** : 25 µs à k=4, ~13,6 ms à k=10 000 (lecture + devinage de structure +
  redérivation des contraintes).
- **L'écriture isolée a un plancher ~8,4 ms** presque indépendant de k jusqu'à 1 000 lignes : c'est
  le coût du `fsync` (durabilité) de la réécriture par substitution, pas le travail O(n). La part
  O(n) (clone + rendu + octets) ne devient visible qu'à 10 000 lignes (~17 ms).
- **Une suite de 100 écritures** est ~0,84 s à 1,7 s, soit **~8 à 17 ms par écriture** — cohérent
  avec le plancher `fsync` par réécriture.

## Ce que ça informe (brief §4)

Le pari « la réécriture complète reste imperceptible » **tient** pour l'échelle du MVP : une
écriture isolée reste **sous ~20 ms** jusqu'à 10 000 lignes, dominée par la durabilité (`fsync`),
non par la taille. La réécriture O(n) n'est pas le facteur limitant à cette échelle. Une écriture en
rafale paie un `fsync` par appel ; un regroupement (hors MVP) serait le levier si le débit devenait
un objectif.
