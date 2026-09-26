<!-- charpente-navigation -->
**Index** : [brief — Moteur de base de données en mémoire pilotée par YAML](00-index.md)  
**Précédent** : [5. Le cadre imposé](02-cadre-et-acteurs.md)  
**Suivant** : [9. Les fonctionnalités décisives](04-fonctionnalite-decisive.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## 7. Les besoins, séparés des solutions

| Formulé en solution | Le besoin dessous | Ce que ça change |
|---|---|---|
| « un fichier YAML qui EST la base de données » | ne plus avoir à garder d'accord un schéma, un script de données et des fixtures de test qui vivent dans trois fichiers séparés | ce n'est pas un format d'export de plus : c'est la seule source de vérité — il n'existe nulle part une deuxième version des données à tenir synchronisée |
| « éviter d'avoir un schéma dans le fichier, et que l'outil lise directement les choses » | ne pas payer, dès la première ligne écrite, le coût de décrire une forme avant de pouvoir écrire une donnée | ce n'est pas une simplification cosmétique : ce qui est d'habitude le plus long à déclarer — les relations entre tables — est deviné aussi, pas seulement le type d'une valeur (§3.1) |
| « interroger façon SQL » | écrire des scénarios de test réalistes — filtrer, croiser des tables selon leurs relations — sans écrire à la main du code d'accès aux données, et sans que ce code diffère d'un langage à l'autre | le texte de la requête devient portable : la même requête, collée dans un test JavaScript et un test Go, interroge exactement la même chose (§5.1) |
| « manipuler directement l'état » | pouvoir tester une vraie règle métier (retirer de l'argent, changer un rôle) sans installer, démarrer ni réinitialiser une vraie base de données entre deux tests | la modification n'est pas un brouillon oublié à la fin du test : elle est réécrite dans le fichier. Remettre les données à zéro n'est donc pas automatique — c'est la responsabilité de qui appelle Memless, avec un geste que Memless offre pour la part qu'il peut tenir (recharger sa mémoire depuis le fichier, §8.9) et un geste externe pour restaurer le fichier lui-même (§15, décision 4) |
| « utilisable nativement en JS/TS, Go et PHP » | qu'une équipe qui travaille dans plusieurs langages sur des services différents n'ait pas à reconstruire ce mécanisme trois fois, avec le risque que les trois versions se comportent différemment | un seul cœur fait autorité ; un pont qui se comporterait différemment des deux autres est un défaut, jamais une variante acceptée (§5.1) |

Un besoin ne figure dans aucune de ces formulations, et c'est lui qui rend possible, avec ce que le
moteur fait déjà, la fonctionnalité décisive du §9 : une équipe qui doit comprendre pourquoi un test
s'est mis à échouer après qu'une autre personne a changé les fixtures n'a aujourd'hui **aucun
historique lisible de ce changement** — un fichier binaire, ou une vraie base de données, ne se lit
pas dans une revue de code. Transformer chaque changement de donnée en une ligne d'historique Git
répond à un besoin que personne, dans la demande de départ, n'avait nommé.

---

## 8. Le produit, fonctionnalité par fonctionnalité

**Neuf fonctionnalités, et rien d'autre au lancement.**

### 8.1 Un seul fichier, uniquement des données — Memless devine le reste

**Ce que c'est.** Le fichier YAML ne contient que des tables et leurs lignes : chaque table est une
clé de premier niveau, sa valeur une liste de lignes. Aucune section à part ne décrit leur forme —
Memless la devine lui-même, comme on reconnaît le contenu d'une boîte non étiquetée en la regardant,
en trois gestes, repris à chaque fois qu'un état doit être vérifié plutôt que figés une fois pour
toutes (§10) :

- **le type** de chaque valeur se lit dans son écriture YAML : un mot ou une phrase est du texte, un
  nombre sans virgule est un nombre entier, un nombre à virgule est un nombre décimal (approché, pas
  garanti exact — §15, décision 20), `true` ou `false` est un vrai/faux — et toute autre valeur qui
  n'est ni l'un de ces nombres ni un vrai/faux est du texte, y compris une date écrite sans guillemets
  (`2024-01-01`) : Memless ne reconnaît aucun type dédié aux dates. Une valeur explicitement vide
  (`null`) compte comme absente (§15, décision 35) ; une valeur qui est elle-même une liste ou un
  ensemble de champs n'est reconnue dans aucun de ces types, et fait échouer le chargement (§13). Le
  type se lit valeur par valeur : deux lignes d'une même colonne peuvent porter des types différents ;
