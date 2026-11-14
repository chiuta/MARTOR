# MARTOR — az első tíz feladat

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · **Magyar**

Hozd létre őket a GitHubon a `scripts/create-issues.sh OWNER/REPO` paranccsal. Szólj hozzá egy issue-hoz, hogy lefoglald.

1. **Értékeld a `warc`-ot, a `warcat`-ot és egy saját writert; a döntést írd a `docs/ADR-001.md`-be** — `help wanted`, `docs`, `area:warc`
   Hasonlítsd össze a licenceket (csak MIT-kompatibilisek), az API-t, a karbantartást és a rekordonkénti gzip-támogatást. Mindegyikkel készíts prototípust egy `response` rekordból.

2. **Írj WARC 1.1 kérés- és válaszrekordokat egyetlen HTTPS GET-hez** — `help wanted`, `area:warc`
   Tartalmazza a `WARC-Date`, `WARC-Payload-Digest: sha256:…`, `WARC-Target-URI` mezőket. Teszteld egy független olvasóval való újraolvasással.

3. **`WARC-Payload-Digest` és `WARC-Block-Digest` a fában lévő SHA-256-tal** — `good first issue`, `area:warc`
   Használd a `martor_core::sha256`-ot; adj hozzá streaming API-t (`update`/`finish`), hogy a nagy törzseknek ne kelljen beleférniük a memóriába.

4. **WACZ 1.1.1 csomagolása: `datapackage.json`, `pages.jsonl`, CDXJ index** — `help wanted`, `area:wacz`
   Tömörítés nélküli zip a WARC-hoz. Meg kell nyílnia a ReplayWeb.page-en (dokumentáld, hogyan ellenőrizted).

5. **WACZ-Auth aláírás helyi kulccsal (először Ed25519)** — `help wanted`, `area:signing`
   Állítsd elő a `datapackage-digest.json`-t a tervezet szerint; a kódban jegyezd fel a tervezettől való eltéréseket.

6. **RFC 3161 időbélyegző kliens beállítható TSA URL-lel** — `help wanted`, `area:tsa`
   Építsd fel a kérést, elemezd a válasz tokenjét, a `.tsr`-t tárold az archívum mellett. Nincs alapértelmezett, beégetett TSA; dokumentáld, hogyan lehet választani.

7. **`martor verify FILE.wacz` — offline ellenőrző egyértelmű kilépési kódokkal** — `help wanted`, `area:cli`
   0 = minden ellenőrzés sikeres, 1 = manipulált, 2 = nem támogatott/hibás. Írd ki az egyes ellenőrzéseket.

8. **Manipulációs tesztcsomag: fordíts meg egy bájtot az archívum minden fájljában** — `good first issue`, `tests`
   Generálj a tesztben mintaarchívumot, minden bájttartományt egyszer módosíts, és ellenőrizd, hogy a `verify` megbukik, és megmondja, hol.

9. **Az oldal alerőforrásainak (CSS, képek, srcset) rögzítése a VERIGA hivatkozáskinyerési szabályaival** — `help wanted`, `area:capture`
   Tanulmányozd a VERIGA kinyerőjét (MIT), és portáld a szabályokat; alapértelmezetten tartsd tiszteletben a robots.txt-t; udvarias sebességkorlátok.

10. **Opcionális böngészős rögzítés a DevTools protokollon át JavaScript-nehéz oldalakhoz** — `help wanted`, `area:capture`
   Cargo feature mögött; először tervezési jegyzet (melyik böngésző, hogyan rögzítsük a hálózati réteget).
