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
| .charpente/cadrage/2026-09-21-palier-2-interroger.md | cadrage | Palier 2 — Interroger | valide | — | .charpente/mvp | .charpente/conceptions/2026-09-21-palier-2-interroger |
| .charpente/cadrage/2026-09-21-palier-3-ecrire.md | cadrage | Palier 3 — Écrire | valide | — | .charpente/mvp | .charpente/conceptions/2026-09-21-palier-3-ecrire |
| .charpente/cadrage/2026-09-22-palier-4-transiger.md | cadrage | Palier 4 — Transiger | valide | — | .charpente/mvp | .charpente/conceptions/2026-09-22-palier-4-transiger |
| .charpente/conceptions/2026-09-21-palier-1-charger-refuser/ | conception | Palier 1 — Charger et refuser | valide | 01-modele-et-ports.md · 02-domaine.md · 03-application-et-yaml.md · 04-capi-et-ponts.md · 05-banc-de-parite-et-workspace.md · 06-decisions-et-hors-perimetre.md | .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md | .charpente/plans/2026-09-21-palier-1-charger-refuser |
| .charpente/conceptions/2026-09-21-palier-2-interroger/ | conception | Palier 2 — Interroger | valide | 01-domaine.md · 02-moteur-et-capi.md · 03-ponts-et-parite.md · 04-decisions-et-hors-perimetre.md | .charpente/cadrage/2026-09-21-palier-2-interroger.md | .charpente/plans/2026-09-21-palier-2-interroger |
| .charpente/conceptions/2026-09-21-palier-3-ecrire/ | conception | Palier 3 — Écrire | valide | 01-domaine.md · 02-ecrivain.md · 03-moteur-capi.md · 04-ponts-parite-banc.md | .charpente/cadrage/2026-09-21-palier-3-ecrire.md | .charpente/plans/2026-09-21-palier-3-ecrire |
| .charpente/conceptions/2026-09-22-palier-4-transiger/ | conception | Palier 4 — Transiger | valide | 01-domaine.md · 02-moteur.md · 03-capi.md · 04-ponts-parite-banc.md | .charpente/cadrage/2026-09-22-palier-4-transiger.md | .charpente/plans/2026-09-22-palier-4-transiger |
| .charpente/glossaire/le-moteur.md | glossaire | Glossaire — le moteur | valide | — | — | — |
| .charpente/mvp/ | mvp | Prouver la promesse de Memless | valide | 01-le-geste-et-le-tri.md · 02-faiblesses-et-garanties.md · 03-paliers-fini-trompe.md | .charpente/brief | .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md · .charpente/cadrage/2026-09-21-palier-2-interroger.md · .charpente/cadrage/2026-09-21-palier-3-ecrire.md · .charpente/cadrage/2026-09-22-palier-4-transiger.md |
| .charpente/plans/2026-09-21-palier-1-charger-refuser/ | plan | Palier 1 — Charger et refuser | valide | 01-fichiers-figes.md · 02-pr1-domaine.md · 03-pr2-moteur-et-batterie.md · 04-capi-et-ponts.md · 05-pr6-parite-et-decoupage.md | .charpente/conceptions/2026-09-21-palier-1-charger-refuser | — |
| .charpente/plans/2026-09-21-palier-2-interroger/ | plan | Palier 2 — Interroger | valide | 01-phases-et-tests.md | .charpente/conceptions/2026-09-21-palier-2-interroger | — |
| .charpente/plans/2026-09-21-palier-3-ecrire/ | plan | Palier 3 — Écrire | livre | 01-phases-et-tests.md | .charpente/conceptions/2026-09-21-palier-3-ecrire | — |
| .charpente/plans/2026-09-22-palier-4-transiger/ | plan | Palier 4 — Transiger | livre | 01-phases-et-tests.md | .charpente/conceptions/2026-09-22-palier-4-transiger | — |
