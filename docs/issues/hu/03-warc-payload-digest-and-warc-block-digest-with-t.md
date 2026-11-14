`WARC-Payload-Digest` és `WARC-Block-Digest` a fában lévő SHA-256-tal
good first issue,area:warc

Használd a `martor_core::sha256`-ot; adj hozzá streaming API-t (`update`/`finish`), hogy a nagy törzseknek ne kelljen beleférniük a memóriába.

Lásd a `CONTRIBUTING.md` fájlt. Add meg a követett nyilvános forrást a PR-ban és a `CLEANROOM.md`-ben.
