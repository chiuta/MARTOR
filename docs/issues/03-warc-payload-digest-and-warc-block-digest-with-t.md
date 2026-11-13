`WARC-Payload-Digest` and `WARC-Block-Digest` with the in-tree SHA-256
good first issue,area:warc

Use `martor_core::sha256`; add streaming API (`update`/`finish`) so large bodies need not fit in memory.

See `CONTRIBUTING.md`. Cite the public source you follow in the PR and in `CLEANROOM.md`.