- **la clé primaire** — la colonne qui identifie chaque ligne sans ambiguïté, comme un numéro de
  sécurité sociale identifie une personne — est toujours la colonne appelée `id`, seule colonne requise
  sur chaque ligne, et dont la valeur est un texte ou un nombre entier (§15, décision 30) ;
- **une clé étrangère** — une colonne qui pointe vers la ligne d'une autre table — est devinée par son
  nom : une colonne se terminant par `_id` pointe vers la table portant, au pluriel, le nom qui
  précède ce suffixe — `user_id` pointe vers `users`, `order_item_id` pointe vers `order_items` — si
  cette table existe ; sinon, c'est une colonne ordinaire, pas une relation. Le pluriel s'obtient en
  ajoutant simplement la lettre `s` — littéralement, pas selon les règles de la grammaire anglaise :
  une table dont le pluriel est irrégulier (comme « people ») ne sera jamais reconnue par une colonne
  `person_id` — §17.3. Rien n'empêche une relation de pointer vers sa propre table, comme une colonne
  `user_id` dans la table `users` elle-même.

**Une seule règle gouverne toute comparaison entre deux valeurs, dans tout le moteur** — pour
retrouver une relation, vérifier l'unicité d'un `id`, ou comparer dans une requête : **deux valeurs de
types différents ne sont jamais égales**, quelle que soit leur ressemblance (`"5"` et `5` ne sont
jamais la même valeur), et ne s'ordonnent pas non plus l'une par rapport à l'autre — comparer un texte
à un nombre ne vaut jamais vrai (§15, décision 34). C'est cette règle, et elle seule, qui décide
si `users: [{id: 5}, {id: "5"}]` déclare une seule ligne ou deux : deux, puisque `5` et `"5"` sont de
types différents, donc jamais égaux. Le nombre entier et le nombre décimal sont eux aussi deux types
distincts : `5` et `5.0` ne sont jamais égaux, `id: 5` et `id: 5.0` déclarent deux lignes, et une
comparaison entre deux types différents vaut toujours faux — jamais une erreur. Cette règle est une
exigence du produit : si un moteur réutilisé pour bâtir Memless compare autrement, le cas échéant,
c'est à Memless de l'imposer (§5.2).

La même règle s'étend à l'absence : une colonne qu'une ligne ne porte pas (§15, décision 32) ne
contient aucune valeur à comparer, donc une condition qui la compare à quoi que ce soit —
`WHERE role = 'ADMIN'`, `WHERE role <> 'ADMIN'` — ne vaut jamais vrai pour cette ligne, qui sort du
résultat sans erreur ; seul `IS NULL` / `IS NOT NULL` reconnaît cette absence (§15, décision 35). Une
somme ou un compte sur cette colonne ignore les lignes qui ne la portent pas, exactement comme SQL
ignore une valeur absente dans ces mêmes opérations.

Cette même règle explique un piège fréquent en écriture manuelle : si `users` porte `id: "5"` (entre
guillemets, donc du texte) et que `wallets` porte `user_id: 5` (sans guillemets, donc un nombre), la
relation devinée entre les deux tables ne reconnaîtra jamais cette ligne comme la bonne — les deux
valeurs sont de types différents, donc jamais égales (§15, décision 34) — et le chargement échouera
avec une relation qui semble pointer vers une ligne absente (§8.2), alors que la ligne existe bel et
bien, juste écrite avec un autre type.

Par exemple, ce fichier suffit — rien d'autre à écrire :

```yaml
users:
  - id: "01H7B2"
    email: "admin@kweek.com"
    role: "ADMIN"
  - id: "01H7B3"
    email: "client@kweek.com"
    role: "USER"

wallets:
  - id: "w_123"
    user_id: "01H7B3"
    balance: 5000
```

Memless y reconnaît deux tables, leurs colonnes et leurs types, l'identifiant de chacune (`id`), et
que `wallets.user_id` pointe vers `users` — sans qu'aucune de ces informations n'ait été déclarée à
part.

