# Core migration benchmark

Measured on Apple M3 with Rust 1.98.0, release optimization (`opt-level = "s"`, LTO).
The baseline was copied before the module and syntax-layer migration. Both versions
include the SmolStr migration and use the same prepared grammars and input files.
Both implementations were linked into one benchmark executable.

The current implementation streams one-shot syntax layers, releasing each tree
before parsing the next one. Documents still retain trees for incremental updates.
Per-layer highlighting, injection range discovery, and token composition are shared;
only traversal orchestration differs.

Each timing is the median of 11 samples, alternating which implementation runs
first. Iteration counts were calibrated to at least 20 ms per sample when possible.
Initialization and correctness checks are outside the timed sections. Times include
constructing and dropping highlight output, but not HTML/ANSI rendering.

## Timing

Negative changes mean faster. These small differences do not demonstrate a material
speedup or slowdown; the migration simplifies module ownership without requiring
one-shot calls to retain every tree.

| Input                       | Operation        |     Before |      After | Change |
| --------------------------- | ---------------- | ---------: | ---------: | -----: |
| Rust, 15.8 KB               | One-shot         |  41.121 ms |  41.058 ms |  -0.2% |
| Rust, 15.8 KB               | Cached document  |   79.94 µs |   80.77 µs |  +1.0% |
| Rust, 15.8 KB               | Edit only        |  709.63 µs |  708.29 µs |  -0.2% |
| Rust, 15.8 KB               | Edit + highlight |  40.596 ms |  40.541 ms |  -0.1% |
| TSX, 9.8 KB                 | One-shot         |  23.335 ms |  23.299 ms |  -0.2% |
| TSX, 9.8 KB                 | Cached document  |   54.91 µs |   55.55 µs |  +1.2% |
| TSX, 9.8 KB                 | Edit only        |  527.09 µs |  526.95 µs |  -0.0% |
| TSX, 9.8 KB                 | Edit + highlight |  22.482 ms |  22.388 ms |  -0.4% |
| Markdown, 30 HTML/JS blocks | One-shot         |   3.043 ms |   3.043 ms |  +0.0% |
| Markdown, 30 HTML/JS blocks | Cached document  |   19.16 µs |   19.32 µs |  +0.8% |
| Markdown, 30 HTML/JS blocks | Edit only        |  527.46 µs |  525.37 µs |  -0.4% |
| Markdown, 30 HTML/JS blocks | Edit + highlight |   2.487 ms |   2.469 ms |  -0.7% |
| HTML, 200 JS blocks         | One-shot         |  13.477 ms |  13.439 ms |  -0.3% |
| HTML, 200 JS blocks         | Cached document  |  116.54 µs |  116.20 µs |  -0.3% |
| HTML, 200 JS blocks         | Edit only        |   1.873 ms |   1.876 ms |  +0.1% |
| HTML, 200 JS blocks         | Edit + highlight |  12.302 ms |  12.234 ms |  -0.6% |
| Rust, 78.9 KB               | One-shot         | 244.113 ms | 243.781 ms |  -0.1% |
| Rust, 78.9 KB               | Cached document  |  406.05 µs |  410.20 µs |  +1.0% |
| Rust, 78.9 KB               | Edit only        |   6.134 ms |   6.139 ms |  +0.1% |
| Rust, 78.9 KB               | Edit + highlight | 242.437 ms | 241.790 ms |  -0.3% |

The Rust input is the pre-migration `engine.rs`; TSX is the playground component.
Large Rust repeats the same input five times. Edits alternate between two source
versions, changing text inside the document. The Markdown input contains nested
HTML and JavaScript injections.

Before timing, both versions produced identical line numbers, selected-line flags,
token text, capture names, and bracket depths for original source, edited source,
Unicode prefixes, and empty documents, with rainbow highlighting enabled and disabled.

## Peak process memory

Separate executions measured maximum resident set size with macOS `/usr/bin/time -l`.
These are single-run whole-process measurements, including grammar and query setup,
not isolated Rust allocation counts.

| One-shot input        |    Before |     After | Difference |
| --------------------- | --------: | --------: | ---------: |
| Rust, 315.8 KB        | 231.2 MiB | 231.0 MiB |  -0.25 MiB |
| HTML, 5,000 JS blocks | 207.4 MiB | 207.6 MiB |  +0.27 MiB |

The initial migration retained every tree during one-shot highlighting and reached
221.2 MiB for the 5,000-script input. Restoring streaming traversal brought that
case back to 207.6 MiB, close to the pre-migration baseline. These measurements
come from separate executions; small differences are not precise per-tree costs.

## Local artifacts

The frozen baseline, benchmark source, fixture copies, and raw measurements are
saved in `/tmp/comfit-core-migration/`. Current results are in
`timings-streaming.txt` and `memory-streaming-*.stats`; the initial retained-tree
results remain in `timings.txt` and `memory-{old,new}-*.stats`.
These are not package or workspace dependencies. Temporary artifacts must be
preserved to reproduce this specific comparison.
