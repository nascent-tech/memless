<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§4. Composants](components.md)  
**Suivant** : [§6. Fondations logicielles](software-foundations.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §5. Solidité du système

Un pont tient par la façon dont ses forces se répartissent : Memless n'a pas de tablier suspendu au-
dessus d'un réseau, mais un seul appui — le fichier sur le disque — qui doit rester intact quoi qu'il
arrive. Sa solidité se lit donc autrement que celle d'un service.

### 5.1 Échelle

Memless est une **bibliothèque embarquée**, pas un service : il n'y a ni mise à l'échelle
horizontale (ajouter des machines) ni verticale (grossir une machine) à décider. Chaque instance vit
dans le processus qui l'appelle et détient sa propre copie en mémoire — donc **avec état** (*stateful*),
jamais partagée (décision 6 du brief).

La seule « échelle » qui compte est la **taille des fixtures** et le **nombre d'écritures** d'une
suite de tests, parce que chaque transaction validée réécrit le fichier entier (§8.6 du brief). Le
coût grandit avec la taille totale du fichier × le nombre d'écritures, pas avec la taille du
changement. Le seuil à partir duquel ce coût devient perceptible est **à définir** : c'est une
hypothèse risquée non mesurée (§17.2 du brief), et sa mesure est le banc de mesure (§11).

### 5.2 Disponibilité

Sans objet au sens réseau : pas de réplication, pas de répartition de charge, aucun point d'écoute à
maintenir en ligne. La « disponibilité » se réduit à deux choses — que la bibliothèque native se
charge dans le processus hôte (une question d'empaquetage par plateforme, §12.3) et que le fichier
reste toujours lisible. Le seul point unique de panne est le fichier lui-même ; sa protection est
traitée en 5.3.

### 5.3 Résilience

Il n'y a ni réseau ni service distant, donc **ni retries, ni disjoncteurs, ni file d'attente morte,
ni healthchecks** — ces mécanismes n'auraient rien à protéger. La résilience de Memless est celle de
son écriture fichier, et le mécanisme doit tenir jusqu'à la **coupure de courant**, pas seulement au
plantage du processus :

- **Écriture atomique complète** (décision 17 du brief) : sérialiser tout l'état dans un **temporaire
  créé dans le même répertoire** que le fichier cible (sinon le `rename` n'est pas atomique) → `fsync`
  du temporaire → `rename` sur le fichier d'origine → `fsync` du répertoire. Une coupure laisse alors
  l'ancien fichier intact ou le nouveau complet, jamais tronqué ni vide.
- **Nom réservé du temporaire** : un préfixe distinctif (par exemple `.<nom>.memless-tmp`) permet
  qu'un temporaire laissé par un arrêt brutal soit reconnu. Le brief se contente de « l'ignorer »
  (§8.6 du brief) ; ce document **propose** en plus de le **nettoyer au chargement suivant**. Une
  transaction échouée supprime le sien elle-même — la décision 17 du brief (« temporaire non abouti
  toujours supprimé ») est ainsi tenue, seul un `kill` brutal reportant la suppression. Memless ne
  committe jamais ce résidu ; le motif de nom est documenté pour que l'équipe l'ajoute à son
  `.gitignore`, ce qui tient l'engagement de la décision 17 du brief (« aucun résidu visible dans
  l'historique Git ») jusqu'au nettoyage.
- **Tout-ou-rien transactionnel** (§8.5 du brief) : une étape qui échoue n'en laisse aucune atteindre
  le disque.
- **Annulation en mémoire sur échec disque** (décision 18 du brief) : si l'écriture échoue, le
  changement est aussi annulé en mémoire — la mémoire ne retient jamais ce que le disque n'a pas reçu.

### 5.4 Communication

Un seul lien existe : la **frontière FFI** entre le langage hôte et le cœur, dans la même mémoire.

| Lien | Mécanisme | Contrat | Couplage | Livraison |
|---|---|---|---|---|
| Pont ↔ cœur | appel de fonction natif (C ABI) | le format d'échange (§12.3, question 2) — texte SQL en entrée, lignes/erreur en sortie | fort et assumé : les trois ponts dépendent du même cœur, par conception (§5.1 du brief) | synchrone, en mémoire, sans perte |

Il n'y a **aucun appel entre services ni entre contextes** : le couplage fort des trois ponts au cœur
n'est pas une dette, c'est la garantie même que les trois langages se comportent à l'identique.

### 5.5 Flux critiques

1. **Écrire puis persister** (§3.4) : `begin → écritures → garde SQL → commit → invariants → écriture
   atomique`. Protégé par le tout-ou-rien et l'écriture atomique complète ; l'échec disque annule la
   mémoire. C'est le flux le plus proche d'une opération irréversible, et il porte les décisions 5,
   15, 17, 18, 19 du brief.
2. **Charger et valider** (§3.6) : lecture du fichier → inférence → vérification des trois invariants
   → état en mémoire, ou refus complet sans état partiel.
3. **Recharger** (§3.6, §8.9 du brief) : construire le nouvel état validé, puis basculer d'un coup ;
   un rechargement raté laisse l'état précédent intact. Refusé si une transaction est ouverte.

Aucun de ces flux ne franchit un réseau : il n'y a donc pas d'outbox ni de saga à prévoir (§8.4). La
frontière transactionnelle est entièrement locale, tenue par le gestionnaire de transaction et
l'écriture atomique.
