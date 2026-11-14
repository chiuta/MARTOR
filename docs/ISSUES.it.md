# MARTOR — primi dieci compiti

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · **Italiano** · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creale su GitHub con `scripts/create-issues.sh OWNER/REPO`. Commenta una issue per prenderla in carico.

1. **Valuta `warc` vs `warcat` vs un writer proprio; scrivi la decisione in `docs/ADR-001.md`** — `help wanted`, `docs`, `area:warc`
   Confronta le licenze (solo compatibili MIT), l’API, la manutenzione, il supporto gzip per record. Prototipa un record `response` con ciascuno.

2. **Scrivi record WARC 1.1 di richiesta + risposta per un singolo GET HTTPS** — `help wanted`, `area:warc`
   Includi `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI`. Testa rileggendo con un lettore indipendente.

3. **`WARC-Payload-Digest` e `WARC-Block-Digest` con lo SHA-256 dell’albero** — `good first issue`, `area:warc`
   Usa `martor_core::sha256`; aggiungi un’API in streaming (`update`/`finish`) così i corpi grandi non devono stare in memoria.

4. **Impacchetta un WACZ 1.1.1: `datapackage.json`, `pages.jsonl`, indice CDXJ** — `help wanted`, `area:wacz`
   Zip senza compressione per il WARC. Deve aprirsi in ReplayWeb.page (documenta come l’hai verificato).

5. **Firma WACZ-Auth con una chiave locale (prima Ed25519)** — `help wanted`, `area:signing`
   Produci `datapackage-digest.json` secondo la bozza; annota nel codice le differenze dalla bozza.

6. **Client di marca temporale RFC 3161 con URL TSA configurabile** — `help wanted`, `area:tsa`
   Costruisci la richiesta, analizza il token della risposta, salva il `.tsr` accanto all’archivio. Nessun TSA predefinito nel codice; documenta come sceglierne uno.

7. **`martor verify FILE.wacz` — verificatore offline con codici di uscita chiari** — `help wanted`, `area:cli`
   0 = tutti i controlli passano, 1 = manomesso, 2 = non supportato/malformato. Stampa ogni controllo.

8. **Suite di test di manomissione: inverti un byte in ogni file di un archivio** — `good first issue`, `tests`
   Genera un archivio di esempio nel test, modifica ogni intervallo di byte una volta, verifica che `verify` fallisca e dica dove.

9. **Cattura le sottorisorse della pagina (CSS, immagini, srcset) con le regole di estrazione dei link di VERIGA** — `help wanted`, `area:capture`
   Studia l’estrattore di VERIGA (MIT) e porta le regole; rispetta robots.txt per impostazione predefinita; limiti di frequenza cortesi.

10. **Cattura opzionale con browser tramite il protocollo DevTools per pagine ricche di JavaScript** — `help wanted`, `area:capture`
   Dietro una funzionalità cargo; prima una nota di progetto (quale browser, come registrare il livello di rete).
