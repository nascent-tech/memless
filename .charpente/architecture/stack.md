<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§6. Fondations logicielles](software-foundations.md)  
**Suivant** : [§8. Données](data.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §7. Stack

Chaque outil est retenu pour ce que le brief exige, jamais pour sa popularité. Les versions ci-dessous
sont celles constatées dans le code et les manifestes du dépôt.

| Catégorie | Outil et version | Métaphore et justification |
|---|---|---|
| Langage du cœur | **Rust**, édition 2021, toolchain stable | le langage qui rattrape les erreurs de mémoire avant même de tourner, comme un filet posé sous le trapèze : retenu par l'architecture (la décision 12 du brief la lui renvoie) pour un moteur transactionnel, et recommandé par les agences de cybersécurité (§18 du brief) |
| Analyseur SQL | **`sqlparser`** `=0.54.0` | le grammairien qui découpe le texte SQL en arbre, sans rien exécuter ; l'exécution (lire, trier, joindre, agréger, écrire) est écrite à la main dans `memless-domain`, parce que la règle de comparaison unique et la sémantique de l'absence n'y sont d'aucune bibliothèque générique (§5.2 du brief) |
| Lecture/écriture YAML | **`serde-saphyr`** `=1.3.0` | l'ouvre-boîte et le copiste du fichier — `serde_yaml` est archivé (mars 2024) ; `serde-saphyr` est maintenu, s'intègre à serde ; l'ordre des colonnes est porté hors de la bibliothèque, par `RawDocument` (`memless-domain::document`), pas par un `IndexMap` de la bibliothèque elle-même ; son budget anti-« YAML bomb » est **saturé** (`usize::MAX`), dette assumée (§12.2) |
| Pont Node/TS | **`koffi`** (FFI dynamique, ≥ 2.16), Node ≥ 18 | le traducteur qui charge la même `cdylib` que Go et PHP à l'exécution, sans rien compiler côté Node — décision humaine du 2026-09-22, en place de `napi-rs` |
| Pont PHP | **FFI** intégré à PHP (`ext-ffi`), PHP ≥ 8.1 (testé 8.3 en CI, 8.5 en local) | la porte déjà percée dans PHP depuis 2019 pour appeler une bibliothèque native, sans rien compiler de spécifique à PHP (§5.2 du brief) |
| Pont Go | **`purego`** `v0.10.2`, Go 1.21 | l'appel direct d'une bibliothèque native sans outillage de compilation, sans `cgo` |
| En-tête C | **écrit à la main** (`crates/memless-capi/include/memless.h`) | pas de `cbindgen` : la surface C ABI est petite et stable, l'en-tête à jour tient dans un seul fichier maintenu à la main (`FFI_SCOPE "memless"`) |
| Format de la bibliothèque | **cdylib + staticlib + rlib**, `memless-capi` (`.so`/`.dylib`, jamais `.dll`) | un **cœur** compilé une fois, chargé par les trois ponts sans logique dupliquée ; Windows et le format `.dll` sont exclus au lancement (`ARCHITECTURE.md` §12.3 Q7) |
| Tests | **`cargo test`** + **banc de parité** multi-langages (`harness/parity/`) | le contrôle qualité : le banc de parité est le test décisif du §5.1 du brief (même SQL + même état → même résultat dans les trois langages) |
| Qualité | **`rustfmt`**, **`clippy`** (`-D warnings`) | le gabarit et le détecteur de défauts du code Rust |
| CI/CD | **GitHub Actions** — matrice sur les quatre familles de systèmes retenues (`ARCHITECTURE.md` §12.3 Q7) | l'atelier qui compile la bibliothèque pour chaque plateforme et rejoue le banc de parité sur chaque famille, à chaque push et PR, et publie sur tag |
| Protocole API | **aucun** | Memless n'expose pas d'API réseau : le seul « protocole » est l'appel FFI en mémoire |
| Edge/CDN, BFF, broker | **aucun** | ni réseau, ni distribution, ni messagerie — hors sujet pour une bibliothèque embarquée (§4.1) |
| Observabilité | **aucune au lancement** | pas de service à surveiller ; les erreurs remontent comme valeurs de retour au code appelant, l'audit des données passe par Git (§9.1 du brief) |
| Cloud | **aucun** | rien à héberger ; distribution par **GitHub Releases** sur tag (`ARCHITECTURE.md` §12.3 Q6) |

**Sous-ensemble SQL constaté** : lecture avec filtre, tri (`ORDER BY`), jointure par relation devinée,
compte et somme ; insertion, mise à jour, suppression ; `BEGIN`/`COMMIT`/`ROLLBACK` ; `LIMIT`,
`OFFSET`, `GROUP BY`, `DISTINCT` restent hors sous-ensemble (refusés « outside the supported SQL
subset »). Les familles de systèmes publiées au lancement sont les quatre de Q7 ; le rechargement
(`memless_reload`) est un verbe natif, pas du SQL (§4).
