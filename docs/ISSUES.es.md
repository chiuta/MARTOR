# MARTOR — primeras diez tareas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · **Español** · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créalas en GitHub con `scripts/create-issues.sh OWNER/REPO`. Comenta en una issue para reclamarla.

1. **Evalúa `warc` vs `warcat` vs un writer propio; escribe la decisión en `docs/ADR-001.md`** — `help wanted`, `docs`, `area:warc`
   Compara licencias (solo compatibles con MIT), API, mantenimiento y soporte de gzip por registro. Prototipa un registro `response` con cada uno.

2. **Escribe registros WARC 1.1 de petición + respuesta para un solo GET HTTPS** — `help wanted`, `area:warc`
   Incluye `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI`. Prueba releyendo con un lector independiente.

3. **`WARC-Payload-Digest` y `WARC-Block-Digest` con el SHA-256 del árbol** — `good first issue`, `area:warc`
   Usa `martor_core::sha256`; añade una API de streaming (`update`/`finish`) para que los cuerpos grandes no tengan que caber en memoria.

4. **Empaqueta un WACZ 1.1.1: `datapackage.json`, `pages.jsonl`, índice CDXJ** — `help wanted`, `area:wacz`
   Zip sin compresión para el WARC. Debe abrirse en ReplayWeb.page (documenta cómo lo comprobaste).

5. **Firma WACZ-Auth con una clave local (primero Ed25519)** — `help wanted`, `area:signing`
   Produce `datapackage-digest.json` según el borrador; anota en el código las desviaciones respecto al borrador.

6. **Cliente de sello de tiempo RFC 3161 con URL de TSA configurable** — `help wanted`, `area:tsa`
   Construye la petición, analiza el token de respuesta, guarda el `.tsr` junto al archivo. Sin ningún TSA fijado por defecto; documenta cómo elegir uno.

7. **`martor verify FILE.wacz` — verificador sin conexión con códigos de salida claros** — `help wanted`, `area:cli`
   0 = todas las comprobaciones pasan, 1 = alterado, 2 = no admitido/mal formado. Imprime cada comprobación.

8. **Batería de pruebas de alteración: invierte un byte en cada archivo de un archivo comprimido** — `good first issue`, `tests`
   Genera un archivo de muestra en la prueba, modifica cada rango de bytes una vez, comprueba que `verify` falla y dice dónde.

9. **Captura los subrecursos de la página (CSS, imágenes, srcset) con las reglas de extracción de enlaces de VERIGA** — `help wanted`, `area:capture`
   Estudia el extractor de VERIGA (MIT) y porta las reglas; respeta robots.txt por defecto; límites de frecuencia corteses.

10. **Captura opcional con navegador mediante el protocolo DevTools para páginas con mucho JavaScript** — `help wanted`, `area:capture`
   Tras una funcionalidad de cargo; primero una nota de diseño (qué navegador, cómo registrar la capa de red).
