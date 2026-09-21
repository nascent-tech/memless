<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§9. Dépendances externes](dependencies.md)  
**Suivant** : [§11. Développement local](dev-environment.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §10. Sécurité

La porte de Memless ne donne pas sur la rue : elle s'ouvre uniquement depuis l'intérieur du processus
qui l'appelle. La sécurité classique d'un service — authentifier un inconnu, filtrer un trafic —
n'a pas d'objet. Ce qui reste sensible tient en deux points : la **frontière FFI** (des pointeurs et
de la mémoire venus du code hôte) et la **robustesse du chargement** d'un fichier qui pourrait être
hostile.

### 10.1 Trajet entrant

Il n'y a **ni Internet, ni edge, ni load balancer, ni gateway, ni BFF**. Le seul « entrant » est un
appel de fonction depuis le code hôte, dans la même mémoire, à travers le pont natif. L'ordre
habituel (TLS → bots → rate limiting → authentification → autorisation → métier) n'existe pas ; le
seul étage réel est :

```text
Code hôte (de confiance) → pont natif → surface FFI → cœur → fichier local
```

Le code appelant est **de confiance par construction** : c'est le test de la développeuse, dans son
propre processus. La menace n'est donc pas un attaquant réseau, mais un **appel malformé** (mauvais
pointeur, longueur fausse) ou un **fichier d'entrée hostile**.

### 10.2 Identités

**Aucune.** Pas d'utilisateur, pas de session, pas de jeton d'accès ou de rafraîchissement, pas de
révocation. Il n'y a personne à authentifier : Memless ne connaît que le code qui l'a chargé.

### 10.3 Autorisations

**Aucune.** Toute la surface est accessible au code hôte, qui possède déjà le processus. Il n'y a pas
de ressource à cloisonner par propriétaire : le seul contrôle d'accès pertinent est celui du système
de fichiers sur le fichier YAML, hors du ressort de Memless.

### 10.4 Protection des données

- **Pas de chiffrement** : le fichier YAML est en clair **par conception** — c'est ce qui le rend
  lisible en diff Git (§9.1 du brief). Une développeuse qui y place des données sensibles en assume
  le choix ; Memless ne chiffre ni ne pseudonymise.
- **Aucun secret** n'entre dans Memless : ni identifiant, ni clé, ni mot de passe.
- **Sûreté mémoire du cœur** garantie par Rust (décision 12 du brief) — mais **seulement à
  l'intérieur** du cœur. La frontière `extern "C"` est la couture à surveiller (10.5).

### 10.5 Entrées et frontière FFI

Le fichier et l'appel FFI sont les deux entrées à traiter strictement.

- **Le fichier YAML** : refuser un YAML invalide, une valeur imbriquée, une clé de premier niveau qui
  n'est pas une liste de lignes, une ligne sans `id`, un `id` dupliqué, une relation cassée — jamais
  de chargement partiel (§8.1, §8.2, §13 du brief). Le risque « YAML bomb » (alias/ancres qui
  explosent en mémoire, fichier démesuré) est **borné par le `Budget` de serde-saphyr** (limites de
  profondeur, d'alias, de taille) ; ses défauts étant « intentionnellement permissifs », les
  resserrer pour un fichier de données Memless reste **à faire au plan** (§12.3, question 4-bis).
- **La frontière FFI** est `unsafe` par nature : c'est là, et seulement là, que la garantie mémoire de
  Rust ne couvre plus rien. Quatre protections y sont indispensables (dette élevée si l'une manque,
  §12.2) :
  1. **Valider chaque entrée** — pointeur non nul, longueur cohérente, chaîne UTF-8 — avant tout
     usage.
  2. **Propriété claire des sorties** — toute chaîne ou tout résultat rendu au langage hôte est alloué
     par le cœur et libéré par une fonction dédiée (`memless_free`) ; jamais libéré par l'hôte, jamais
     deux fois. Fermer une instance passe de même par une fonction explicite (le brief §8.5 prévoit un
     « appel explicite de fermeture »).
  3. **Attraper les paniques** — une panique Rust qui traverse `extern "C"` est un comportement
     indéfini ; chaque point d'entrée l'enveloppe (`catch_unwind`) et la traduit en erreur.
  4. **Protéger la table de handles** — le brief ne promet aucune coordination des *données* entre
     acteurs concurrents (§12.2 du brief) ; la sûreté *mémoire* de la table de handles est une autre
     affaire : deux instances distinctes appelées depuis deux fils (goroutines, worker threads Node)
     touchent la même table, qui doit donc être protégée par un verrou (ou la bibliothèque déclarée
     mono-fil, ce qui serait plus pauvre).

Le **texte SQL** n'est pas exécuté tel quel : il passe d'abord par la **garde du sous-ensemble SQL**
(§4), qui refuse le DDL et le hors-sous-ensemble (décisions 23, 24 du brief). Memless ne « nettoie »
pas la requête elle-même — le texte vient du code de test, de confiance ; si un test construit du SQL
depuis une entrée externe, l'assainir est la responsabilité de l'appelant.

### 10.6 Audit

Le journal d'audit **des données** est l'historique Git du fichier (§9.1 du brief) : il montre ce qui
a changé, ligne par ligne, horodaté par le commit. Il ne dit **pas qui a déclenché** le changement —
c'est un audit applicatif, hors périmètre (§14 du brief). Memless ne tient aucun journal interne et
n'écrit aucun secret : il n'a rien de tel à enregistrer.
