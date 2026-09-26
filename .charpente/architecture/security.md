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
  n'est pas une liste de lignes, une clé qui n'est pas un texte, une ligne sans `id`, un `id` dupliqué,
  une relation cassée — jamais de chargement partiel (§8.1, §8.2, §13 du brief). Le risque « YAML
  bomb » (alias/ancres qui explosent en mémoire, fichier démesuré) **n'est pas borné** : le budget de
  `serde-saphyr` (`saturate_options()`) met chaque limite à `usize::MAX` — dette assumée (§12.2) plutôt
  que resserrée, le fichier venant de la développeuse elle-même, jamais d'une source non fiable.
- **La frontière FFI** est `unsafe` par nature : c'est là, et seulement là, que la garantie mémoire de
  Rust ne couvre plus rien. Quatre protections existent (`memless-capi`) :
  1. **Valider chaque entrée** — un `path` nul ou non-UTF-8 rend `InvalidArgument`, avant tout usage.
  2. **Propriété claire des sorties** — tout message rendu au langage hôte est alloué par le cœur et
     libéré par `memless_free_string` ; jamais libéré par l'hôte, jamais deux fois. Fermer une
     instance ou un résultat passe par `memless_release` / `memless_result_release`.
  3. **Attraper les paniques** — chaque point d'entrée C ABI enveloppe son corps dans un `guard`
     (`catch_unwind`) qui traduit une panique Rust en erreur `Internal`, plutôt que de laisser un
     comportement indéfini traverser `extern "C"`. Le profil du workspace force `panic = "unwind"`
     (`Cargo.toml`) : un profil `abort` désarmerait ce garde et arrêterait le processus hôte sur la
     moindre panique.
  4. **Protéger la table de handles** — le brief ne promet aucune coordination des *données* entre
     acteurs concurrents (§12.2 du brief) ; la sûreté *mémoire* de la table de handles est une autre
     affaire : elle est protégée par un `Mutex`, pour que deux fils (goroutines, worker threads Node)
     appelant deux instances distinctes ne corrompent jamais la table elle-même.

Le **texte SQL** n'est pas exécuté tel quel : il passe d'abord par la garde du sous-ensemble
(`memless-engine::sql`), qui refuse le DDL et le hors-sous-ensemble (`OutsideSubset` — décisions 23, 24
du brief). Memless ne « nettoie » pas la requête elle-même — le texte vient du code de test, de
confiance ; si un test construit du SQL depuis une entrée externe, l'assainir est la responsabilité de
l'appelant.

### 10.6 Publication (depuis le palier 6)

La publication sur les registres externes ouvre une surface qui n'existait pas avant le palier 6 : des
identités de dépôt et des jetons, à traiter avec le même soin que le reste.

- **Extraction dans le cache utilisateur (Go)** : la bibliothèque embarquée par `//go:embed` est
  extraite une fois dans `os.UserCacheDir()/memless/<version>-<sha256 court>/` : dossier de cache créé
  0700, bibliothèque 0755, écriture par fichier temporaire puis `rename` (jamais un fichier
  partiellement écrit visible du chargeur). **Avant tout
  `dlopen`**, le SHA-256 complet du fichier présent dans le cache est comparé aux octets embarqués dans
  le binaire ; un écart réécrit le fichier avant de l'ouvrir — le cache ne peut pas servir une
  bibliothèque altérée sans que la vérification le détecte.
- **Vérification SHA-256** plus largement : `SHA256SUMS`, déjà publié en Release GitHub depuis le
  palier 5, reste le point de recoupement pour qui veut vérifier une bibliothèque obtenue par un
  registre — npm garantit par ailleurs l'intégrité de son propre paquet à l'installation.
- **Clé de déploiement limitée au miroir** : la CI pousse le miroir `nascent-tech/memless-php` par une
  **clé de déploiement SSH en écriture**, dont la portée est ce seul dépôt — jamais une clé ou un
  jeton à portée du dépôt principal `nascent-tech/memless`. Le secret (`PHP_MIRROR_DEPLOY_KEY`) n'est
  exposé qu'au step de poussée du job `publish-php`.
- **Deuxième clé de déploiement, limitée au miroir Go** : sur le même patron, la CI pousse le miroir
  `nascent-tech/memless-go` par une **clé de déploiement SSH en écriture** distincte, dont la portée
  est ce seul dépôt — jamais une clé ou un jeton à portée du dépôt principal. Le secret
  (`GO_MIRROR_DEPLOY_KEY`) n'est exposé qu'au step de poussée du job `publish-go`, qui n'a donc plus besoin de
  `contents: write` sur le dépôt principal.
- **Jeton npm granulaire temporaire, puis Trusted Publishing** : la toute première publication npm
  s'authentifie par un jeton **granulaire** (portée publish sur `@nascent-tech/*`, Bypass 2FA, ≤ 90
  jours, secret `NPM_TOKEN`), parce que Trusted Publishing ne peut pas amorcer un paquet qui n'existe
  pas encore sur npm. Une fois la première version publiée, le propriétaire crée les Trusted
  Publishers (OIDC, `id-token: write`) et supprime le jeton : les publications suivantes n'exposent
  plus aucun secret npm de longue durée. Les jobs de publication ont des permissions minimales et
  cloisonnées : `publish` (`contents: write`, pour créer la Release GitHub), `publish-go`
  (`contents: read`, la poussée se fait par la clé de déploiement SSH du miroir, jamais par le jeton
  GitHub Actions), `publish-npm` (`id-token: write`, `contents: read`), `publish-php`
  (`contents: read`) — chaque secret n'est visible que du job qui l'utilise.

### 10.7 Audit

Le journal d'audit **des données** est l'historique Git du fichier (§9.1 du brief) : il montre ce qui
a changé, ligne par ligne, horodaté par le commit. Il ne dit **pas qui a déclenché** le changement —
c'est un audit applicatif, hors périmètre (§14 du brief). Memless ne tient aucun journal interne et
n'écrit aucun secret : il n'a rien de tel à enregistrer.
