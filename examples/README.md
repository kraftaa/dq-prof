Example datasets:
- clean_sales.csv: small healthy sample (10 rows).
- bad_sales.csv: same columns but older timestamps and a null revenue to trigger warnings.

Try:
  ./dq-prof examples/clean_sales.csv
  ./dq-prof examples/bad_sales.csv --fail-on warning
