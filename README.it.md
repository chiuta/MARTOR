# MARTOR — Cattura web con valore probatorio: WARC/WACZ con hash, firme e marca temporale affidabile, verificabili offline.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · **Italiano** · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versione: 0.1.0-alpha.1 · Ultimo aggiornamento: 2026-11-14

🌐 https://martor.chiuta.ro

## Stato

Scheletro iniziale (pre-alfa): il workspace compila e i test passano in CI. Ogni funzione prevista è un'attività aperta in `docs/issues/`.

## Ambito previsto

- Record WARC 1.1 di richiesta/risposta per una cattura HTTPS, con digest di payload e di blocco (SHA-256).
- Impacchettamento WACZ 1.1.1: `datapackage.json`, `pages.jsonl` e un indice CDXJ.
- Firma con una chiave Ed25519 locale (WACZ-Auth) e marca temporale fidata RFC 3161 da una TSA configurabile.
- `martor verify FILE.wacz`: verificatore offline con codici di uscita chiari, sostenuto da una suite di test di manomissione.
- Cattura delle sottorisorse (CSS, immagini, srcset) e, facoltativamente, cattura via browser con il protocollo DevTools.

Nulla di tutto ciò è ancora implementato: ogni voce è un compito aperto in `docs/issues/`.

## Compilare e testare

```bash
cargo test --workspace
cargo run -p martor-cli -- sha256 Cargo.toml
```

## Contribuire

Leggi `CONTRIBUTING.md`, scegli un'attività con etichetta `good first issue` in `docs/issues/` e apri una pull request. Nessun contributo è troppo piccolo: una riga di tabella, un test o una traduzione contano.

## Implementazione indipendente

Scritta a partire da specifiche pubbliche e manuali; nessun codice sorgente di terzi è stato copiato (vedi `CLEANROOM.md`).

## Licenza

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sostegno

Trade-free: niente tracciamento, niente pubblicità. Se ti è utile, puoi sostenere l'autore: https://buymeacoffee.com/echo.of.the.strings
