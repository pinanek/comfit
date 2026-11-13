"""Maintain one issue reporting upstream grammar revisions without changing pins."""

import json
import os
from pathlib import Path
import subprocess
import sys
import tomllib


MARKER = "<!-- comfit:upstream-grammar-updates -->"
TITLE = "Upstream grammar updates"


def gh(*args):
    return subprocess.check_output(["gh", *args], text=True)


def main():
    report = Path(sys.argv[1])
    registry = tomllib.loads(Path("registry/languages.toml").read_text())
    rows = []

    for line in report.read_text().splitlines()[1:]:
        grammar, current, upstream, _status = line.split("\t")
        if current == upstream:
            continue

        repository = registry["grammars"][grammar]["repository"].removesuffix(".git").rstrip("/")
        rows.append(
            f"| `{grammar}` | [`{current[:12]}`]({repository}/commit/{current}) "
            f"| [`{upstream[:12]}`]({repository}/commit/{upstream}) "
            f"| [Compare]({repository}/compare/{current}...{upstream}) |"
        )

    repo = os.environ["GH_REPO"]
    pages = json.loads(gh(
        "api", f"repos/{repo}/issues", "--method", "GET",
        "-f", "state=all", "-f", "per_page=100", "--paginate", "--slurp",
    ))
    issue = next((
        issue
        for page in pages
        for issue in page
        if "pull_request" not in issue
        and issue["user"]["login"] == "github-actions[bot]"
        and MARKER in (issue["body"] or "")
    ), None)

    if not rows and issue is None:
        print("All grammar pins match upstream HEADs; no issue to create.")
        return

    run_url = f"{os.environ['GITHUB_SERVER_URL']}/{repo}/actions/runs/{os.environ['GITHUB_RUN_ID']}"
    body = [
        MARKER,
        "This report compares `registry/languages.toml` pins with upstream default-branch HEADs.",
        "Different revisions need compatibility validation; this check does not change pins.",
        f"Latest check: [workflow run]({run_url}).",
        "",
    ]
    if rows:
        body.extend([
            f"{len(rows)} grammar(s) have different upstream revisions.",
            "",
            "| Grammar | Pinned revision | Upstream HEAD | Changes |",
            "| --- | --- | --- | --- |",
            *rows,
        ])
    else:
        body.append("All grammar pins currently match upstream HEADs.")
    body.extend(["", "This workflow never closes or reopens this issue.", ""])
    body_file = report.with_suffix(".md")
    body_file.write_text("\n".join(body))

    if issue is None:
        print(gh("issue", "create", "--repo", repo, "--title", TITLE, "--body-file", str(body_file)))
    else:
        print(gh("issue", "edit", str(issue["number"]), "--repo", repo, "--body-file", str(body_file)))


if __name__ == "__main__":
    main()
