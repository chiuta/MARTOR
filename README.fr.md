# MARTOR — Capture web à valeur probante : WARC/WACZ avec empreintes, signatures et horodatage de confiance, vérifiable hors ligne.

[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version : 0.1.0-alpha.1 · Dernière mise à jour : 2026-11-14

🌐 https://martor.chiuta.ro

## État

Squelette précoce (pré-alpha) : le workspace compile et les tests passent en CI. Chaque fonctionnalité prévue est une tâche ouverte dans `docs/issues/`.

## Périmètre prévu

- Enregistrements WARC 1.1 de requête/réponse pour une capture HTTPS, avec empreintes de charge utile et de bloc (SHA-256).
- Empaquetage WACZ 1.1.1 : `datapackage.json`, `pages.jsonl` et un index CDXJ.
- Signature avec une clé Ed25519 locale (WACZ-Auth) et horodatage de confiance RFC 3161 auprès d’une TSA configurable.
- `martor verify FILE.wacz` : vérificateur hors ligne aux codes de sortie clairs, appuyé par une suite de tests d’altération.
- Capture des sous-ressources (CSS, images, srcset) et, en option, capture par navigateur via le protocole DevTools.

Rien de tout cela n’est encore implémenté : chaque point est une tâche ouverte dans `docs/issues/`.

## Compiler et tester

```bash
cargo test --workspace
cargo run -p martor-cli -- sha256 Cargo.toml
```

## Contribuer

Lisez `CONTRIBUTING.md`, choisissez une tâche étiquetée `good first issue` dans `docs/issues/` et ouvrez une pull request. Aucune contribution n'est trop petite : une ligne de table, un test ou une traduction comptent.

## Implémentation indépendante

Écrite à partir de spécifications publiques et de manuels ; aucun code source tiers n'a été copié (voir `CLEANROOM.md`).

## Licence

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Soutien

Trade-free : ni pistage ni publicité. Si cela vous est utile, vous pouvez soutenir l'auteur : https://buymeacoffee.com/echo.of.the.strings
