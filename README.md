# dq-prof 0.1.8

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

Titanic (real public data):
```bash
./examples/download_titanic.sh
dq-prof examples/titanic.csv --fail-on warning

DATA HEALTH: FAIL
Rows: 891 sampled of 891 (mode=Head)

CRITICAL
- age: high null ratio 19.87% (obs=0.1987, exp=< 0.05)
- deck: high null ratio 77.22% (obs=0.7722, exp=< 0.05)

WARNING
- sibsp: outlier ratio 3.37% (obs=0.0337, exp=< 0.03)
- survived: distinct ratio very low (obs=0.0022, exp=> 0.01)
- pclass: distinct ratio very low (obs=0.0034, exp=> 0.01)
- sex: distinct ratio very low (obs=0.0022, exp=> 0.01)
- sibsp: distinct ratio very low (obs=0.0079, exp=> 0.01)
```

Baseline vs drift example:
```bash
dq-prof examples/clean_sales.csv --full-scan --save-baseline baseline_clean.json
dq-prof examples/drift_sales.csv --baseline baseline_clean.json --fail-on warning --color never

DATA HEALTH: WARN
CRITICAL
- region: top value dominates 100.0% of rows (obs=1.0000, exp=< 0.75)

WARNING
- region: top value share increased by 40.0pp vs baseline (obs=1.0000, exp=0.6000)
```

## Why dq-prof

- **Zero config** – no YAML, no expectations to write
- **Fast** – runs in seconds on sampled data
- **Catches real issues** – null spikes, skew, outliers, freshness, schema drift
- **Baseline-aware** – detect changes vs previous runs
- **CI-friendly** – fail pipelines when data looks wrong

## Install

Install via pip:
```bash
pip install dq-prof
dq-prof --help
```

## Examples

Inspect a file:
```bash
dq-prof examples/clean_sales.csv
```

Compare to baseline (full scan required to save):
```bash
dq-prof data.csv --full-scan --save-baseline baseline.json
dq-prof data.csv --baseline baseline.json --fail-on warning
```

Postgres:
```bash
dq-prof public.sales \
  --pg-url postgres://user:pass@host/db \
  --sample-rows 50000
```

JSON output:
```bash
dq-prof data.parquet --format json
```

## Output

Text or JSON with severity:
- **CRITICAL** – likely broken data
- **WARNING** – suspicious change

## Philosophy

dq-prof is not a data observability platform. It’s a fast sanity check you run inline — a linter for data — before or after a pipeline step to catch issues immediately.
