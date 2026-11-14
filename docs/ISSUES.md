# MARTOR — first ten tasks

**English** · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **Evaluate `warc` vs `warcat` vs own writer; write the decision in `docs/ADR-001.md`** — `help wanted`, `docs`, `area:warc`
   Compare licences (MIT-compatible only), API, maintenance, gzip per-record support. Prototype one `response` record with each.

2. **Write WARC 1.1 request + response records for a single HTTPS GET** — `help wanted`, `area:warc`
   Include `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI`. Test by re-reading with an independent reader.

3. **`WARC-Payload-Digest` and `WARC-Block-Digest` with the in-tree SHA-256** — `good first issue`, `area:warc`
   Use `martor_core::sha256`; add streaming API (`update`/`finish`) so large bodies need not fit in memory.

4. **Package a WACZ 1.1.1: `datapackage.json`, `pages.jsonl`, CDXJ index** — `help wanted`, `area:wacz`
   Zip without compression for the WARC. Must open in ReplayWeb.page (document how you checked).

5. **WACZ-Auth signing with a local key (Ed25519 first)** — `help wanted`, `area:signing`
   Produce `datapackage-digest.json` per the draft; note deviations from the draft in the code.

6. **RFC 3161 timestamp client with a configurable TSA URL** — `help wanted`, `area:tsa`
   Build the request, parse the response token, store `.tsr` next to the archive. No default TSA hard-coded; document how to choose one.

7. **`martor verify FILE.wacz` — offline verifier with clear exit codes** — `help wanted`, `area:cli`
   0 = all checks pass, 1 = tampered, 2 = unsupported/malformed. Print each check.

8. **Tamper test-suite: flip one byte in every file of an archive** — `good first issue`, `tests`
   Generate a sample archive in the test, flip each byte range once, assert `verify` fails and says where.

9. **Capture page sub-resources (CSS, images, srcset) with VERIGA's link extraction rules** — `help wanted`, `area:capture`
   Study VERIGA's extractor (MIT) and port the rules; respect robots.txt by default; polite rate limits.

10. **Optional browser capture via the DevTools protocol for JavaScript-heavy pages** — `help wanted`, `area:capture`
   Behind a cargo feature; design note first (which browser, how to record the network layer).
