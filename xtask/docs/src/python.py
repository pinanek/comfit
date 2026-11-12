"""Render one Markdown page per public Python class or function."""

import os
from pathlib import Path
import re
import sys

from griffe import GriffeLoader, Parser
from griffe2md import render_object_docs


ROOT = Path(__file__).resolve().parents[3]
HEADING = re.compile(r"^(#{1,6}) `(comfit\.[^`]+)`$", re.MULTILINE)
LINK = re.compile(r"\[([^\]]+)\]\(#([^)]+)\)")


def main():
    output = Path(sys.argv[1])
    loader = GriffeLoader(
        search_paths=[ROOT / "python/comfit"],
        allow_inspection=False,
        docstring_parser=Parser.google,
        docstring_options={"ignore_init_summary": True},
    )
    module = loader.load("comfit")
    loader.resolve_aliases(external=True)

    pages = {}
    aliases = {}
    targets = {}
    index = ["# comfit", "", module.docstring.value, ""]

    for kind, directory in (("class", "classes"), ("function", "functions")):
        index.extend([f"## {directory.title()}", ""])

        for name in sorted(module.exports):
            member = module.members[name]
            if member.kind.value != kind:
                continue

            path = Path(directory) / f"{name}.md"
            aliases[member.canonical_path] = member.path
            markdown = render_object_docs(member, {"show_signature_annotations": True})
            markdown = HEADING.sub(
                lambda match: f'<a id="{match[2]}"></a>\n\n{match[0]}', markdown
            )
            pages[path] = markdown

            for heading in HEADING.finditer(markdown):
                targets[heading[2]] = path

            index.append(f"- [{name}]({path.as_posix()})")

        index.append("")

    def resolve(match, page):
        label, target = match.groups()
        for canonical, public in aliases.items():
            if target == canonical or target.startswith(f"{canonical}."):
                target = public + target[len(canonical):]
                break

        destination = targets.get(target)
        if destination is None:
            # Griffe also emits references to undocumented builtins such as list.
            return label

        relative = "" if destination == page else Path(
            os.path.relpath(destination, page.parent)
        ).as_posix()
        return f"[{label}]({relative}#{target})"

    for path, markdown in pages.items():
        destination = output / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(
            LINK.sub(lambda match: resolve(match, path), markdown), encoding="utf-8"
        )

    (output / "index.md").write_text("\n".join(index), encoding="utf-8")


if __name__ == "__main__":
    main()
