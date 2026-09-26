<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§8. Données](data.md)  
**Suivant** : [§10. Sécurité](security.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §9. Dépendances externes

Memless ne dépend d'**aucun service** : ni base distante, ni API, ni file de messages, ni fournisseur
d'authentification. Ses seules dépendances sont des **bibliothèques liées à la compilation** et les
mécanismes de pont de chaque langage. Il n'y a donc pas de fallback réseau ni de timeout à prévoir :
une dépendance absente est un échec de compilation ou de chargement, pas une panne à l'exécution.

| Dépendance | Rôle | Intégration | Criticité |
|---|---|---|---|
| **`sqlparser`** (crate Rust, `=0.54.0`) | analyse syntaxiquement le texte SQL et l'abaisse vers `Statement` | liée à la compilation du cœur | **critique pour l'analyse** ; l'exécution (lire, trier, joindre, agréger, écrire) n'en dépend pas — elle est écrite dans `memless-domain`. Version épinglée, évolutions à suivre (§17 du brief) |
| **`serde-saphyr`** (`=1.3.0`) | lit le fichier, sérialise l'état à ordre stable | liée à la compilation du cœur | **critique** : porte la lecture et l'écriture. Retenu après recherche (`serde_yaml` archivé en mars 2024, `serde_yml` frappé par RUSTSEC-2025-0068) ; maintenu. Risques à porter : **mainteneur unique** (§12.2), et budget anti-« YAML bomb » **saturé**, non resserré (§10.5, §12.2) |
| **`koffi`** (paquet npm, ≥ 2.16) | expose le cœur à Node/TS par FFI dynamique | chargée à l'exécution du pont Node | **critique pour le pont Node** ; charge la même `cdylib` que Go et PHP à l'exécution, aucune compilation côté Node |
| **FFI de PHP** (extension intégrée) | permet à PHP d'appeler la bibliothèque native | activée côté PHP à l'exécution | critique pour le pont PHP ; **non activée par défaut sur certaines distributions Linux** (§17.1 du brief) — à documenter |
| **`purego`** (module Go, `v0.10.2`) | appelle la bibliothèque native sans cgo | liée à la compilation du pont Go | critique pour le pont Go |

**Le point de fragilité connu est le pont PHP** (§17.1 du brief) : sur les moteurs comparables, il est
systématiquement le plus faible. La parade retenue n'est pas un fallback à l'exécution mais une
**exigence de construction** — bâtir et vérifier ce pont avec le même soin que les deux autres dès le
premier jour, et le couvrir par le banc de parité (§11) sur chaque plateforme cible. La façon dont le
binaire natif parvient à chaque écosystème (PHP a besoin du chemin d'un `.so`, un module Go ne
transporte pas de binaire) est tranchée par la distribution sur GitHub Releases (§12.3, question 6).

Aucune de ces dépendances n'introduit de service à surveiller, de secret à stocker ou de quota à
gérer : la surface d'exploitation de Memless est vide au sens réseau (§7, observabilité « aucune »).
