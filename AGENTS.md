# AGENTS.md

This is the working guide for contributors and coding agents in the
`gpui-es-fluent` repository.

Use it to decide:

1. which surface owns a change,
2. whether the change affects the public crate API,
3. which rustdoc, README, book, and feature-gate guidance must change together,
4. which narrow validation command should run before handoff.

Start in `src/lib.rs` for API and behavior changes. Its crate-level rustdoc
includes `README.md`, and its item-level rustdocs describe the public helper
API.
Use `book/src/SUMMARY.md` for user-guide routing and `web/src/lib.rs` for the
public catalog.
Start with `just --list`; the root `justfile` is the repository command index
for format, clippy, check, test, and publish dry-run recipes.

## Project Summary

`gpui-es-fluent` is a Rust workspace centered on one public crate that provides
shared GPUI integration helpers for `es-fluent`, plus documentation,
publishing, and a dual-target demo application.

Its priorities are:

1. **Public API clarity**: keep the `I18n` global, language helpers,
   hard-failing localization, fallible `try_*` lookups, and locale functions
   easy to use from GPUI applications.
2. **Feature fit**: keep `gpui-component` integration behind the `component`
   feature.

## Quick Decision Flow

Before editing, classify the change:

1. **Public helper API**: update `src/lib.rs`, then sync `README.md` when
   public usage changes, including matching `book/src` chapters.
2. **Feature-gated component behavior**: keep the `component` feature in
   `Cargo.toml` aligned with the `#[cfg(feature = "component")]` API in
   `src/lib.rs`.
3. **Validate narrowly**: run the smallest cargo or `just` command that proves
   the edited crate, feature set, docs, or workflow still compiles or runs.
4. **Match the command surface**: use the root `justfile` for local recipes and
   the workflow commands in `.github/workflows/ci.yml` when matching CI matters.

## Audience Labels

- **User-facing**: normal entry points for GPUI application developers.

## Documentation Placement

Treat `README.md`, `book/src`, and public rustdocs in `src/lib.rs` as
user-facing. Keep them concise and example-first when adding usage guidance.

`README.md` is the canonical usage guide for the public crate and is included
as the crate-level rustdoc. Keep item-level rustdocs beside the public items
they describe.

## Synchronization Rules

- When public helper behavior, public function names, fallback rendering,
  locale-selection behavior, or supported language bounds change, update
  `src/lib.rs`, item-level rustdocs, the affected README usage text, and
  matching book chapters and applicable public usage skill guidance in the same
  change.
- When `gpui-component` locale integration changes, keep the `component`
  feature, public API, README, `book/src/component.md`, and GPUI demo aligned.
- When the public catalog or its destinations change, update `web/src/lib.rs`.
- Treat `.es-fluent/` as an ignored `es-fluent` CLI scratch directory, not as
  checked-in source. Do not hand-edit it to change public crate behavior.
- Build `web/public/book`, `web/public/llms*`, `web/public/gpui-demo`, and
  `web/dist` through `cargo xtask`; edit `book/src`, `web/src`, and
  `examples/gpui-demo` as their sources.

## Repository Map

### Public Crate

- `src/lib.rs`
  Audience: **User-facing**
  Role: single public crate for GPUI app-global `I18n`, typed language bounds,
  locale selection helpers, strict localization, and optional
  `gpui-component` locale integration.

### Documentation, Demo, and Publishing

- `book/src`
  Audience: **User-facing**
  Role: mdBook source for setup, global state, locale changes, and
  `gpui-component` synchronization.

- `examples/gpui-demo`
  Audience: **User-facing**
  Role: native and nightly Wasm GPUI demo with embedded English and French
  Fluent assets and a live locale switch.

- `web`
  Audience: **User-facing**
  Role: Dioxus catalog for the book, GPUI demo, API docs, and source.

- `xtask`
  Audience: **Internal**
  Role: book, `llms.txt`, GPUI demo, and Pages-site build orchestration.

## Validation and Editing Rules

### Validation After Changes

- Run the narrowest command that proves the edited behavior works.
- Use `just --list` to inspect available repository recipes.
- Use `just fmt`, `just clippy`, `just check`, or `just test` when the edit
  needs the corresponding repository workflow.
- Use `cargo check` for default-feature API and behavior changes.
- Use `cargo check --all-features` for `component` feature-gate changes.
- Use `cargo doc --workspace --all-features --no-deps --locked` when matching
  the CI documentation job for rustdoc or crate README changes.
- Use `cargo test --all-features` when behavior changes are covered by tests.
- Use `cargo xtask build book` and `cargo xtask build llms-txt` for book
  changes, `cargo xtask build gpui-demo` for the nightly Wasm demo, and
  `cargo xtask build web` for the catalog. `just web-build` runs the complete
  publication pipeline.
- CI runs locked workspace-form tests, formatting, clippy, docs,
  cargo-machete, coverage, and Codecov publishing from `.github/workflows/ci.yml`.
- If validation cannot be run, state why and what remains unvalidated.
- Do not claim a change works unless it was validated or the remaining risk is
  explicitly documented.
