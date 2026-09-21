# Le maillage des documents

Généré — ne s'édite pas à la main : il se retisse à chaque écriture d'un document, et
/charpente:maillage le recrée s'il manque. Chaque dossier de `.charpente/` porte son propre
`maillage.md`, réduit à ses documents.

## Les documents de la racine

- IDEE.md — absent
- DESIGN.md — absent
- EXPERIENCE.md — absent
- ARCHITECTURE.md — présent · parties : diagrams.md · components.md · system-design.md · software-foundations.md · stack.md · data.md · dependencies.md · security.md · dev-environment.md · roadmap.md · glossary.md

## Les artefacts

| Document | Type | Titre | Statut | Parties | Amont | Aval |
|---|---|---|---|---|---|---|
| .charpente/brief/ | brief | Moteur de base de données en mémoire pilotée par YAML | valide | 01-le-produit-et-son-public.md · 02-cadre-et-acteurs.md · 03-besoins-et-fonctionnalites.md · 04-fonctionnalite-decisive.md · 05-regles-et-limites.md · 06-interdits-et-hors-perimetre.md · 07-decisions.md · 08-suite-hypotheses-sources.md | — | .charpente/mvp |
| .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md | cadrage | Palier 1 — Charger et refuser | valide | — | .charpente/mvp | .charpente/conceptions/2026-09-21-palier-1-charger-refuser |
| .charpente/conceptions/2026-09-21-palier-1-charger-refuser/ | conception | Palier 1 — Charger et refuser | valide | 01-modele-et-ports.md · 02-domaine.md · 03-application-et-yaml.md · 04-capi-et-ponts.md · 05-banc-de-parite-et-workspace.md · 06-decisions-et-hors-perimetre.md | .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md | .charpente/plans/2026-09-21-palier-1-charger-refuser |
| .charpente/glossaire/le-moteur.md | glossaire | Glossaire — le moteur | valide | — | — | — |
| .charpente/mvp/ | mvp | Prouver la promesse de Memless | valide | 01-le-geste-et-le-tri.md · 02-faiblesses-et-garanties.md · 03-paliers-fini-trompe.md | .charpente/brief | .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md |
| .charpente/plans/2026-09-21-palier-1-charger-refuser/ | plan | Palier 1 — Charger et refuser | valide | 01-fichiers-figes.md · 02-pr1-domaine.md · 03-pr2-moteur-et-batterie.md · 04-capi-et-ponts.md · 05-pr6-parite-et-decoupage.md | .charpente/conceptions/2026-09-21-palier-1-charger-refuser | — |
