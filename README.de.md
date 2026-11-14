# MARTOR — Beweissichere Webarchivierung: WARC/WACZ mit Hashes, Signaturen und vertrauenswürdigem Zeitstempel, offline prüfbar.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · **Deutsch** · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Letzte Aktualisierung: 2026-11-14

🌐 https://martor.chiuta.ro

## Status

Frühes Grundgerüst (Pre-Alpha): Das Workspace baut, und die Tests laufen in der CI durch. Jede geplante Funktion ist eine offene Aufgabe in `docs/issues/`.

## Geplanter Umfang

- WARC-1.1-Anfrage-/Antwort-Datensätze für eine HTTPS-Erfassung, mit Nutzdaten- und Block-Digests (SHA-256).
- WACZ-1.1.1-Paketierung: `datapackage.json`, `pages.jsonl` und ein CDXJ-Index.
- Signieren mit einem lokalen Ed25519-Schlüssel (WACZ-Auth) und vertrauenswürdige RFC-3161-Zeitstempel einer konfigurierbaren TSA.
- `martor verify FILE.wacz`: Offline-Prüfer mit klaren Exit-Codes, abgesichert durch eine Manipulationstestsuite.
- Erfassung von Unterressourcen (CSS, Bilder, srcset) und optional Erfassung über den Browser per DevTools-Protokoll.

Nichts davon ist bisher umgesetzt: Jeder Punkt ist eine offene Aufgabe in `docs/issues/`.

## Bauen und testen

```bash
cargo test --workspace
cargo run -p martor-cli -- sha256 Cargo.toml
```

## Mitmachen

Lies `CONTRIBUTING.md`, wähle in `docs/issues/` eine Aufgabe mit dem Label `good first issue` und öffne einen Pull Request. Kein Beitrag ist zu klein: eine Tabellenzeile, ein Test oder eine Übersetzung zählt.

## Unabhängige Implementierung

Geschrieben nach öffentlichen Spezifikationen und Lehrbüchern; es wurde kein fremder Quellcode kopiert (siehe `CLEANROOM.md`).

## Lizenz

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Unterstützung

Trade-free: kein Tracking, keine Werbung. Wenn es dir nützt, kannst du den Autor unterstützen: https://buymeacoffee.com/echo.of.the.strings
