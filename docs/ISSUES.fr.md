# MARTOR — dix premières tâches

[English](ISSUES.md) · [Română](ISSUES.ro.md) · **Français** · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créez-les sur GitHub avec `scripts/create-issues.sh OWNER/REPO`. Commentez un ticket pour le réclamer.

1. **Évaluer `warc` vs `warcat` vs un writer maison ; consigner la décision dans `docs/ADR-001.md`** — `help wanted`, `docs`, `area:warc`
   Comparer les licences (compatibles MIT uniquement), l’API, la maintenance, la prise en charge de gzip par enregistrement. Prototyper un enregistrement `response` avec chacun.

2. **Écrire des enregistrements WARC 1.1 de requête et de réponse pour un seul GET HTTPS** — `help wanted`, `area:warc`
   Inclure `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI`. Tester en relisant avec un lecteur indépendant.

3. **`WARC-Payload-Digest` et `WARC-Block-Digest` avec le SHA-256 du dépôt** — `good first issue`, `area:warc`
   Utiliser `martor_core::sha256` ; ajouter une API de flux (`update`/`finish`) pour que les gros corps n’aient pas à tenir en mémoire.

4. **Empaqueter un WACZ 1.1.1 : `datapackage.json`, `pages.jsonl`, index CDXJ** — `help wanted`, `area:wacz`
   Zip sans compression pour le WARC. Doit s’ouvrir dans ReplayWeb.page (documenter comment vous l’avez vérifié).

5. **Signature WACZ-Auth avec une clé locale (Ed25519 d’abord)** — `help wanted`, `area:signing`
   Produire `datapackage-digest.json` selon le brouillon ; noter dans le code les écarts par rapport au brouillon.

6. **Client d’horodatage RFC 3161 avec une URL TSA configurable** — `help wanted`, `area:tsa`
   Construire la requête, analyser le jeton de réponse, stocker le `.tsr` à côté de l’archive. Aucun TSA codé en dur par défaut ; documenter comment en choisir un.

7. **`martor verify FILE.wacz` — vérificateur hors ligne avec des codes de sortie clairs** — `help wanted`, `area:cli`
   0 = toutes les vérifications passent, 1 = altéré, 2 = non pris en charge/mal formé. Afficher chaque vérification.

8. **Suite de tests d’altération : inverser un octet dans chaque fichier d’une archive** — `good first issue`, `tests`
   Générer une archive d’exemple dans le test, modifier chaque plage d’octets une fois, vérifier que `verify` échoue et indique où.

9. **Capturer les sous-ressources de la page (CSS, images, srcset) avec les règles d’extraction de liens de VERIGA** — `help wanted`, `area:capture`
   Étudier l’extracteur de VERIGA (MIT) et porter les règles ; respecter robots.txt par défaut ; limites de débit courtoises.

10. **Capture optionnelle par navigateur via le protocole DevTools pour les pages riches en JavaScript** — `help wanted`, `area:capture`
   Derrière une fonctionnalité cargo ; note de conception d’abord (quel navigateur, comment enregistrer la couche réseau).
