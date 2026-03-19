Example datasets:
- clean_sales.csv: small healthy sample (10 rows).
- bad_sales.csv: stale timestamps + null revenue to trigger issues.
- wow_demo.csv: guaranteed CRITICAL/WARNING output for screenshots.
- titanic.csv: download via ./download_titanic.sh (public seaborn data).
- yellow.parquet: download via ./download_yellow.sh (NYC taxi Jan 2024, ~300MB).

Commands:
  dq-prof examples/wow_demo.csv --fail-on warning --color never
  dq-prof examples/titanic.csv --fail-on warning   # after download
  dq-prof examples/yellow.parquet --sample-rows 500000 --allow-sampled-baseline --save-baseline baseline.json
  dq-prof examples/yellow.parquet --baseline baseline.json --fail-on warning
