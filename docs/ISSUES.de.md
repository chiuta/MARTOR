# MARTOR — die ersten zehn Aufgaben

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · **Deutsch** · [Magyar](ISSUES.hu.md)

Lege sie auf GitHub mit `scripts/create-issues.sh OWNER/REPO` an. Kommentiere ein Issue, um es zu übernehmen.

1. **`warc` vs `warcat` vs eigenen Writer bewerten; Entscheidung in `docs/ADR-001.md` festhalten** — `help wanted`, `docs`, `area:warc`
   Lizenzen (nur MIT-kompatible), API, Wartung und gzip-Unterstützung pro Datensatz vergleichen. Mit jedem einen `response`-Datensatz als Prototyp bauen.

2. **WARC-1.1-Anfrage- und Antwortdatensätze für einen einzelnen HTTPS-GET schreiben** — `help wanted`, `area:warc`
   `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI` einschließen. Testen, indem mit einem unabhängigen Leser erneut gelesen wird.

3. **`WARC-Payload-Digest` und `WARC-Block-Digest` mit dem SHA-256 aus dem Repository** — `good first issue`, `area:warc`
   `martor_core::sha256` verwenden; eine Streaming-API (`update`/`finish`) ergänzen, damit große Bodys nicht in den Speicher passen müssen.

4. **Ein WACZ 1.1.1 verpacken: `datapackage.json`, `pages.jsonl`, CDXJ-Index** — `help wanted`, `area:wacz`
   Zip ohne Kompression für das WARC. Muss sich in ReplayWeb.page öffnen lassen (dokumentieren, wie geprüft wurde).

5. **WACZ-Auth-Signatur mit einem lokalen Schlüssel (zuerst Ed25519)** — `help wanted`, `area:signing`
   `datapackage-digest.json` gemäß Entwurf erzeugen; Abweichungen vom Entwurf im Code vermerken.

6. **RFC-3161-Zeitstempel-Client mit konfigurierbarer TSA-URL** — `help wanted`, `area:tsa`
   Anfrage bauen, Antwort-Token parsen, `.tsr` neben dem Archiv ablegen. Keine fest eingebaute Standard-TSA; dokumentieren, wie man eine wählt.

7. **`martor verify FILE.wacz` — Offline-Prüfer mit klaren Exit-Codes** — `help wanted`, `area:cli`
   0 = alle Prüfungen bestanden, 1 = manipuliert, 2 = nicht unterstützt/fehlerhaft. Jede Prüfung ausgeben.

8. **Manipulationstest-Suite: in jeder Datei eines Archivs ein Byte kippen** — `good first issue`, `tests`
   Im Test ein Beispielarchiv erzeugen, jeden Byte-Bereich einmal verändern, prüfen, dass `verify` fehlschlägt und sagt, wo.

9. **Teilressourcen der Seite (CSS, Bilder, srcset) mit den Link-Extraktionsregeln von VERIGA erfassen** — `help wanted`, `area:capture`
   Den Extraktor von VERIGA (MIT) studieren und die Regeln portieren; robots.txt standardmäßig beachten; höfliche Ratenbegrenzung.

10. **Optionale Browser-Erfassung über das DevTools-Protokoll für JavaScript-lastige Seiten** — `help wanted`, `area:capture`
   Hinter einem Cargo-Feature; zuerst eine Designnotiz (welcher Browser, wie die Netzwerkschicht aufgezeichnet wird).
