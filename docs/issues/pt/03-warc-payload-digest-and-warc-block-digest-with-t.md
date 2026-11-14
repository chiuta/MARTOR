`WARC-Payload-Digest` e `WARC-Block-Digest` com o SHA-256 da árvore
good first issue,area:warc

Use `martor_core::sha256`; adicione uma API de streaming (`update`/`finish`) para que corpos grandes não precisem caber na memória.

Veja `CONTRIBUTING.md`. Cite a fonte pública que seguir no PR e em `CLEANROOM.md`.
