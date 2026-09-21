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
| .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md | cadrage | Palier 1 — Charger et refuser | valide | — | .charpente/mvp | — |
| .charpente/glossaire/le-moteur.md | glossaire | Glossaire — le moteur | valide | — | — | — |
| .charpente/mvp/ | mvp | Prouver la promesse de Memless | valide | 01-le-geste-et-le-tri.md · 02-faiblesses-et-garanties.md · 03-paliers-fini-trompe.md | .charpente/brief | .charpente/cadrage/2026-09-21-palier-1-charger-refuser.md |
