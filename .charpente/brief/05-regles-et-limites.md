<!-- charpente-navigation -->
**Index** : [brief — Moteur de base de données en mémoire pilotée par YAML](00-index.md)  
**Précédent** : [9. Les fonctionnalités décisives](04-fonctionnalite-decisive.md)  
**Suivant** : [13. Ce qui est interdit](06-interdits-et-hors-perimetre.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 10. Les règles transverses

Ce qui vaut partout, et qu'aucune fonctionnalité du §8 ne redit.

- **Toute donnée qui entre dans l'état — au chargement (§8.2) ou par une écriture (§8.4) — respecte
  la structure devinée (§8.1).** Une violation refuse l'entrée entière ; il n'existe pas d'entrée
  « partielle ».
- **La structure devinée n'est jamais figée à un instant : elle se redérive, depuis les mêmes règles
  (§8.1), à chaque fois qu'un état doit être vérifié** — au chargement, à la validation d'une
  transaction (contre l'état que cette transaction propose, jamais un état abandonné ou dont la
  validation a échoué), à un rechargement (§8.9). Une transaction qui rendrait l'état invalide une fois
  la structure ainsi redérivée échoue à sa validation, exactement comme toute autre violation (§8.4) : il est donc impossible
  d'écrire, par une transaction acceptée, un fichier que Memless refuserait ensuite de charger.
  **Une table existe tant que son nom porte une clé dans le fichier — au chargement comme après un
  rechargement, sans dépendre d'un historique** — y compris une table apparue puis vidée à l'intérieur
  de la transaction qui vient de se valider : elle garde sa place exactement comme une table plus
  ancienne réduite à une liste vide, puisque rien ici ne distingue une table selon la date à laquelle
  elle a commencé d'exister, et toute colonne qui la référence reste reconnue comme une relation.
  **Une colonne, elle, n'existe que tant qu'au moins une ligne de sa table la porte** : elle disparaît
  de ce qui est deviné dès que plus aucune ligne ne la renseigne, sans garder de trace de ce qu'elle a
  été.
- **Une transaction est tout ou rien.** À sa validation, soit son état final entre entièrement dans le
  fichier, soit rien n'entre. Une instruction refusée en cours de route n'a jamais eu lieu et laisse la
  transaction ouverte (§8.5, §15 décision 29) ; c'est la validation qui peut échouer — état final
  invalide, ou écriture disque impossible — et referme alors la transaction sans rien laisser au fichier
  (§8.6). Une écriture isolée, non regroupée explicitement avec d'autres, est elle-même une transaction
  d'une seule instruction : §8.4 et §8.5 ne sont pas deux mécanismes séparés, le second regroupe
  simplement plusieurs occurrences du premier.
- **Le fichier sur le disque n'est jamais lisible, ni laissé, dans un état intermédiaire.** Memless
  écrit d'abord à part, puis substitue d'un coup (§8.6) : un programme qui lirait le fichier pendant
  l'écriture voit soit l'état d'avant, soit l'état d'après, jamais un mélange des deux ; et une
  interruption pendant l'écriture — coupure, plantage — laisse le fichier d'origine intact, jamais à
  moitié écrit.
- **Les trois langages exécutent le même cœur.** Une même requête, sur le même état, rend le même
  résultat et, en cas de refus, le même message d'erreur (règle, table, ligne), quel que soit le langage
  appelant — §5.1, §15 décision 25. Un écart de comportement entre deux langages est un défaut, jamais
  une variante ; puisqu'un seul cœur partagé écrit le fichier, les trois produisent aussi le même
  fichier à l'octet près pour un même état — §8.8.
- **Toute erreur nomme la règle précise qu'elle a empêché de violer** — quelle contrainte, quelle
  table, quelle ligne — jamais un message générique qui obligerait à deviner.
- **Le fichier YAML réécrit conserve l'ordre des tables, des lignes et des colonnes déjà présentes sur
  une ligne, d'une écriture à l'autre**, pour rester lisible en diff — §9.1. Un seul cœur partagé
  écrivant le fichier, cette stabilité vaut aussi bien d'une écriture à la suivante que d'un langage à
  l'autre : le même état produit le même fichier à l'octet près — §8.8, §15 décision 25.

---

## 12. Les limites, et où vivent les chiffres

### 12.1 Ce que la loi ou le contrat fixe

Aucune. Memless est publié sous une licence permissive (§15, décision 1) ; le seul engagement qui
s'impose est celui, standard, de toute publication open source : fourni sans garantie, sans
obligation de maintenance.

### 12.2 Ce que le produit décide

- Rien n'empêche, ni ne coordonne, plusieurs acteurs qui toucheraient le même fichier en même temps
  au lancement — deux processus, deux instances du moteur dans un même processus, ou une personne qui
  édite le fichier à la main pendant qu'un moteur l'a déjà chargé. La même absence de coordination vaut
  à l'intérieur d'une seule instance appelée en même temps par deux exécutions concurrentes du code
  appelant (deux fils, deux tâches) : sérialiser ces appels est la responsabilité de qui appelle
  Memless, pas une garantie du moteur. Cette absence de coordination porte sur les **données** — l'ordre
  et le résultat de deux accès concurrents ne sont pas garantis ; la **sûreté mémoire** du processus,
  elle, tient : le moteur ne corrompt jamais sa propre mémoire ni ne plante parce que deux fils
  l'appellent (§15, décision 7). Chaque instance garde sa propre copie en mémoire (§3.4) ; ce que
  produit concrètement cette absence de coordination est nommé en §14.
- Le texte SQL est la seule façon d'interroger ou de modifier l'état au lancement — §8.3, §8.4, §14.
- Casser une relation devinée — par suppression ou par modification — est refusé ; ni suppression en
  cascade, ni mise à `NULL` automatique, au lancement — §8.7, §14.
- Aucune colonne n'est jamais obligatoire par construction, à l'exception de `id` : sans schéma
  déclaré, rien n'empêche une ligne d'omettre tout autre champ que d'autres lignes de la même table
  renseignent — §15, décision 32.

### 12.3 Ce qu'on ne connaît pas encore

Aucune valeur chiffrée n'est nécessaire au fonctionnement du produit à ce stade : ni plafond, ni
délai, ni prix ne conditionne une autorisation ou un refus — Memless n'a donc pas de mesure en
attente au sens du mécanisme habituel. Ce qui reste réellement incertain — la taille de fixtures et
le nombre d'écritures à partir desquels la réécriture complète du fichier ralentit de façon
perceptible — ne bloque aucune fonctionnalité : c'est une hypothèse risquée, pas une mesure, et elle
vit au §17.2.
