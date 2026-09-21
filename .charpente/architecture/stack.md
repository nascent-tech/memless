<!-- charpente-navigation -->
**Index** : [Memless — Architecture](../../ARCHITECTURE.md)  
**Précédent** : [§6. Fondations logicielles](software-foundations.md)  
**Suivant** : [§8. Données](data.md)

*Bloc généré — il se retisse à chaque écriture, ne l'édite pas à la main.*
<!-- /charpente-navigation -->

## §7. Stack

Chaque outil est retenu pour ce que le brief exige, jamais pour sa popularité. Les versions citées par
le brief sont reprises telles quelles ; celles marquées **« à figer au plan »** seront choisies et
épinglées à `/charpente:plan`, faute de source dans le brief à ce stade.

| Catégorie | Outil et version | Métaphore et justification |
|---|---|---|
| Langage du cœur | **Rust** — édition à figer au plan | le langage qui rattrape les erreurs de mémoire avant même de tourner, comme un filet posé sous le trapèze : imposé par le brief (décision 12 du brief) pour un moteur transactionnel, et recommandé par les agences de cybersécurité (§18 du brief) |
| Moteur SQL | **GlueSQL** (0.20.0, Apache-2.0) | le juriste qui lit et applique le SQL sans rien inventer : réutilisé pour ne pas réécrire la partie la plus risquée d'un moteur ; utilisé **sans schéma déclaré**, seul mode acceptant des types mêlés par colonne (§5.2 du brief) |
| Lecture/écriture YAML | **serde-saphyr** 1.3.0 + **`IndexMap`** (repli : `serde_yaml_ng` 0.10.0) | l'ouvre-boîte et le copiste du fichier — `serde_yaml` est archivé (mars 2024) ; `serde-saphyr` est maintenu, s'intègre à serde et **borne le « YAML bomb »** par un `Budget` configurable (§10) ; l'ordre des colonnes passe par `IndexMap` côté modèle (décision tranchée, ex-question 4) |
| Pont Node/TS | **napi-rs** — version à figer au plan (dernière stable vérifiée) | le traducteur qui fait passer le cœur pour une fonction JavaScript ordinaire, sur Node-API stable (§18 du brief) |
| Pont PHP | **FFI** intégré à PHP — version à figer au plan (brief : ≥ 7.4, §18 du brief) | la porte déjà percée dans PHP depuis 2019 pour appeler une bibliothèque native, sans rien compiler de spécifique à PHP (§5.2 du brief) |
| Pont Go | **purego** (0.10.2), repli **cgo** | l'appel direct d'une bibliothèque native sans outillage de compilation ; `purego` se dit bêta, d'où `cgo` documenté en repli (§5.2, §18 du brief) |
| En-tête C | **cbindgen** — version à figer au plan | le plan qui décrit à Go et PHP la forme exacte des fonctions exposées par la bibliothèque compilée |
| Format de la bibliothèque | **cdylib** `memless-capi` (`.so`/`.dylib`/`.dll`) + **addon** `.node` (napi-rs) | un **cœur** compilé une fois, exposé par deux surfaces minces sans logique dupliquée : `memless-capi` (C ABI, Go/PHP) et l'addon Node — le brief partage le cœur, pas nécessairement un fichier natif unique pour Node (§5.2 du brief, §12.3 question 3) |
| Tests | **cargo test** + **banc de parité** multi-langages | le contrôle qualité : le banc de parité est le test décisif du §5.1 du brief (même SQL + même état → même résultat dans les trois langages) |
| Qualité | **rustfmt**, **clippy** | le gabarit et le détecteur de défauts du code Rust |
| CI/CD | **à choisir** (§12.3, question 5) — matrice OS × langage | l'atelier qui compile la bibliothèque pour chaque plateforme et rejoue le banc de parité |
| Protocole API | **aucun** | Memless n'expose pas d'API réseau : le seul « protocole » est l'appel FFI en mémoire |
| Edge/CDN, BFF, broker | **aucun** | ni réseau, ni distribution, ni messagerie — hors sujet pour une bibliothèque embarquée (§4.1) |
| Observabilité | **aucune au lancement** | pas de service à surveiller ; les erreurs remontent comme valeurs de retour au code appelant, l'audit des données passe par Git (§9.1 du brief) |
| Cloud | **aucun** | rien à héberger ; la distribution se fait par paquet (npm, Packagist, module Go — §12.3, question 6) |

**Ce que le brief laisse à `/charpente:plan`** : l'étendue exacte du sous-ensemble SQL supporté (§8.3
du brief), le nombre de combinaisons OS × famille de processeurs à publier au lancement (§5.2 du
brief), et la bibliothèque YAML précise. Aucun de ces points ne bloque l'architecture ; chacun est un
choix de mise en œuvre.
