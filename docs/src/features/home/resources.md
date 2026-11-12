<section aria-labelledby="home-quick-start" data-layout="split">
  <header>
    <h2 id="home-quick-start">Get started in your environment.</h2>
    <p>Install Comfit and render a short Rust snippet as highlighted HTML from your own project.</p>
  </header>

<CodeTabs label="Quick start environment">
  <CodeTab label="Rust">

```sh title="Install"
cargo add comfit --features rust,render-html
```

```rust title="src/main.rs"
fn main() -> Result<(), comfit::Error> {
    let mut highlighter = comfit::Highlighter::new()?;
    let result = highlighter.highlight("fn main() {}", "rust", &Default::default())?;
    println!("{}", result.render_html_classes(&Default::default()));
    Ok(())
}
```

<p>The HTML uses classes you can style with CSS. <Link to="/api-references/rust">Rust API reference</Link></p>

  </CodeTab>
  <CodeTab label="Node.js">

```sh title="Install"
npm install @comfit/node
```

```javascript title="example.mjs"
import { renderHtml, RenderHtmlConfig, RenderHtmlStyle } from "@comfit/node";

const config = new RenderHtmlConfig();
config.style = RenderHtmlStyle.Inline;

console.log(renderHtml("fn main() {}", "rust", "catppuccin-mocha", config));
```

<p>The HTML includes inline theme styles. <Link to="/api-references/javascript/node">Node.js API reference</Link></p>

  </CodeTab>
  <CodeTab label="Python">

```sh title="Install"
python -m pip install comfit
```

```python title="example.py"
import comfit

config = comfit.RenderHtmlConfig(style=comfit.RenderHtmlStyle.Inline)
print(comfit.render_html("fn main() {}", "rust", "catppuccin-mocha", config))
```

<p>The HTML includes inline theme styles. <Link to="/api-references/python">Python API reference</Link></p>

  </CodeTab>
</CodeTabs>
</section>

<section aria-labelledby="home-integrations">
  <header>
    <h2 id="home-integrations">Use Comfit in your workflow.</h2>
    <p>Highlight code when you build a site, in the browser, or directly in a terminal.</p>
  </header>

  <dl>
    <div>
      <dt><Link to="/api-references/javascript/rehype">Markdown with rehype</Link></dt>
      <dd>
        <p>Add Comfit after remark-rehype and before rendering HTML in your unified pipeline.</p>

```javascript
pipeline.use(rehypeComfit, { theme: "catppuccin-mocha" });
```

      </dd>
    </div>
    <div>
      <dt><Link to="/api-references/javascript/wasm">Browser with WebAssembly</Link></dt>
      <dd>
        <p>Initialize the runtime, load the Rust grammar, and highlight source in the browser.</p>

```javascript
const tokens = highlighter.highlight(source, "rust");
```

      </dd>
    </div>
    <div>
      <dt><a href="https://github.com/pinanek/comfit/blob/HEAD/crates/comfit_cli/README.md">Terminal with the CLI</a></dt>
      <dd>
        <p>Use <code>comfit-cli</code> to print a source file with ANSI colors and line numbers.</p>

```sh
comfit ansi main.rs --theme catppuccin-mocha --line-numbers
```

      </dd>
    </div>

  </dl>
</section>

<section aria-labelledby="home-custom-theme" data-layout="split">
  <header>
    <h2 id="home-custom-theme">Bring your own theme.</h2>
    <p>Convert a local VS Code theme to Comfit’s TOML format, then use it to color the same Rust snippet in your terminal.</p>
  </header>

```sh title="Convert and apply"
comfit convert theme.json --name "Custom" -o custom.toml
printf 'fn main() {}\n' |
  comfit ansi --language rust --theme-file custom.toml
```

</section>
