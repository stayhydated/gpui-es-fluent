# Working in gpui-es-fluent

Start in `src/lib.rs` for the public API, implementation, and unit tests.
`README.md` is included as crate-level rustdoc. Use `just --list` for the
repository's command index.

## Change ownership

| Surface | Responsibility |
| --- | --- |
| `src/lib.rs` | Public `I18n` and `CurrentLanguage` globals, lookups, locale helpers, and feature-gated component integration |
| `README.md`, `book/src/` | User-facing usage; `book/src/SUMMARY.md` owns chapter navigation |
| `examples/gpui-demo/` | Native and Wasm demo, typed messages, embedded locale assets, and runtime locale switching |
| `web/src/lib.rs` | Public catalog and its book, demo, API, and source destinations |
| `xtask/src/commands/` | Book, LLM text, demo, and Pages build orchestration |

## Keep related surfaces aligned

- When public helper names, initialization, lookup failures, language bounds,
  or locale selection change, update their rustdoc, README examples, relevant
  book chapters, and unit tests in `src/lib.rs` together.
- Keep GPUI Kit component integration behind `component`. Changes to that API
  should update the feature-gated implementation, `book/src/component.md`,
  README feature guidance, and the GPUI demo.
- Demo messages belong in `examples/gpui-demo/src/lib.rs`; its `src/i18n.rs`
  registers the embedded manager and `build.rs` tracks locale assets. Keep
  message changes aligned with `examples/gpui-demo/assets/i18n/` and the
  rendered demo.
- Edit `book/src/`, `web/src/`, and `examples/gpui-demo/` as the sources for
  published content. Generate the book, LLM text, demo, and site through the
  corresponding `cargo xtask build` commands.

## Validation

Run the narrowest check for the changed surface:

- Public crate behavior: `cargo test -p gpui-es-fluent --locked`.
- Component behavior: `cargo test -p gpui-es-fluent --all-features --locked`.
- README and rustdoc: `cargo doc -p gpui-es-fluent --all-features --no-deps --locked`
  and `cargo test -p gpui-es-fluent --doc --all-features --locked` for examples.
- Markdown: `rumdl check README.md AGENTS.md book/src`.
- Book content: `cargo xtask build book` and `cargo xtask build llms-txt`.
- Wasm demo: `cargo xtask build gpui-demo` uses nightly.
- Catalog: `cargo xtask build web`; `just web-build` runs the complete
  publication build.

Use `.github/workflows/ci.yml` when reproducing the workspace checks required
by CI. Report which checks ran, which failed, and which were not run.
