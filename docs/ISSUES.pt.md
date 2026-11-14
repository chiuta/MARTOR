# MARTOR — primeiras dez tarefas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · **Português** · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Crie-as no GitHub com `scripts/create-issues.sh OWNER/REPO`. Comente numa issue para a reivindicar.

1. **Avalie `warc` vs `warcat` vs um writer próprio; escreva a decisão em `docs/ADR-001.md`** — `help wanted`, `docs`, `area:warc`
   Compare licenças (apenas compatíveis com MIT), API, manutenção e suporte a gzip por registro. Faça um protótipo de um registro `response` com cada um.

2. **Escreva registros WARC 1.1 de requisição + resposta para um único GET HTTPS** — `help wanted`, `area:warc`
   Inclua `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI`. Teste relendo com um leitor independente.

3. **`WARC-Payload-Digest` e `WARC-Block-Digest` com o SHA-256 da árvore** — `good first issue`, `area:warc`
   Use `martor_core::sha256`; adicione uma API de streaming (`update`/`finish`) para que corpos grandes não precisem caber na memória.

4. **Empacote um WACZ 1.1.1: `datapackage.json`, `pages.jsonl`, índice CDXJ** — `help wanted`, `area:wacz`
   Zip sem compressão para o WARC. Deve abrir no ReplayWeb.page (documente como verificou).

5. **Assinatura WACZ-Auth com uma chave local (primeiro Ed25519)** — `help wanted`, `area:signing`
   Produza `datapackage-digest.json` conforme o rascunho; anote no código os desvios em relação ao rascunho.

6. **Cliente de carimbo de tempo RFC 3161 com URL de TSA configurável** — `help wanted`, `area:tsa`
   Monte a requisição, analise o token da resposta, guarde o `.tsr` ao lado do arquivo. Nenhum TSA fixo por padrão; documente como escolher um.

7. **`martor verify FILE.wacz` — verificador offline com códigos de saída claros** — `help wanted`, `area:cli`
   0 = todas as verificações passam, 1 = adulterado, 2 = não suportado/malformado. Imprima cada verificação.

8. **Suíte de testes de adulteração: inverta um byte em cada arquivo de um pacote** — `good first issue`, `tests`
   Gere um pacote de exemplo no teste, altere cada intervalo de bytes uma vez, confirme que `verify` falha e diz onde.

9. **Capture os subrecursos da página (CSS, imagens, srcset) com as regras de extração de links do VERIGA** — `help wanted`, `area:capture`
   Estude o extrator do VERIGA (MIT) e porte as regras; respeite o robots.txt por padrão; limites de taxa educados.

10. **Captura opcional por navegador via protocolo DevTools para páginas com muito JavaScript** — `help wanted`, `area:capture`
   Atrás de uma funcionalidade do cargo; primeiro uma nota de projeto (qual navegador, como registrar a camada de rede).
