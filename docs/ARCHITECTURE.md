# MARTOR — architecture

`martor-core` holds hashing and the manifest (std-only). Planned crates: `martor-warc` (records), `martor-wacz` (package + WACZ-Auth signing),
`martor-tsa` (RFC 3161 client), `martor-cli` (`capture`, `verify`). Standards first: WARC 1.1, WACZ 1.1.1, WACZ-Auth draft, RFC 3161.
Rule: a `.wacz` made by MARTOR must open in ReplayWeb.page, and a `.wacz` made elsewhere must verify in `martor verify` when it carries a signature.
