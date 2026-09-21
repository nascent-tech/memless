---
type: glossaire
titre: Glossaire — le moteur
slug: le-moteur
cree_le: 2026-09-21T15:19:48+0000
mis_a_jour_le: 2026-09-21T16:00:07+0000
branche: main
statut: valide
---

**Sorte** : cœur de métier — décidé le 2026-09-21.

Prononcé au cadrage
[palier 1 — charger et refuser](../cadrage/2026-09-21-palier-1-charger-refuser.md) (point 5).

Contexte borné unique de Memless. Il porte ce qui distingue le produit d'un simple lecteur
YAML : deviner une structure depuis les seules données, reconnaître une relation par
convention de nom, imposer la règle de comparaison unique, refuser en nommant la règle.

## Termes

Termes propres au contexte, du plus court nom qui les désigne. Les définitions reprennent le
brief (§) et le cadrage du palier 1.

- **Base** — l'état deviné et cohérent d'un fichier accepté : toutes ses tables, une fois
  la structure devinée et les contraintes vérifiées. C'est ce qu'une **instance** tient en
  mémoire. Ne pas confondre avec la **structure devinée** (la forme, pas l'état) ni avec le
  **fichier** (la source de vérité sur le disque). Nom en code : `Base`.
- **Structure devinée** — la forme des tables (types, `id`, relations) déduite des seules
  données, jamais déclarée à part ; redérivée à chaque vérification (§10, décision 37).
- **Relation devinée** — une colonne dont le nom finit par `_id`, pointant vers la table au
  pluriel (ajout d'un `s`) du nom qui précède, si elle existe (décision 31).
- **Règle de comparaison** — deux valeurs de types différents ne sont jamais égales ni
  ordonnées (`"5"` ≠ `5`, `5` ≠ `5.0`) ; imposée par Memless, jamais héritée d'un moteur
  (décision 34, §5.2).
- **Contrainte** — une condition que la structure devinée impose à tout état accepté : chaque
  ligne un `id`, aucun `id` dupliqué, aucune relation cassée (remplace « invariant », exclu
  §6.1).
- **Scalaire** — une valeur d'un des quatre types reconnus : texte, entier, décimal,
  vrai/faux (§8.1). Une date sans guillemets est du texte.
- **Valeur imbriquée** — une valeur qui est elle-même une liste ou un ensemble de champs là où
  un scalaire est attendu ; refusée (§8.1, décision 35).
- **Chargement** — le geste unique par lequel une instance lit le fichier, devine sa
  structure, vérifie sa cohérence et démarre si tout tient — en entier ou pas du tout (§8.1,
  §8.2).
- **Refus** — l'issue d'un chargement où aucune ligne n'entre en mémoire et aucune instance ne
  démarre, avec une erreur nommant la règle, la table et la ligne (§10, décision 21).
- **Message de refus** — le texte d'un refus (règle, table, ligne), identique d'un langage à
  l'autre (décision 25) ; en anglais (règle de langage du projet).
- **Fichier irrecevable** — refusé avant toute vérification de cohérence (introuvable,
  illisible, vide, YAML invalide, sans table, forme invalide, valeur imbriquée, clé
  dupliquée) ; distinct d'une **structure incohérente** (forme correcte, données qui se
  contredisent) (§8.1, §8.2).
- **Résidu** — le fichier à part qu'une écriture a commencé et qu'un arrêt brutal a laissé
  sans substitution ; nom reconnaissable, jamais officiel, jamais touché par Memless (§8.6,
  décision 17). Le brief dit « fichier à part ».
- **Parité** — sur le même fichier, deux ponts produisent la même issue : même acceptation, ou
  même refus avec le même message (§10, décision 25). Un écart est un défaut.
- **Réécriture par substitution** — écrire d'abord un fichier à part, forcer son écriture sur
  le disque, puis le substituer d'un coup, de sorte qu'une coupure laisse l'ancien fichier
  intact ou le nouveau complet (décision 17 ; remplace « réécriture atomique », « atomique »
  exclu §6.1).
