# MARTOR — Captură web cu valoare de probă: WARC/WACZ cu hash-uri, semnături și timestamp de încredere, verificabile offline.

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0-alpha.1 · Ultima actualizare: 2026-11-14

🌐 https://martor.chiuta.ro

## Stare

Schelet timpuriu (pre-alfa): workspace-ul se compilează, iar testele trec în CI. Fiecare funcție planificată este o sarcină deschisă în `docs/issues/`.

## Domeniu planificat

- Înregistrări WARC 1.1 de cerere/răspuns pentru o captură HTTPS, cu rezumate de conținut și de bloc (SHA-256).
- Împachetare WACZ 1.1.1: `datapackage.json`, `pages.jsonl` și un index CDXJ.
- Semnare cu o cheie Ed25519 locală (WACZ-Auth) și timestamp de încredere RFC 3161 de la un TSA configurabil.
- `martor verify FILE.wacz`: verificator offline cu coduri de ieșire clare, susținut de o suită de teste de alterare.
- Captura subresurselor (CSS, imagini, srcset) și, opțional, captură prin browser cu protocolul DevTools.

Nimic din asta nu este implementat încă: fiecare punct este o sarcină deschisă în `docs/issues/`.

## Compilare și testare

```bash
cargo test --workspace
cargo run -p martor-cli -- sha256 Cargo.toml
```

## Contribuie

Citește `CONTRIBUTING.md`, alege o sarcină etichetată `good first issue` din `docs/issues/` și deschide un pull request. Nicio contribuție nu e prea mică: un rând de tabel, un test sau o traducere contează.

## Implementare independentă

Scrisă pornind de la specificații publice și manuale; nu s-a copiat cod sursă de la terți (vezi `CLEANROOM.md`).

## Licență

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sprijin

Trade-free: fără urmărire, fără reclame. Dacă ți-e de folos, poți sprijini autorul: https://buymeacoffee.com/echo.of.the.strings
