"""Test a supplied Python wheel, or build one, outside the checkout."""

import argparse
import os
from pathlib import Path
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--wheel", help="Test an existing wheel without rebuilding it")
args = parser.parse_args()
wheel = Path(args.wheel).resolve() if args.wheel else None
if wheel is not None and (not wheel.is_file() or wheel.suffix != ".whl"):
    parser.error(f"wheel must be an existing .whl file: {wheel}")


def run(*args, cwd=ROOT):
    subprocess.run(args, cwd=cwd, check=True)


with tempfile.TemporaryDirectory(prefix="comfit-python-") as temporary:
    temporary = Path(temporary)
    if wheel is None:
        wheels = temporary / "wheels"
        run("uv", "build", "--package", "comfit-hl", "--wheel", "--out-dir", wheels)
        wheel = next(wheels.glob("*.whl"), None)
        if wheel is None:
            raise RuntimeError("No Python wheel was built")

    environment = temporary / "venv"
    run("uv", "venv", environment)
    python = environment / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
    run("uv", "pip", "install", "--python", python, "--no-index", wheel)
    run(
        python,
        "-I",
        "-c",
        "\n".join([
            "import comfit, sys",
            "from pathlib import Path",
            "from importlib.resources import files",
            "assert Path(comfit.__file__).resolve().is_relative_to(Path(sys.prefix).resolve())",
            "assert files('comfit').joinpath('py.typed').is_file()",
            "assert files('comfit').joinpath('_native.pyi').is_file()",
            "assert files('comfit').joinpath('THIRD_PARTY_NOTICES.txt').is_file()",
        ]),
        cwd=temporary,
    )
    run(
        python, "-I", "-m", "unittest", "discover",
        "-s", ROOT / "python/comfit/tests", "-v",
        cwd=temporary,
    )
