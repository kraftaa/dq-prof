import os
import subprocess
import sys
from pathlib import Path


def _binary_path() -> Path:
    here = Path(__file__).resolve().parent
    bin_path = here / "bin" / "dq-prof"
    if not bin_path.exists():
        raise SystemExit("dq-prof binary missing; run prepare_binary.sh or install platform wheel")
    return bin_path


def main():
    bin_path = _binary_path()
    cmd = [str(bin_path)] + sys.argv[1:]
    os.execv(cmd[0], cmd)


if __name__ == "__main__":
    main()
