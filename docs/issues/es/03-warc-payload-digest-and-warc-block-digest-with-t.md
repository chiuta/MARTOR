`WARC-Payload-Digest` y `WARC-Block-Digest` con el SHA-256 del árbol
good first issue,area:warc

Usa `martor_core::sha256`; añade una API de streaming (`update`/`finish`) para que los cuerpos grandes no tengan que caber en memoria.

Consulta `CONTRIBUTING.md`. Cita la fuente pública que sigas en el PR y en `CLEANROOM.md`.
