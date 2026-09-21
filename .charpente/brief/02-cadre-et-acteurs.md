<!-- charpente-navigation -->
**Index** : [brief — Moteur de base de données en mémoire pilotée par YAML](00-index.md)  
**Précédent** : [1. Le projet en une page](01-le-produit-et-son-public.md)  
**Suivant** : [7. Les besoins, séparés des solutions](03-besoins-et-fonctionnalites.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 5. Le cadre imposé

### 5.1 Un seul moteur, trois langages qui s'y comportent à l'identique

C'est la contrainte fondatrice du projet, posée dès la demande de départ, et elle s'impose au produit
plutôt qu'elle n'en découle : le moteur central est unique, et JavaScript/TypeScript, Go et PHP
doivent tous les trois pouvoir l'utiliser nativement — par des adaptateurs, des ponts ou des
bibliothèques — plutôt que par trois réécritures séparées de la même logique.

Imagine trois traducteurs (un pour le JavaScript, un pour le Go, un pour le PHP) qui parlent tous les
trois pour la même personne : le moteur. Si chaque traducteur invente ses propres réponses au lieu de
répéter fidèlement ce que dit le moteur, les trois langages finissent par raconter des choses
différentes pour la même question. La règle : les trois traducteurs se contentent de traduire, jamais
de décider à la place du moteur.

**Ce que ça impose au produit.** Toute règle du métier — une **contrainte** de structure (une condition
que la donnée doit respecter, comme une serrure qui n'accepte qu'une clé de la bonne forme — devinée
ou non, la contrainte s'applique pareil une fois reconnue), un
refus, ce qui se passe dans une **transaction** (un ensemble de changements qui doivent tous réussir
ensemble ou pas du tout, comme un virement entre deux tirelires : soit l'argent change de main des
deux côtés, soit rien ne bouge — §8.5) — vit dans le moteur lui-même, jamais dans un des trois
traducteurs : sinon, un des trois langages finit par se comporter différemment des deux autres, sans
que personne ne l'ait décidé. Cela vaut aussi pour la lecture et l'écriture du fichier YAML lui-même,
et pour les trois gestes qui devinent sa structure (§8.1) : si chaque pont lisait le YAML avec sa
propre bibliothèque plutôt que de laisser le cœur commun s'en charger, deux langages pourraient
deviner deux structures différentes à partir du même fichier, sans qu'aucune requête SQL n'y soit pour
quelque chose. Ce que le texte SQL sait faire (§8) ne peut donc pas dépendre d'une bibliothèque propre
à un seul des trois écosystèmes.

**Ce qu'un brief ne tranche pas.** Quel langage de programmation et quelles bibliothèques servent à
écrire ce cœur commun, et par quelle voie chacun des trois langages l'appelle, sont des décisions
d'architecture, pas des décisions de produit : elles reviennent à `ARCHITECTURE.md` — informées si
besoin par une comparaison chiffrée entre options — et non à ce document. Ce que le brief exige tient
tout entier dans ce §5.1 et dans les fonctionnalités du §8 ; la manière de le réaliser se décide en
aval, où elle peut être comparée, mesurée et révisée sans toucher à ce que le produit promet.

**Une donnée pour la suite, pas une raison d'alléger l'exigence.** Sur les moteurs comparables
trouvés en recherche (DuckDB, libSQL/Turso, SurrealDB), le pont vers PHP est systématiquement le plus
fragile des trois — tertiaire, en préversion, ou limité à une connexion distante sans mode embarqué
(§18). Le risque que ça représente pour Memless est nommé en hypothèse risquée, §17.1.

### 5.2 Le choix technique appartient à l'architecture

Comment le cœur commun est construit — le langage qui l'écrit, le moteur SQL éventuellement réutilisé,
la voie par laquelle chacun des trois langages l'appelle, le format sous lequel les résultats
traversent la frontière — n'est pas tranché par ce brief : c'est une décision d'architecture
(`ARCHITECTURE.md`), pas une promesse de produit. Ainsi, ce document ne dit que ce que Memless doit
devenir, jamais comment le bâtir — et la manière de le bâtir peut être comparée, mesurée et révisée en
aval sans toucher à ce que le produit promet.

Une seule contrainte de produit encadre ce choix, et elle est déjà dite au §5.1 : le cœur reste
**unique**, les trois langages l'utilisent nativement sans réécrire sa logique (§8.8), et aucune règle
du métier — comparaison de deux valeurs, unicité d'`id`, relation devinée, transaction — ne vit dans un
pont plutôt que dans le cœur. En particulier, la règle unique de comparaison (§8.1, §15 décision 34) est
une exigence que le produit pose : si le moteur réutilisé ne l'applique pas de lui-même, c'est à Memless
de l'imposer — l'architecture dira comment.

### 5.3 Cadre légal

Memless est publié sous une licence permissive (§15, décision 1). Aucune autre contrainte légale ou
contractuelle ne s'impose au produit à ce stade : le projet n'a ni société, ni client, ni contrat en
cours.

---

## 6. Les acteurs

| Acteur | Ce qu'il est, ce qu'il peut faire, ce qu'il ne peut pas |
|---|---|
| L'auteure du fichier | la développeuse qui écrit et fait évoluer le fichier YAML — uniquement des données, pour son propre projet. Elle seule décide de la forme des tables, sans jamais la déclarer à part : ses noms de colonnes et ses valeurs suffisent, Memless devine le reste (§3.1, §8.1). |
| Le code appelant | le test automatisé ou l'application de démonstration qui charge le fichier, interroge et modifie l'état à travers Memless, dans l'un des trois langages. Il peut lire et écrire l'état par du texte SQL (§8.3, §8.4) ; il ne peut pas contourner une relation que Memless a devinée depuis les noms de colonnes (§13). |
| La mainteneuse du moteur | qui fait évoluer le cœur partagé et les trois ponts natifs. Elle peut changer le comportement du moteur pour les trois langages à la fois ; elle ne peut pas faire diverger un pont des deux autres sans que ce soit un défaut (§5.1, §10). |

Memless n'a pas d'acteur « utilisateur final » au sens d'une personne qui utiliserait le produit fini
sans être développeuse : le premier usage retenu (§2.1, §15 décision 2) place toujours une
développeuse entre Memless et quiconque verrait le résultat d'un test ou d'une démonstration.

### 6.1 Le vocabulaire qui fait autorité

Ces mots ont un sens unique dans tout le brief, et le code comme l'architecture s'y tiennent (§5.1, §10).

| Mot | Ce qu'il désigne |
|---|---|
| **table**, **ligne**, **colonne** | une clé de premier niveau du fichier, un élément de sa liste, un champ d'un élément. |
| **structure devinée** | ce que Memless déduit des données seules — type de chaque valeur, colonne `id`, relations (§8.1). Jamais appelée « schéma » : aucun schéma n'est déclaré. |
| **contrainte** | une condition que la structure devinée impose à l'état (un `id` par ligne, unique ; une relation qui pointe vers une ligne existante). |
| **relation devinée** | une colonne reconnue comme pointant vers une autre table par la convention de nom (§8.1). |
| **transaction** | un ensemble de changements tout-ou-rien. On **ouvre**, puis on **valide** ou on **abandonne** une transaction — jamais « commit » : ce mot est réservé à Git (§9.1). |
| **cœur**, **pont**, **instance** | le moteur unique et partagé ; l'accès natif d'un langage à ce cœur ; une copie en mémoire indépendante détenue par un appelant (§3.4, §12.2). |

**Deux mots à ne pas confondre, pour une transaction.** Une instruction **refusée** (texte SQL invalide,
table ou colonne absente en lecture, table absente en `UPDATE`/`DELETE`, hors du sous-ensemble
supporté) n'a jamais eu lieu : elle laisse la transaction ouverte et inchangée (§8.5, §15 décision 29). Une transaction **échoue** seulement à sa
validation — parce que son état final viole la structure devinée, ou parce que l'écriture sur le disque
est impossible — et se referme alors d'office sans rien laisser (§8.4, §8.6). « Refusée » qualifie une
instruction, « échoue » qualifie la validation d'une transaction.

**Termes exclus.** « commit » (sauf pour Git), « invariant », « ACID », « atomique » comme jargon, et
« librairie » : le brief dit respectivement « valider », « contrainte » / « structure devinée »,
« tout-ou-rien » / « écrit d'un coup », et « bibliothèque ». « ACID » en particulier promettrait une
isolation entre appels concurrents que le produit refuse justement de garantir (§12.2).
