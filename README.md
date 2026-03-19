# dq-prof

Fast CLI to sanity-check CSV/Parquet/Postgres datasets, flag nulls/skew/outliers/freshness/schema drift, and fail CI if data looks wrong.

## Install & run (no Rust needed)
- Download release binary from GitHub Releases (tarball includes `dq-prof`), `chmod +x dq-prof`, then run: `./dq-prof your.parquet`.
- Or pip: `pip install dq-prof` (bundles the binary) then run `dq-prof --help`.

## Quick examples
- Inspect current file: `dq-prof data.parquet --format text`
- Compare to baseline: `dq-prof data.csv --baseline baseline.json --fail-on warning`
- Postgres table sampled randomly: `dq-prof public.sales --pg-url postgres://user:pass@host/db --sample-mode random --sample-rows 50000`

Sample output (text):
```
DATA HEALTH: WARN
Rows: 50k sampled of 2.4M (mode=Random)
CRITICAL
- created_at: stale timestamps (obs=2025-03-01T00:00:00Z)
WARNING
- revenue: high null ratio (obs=0.12, exp=< 0.05)
```

## Why use dq-prof
- Zero-config: sensible built-in rules; no YAML test authoring.
- Fast defaults: sampling with full row counts; Polars-backed stats.
- Baseline-aware: save/compare tiny JSON baselines; warns on schema or distribution drift.
- CI-friendly: exit codes, `--color never`, text/JSON outputs, works on local files or Postgres views.
- Lightweight: single binary or pip wheel; no agents, servers, or heavyweight expectations frameworks.
