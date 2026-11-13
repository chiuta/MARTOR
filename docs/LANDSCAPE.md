# Related projects and where MARTOR fits

*Checked with a limited web search on 13 November 2026. Facts change quickly: re-check before announcing or depending on anything below.*

- **WARC in Rust:** `warc` (reader/writer), `warcat` (CLI/library, MPL-2.0), `webaves` (suite, work in progress: crawler and capture are still planned).
- **WACZ in Rust:** `wacksy` (Bodleian Libraries; experimental, writing only): <https://blogs.bodleian.ox.ac.uk/archivesandmanuscripts/2025/12/17/introducing-wacksy-a-library-for-writing-wacz-collections/>.
- **Signing standard:** WACZ-Auth draft (X.509 signature + RFC 3161 timestamp): <https://specs.webrecorder.net/wacz-auth/0.1.0/>.
- **Others:** Scoop (Harvard LIL, browser-based capture), Webrecorder tools (JS/Python), commercial services such as PageCrawl that add RFC 3161 timestamps.
- **Gap targeted:** one Rust binary that captures, signs, timestamps and *verifies offline*, reusing the crawler from VERIGA. Evidential value depends on jurisdiction and procedure — nothing here is legal advice.