**Ce qu'il faut.** Un chemin de fichier lisible, qui existe, et dont le contenu est un YAML valide
déclarant au moins une table. Démarrer un fichier totalement neuf demande donc d'y écrire à la main
au moins une table, même vide — par exemple `users: []` — avant le tout premier chargement : Memless
devine une structure depuis des données, il n'en invente pas depuis rien.

**Ce qui est refusé.** Un chemin qui ne désigne aucun fichier, un fichier qu'on n'a pas le droit de
lire, un fichier vide, un YAML syntaxiquement invalide, un YAML valide qui ne déclare aucune table, une
clé de premier niveau dont la valeur n'est pas une liste de lignes — ou dont un élément n'est pas
lui-même un ensemble de champs —, une valeur imbriquée là où un scalaire est attendu, une clé de table
ou de colonne qui n'est pas un texte (un nombre, un vrai/faux), et une clé de
table ou de colonne écrite deux fois : chacun est refusé avec une erreur qui dit laquelle de ces
raisons s'applique (§15, décision 21). Laisser la dernière valeur d'une clé dupliquée l'emporter en
silence serait deviner ce qui était voulu — un fichier n'est jamais chargé à moitié (§13). Une table
existe dès que son nom porte une clé dans le fichier, même si sa liste de lignes est vide (§10) : c'est
cette seule présence, jamais un historique, qui compte pour ce refus comme pour reconnaître une
relation. Le dialecte YAML exact — quelle version, et comment se lisent les écritures scalaires
limites — est un détail de conception, comme l'étendue du sous-ensemble SQL (§8.3) : il est fixé par
l'architecture, pas par ce brief.

### 8.2 Le chargement vérifie la structure devinée et refuse un fichier incohérent

**Ce que c'est.** Au chargement, Memless vérifie que ce qu'il a deviné (§8.1) est cohérent, ligne par
ligne : chaque ligne porte une colonne `id`, deux lignes d'une même table n'ont jamais le même `id`
(§15, décision 34), et chaque colonne reconnue comme une relation pointe vers une ligne qui existe
réellement dans la table visée. Une table sans aucune ligne satisfait ces trois conditions sans
exception à vérifier : rien n'y viole rien.

**Ce qu'il faut.** Des données dont la forme, une fois devinée, est cohérente avec elle-même.

**Ce qui est refusé.** Une ligne sans colonne `id`, un `id` qui n'est ni un texte ni un nombre entier
(§15, décision 30), deux lignes de même `id` dans une même table, ou une relation devinée qui pointe
vers une ligne absente, font chacune échouer le chargement pour le fichier entier ; aucune ligne n'entre en mémoire, même seule, et aucun état partiellement valide n'est
chargé (§13).

### 8.3 Interroger l'état en SQL

**Ce que c'est.** Lire l'état chargé avec du texte SQL : filtrer, trier, croiser des tables selon
leurs relations devinées, agréger (compter, sommer).

**Ce qu'il faut.** Une requête de lecture, exprimée en SQL, contre un état déjà chargé.

**Ce qui est refusé.** Un texte qui ne s'analyse pas comme du SQL valide, une requête qui désigne une
table ou une colonne absente de ce qui a été deviné, et une requête par ailleurs valide mais qui
dépasse ce que ce sous-ensemble de SQL sait exécuter, sont chacune refusées avec une erreur qui le
dit — jamais exécutées à moitié, jamais ignorées en silence. Trier une colonne dont deux lignes du
résultat portent des types différents est refusé — deux types ne s'ordonnent jamais (§15, décision 34) ;
une ligne qui ne porte pas la colonne triée se place en dernier. L'étendue exacte de ce sous-ensemble est
un détail de conception, pas de ce document. Un nom de table ou de colonne lu depuis le fichier n'est,
lui, jamais contraint en amont : il vient tel quel de la clé YAML qui l'a introduit, espace ou mot
réservé SQL compris. Dans l'autre sens — un nom introduit par une écriture SQL (§8.4) plutôt que lu
depuis le fichier — c'est le texte SQL qui contraint ce qui est possible, une contrainte plus stricte
que celle d'une clé YAML : un nom ainsi créé se réécrit donc toujours dans le fichier sous une forme
qu'un rechargement relit à l'identique (§10). La façon de désigner un nom hors norme dans une requête
(avec ou sans guillemets) relève du même détail de conception que l'étendue du sous-ensemble SQL.

### 8.4 Modifier l'état en SQL

