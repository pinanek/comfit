"""Run Python API tests against the Cargo-built extension without building a wheel."""

import argparse
import importlib.machinery
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import unittest


ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--workspace", action="store_true", help="Match the workspace test feature graph")
args = parser.parse_args()
selection = ["--workspace", "--all-features"] if args.workspace else ["--package", "comfit-python"]

result = subprocess.run(
    [
        "cargo", "build", "--locked", *selection, "--message-format=json-render-diagnostics",
    ],
    cwd=ROOT,
    env={**os.environ, "PYO3_PYTHON": sys.executable, "PYO3_BUILD_EXTENSION_MODULE": "1"},
    stdout=subprocess.PIPE,
    text=True,
)
library = None
for line in result.stdout.splitlines():
    message = json.loads(line)
    if message["reason"] == "compiler-message":
        rendered = message["message"].get("rendered")
        if rendered:
            print(rendered, file=sys.stderr, end="")
    elif message["reason"] == "compiler-artifact" and message["target"]["name"] == "_native":
        library = next(
            (path for path in message["filenames"] if Path(path).suffix in {".so", ".dylib", ".dll"}),
            None,
        )
result.check_returncode()
if library is None:
    raise RuntimeError("Cargo did not produce the Python extension")

sys.path.insert(0, str(ROOT / "python/comfit"))
name = "comfit._native"
loader = importlib.machinery.ExtensionFileLoader(name, library)
spec = importlib.util.spec_from_file_location(name, library, loader=loader)
module = importlib.util.module_from_spec(spec)
sys.modules[name] = module
loader.exec_module(module)

suite = unittest.defaultTestLoader.discover(str(ROOT / "python/comfit/tests"))
sys.exit(not unittest.TextTestRunner(verbosity=2).run(suite).wasSuccessful())
