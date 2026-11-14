# MARTOR — primele zece sarcini

[English](ISSUES.md) · **Română** · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creează-le pe GitHub cu `scripts/create-issues.sh OWNER/REPO`. Comentează la un issue ca să-l revendici.

1. **Evaluează `warc` vs `warcat` vs un writer propriu; scrie decizia în `docs/ADR-001.md`** — `help wanted`, `docs`, `area:warc`
   Compară licențele (doar compatibile MIT), API-ul, întreținerea, suportul gzip per înregistrare. Fă un prototip cu o înregistrare `response` pentru fiecare.

2. **Scrie înregistrări WARC 1.1 de cerere + răspuns pentru un singur GET HTTPS** — `help wanted`, `area:warc`
   Include `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI`. Testează recitind cu un cititor independent.

3. **`WARC-Payload-Digest` și `WARC-Block-Digest` cu SHA-256 din arbore** — `good first issue`, `area:warc`
   Folosește `martor_core::sha256`; adaugă un API de streaming (`update`/`finish`) ca trupurile mari să nu trebuiască să încapă în memorie.

4. **Împachetează un WACZ 1.1.1: `datapackage.json`, `pages.jsonl`, index CDXJ** — `help wanted`, `area:wacz`
   Zip fără compresie pentru WARC. Trebuie să se deschidă în ReplayWeb.page (documentează cum ai verificat).

5. **Semnare WACZ-Auth cu o cheie locală (întâi Ed25519)** — `help wanted`, `area:signing`
   Produce `datapackage-digest.json` conform proiectului de specificație; notează în cod abaterile față de proiect.

6. **Client de ștampilă temporală RFC 3161 cu URL TSA configurabil** — `help wanted`, `area:tsa`
   Construiește cererea, analizează tokenul din răspuns, păstrează `.tsr` lângă arhivă. Niciun TSA implicit fixat în cod; documentează cum se alege unul.

7. **`martor verify FILE.wacz` — verificator offline cu coduri de ieșire clare** — `help wanted`, `area:cli`
   0 = toate verificările trec, 1 = alterat, 2 = nesuportat/malformat. Afișează fiecare verificare.

8. **Suită de teste de alterare: schimbă un octet în fiecare fișier al unei arhive** — `good first issue`, `tests`
   Generează o arhivă de probă în test, modifică fiecare interval de octeți o dată, verifică că `verify` eșuează și spune unde.

9. **Capturează subresursele paginii (CSS, imagini, srcset) cu regulile de extragere a legăturilor din VERIGA** — `help wanted`, `area:capture`
   Studiază extractorul VERIGA (MIT) și portează regulile; respectă implicit robots.txt; limite de rată politicoase.

10. **Captură opțională cu browser prin protocolul DevTools pentru pagini cu mult JavaScript** — `help wanted`, `area:capture`
   În spatele unei funcționalități cargo; întâi o notă de proiectare (ce browser, cum se înregistrează stratul de rețea).