**Ce que c'est.** Ajouter, changer ou retirer des lignes avec du texte SQL, en respectant la structure
que Memless devine (§8.1) — redérivée depuis l'état complet à chaque vérification, jamais figée depuis
le chargement (§10). Écrire dans une table qui n'existe pas encore la crée ; donner à une ligne une
colonne qu'aucune autre ligne de sa table ne porte l'ajoute pour cette ligne-là — il n'existe pas de
geste séparé pour « créer » une table ou une colonne, puisqu'aucune n'est déclarée à l'avance : une
table ou une colonne existe dès qu'une ligne l'utilise. Une table existe tant que son nom porte une
clé dans le fichier, même réduite à une liste vide (§10) ; une colonne, elle, n'existe que tant qu'au
moins une ligne de sa table la porte encore : la mettre à `NULL` par une écriture SQL
(`SET colonne = NULL`) la rend absente pour cette ligne — un `NULL` est indiscernable d'une colonne
absente (§15, décision 35) —, et retirer sa dernière valeur, ainsi ou en supprimant la dernière ligne
qui la portait, la fait disparaître de ce que Memless a deviné, exactement comme au chargement.
Modifier ou supprimer des lignes d'une table qui n'existe pas est refusé, avec une erreur qui nomme la
table absente — comme une lecture qui la désignerait (§8.3), et pour la même raison : une faute de
frappe sur un nom de table ne doit jamais passer pour un changement sans effet. Seul un `INSERT` fait
apparaître une table jusque-là absente. Faire disparaître entièrement une table existante — retirer sa clé du fichier — n'existe pas
non plus par une instruction SQL, pour la même raison que `CREATE TABLE` et `ALTER TABLE` n'existent
pas (§15, décision 24) : aucune forme n'étant déclarée à l'avance, aucune instruction ne peut la
« supprimer ». Seule une édition à la main du fichier, suivie d'un rechargement (§8.9), fait
disparaître une table — jamais une transaction validée.

**Ce qu'il faut.** Une instruction d'écriture SQL contre un état déjà chargé.

**Ce qui est refusé.** Une transaction n'est jamais validée si son état final viole la structure
redérivée — deux lignes de même `id` dans une même table, une ligne sans `id`, un `id` qui n'est ni un
texte ni un nombre entier (§15, décision 30), ou une relation cassée
(§8.7), **quelle qu'en soit la cause** : supprimer ou modifier la ligne visée par une relation, mais
aussi faire apparaître, dans la même transaction, la table qui transforme une colonne jusque-là
ordinaire en relation dont la valeur ne correspond à aucune ligne réelle — l'erreur nomme alors les
deux tables en cause. Rien n'empêche un état intermédiaire, à l'intérieur d'une transaction pas encore
validée, de sembler transitoirement invalide — comme l'instant où l'argent a quitté la première
tirelire sans être encore arrivé dans la seconde (§5.1) : ça ne compte pas comme perdu, tant que la
seconde moitié du geste suit avant la fin. Seul l'état au moment de la validation compte (§8.5), et
une transaction abandonnée, ou dont la validation échoue, ne compte pour rien : aucune table ni colonne
qu'elle proposait n'entre dans ce que Memless a deviné. Une écriture isolée est sa propre transaction (§10) : état
intermédiaire et état final s'y confondent, la contrainte est donc vérifiée avant de rendre la main.

### 8.5 Regrouper plusieurs écritures en une transaction tout ou rien

**Ce que c'est.** Plusieurs instructions d'écriture peuvent être regroupées : soit toutes réussissent
ensemble, soit aucune n'a lieu — c'est à la validation que ce tout-ou-rien se joue, comme un virement
entre deux tirelires (§5.1). À l'intérieur d'une transaction ouverte, une lecture voit les écritures déjà faites dans
cette même transaction, même si rien n'est encore validé. Une transaction peut aussi être abandonnée
volontairement, sans qu'aucune instruction n'ait été refusée — pour un scénario qu'on veut observer
puis ne pas garder. Ouvrir, valider et abandonner s'écrivent en texte SQL, avec les trois mots du
standard — `BEGIN`, `COMMIT`, `ROLLBACK` — passés au même geste d'écriture que toute instruction
(§12.2) ; chaque pont peut les envelopper dans trois appels du même nom.

