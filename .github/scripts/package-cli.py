"""Package a CLI release binary with its documentation, licenses, and checksum."""

import argparse
import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import tomllib
import zipfile


ROOT = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("target", help="Rust target triple used to build the CLI")
args = parser.parse_args()

if not args.target or any(char not in "abcdefghijklmnopqrstuvwxyz0123456789_-" for char in args.target):
    parser.error("target must be a Rust target triple")

with (ROOT / "crates/comfit_cli/Cargo.toml").open("rb") as manifest:
    version = tomllib.load(manifest)["package"]["version"]

if os.environ.get("RELEASE_TAG") != f"comfit-cli-v{version}":
    parser.error("RELEASE_TAG must match the CLI package version")

windows = args.target.endswith("-windows-msvc")
binary_name = "comfit.exe" if windows else "comfit"
binary = ROOT / "target" / args.target / "release" / binary_name
output = ROOT / "dist"
name = f"comfit-v{version}-{args.target}"
archive = output / f"{name}{'.zip' if windows else '.tar.gz'}"

output.mkdir(exist_ok=True)

with tempfile.TemporaryDirectory(prefix="comfit-cli-") as temporary:
    directory = Path(temporary) / name
    directory.mkdir()
    shutil.copy2(binary, directory / binary_name)

    if not windows:
        (directory / binary_name).chmod(0o755)

    for filename in ["README.md", "LICENSE", "THIRD_PARTY_NOTICES.txt"]:
        shutil.copy2(ROOT / "crates/comfit_cli" / filename, directory / filename)

    if windows:
        with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as package:
            for path in sorted(directory.iterdir()):
                package.write(path, arcname=f"{name}/{path.name}")
    else:
        with tarfile.open(archive, "w:gz") as package:
            package.add(directory, arcname=name)

    installed = Path(temporary) / "installed"
    shutil.unpack_archive(archive, installed)
    executable = installed / name / binary_name
    actual = subprocess.check_output([executable, "--version"], text=True).strip()

    if actual != f"comfit {version}":
        raise RuntimeError(f"Binary version mismatch: {actual}")

    html = subprocess.check_output(
        [executable, "html", "--language", "rust", "--theme", "catppuccin-mocha"],
        input="fn main() {}\n", text=True,
    )

    if "<pre" not in html or "</pre>" not in html or "main" not in html:
        raise RuntimeError("Packaged CLI did not render Rust source as HTML")

with archive.open("rb") as package:
    checksum = hashlib.file_digest(package, "sha256").hexdigest()

archive.with_name(f"{archive.name}.sha256").write_text(
    f"{checksum}  {archive.name}\n", encoding="utf8", newline="\n"
)
print(archive)
