# dq-prof 0.1.4

Fast, zero-config data sanity checks for pipelines. Like Ruff, but for datasets.

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
- **Fast** – samples by default; runs in seconds
- **Catches real issues** – null spikes, skew, outliers, freshness, schema drift
- **Baseline-aware** – compare to previous runs to spot drift
- **CI-friendly** – fail pipelines when data looks suspicious

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
