import json
from pathlib import Path
import unittest

import comfit


FIXTURES = json.loads(
    (Path(__file__).resolve().parents[3] / "crates/comfit/tests/fixtures/highlighting.json").read_text()
)


def lines_value(lines):
    return [
        {
            "number": line.number,
            "highlighted": line.highlighted,
            "tokens": [[token.text, token.highlight, token.rainbow] for token in line.tokens],
        }
        for line in lines
    ]


class ContractTests(unittest.TestCase):
    def test_shared_fixtures(self):
        for fixture in FIXTURES:
            with self.subTest(fixture=fixture["name"]):
                config = comfit.HighlightConfig(
                    highlighted_lines=fixture["highlightedLines"], rainbow=fixture["rainbow"]
                )
                lines = lines_value(comfit.highlight(fixture["source"], fixture["language"], config))
                self.assertEqual([line["tokens"] for line in lines], fixture["tokens"])
                self.assertEqual(
                    "\n".join("".join(token[0] for token in line["tokens"]) for line in lines),
                    fixture["source"],
                )
                self.assertEqual(len(lines), len(fixture["source"].split("\n")))
                for number, line in enumerate(lines, 1):
                    self.assertEqual(line["number"], number)
                    self.assertEqual(line["highlighted"], number in fixture["highlightedLines"])
                if "\r" in fixture["source"]:
                    html = comfit.render_html(fixture["source"], fixture["language"], "catppina-dark")
                    self.assertIn("&#13;", html)
                    self.assertNotIn("\r", html)
                with comfit.create_document(language=fixture["language"], source="") as document:
                    document.update(fixture["source"])
                    self.assertEqual(lines_value(document.highlight(config).lines), lines)

    def test_document_lifecycle_and_cache(self):
        source = 'fn café() { let x = [1, (2)]; }\n'
        document = comfit.create_document(language="rs", source=source)
        snapshot = document.highlight()
        expected = lines_value(comfit.highlight(source, "rust"))
        with document, comfit.create_document(language="rust", source="fn other() {}") as other:
            for next_source in [source, "", "\n", 'fn longer() { println!("😀"); }\n', source]:
                document.update(next_source)
                self.assertEqual(document.source, next_source)
                for rainbow in [False, True]:
                    for highlighted_lines in [[1], [2], []]:
                        config = comfit.HighlightConfig(
                            rainbow=rainbow, highlighted_lines=highlighted_lines
                        )
                        self.assertEqual(
                            lines_value(document.highlight(config).lines),
                            lines_value(comfit.highlight(next_source, "rust", config)),
                        )
            self.assertEqual(other.source, "fn other() {}")
        document.dispose()
        document.dispose()
        self.assertEqual(lines_value(snapshot.lines), expected)
        self.assertEqual(
            snapshot.render_html("catppina-dark"),
            comfit.render_html(source, "rust", "catppina-dark"),
        )
        self.assertEqual(
            snapshot.render_ansi("catppina-dark"),
            comfit.render_ansi(source, "rust", "catppina-dark"),
        )
        for action in [lambda: document.source, lambda: document.update(source), document.highlight]:
            with self.assertRaisesRegex(RuntimeError, "disposed"):
                action()

    def test_context_manager_disposes_on_exception(self):
        document = comfit.create_document(language="rust", source="")
        with self.assertRaisesRegex(ValueError, "test exception"):
            with document:
                raise ValueError("test exception")
        with self.assertRaisesRegex(RuntimeError, "disposed"):
            document.highlight()

    def test_error_types(self):
        with self.assertRaises(ValueError):
            comfit.highlight("", "not-a-language")
        with self.assertRaises(ValueError):
            comfit.render_html("", "rust", "not-a-theme")
        with self.assertRaises(TypeError):
            comfit.highlight(None, "rust")
        for value in [-1, 1.5, 2**100]:
            with self.subTest(value=value):
                with self.assertRaises((TypeError, OverflowError)):
                    comfit.HighlightConfig(highlighted_lines=[value])

    def test_html_and_ansi_safety(self):
        source = '// <script>&"\'\x1b[31m\0\t\n'
        html = comfit.render_html(
            source, "rust", "catppina-dark", comfit.RenderHtmlConfig(pre_class='"<&\'')
        )
        self.assertIn('class="&quot;&lt;&amp;&#39;"', html)
        self.assertNotIn("<script>", html)
        self.assertIn("&lt;script&gt;&amp;&quot;&#39;", html)
        ansi = comfit.render_ansi(source, "rust", "catppina-dark")
        self.assertIn(r"\u{1b}[31m", ansi)
        self.assertIn(r"\u{0}", ansi)
        self.assertNotIn("\0", ansi)
        self.assertIn("\t", ansi)
        self.assertTrue(ansi.endswith("\n"))


if __name__ == "__main__":
    unittest.main()
