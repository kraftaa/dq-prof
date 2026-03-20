import json
import subprocess
from pathlib import Path

import pandas as pd

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "examples"


def run_dq_prof(path: Path):
    result = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--bin",
            "dq-prof",
            "--",
            str(path),
            "--format",
            "json",
        ],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(result.stdout)


def pandas_profile(df: pd.DataFrame):
    n_rows = len(df)
    profile = {}
    for col in df.columns:
        series = df[col]
        null_ratio = series.isna().mean()
        distinct_ratio = series.nunique(dropna=True) / n_rows if n_rows else 0
        stats = {
            "null_ratio": null_ratio,
            "distinct_ratio": distinct_ratio,
        }
        if pd.api.types.is_numeric_dtype(series):
            stats["min"] = series.min()
            stats["max"] = series.max()
            stats["mean"] = series.mean()
        profile[col] = stats
    return {"row_count": n_rows, "columns": profile}


def compare_profiles(dq, pdp, tol=1e-9):
    assert dq["dataset"]["row_count"] == pdp["row_count"]
    for col in pdp["columns"]:
        dq_col = next(c for c in dq["dataset"]["columns"] if c["name"] == col)
        assert abs(dq_col["null_ratio"] - pdp["columns"][col]["null_ratio"]) < tol
        if pdp["columns"][col]["distinct_ratio"] is not None:
            assert abs(
                dq_col["distinct_ratio"] - pdp["columns"][col]["distinct_ratio"]
            ) < tol
        if "min" in pdp["columns"][col]:
            assert abs(dq_col["numeric"]["min"] - pdp["columns"][col]["min"]) < tol
            assert abs(dq_col["numeric"]["max"] - pdp["columns"][col]["max"]) < tol
            assert abs(dq_col["numeric"]["mean"] - pdp["columns"][col]["mean"]) < tol


def test_clean_sales_matches_pandas():
    path = FIXTURES / "clean_sales.csv"
    dq = run_dq_prof(path)
    pdp = pandas_profile(pd.read_csv(path))
    compare_profiles(dq, pdp)