**Ce qu'il faut.** Un scénario métier qui a besoin que plusieurs changements réussissent ensemble —
par exemple, retirer d'un portefeuille et ajouter à un autre (§7).

**Ce qui est refusé.** Une transaction abandonnée volontairement, ou dont la validation échoue, ne
laisse trace d'aucune de ses étapes, y compris celles qui avaient déjà réussi (§10, §13) —
seul l'état final d'une transaction validée doit respecter la structure devinée (§8.4) : deux lignes
qui se référencent l'une l'autre peuvent ainsi être supprimées ensemble dans la même transaction,
alors qu'aucun ordre ne le permettrait instruction par instruction (§15, décision 15). Une instance du
moteur ne peut avoir qu'une seule transaction ouverte à la fois — la même instance dont §12.2 dit
qu'elle garde sa propre copie en mémoire : en ouvrir une deuxième avant d'avoir validé ou abandonné la
première est refusé. Valider ou abandonner alors qu'aucune transaction n'est ouverte est également
refusé, avec une erreur qui le dit — ni l'un ni l'autre n'est un geste silencieux sans effet.

Une instruction refusée à l'intérieur d'une transaction ouverte — texte SQL invalide, table ou colonne
absente en lecture, table absente en `UPDATE`/`DELETE`, hors du sous-ensemble supporté (§8.3, §8.4) —
ne referme pas la transaction : elle reste ouverte,
inchangée par la tentative refusée, et le code appelant peut essayer une autre instruction, ou la
valider, ou l'abandonner. Seul l'état final au moment de la validation compte (§8.4) — une instruction
refusée n'en fait jamais partie, puisqu'elle n'a jamais eu lieu.

Mettre fin à une instance elle-même — au sens de libérer les ressources qu'elle occupe une fois
qu'on n'en a plus besoin — n'est pas un dixième geste du produit : la surface native expose une
libération que chaque langage appelant enveloppe dans son mécanisme habituel de fin de vie d'un objet
(ramasse-miettes, ou un appel explicite de fermeture), sans que ce soit une fonctionnalité de plus au
sens du §8. Libérer une instance qui a une transaction ouverte l'abandonne : rien n'atteint le disque.

### 8.6 Toute transaction validée est aussitôt réécrite dans le fichier YAML

**Ce que c'est.** Dès qu'une transaction est validée et qu'elle a réellement changé l'état, Memless
réécrit le fichier YAML d'origine pour qu'il reflète le nouvel état — en écrivant d'abord un fichier
à part, puis en le substituant d'un coup à l'ancien, jamais en écrivant par-dessus morceau par
morceau. Une coupure ou un plantage pendant l'écriture laisse donc toujours soit l'ancien fichier
intact, soit le nouveau complet — jamais un mélange des deux. Une transaction qui échoue avant la
substitution supprime elle-même le fichier à part qu'elle avait commencé ; seul un arrêt brutal du
processus entre les deux peut laisser ce fichier à part derrière lui, sans jamais l'avoir rendu
officiel — le chargement suivant l'ignore ; l'écriture suivante le remplace, sans jamais le confondre
avec le fichier réel ; ce fichier à part porte un nom reconnaissable, pour que l'équipe puisse l'ignorer côté Git (§15,
décision 17). Une transaction validée qui ne change rien à l'état final — même si des instructions ont été exécutées en cours de route — ne
déclenche aucune réécriture.

**Ce qu'il faut.** Un chemin de fichier accessible en écriture.

**Ce qui est refusé.** Une transaction non validée, ou interrompue en cours de route, n'atteint jamais
le disque, même partiellement (§10, §13). Si l'écriture sur le disque échoue — chemin devenu
inaccessible, espace insuffisant — la transaction entière échoue : la
mémoire ne garde jamais un changement que le disque n'a pas reçu, pour que la promesse du §1 (le
fichier est toujours la vérité) reste vraie même dans ce cas.

