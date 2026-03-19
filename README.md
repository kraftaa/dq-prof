# dq-prof 0.1.4

[![Release](https://img.shields.io/github/v/release/kraftaa/dq-prof)](https://github.com/kraftaa/dq-prof/releases)

Fast, zero-config data sanity checks for pipelines.

Run one command and instantly see if your data is broken.
Like Ruff, but for datasets.

## Quick example

```bash
dq-prof data.parquet
DATA HEALTH: WARN

CRITICAL
- created_at: stale timestamps (last value 2025-03-01)

WARNING
- revenue: nulls 12% (expected <5%)
- region: skew (US = 78%)
```

## Why dq-prof

- **Zero config** – no YAML, no expectations to write
- **Fast** – runs in seconds on sampled data
- **Catches real issues** – null spikes, skew, outliers, freshness, schema drift
- **Baseline-aware** – detect changes vs previous runs
- **CI-friendly** – fail pipelines when data looks wrong

## Install

Download a release binary and run:
```bash
chmod +x dq-prof
./dq-prof data.parquet
```

Or pip:
```bash
pip install dq-prof
dq-prof --help
```

## Examples

Inspect a file:
```bash
dq-prof examples/clean_sales.csv
```

Compare to baseline:
```bash
dq-prof data.csv --baseline baseline.json --fail-on warning
```

Postgres:
```bash
dq-prof public.sales \
  --pg-url postgres://user:pass@host/db \
  --sample-rows 50000
```

## Output

Text or JSON with severity:
- **CRITICAL** – likely broken data
- **WARNING** – suspicious change

## Philosophy

dq-prof is not a data observability platform. It’s a fast sanity check you run inline — a linter for data — before or after a pipeline step to catch issues immediately.
