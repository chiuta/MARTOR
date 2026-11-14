# MARTOR — Captura web com valor probatório: WARC/WACZ com hashes, assinaturas e carimbo de tempo confiável, verificável offline.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0-alpha.1 · Última atualização: 2026-11-14

🌐 https://martor.chiuta.ro

## Estado

Esqueleto inicial (pré-alfa): o workspace compila e os testes passam no CI. Cada funcionalidade prevista é uma tarefa aberta em `docs/issues/`.

## Âmbito previsto

- Registos WARC 1.1 de pedido/resposta para uma captura HTTPS, com resumos de conteúdo e de bloco (SHA-256).
- Empacotamento WACZ 1.1.1: `datapackage.json`, `pages.jsonl` e um índice CDXJ.
- Assinatura com uma chave Ed25519 local (WACZ-Auth) e carimbo temporal de confiança RFC 3161 de uma TSA configurável.
- `martor verify FILE.wacz`: verificador offline com códigos de saída claros, apoiado por uma bateria de testes de adulteração.
- Captura de subrecursos (CSS, imagens, srcset) e, opcionalmente, captura por navegador através do protocolo DevTools.

Nada disto está ainda implementado: cada ponto é uma tarefa aberta em `docs/issues/`.

## Compilar e testar

```bash
cargo test --workspace
cargo run -p martor-cli -- sha256 Cargo.toml
```

## Contribuir

Leia `CONTRIBUTING.md`, escolha uma tarefa com a etiqueta `good first issue` em `docs/issues/` e abra um pull request. Nenhuma contribuição é demasiado pequena: uma linha de tabela, um teste ou uma tradução contam.

## Implementação independente

Escrita a partir de especificações públicas e manuais; não foi copiado código-fonte de terceiros (ver `CLEANROOM.md`).

## Licença

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoio

Trade-free: sem rastreamento nem publicidade. Se lhe for útil, pode apoiar o autor: https://buymeacoffee.com/echo.of.the.strings
