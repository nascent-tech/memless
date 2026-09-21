# Memless — Architecture

<!-- charpente-navigation -->
**Parties** : [§3. Diagrammes](.charpente/architecture/diagrams.md) · [§4. Composants](.charpente/architecture/components.md) · [§5. Solidité du système](.charpente/architecture/system-design.md) · [§6. Fondations logicielles](.charpente/architecture/software-foundations.md) · [§7. Stack](.charpente/architecture/stack.md) · [§8. Données](.charpente/architecture/data.md) · [§9. Dépendances externes](.charpente/architecture/dependencies.md) · [§10. Sécurité](.charpente/architecture/security.md) · [§11. Développement local](.charpente/architecture/dev-environment.md) · [§12. Vision et décisions](.charpente/architecture/roadmap.md) · [§13. Glossaire](.charpente/architecture/glossary.md)  
**Maillage** : [maillage.md](.charpente/maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

> **Statut : proposition, avant tout code.** Le dépôt ne contient que le brief (`.charpente/brief/`),
> encore `a_valider` : toute décision reprise ici en hérite et sera relue à sa validation. Ce document
> propose une architecture justifiée par le brief, non un état constaté du code ; ce que le brief
> tranche déjà (§5.2 du brief) est repris tel quel, le reste est une proposition ou une décision à
> prendre, signalée comme telle. Aucun MVP ni dépôt Git n'existe encore. **Convention de renvoi :**
> « §X du brief » / « décision N du brief » pointent vers le brief ; un « §X » nu, vers ce document.

## §1. Le produit

Memless est un **moteur de base de données en mémoire piloté par un seul fichier YAML** : le fichier
ne porte que des données — chaque table est une liste de lignes — et Memless devine tout seul leur
forme (le type de chaque valeur, la colonne `id` qui identifie une ligne, les relations entre tables
lues dans le nom des colonnes). Il charge cet état en mémoire, le laisse interroger et modifier en
texte SQL, et réécrit le fichier à chaque transaction validée : le fichier reste la seule source de
vérité. Imagine un classeur où chaque fiche se range toute seule dès qu'on l'écrit, sans qu'on ait à
dessiner d'abord la forme des cases.

Le public est **une développeuse qui prépare les données de ses tests automatisés et de ses
démonstrations** — jamais une application en production (§14 du brief). Ce choix commande toute
l'architecture : pas de serveur, pas de réseau, pas de coordination entre écritures concurrentes, un
moteur pensé pour la taille d'un test. La contrainte fondatrice (§5.1 du brief) est qu'**un seul
cœur** serve **trois langages** — JavaScript/TypeScript, Go, PHP — à l'identique : toute règle du
métier vit dans le cœur, jamais dans un pont, sous peine de voir un langage diverger des deux autres.

Le brief tranche par exception (§5.2 du brief) le pari technique : cœur en **Rust**, moteur SQL
réutilisé (**GlueSQL**, sans schéma déclaré), chaque langage appelant le cœur par la voie la plus
directe qu'il offre (`napi-rs` pour Node, `FFI` intégré pour PHP, `purego` pour Go). Memless est
**gratuit et ouvert** sous licence permissive (décision 1 du brief) ; il n'a ni société ni dépôt
public publié, et aucun contact technique n'est déclaré (ni CODEOWNERS, ni manifeste, ni config Git)
— à renseigner à la création du premier manifeste.

## §2. Structure du projet

Aucun code n'existe : l'arbre ci-dessous est **proposé**, en architecture hexagonale (§6.1), sous la
forme d'un espace de travail Cargo — un dépôt qui réunit plusieurs paquets Rust (des *crates*), tel un
casier à tiroirs où chaque tiroir se ferme sur ses propres dépendances.

```text
memless/
├── Cargo.toml                  # manifeste d'espace de travail (workspace, manifeste virtuel)
├── crates/
│   ├── memless-domain/         # domaine pur : inférence, règle de comparaison, invariants — AUCUNE dépendance
│   ├── memless-core/           # application (cas d'usage + ports) + adaptateurs sortants
│   │   └── src/
│   │       ├── application/    # cas d'usage + ports (traits), dont le port de garde SQL (SqlGate)
│   │       ├── gluesql/        # adaptateur sortant : exécution SQL, garde du sous-ensemble (SqlGate), ACL vers/depuis GlueSQL
│   │       └── yaml/           # adaptateur sortant : lecture + écriture atomique du fichier
│   ├── memless-capi/           # surface native C ABI (cdylib) — chargée par Go et PHP
│   └── memless-node/           # addon napi-rs (.node) — mince enrobage du cœur, sans logique (§12.3, question 3)
├── bindings/
│   ├── node/                   # paquet npm — empaquette l'addon memless-node
│   ├── php/                    # paquet Composer (FFI) — charge memless-capi
│   └── go/                     # module Go (purego) — charge memless-capi
└── harness/parity/             # banc de parité multi-langage (hors Cargo) : même SQL + même état → même résultat
```

- **Point d'entrée réel :** chaque langage entre par son pont ; Go et PHP chargent la surface native
  `memless-capi` (C ABI), Node charge l'addon `memless-node`. Le brief partage le **cœur**, pas
  nécessairement un unique fichier natif pour Node (§5.2 du brief, §12.3 question 3) ; aucun pont ne
  touche le domaine ni GlueSQL directement.
- **Sens des dépendances :** `bindings/*` → surface native (`memless-capi` ou `memless-node`) →
  `memless-core::application` → `memless-domain`. Le crate `memless-domain` ne dépend d'**aucun**
  crate externe, et **Cargo l'impose** — un cycle entre crates ne compile pas. Les deux surfaces
  natives sont membres du workspace.
- **Point de composition :** une racine par surface native (`memless-capi`, `memless-node`), chacune
  réduite au **même** appel de câblage dans `memless-core` — adaptateurs YAML + GlueSQL → cas d'usage.

## §3. Diagrammes

Vue C4 (contexte, conteneur, composant) et séquences des flux critiques : l'écriture validée avec
réécriture atomique du fichier, le refus d'une transaction qui casserait une relation devinée, et le
chargement qui valide ou refuse en bloc.

→ [Lire le détail](.charpente/architecture/diagrams.md)

## §4. Composants

Le handle d'instance, le chargeur YAML, l'inférence de structure, la règle de comparaison, la garde
du sous-ensemble SQL, le Store GlueSQL, le validateur d'invariants, le gestionnaire de transaction,
l'écrivain YAML atomique, et les trois ponts — chacun avec sa responsabilité et sa technologie.

→ [Lire le détail](.charpente/architecture/components.md)

## §5. Solidité du système

Memless n'est pas un service : pas d'échelle horizontale, pas de disponibilité réseau. Sa solidité
tient à la sûreté du fichier (écriture atomique avec `fsync`), au tout-ou-rien transactionnel, et au
coût — non encore mesuré — de la réécriture complète.

→ [Lire le détail](.charpente/architecture/system-design.md)

## §6. Fondations logicielles

Paradigme hexagonal justifié pour une bibliothèque embarquée à trois ponts ; un contexte borné
unique — le moteur — dont le cœur de métier est l'inférence de structure et l'intégrité, GlueSQL
étant générique et la persistance YAML un support.

→ [Lire le détail](.charpente/architecture/software-foundations.md)

## §7. Stack

Rust, GlueSQL, une bibliothèque YAML, cbindgen, napi-rs, purego, FFI PHP — chaque outil avec sa
version cible et sa justification tirée du brief, jamais de sa popularité.

→ [Lire le détail](.charpente/architecture/stack.md)

## §8. Données

L'état en mémoire détenu par le Store GlueSQL, le fichier YAML comme seule source de vérité, les
identifiants au type libre, la cohérence tout-ou-rien à l'échelle du fichier entier, l'absence de
migrations.

→ [Lire le détail](.charpente/architecture/data.md)

## §9. Dépendances externes

Aucun service réseau. Les seules dépendances sont des bibliothèques liées à la compilation (GlueSQL,
YAML) et les mécanismes de pont de chaque langage.

→ [Lire le détail](.charpente/architecture/dependencies.md)

## §10. Sécurité

Pas de réseau, pas d'authentification, pas de secret. La surface sensible est la **frontière FFI** —
pointeurs, chaînes, propriété des tampons et paniques venus du langage hôte — et la robustesse du
chargement YAML face à un fichier hostile.

→ [Lire le détail](.charpente/architecture/security.md)

## §11. Développement local

Chaîne Rust (cargo), plus les chaînes Node, Go et PHP pour les ponts ; cbindgen pour l'en-tête C ; et
le banc de parité comme test de première classe.

→ [Lire le détail](.charpente/architecture/dev-environment.md)

## §12. Vision et décisions

Feuille de route (serveur, cascade, API native, verrou inter-processus, production), dettes assumées
(concurrence, sûreté FFI, coût de réécriture, parité PHP) et questions ouvertes (Store GlueSQL,
format d'échange, chemin Node, CI, distribution du binaire).

→ [Lire le détail](.charpente/architecture/roadmap.md)

## §13. Glossaire

Les termes propres au projet : structure devinée, relation devinée, règle de comparaison, invariant,
transaction tout-ou-rien, snapshot, ACID, réécriture atomique, cœur commun, pont natif,
port/adaptateur, DDL, ACL, GlueSQL sans schéma, Store GlueSQL, handle d'instance, cdylib, FFI, C ABI,
C4, banc de parité.

→ [Lire le détail](.charpente/architecture/glossary.md)
