# MARTOR — Forensic web capture: WARC/WACZ with hashes, signatures and trusted timestamps, verifiable offline.

**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Last updated: 2026-11-14

🌐 https://martor.chiuta.ro

## Status

Early skeleton (pre-alpha): the workspace builds and the tests pass in CI. Every planned feature is an open task in `docs/issues/`.

## Planned scope

- WARC 1.1 request/response records for an HTTPS capture, with payload and block digests (SHA-256).
- WACZ 1.1.1 packaging: `datapackage.json`, `pages.jsonl` and a CDXJ index.
- Signing with a local Ed25519 key (WACZ-Auth) and RFC 3161 trusted timestamps from a configurable TSA.
- `martor verify FILE.wacz`: offline verifier with clear exit codes, backed by a tamper test-suite.
- Sub-resource capture (CSS, images, srcset) and, optionally, browser capture over the DevTools protocol.

None of this is implemented yet: each item is an open task in `docs/issues/`.

## Build and test

```bash
cargo test --workspace
cargo run -p martor-cli -- sha256 Cargo.toml
```

## Contribute

Read `CONTRIBUTING.md`, pick a task labelled `good first issue` from `docs/issues/` and open a pull request. No contribution is too small: a table row, a test or a translation counts.

## Independent implementation

Written from public specifications and textbooks; no third-party source code is copied (see `CLEANROOM.md`).

## License

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Support

Trade-free: no tracking, no ads. If it is useful, you can support the author: https://buymeacoffee.com/echo.of.the.strings
