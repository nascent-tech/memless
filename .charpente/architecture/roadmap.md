<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§11. Développement local](dev-environment.md)  
**Suivant** : [§13. Glossaire](glossary.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §12. Vision et décisions

### 12.1 Évolutions décidées (feuille de route)

Repoussées sciemment (§16 du brief), chacune conditionnée à un besoin réel constaté — pas encore
implémentées. La colonne de droite dit ce que chacune changerait dans **cette** architecture.

| Évolution (voir §16 du brief) | Ce que ça changerait ici |
|---|---|
| Serveur partagé entre processus/langages | un nouvel adaptateur entrant **réseau** et un état partagé — rompt le « chaque instance sa copie » (décision 6 du brief) |
| Cascade / mise à `NULL` automatique (en option) | une option sur le cas d'usage d'écriture et le validateur d'invariants (§8.7 du brief) |
| API « façon SQL » native par langage | une surface supplémentaire par pont, à tenir en parité |
| Verrou inter-processus sur un même fichier | un nouveau **port** de verrou de fichier et son adaptateur |
| Base utilisable en production | remet en cause les hypothèses de taille (§17.2 du brief) et la durabilité |
| Marque de version dans le format de fichier | un champ de version lu/écrit par l'adaptateur YAML |
| Écriture octet-pour-octet identique entre langages | une forme canonique partagée par les trois écrivains |
| Déclaration explicite d'une relation | une entrée de configuration lue par l'inférence (§17.4 du brief) |
| Inventaire de ce que Memless a deviné | un cas d'usage de lecture exposant la structure devinée |

### 12.2 Dettes

| Dette | Risque | Conséquence |
|---|---|---|
| Aucune coordination entre écritures concurrentes (décision 7 du brief) | **moyen** | deux écrivains simultanés : le dernier gagne en silence, le changement perdu ne laisse aucune trace, même dans Git (§9.1 du brief) ; un second chargement peut aussi nettoyer le temporaire d'une écriture en cours et faire échouer son `rename` (§5.3) |
| Protections de la frontière FFI (§10.5) : validation des entrées, propriété/libération des sorties, `catch_unwind`, verrou sur la table de handles | **élevé** | l'une manquante = corruption mémoire, fuite, double libération ou comportement indéfini — seul endroit hors de la garantie Rust |
| Robustesse du chargement face à un YAML hostile (§10.5) | **moyen** | un fichier « YAML bomb » ou démesuré peut épuiser la mémoire au chargement ; à borner par le choix de la bibliothèque YAML (§12.3, question 4) |
| Coût de la réécriture complète du fichier, non mesuré (§17.2 du brief) | **moyen** | si les fixtures grossissent, la réécriture peut devenir le geste le plus lent de la suite ; à trancher par le banc de mesure |
| Parité réelle du pont PHP sur toutes les plateformes (§17.1 du brief) | **moyen** | PHP pourrait rester citoyen de seconde zone (FFI non activé, comportement divergent) ; prouvé par le banc de parité multi-OS |
| Divergence possible entre la règle du domaine et GlueSQL après une mise à jour (§4.3) | faible | l'unicité d'`id` / les relations et les comparaisons SQL cessent de coïncider ; couvert par le test de coïncidence (§11.4) |
| Repli `cgo` non encore écrit si `purego` échoue sur une plateforme (§5.2 du brief) | faible | une plateforme mal supportée par `purego` reste sans pont Go tant que le repli n'est pas implémenté |
| `serde-saphyr` à mainteneur unique (§7) | faible | une dépendance critique repose sur une seule personne ; le repli `serde_yaml_ng` est identifié si le projet s'arrête |

### 12.3 Questions ouvertes

Deux d'entre elles (1 et 4) ont été **tranchées par recherche** ci-dessous ; les autres restent des
décisions à prendre, avec leurs options et le fait qui les tranche :

1. **Store GlueSQL — TRANCHÉ (recherche).** Réutiliser `gluesql-memory-storage::MemoryStorage`
   derrière un *newtype* Memless (voir §8.1) : ses champs sont publics, il conserve l'ordre des lignes,
   et un Store sur mesure n'apporterait rien de plus — GlueSQL 0.20 range de toute façon les colonnes
   d'une ligne dans un `BTreeMap` trié. *Sous-décision restante (1-bis)* : capter l'ordre des colonnes
   (que Memless porte hors de GlueSQL, §8.6) **par table ou par ligne**, et **quand** — au chargement +
   règle « colonne nouvelle en fin », ou en analysant le littéral de l'`INSERT` dans la garde SQL.
   *Tranché par* : la conception (`/charpente:design`).
2. **Format d'échange à la frontière (le contrat de parité).** Options : JSON partagé par les trois
   ponts (simple, un seul contrat à tester), ou format binaire tagué — une suite d'octets dont un
   premier champ dit le type de ce qui suit (plus rapide, plus de travail). *Recommandation* : un
   **contrat unique et partagé** — c'est ce qui rend la parité du §5.1 du brief vérifiable en un seul
   point. *Tranché par* : le banc de parité et une mesure du coût de sérialisation.
3. **Chemin de Node.** L'addon napi-rs dépend-il de `memless-core` en Rust, ou lie-t-il `memless-capi`
   par C ABI (liaison statique = un seul binaire Node, dynamique = addon + `cdylib` à empaqueter) ? Et
   expose-t-il des types riches ou colle-t-il au contrat partagé ? *Recommandation* : coller au
   contrat partagé (question 2), quitte à un pont Node moins idiomatique, pour ne pas ouvrir une
   seconde surface à vérifier — napi-rs **reste** le mécanisme Node (décision 12 du brief). *Tranché
   par* : la question 2 et une mesure de la taille des binaires.
4. **Bibliothèque YAML — TRANCHÉ (recherche).** `serde-saphyr` 1.3.0 avec `IndexMap` pour l'ordre des
   colonnes ; repli `serde_yaml_ng` 0.10.0 (voir §7). `serde_yaml` est archivé, `serde_yml` frappé par
   RUSTSEC-2025-0068. *Sous-décision restante (4-bis)* : resserrer au plan le `Budget` anti-« YAML
   bomb » de serde-saphyr, dont les défauts sont permissifs (profondeur ~8–16, alias/ancres à quelques
   dizaines si Memless n'en émet pas). *Tranché par* : `/charpente:plan`.
5. **CI/CD.** *Options* : une CI qui compile la bibliothèque native et rejoue le banc de parité sur
   une **matrice OS × famille de processeurs × langage**. *Fait tranchant* : la liste des cibles
   (question 7) et le service de CI retenu. *À choisir au plan.*
6. **Distribution du binaire natif.** Options : embarqué dans le paquet par plateforme (npm
   `optionalDependencies`, Composer avec binaires, Go `embed`) ou téléchargé après installation.
   *Fait tranchant* : taille des binaires et politique des registres — c'est le point où le pont PHP
   des projets comparables devient « tertiaire » (§17.1 du brief).
7. **Cibles à publier au lancement** (OS × famille de processeurs) et **étendue du sous-ensemble
   SQL** : détails de conception renvoyés à `/charpente:plan` par le brief (§5.2, §8.3 du brief).
8. **Snapshot/rollback de l'état en mémoire** pour une transaction : copie complète (simple,
   coûteuse) ou journal d'annulation (économe, plus complexe). *Tranché par* : le coût mesuré sur les
   tailles visées (banc de mesure).