Remettre l'état à son point de départ n'est pas une fonctionnalité de Memless : c'est la
responsabilité de qui l'appelle, en deux gestes distincts, dans cet ordre. D'abord, restaurer le
contenu du fichier lui-même — par exemple depuis un historique de versions — est externe à Memless.
Ensuite, aligner la mémoire sur ce contenu restauré se fait par Memless, à la demande — §8.9 ; tant
que ce second geste n'a pas eu lieu, toute transaction validée entre-temps réécrirait le fichier
restauré avec l'ancien état encore en mémoire, effaçant la restauration — c'est pourquoi les deux
gestes se suivent sans qu'une transaction s'intercale. Les deux gestes sont nécessaires entre deux
exécutions complètes d'une suite ; ni l'un ni l'autre ne l'est entre deux tests d'une même suite qui
n'ont pas besoin de revenir au fichier d'origine (§15, décision 4).

### 8.7 Casser une relation devinée est refusé

**Ce que c'est.** Une relation devinée (§8.1) reste vraie à la fin de toute transaction validée :
aucune ligne ne pointe vers une ligne qui n'existe pas, quelle que soit la façon dont ce vide serait
apparu — en supprimant ou en modifiant la ligne visée, comme un livre de bibliothèque qu'on ne peut
pas laisser définitivement manquant tant que quelqu'un l'a encore emprunté ; ou en insérant une ligne
dont la relation ne correspond à rien, tel un emprunt enregistré pour un livre qui n'existe pas ; ou
en faisant apparaître, dans la même transaction, la table qui transforme une colonne jusque-là
ordinaire en relation orpheline. À l'intérieur d'une même transaction, un ordre peut en remplacer un
autre — supprimer d'abord l'emprunt, puis le livre — tant que plus rien ne pointe vers le vide une
fois la transaction validée (§8.5).

**Ce qu'il faut.** Aucune ligne ne doit référencer la ligne visée au moment où la transaction se
valide.

**Ce qui est refusé.** La suppression en cascade des lignes qui référencent la ligne visée, et la
mise à `NULL` automatique de leur référence, n'existent pas au lancement — §14.

### 8.8 Le même moteur, utilisable nativement en JavaScript/TypeScript, Go et PHP

**Ce que c'est.** Le cœur unique de Memless (§5.1) s'utilise depuis chacun de ces trois langages par
un adaptateur, un pont ou une bibliothèque native — jamais par une réécriture indépendante de la
logique. Chaque pont rend les résultats dans la forme naturelle de son langage (objet, tableau, table
associative) ; les **valeurs**, elles, sont identiques d'un langage à l'autre — « brut » (§3.4)
qualifie l'absence de modèle-classe, pas l'absence de type natif. Un nombre entier accepté par
Memless garde sa valeur exacte dans les trois langages, même au-delà de ce qu'un langage représente
nativement sans perte.

**Ce qu'il faut.** Le langage appelant, avec le pont correspondant installé.

**Ce qui est refusé.** Un pont dont le comportement diffère de celui des deux autres est un défaut,
jamais une variante documentée : sur la même requête et le même état, les trois rendent le même
résultat et, en cas de refus, le même message d'erreur (§10, §15 décision 25) — et devinent la même
structure à partir des mêmes données (§8.1). Puisqu'un seul cœur partagé écrit le fichier, les trois
ponts produisent aussi, à l'octet près, le même fichier pour un même état (§15, décision 25) : une
équipe qui alterne un test Go et un test PHP contre le même fichier ne voit jamais, dans l'historique
Git, un changement de **donnée** qui n'a pas eu lieu — la première réécriture d'un fichier commenté à
la main en est une conséquence connue, avec les autres exceptions de la décision 26 (§9.1).

### 8.9 Recharger l'état depuis le fichier, à la demande

**Ce que c'est.** Le code appelant peut demander à Memless d'oublier l'état qu'il a en mémoire et de
relire le fichier depuis le disque, exactement comme au premier chargement — utile quand le fichier a
été restauré à un état de départ par un moyen externe à Memless, et qu'on veut que la mémoire
s'aligne dessus sans recréer un moteur.

**Ce qu'il faut.** Le même fichier, dans un état à nouveau valide — §8.1 et §8.2 s'appliquent à
l'identique. Aucune transaction ne doit être ouverte : il faut d'abord la valider ou l'abandonner
(§8.5).

**Ce qui est refusé.** Un rechargement demandé pendant qu'une transaction est ouverte est refusé. Un
rechargement qui échouerait l'une des vérifications du §8.1 ou du §8.2 — fichier introuvable,
illisible, vide, sans table, ou incohérent — laisse l'état précédent inchangé en mémoire : un
rechargement raté ne vide jamais l'état existant pour le remplacer par rien.
