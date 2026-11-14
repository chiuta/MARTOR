# MARTOR — Captura web con valor probatorio: WARC/WACZ con hashes, firmas y sello de tiempo de confianza, verificable sin conexión.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0-alpha.1 · Última actualización: 2026-11-14

🌐 https://martor.chiuta.ro

## Estado

Esqueleto inicial (pre-alfa): el workspace compila y las pruebas pasan en CI. Cada función prevista es una tarea abierta en `docs/issues/`.

## Alcance previsto

- Registros WARC 1.1 de solicitud/respuesta para una captura HTTPS, con resúmenes de carga útil y de bloque (SHA-256).
- Empaquetado WACZ 1.1.1: `datapackage.json`, `pages.jsonl` y un índice CDXJ.
- Firma con una clave Ed25519 local (WACZ-Auth) y sello de tiempo de confianza RFC 3161 de una TSA configurable.
- `martor verify FILE.wacz`: verificador sin conexión con códigos de salida claros, respaldado por una batería de pruebas de manipulación.
- Captura de subrecursos (CSS, imágenes, srcset) y, opcionalmente, captura mediante navegador con el protocolo DevTools.

Nada de esto está implementado todavía: cada punto es una tarea abierta en `docs/issues/`.

## Compilar y probar

```bash
cargo test --workspace
cargo run -p martor-cli -- sha256 Cargo.toml
```

## Contribuir

Lee `CONTRIBUTING.md`, elige una tarea con la etiqueta `good first issue` en `docs/issues/` y abre un pull request. Ninguna contribución es demasiado pequeña: una fila de tabla, una prueba o una traducción cuentan.

## Implementación independiente

Escrita a partir de especificaciones públicas y manuales; no se ha copiado código fuente de terceros (véase `CLEANROOM.md`).

## Licencia

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoyo

Trade-free: sin rastreo ni publicidad. Si te resulta útil, puedes apoyar al autor: https://buymeacoffee.com/echo.of.the.strings
