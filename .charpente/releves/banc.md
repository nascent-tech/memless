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

# Banc de transaction — palier 4 « Transiger »

Deuxième relevé, ajouté au palier 4. Mesure, pour un fichier d'une ligne, deux durées : une
**transaction de k écritures validée** (`BEGIN`, k × `UPDATE`, `COMMIT` — **une seule** réécriture)
et **k écritures isolées** (k réécritures). Percentiles **p50/p95** sur **20 répétitions**.
Reproduire par `bash harness/bench/run.sh` (bâtit et lance aussi
`crates/memless-engine/examples/bench_transaction.rs`) ; recopier les nombres ici à la main.

## Relevé — 2026-09-22

Machine : poste de développement macOS (Darwin 27), profil `release`. Durées en **microsecondes**.

| k | transaction p50 | transaction p95 | k isolées p50 | k isolées p95 |
|---|---|---|---|---|
| 1 | 9 851 | 12 383 | 9 969 | 11 826 |
| 10 | 10 204 | 12 360 | 103 834 | 108 368 |
| 100 | 11 275 | 13 400 | 1 003 183 | 1 040 383 |
| 1 000 | 15 360 | 18 140 | 9 882 698 | 10 356 298 |

## Lecture

- **La transaction paie un seul `fsync`, quel que soit k** : ~9,9 ms à k=1, ~15,4 ms à k=1 000. La
  légère montée est le travail O(k) en mémoire (appliquer k écritures sur l'état de travail) plus
  **une** réécrite finale — pas k réécritures.
- **k écritures isolées paient k `fsync`** : la durée croît linéairement, de ~10 ms (k=1) à ~9,88 s
  (k=1 000, soit ~9,9 ms par écriture, le plancher `fsync` du premier relevé).
- **L'écart est le nombre de `fsync` économisés** : à k=1 égalité (une écriture = une transaction
  d'une instruction) ; à k=10 ~**10×** ; à k=100 ~**89×** ; à k=1 000 ~**643×**.

## Ce que ça informe (brief §4)

Le regroupement annoncé au premier relevé comme « le levier si le débit devenait un objectif » est
**mesuré** : une transaction transforme k `fsync` en un seul, sans changer la garantie de durabilité
(la réécrite finale reste atomique par substitution). Le tout-ou-rien du palier 4 n'est donc pas
qu'une sémantique — c'est aussi le seul chemin vers un débit d'écriture élevé sur ce moteur, où la
durabilité, non la taille, domine le coût.
