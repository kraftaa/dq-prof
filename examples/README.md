Example datasets:
- clean_sales.csv: small healthy sample (10 rows).
- bad_sales.csv: stale timestamps + null revenue to trigger issues.
- demo_alerts.csv: guaranteed CRITICAL/WARNING output for screenshots.
- drift_sales.csv: same schema as clean_sales but region collapses to US to show skew drift vs baseline.
- titanic.csv: download via ./download_titanic.sh (public seaborn data).
- yellow.parquet: download via ./download_yellow.sh (NYC taxi Jan 2024, ~300MB).

Commands:
  dq-prof examples/wow_demo.csv --fail-on warning --color never
  dq-prof examples/titanic.csv --fail-on warning   # after download
  dq-prof examples/yellow.parquet --sample-rows 500000 --allow-sampled-baseline --save-baseline baseline.json
  dq-prof examples/yellow.parquet --baseline baseline.json --fail-on warning
  dq-prof examples/clean_sales.csv --full-scan --save-baseline baseline_clean.json
  dq-prof examples/drift_sales.csv --baseline baseline_clean.json --fail-on warning
