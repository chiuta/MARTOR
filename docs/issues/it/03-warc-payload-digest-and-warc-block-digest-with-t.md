`WARC-Payload-Digest` e `WARC-Block-Digest` con lo SHA-256 dell’albero
good first issue,area:warc

Usa `martor_core::sha256`; aggiungi un’API in streaming (`update`/`finish`) così i corpi grandi non devono stare in memoria.

Vedi `CONTRIBUTING.md`. Cita la fonte pubblica che segui nella PR e in `CLEANROOM.md`.
