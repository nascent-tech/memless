---
type: brief
titre: Moteur de base de données en mémoire pilotée par YAML
cree_le: 2026-08-31T14:52:10+0000
mis_a_jour_le: 2026-09-21T14:08:15+0000
branche: hors-depot
statut: valide
---

<!-- charpente-navigation -->
**Le fil** : **brief — Moteur de base de données en mémoire pilotée par YAML** › [mvp — Prouver la promesse de Memless](../mvp/00-index.md)  
**Parties** : [1. Le projet en une page](01-le-produit-et-son-public.md) · [5. Le cadre imposé](02-cadre-et-acteurs.md) · [7. Les besoins, séparés des solutions](03-besoins-et-fonctionnalites.md) · [9. Les fonctionnalités décisives](04-fonctionnalite-decisive.md) · [10. Les règles transverses](05-regles-et-limites.md) · [13. Ce qui est interdit](06-interdits-et-hors-perimetre.md) · [15. Les décisions, et ce que chacune écarte](07-decisions.md) · [16. Ce qui vient après le lancement](08-suite-hypotheses-sources.md)  
**Maillage** : [maillage.md](../maillage.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

# Memless — brief

Ce document dit ce que Memless doit devenir, et pourquoi — jamais comment le construire : aucune ligne
de code ni aucun découpage technique n'y figure, et le §5.2 renvoie explicitement le choix de la
technologie à l'architecture. Il s'adresse à qui construira Memless, qui en parlera, ou qui en décidera
l'usage, sans qu'aucun d'eux n'ait besoin de lire du code pour se comprendre avec les autres.

**Tout y est tranché.** Aucune section ne renvoie une question de produit à plus tard — un détail de
conception explicitement nommé comme tel (l'étendue exacte du SQL supporté, le nombre de systèmes à
publier) n'est pas une question ouverte : c'est un renvoi assumé vers la commande qui le tranchera.
Là où une valeur précise manque encore, ce n'est pas une décision qui manque : le §12.3 dit pourquoi
aucune mesure chiffrée n'est nécessaire au fonctionnement du produit à ce stade, et renvoie ce qui
reste réellement incertain vers les hypothèses risquées (§17).

**Premier usage, premier public : les données de test.** Memless sert d'abord une développeuse qui
prépare les données utilisées par ses tests automatisés et ses démonstrations — jamais une
application qui tourne en production. Ce choix fixe tout le reste : la promesse (un fichier, pas un
serveur), la frontière de ce que Memless n'est pas (§3), et l'ordre dans lequel les fonctionnalités
se justifient (§7 à §9).

## Sommaire

1. [Le produit et son public](01-le-produit-et-son-public.md) — §1 le projet en une page · §2 à qui
   ça s'adresse et sur quel terrain · §3 ce que le produit fait et ce qu'il n'est pas · §4 pourquoi
   le pari technique tient.
2. [Le cadre et les acteurs](02-cadre-et-acteurs.md) — §5 le cadre imposé · §6 les acteurs.
3. [Les besoins et les fonctionnalités](03-besoins-et-fonctionnalites.md) — §7 les besoins, séparés
   des solutions · §8 le produit, fonctionnalité par fonctionnalité.
4. [Les fonctionnalités décisives](04-fonctionnalite-decisive.md) — §9, une seule au lancement.
5. [Les règles transverses et les limites](05-regles-et-limites.md) — §10 les règles transverses ·
   §12 les limites, et où vivent les chiffres.
6. [Ce qui est interdit et le hors périmètre](06-interdits-et-hors-perimetre.md) — §13 ce qui est
   interdit · §14 hors périmètre.
7. [Les décisions](07-decisions.md) — §15, ce que chacune écarte.
8. [La suite, les paris et les sources](08-suite-hypotheses-sources.md) — §16 ce qui vient après le
   lancement · §17 les hypothèses risquées · §18 sources.

**Le §11 (comment le produit se paie) est délibérément absent.** Memless est gratuit et ouvert, sans
société ni vente derrière (§15, décision 1) : il n'y a rien à détailler sur qui paie, quoi, ou
quand.
