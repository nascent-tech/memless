# 05 — La batterie, le lanceur, l'espace de travail

<!-- charpente-navigation -->
**Index** : [conception — Palier 1 — Charger et refuser](00-index.md)  
**Précédent** : [04 — Surface native et ponts](04-capi-et-ponts.md)  
**Suivant** : [06 — Décisions, non-créés, hors périmètre](06-decisions-et-hors-perimetre.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

La **batterie** est l'ensemble des fichiers que le palier doit charger ou refuser ; le
**lanceur** les rejoue depuis PHP **puis** Go et exige la **même issue et le même
message** ; le **relevé de parité** consigne le résultat. Le mot « banc » est réservé à la
mesure de durée (§17.2, paliers 3–4). Tout cela vit hors de Cargo (`harness/parity/`).

---

## `Cargo.toml` (racine) — le manifeste d'espace de travail

Manifeste **virtuel** (pas de paquet principal).

```toml
[workspace]
resolver = "2"
members = ["crates/memless-domain", "crates/memless-engine", "crates/memless-capi"]

[workspace.dependencies]
# versions partagées, figées au plan : la bibliothèque YAML (stack §7)
```

`memless-node` **n'est pas membre** au palier 1 (pont Node hors MVP, partie 6). Les lints
partagés (`workspace.lints`) et le profil de compilation se posent au scaffold — avec la
consigne de **ne pas** poser `panic = "abort"` pour `memless-capi` (partie 4).

**Ne fait pas** : ne déclare ni GlueSQL, ni pont Node, ni `indexmap`.

---

## `.charpente.json` (racine)

Sans lui, les gardes de frontière (`guard-boundaries`, `guard-coupling`) ne contrôlent pas
l'arbre que cette conception invoque (archi §6.1, §6.3). Il est **posé en PR1**, avec les
couches de cet arbre :

Le schéma du plugin attend `layers` comme un **tableau de groupes de noms** ordonnés par
rang (pas un objet chemin→couche), et le garde reconnaît une couche par un **segment de
chemin** portant son nom :

```json
{
  "architecture": "hexagonal",
  "layers": [["domain"], ["application"], ["interface"], ["infrastructure"]]
}
```

Ce que le garde voit réellement au palier 1 : le segment `application/` (dans
`memless-engine`) est reconnu ; les crates (`memless-domain`, `memless-capi`) et le dossier
`yaml/` ne portent pas de segment `domain`/`interface`/`infrastructure`, donc le garde
**intra-crate** ne les classe pas encore. Au palier 1, le sens des dépendances **entre
crates** est garanti par **Cargo** (`memless-domain` sans dépendance ne peut pas cycler) ;
la couverture intra-crate du garde s'affine au scaffold. C'est normalement le geste de
`/charpente:init` ; listé ici pour que PR1 ne l'oublie pas. Le modèle de branches
(work/prod/staging) et le choix des serveurs MCP restent à `/charpente:init` (partie 6).

---

## La batterie — `harness/parity/fixtures/`

Un fichier par cas, nommé en anglais (comme le reste du code) d'après le cas qu'il porte.
Le **fichier de départ** (`start.yaml`) est le seul valide (§8.1 augmenté d'un second
portefeuille : `users` `01H7B2`/`01H7B3`, `wallets` `w_123`→`01H7B3`, `w_124`→`01H7B2`,
montants entiers). La colonne « Cas » dit quelle règle le fichier éprouve — c'est l'identité
du fichier, pas une assertion de test (l'attendu vit dans les tests du build, la
correspondance cas → variante dans `refusal.rs`).

| Fixture | Cas |
|---|---|
| `start.yaml` | fichier de départ, valide |
| `start.yaml` + voisin `.start.yaml.memless-tmp` | résidu ignoré, intact après chargement |
| `empty.yaml` | A3 |
| `invalid-yaml.yaml` | A4 |
| `no-table.yaml` | B1 |
| `table-not-list.yaml` | B2 |
| `row-not-map.yaml` | B3 |
| `nested-value.yaml` | B4 |
| `dup-table-key.yaml` | B5 (D18) |
| `dup-column-key.yaml` | B6 (D18) |
| `non-text-key.yaml` | B7 (D19) |
| `missing-id.yaml` | C1 |
| `id-decimal.yaml` | C2 |
| `id-boolean.yaml` | C3 |
| `dup-id.yaml` | C5 |
| `broken-relation.yaml` | C6 |
| `self-relation.yaml` | C7 |
| `loads-mixed-id.yaml` | non-refus : `id: 5` et `id: "5"` = deux lignes |
| `loads-anchors.yaml` | non-refus : ancres/alias **en nombre borné** (prouve « aucun refus chiffré », D10, sans reproduire une bombe en CI) |

A1/A5 (chemin absent) et A2 (droits retirés) n'ont pas de fichier : ce sont des situations,
montées par les tests. Le nom réservé du résidu (`.start.yaml.memless-tmp`, forme proposée
par `system-design.md`) est une **convention à confirmer par l'architecture** (D11) ; au
palier 1 **aucun code n'en dépend** — le résidu n'est jamais détecté, seule la fixture
l'emploie (partie 6, non-créé N-3).

**Ne fait pas** : aucune fixture de requête, d'écriture ou de transaction (paliers 2–4).

---

## `harness/parity/run.sh` — le lanceur

Le geste du lanceur, sans corps :

- pour chaque fixture, exécuter le pont PHP, **libérer**, exécuter le pont Go, **libérer**
  (jamais deux instances vivantes sur le même fichier, cadrage §8) ;
- comparer les deux issues (accepté / refusé + texte du message) ; un écart est un défaut
  consigné au relevé de parité ;
- vérifier que le résidu voisin est **intact** après un chargement réussi (D16) ;
- sortir `0` si toutes les fixtures produisent la même issue des deux côtés, non-zéro sinon
  avec la liste des écarts.

**Ne fait pas** : ne prononce aucun refus (il **compare** ceux des ponts) ; ne teste qu'une
seule famille de systèmes × deux langages (§7) ; ne mesure aucune durée (paliers 3–4) ; ne
rejoue pas Node.

---

## `harness/parity/README.md`

Dit comment lancer le lanceur, ce qu'est le relevé de parité, et que **zéro écart** ouvre —
sans clore — l'hypothèse §17.1 (la parité réelle de PHP se tranche au palier 4, sur toutes
les familles de systèmes).
