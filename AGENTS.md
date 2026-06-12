# AGENTS.md

This is the working guide for contributors and coding agents in the
`gpui-es-fluent` workspace.

Use it to decide:

1. which surface owns a change,
2. whether the change affects the public crate API,
3. which rustdoc, README, and feature-gate guidance must change together,
4. which narrow validation command should run before handoff.

Start in `crates/gpui-es-fluent/src/lib.rs` for API and behavior changes.
Its crate-level rustdoc includes `crates/gpui-es-fluent/README.md`, and its
item-level rustdocs describe the public helper API.
Start in the root `Cargo.toml` for workspace metadata, shared dependencies,
lint policy, and `replace` rules.

## Project Summary

`gpui-es-fluent` is a small Rust workspace that provides shared GPUI integration
helpers for `es-fluent`.

Its priorities are:

1. **Public API clarity**: keep the `I18n` global, language helpers, fallback
   rendering, and locale functions easy to use from GPUI applications.
2. **Feature fit**: keep `gpui-component` integration behind the `component`
   feature.
3. **Dependency alignment**: keep shared metadata, dependency versions, and lint
   policy in the workspace root `Cargo.toml`.

## Quick Decision Flow

Before editing, classify the change:

1. **Public helper API**: update `crates/gpui-es-fluent/src/lib.rs`, then sync
   the root `README.md` and `crates/gpui-es-fluent/README.md` when public usage
   changes.
2. **Feature-gated component behavior**: keep the `component` feature in
   `crates/gpui-es-fluent/Cargo.toml` aligned with the `#[cfg(feature =
   "component")]` API in `src/lib.rs`.
3. **Workspace metadata or dependency changes**: update the root `Cargo.toml`
   first, then use `workspace = true` in the member crate when the dependency is
   workspace-managed.
4. **Validate narrowly**: run the smallest cargo command that proves the edited
   crate and feature set still compile or test.

## Audience Labels

- **User-facing**: normal entry points for GPUI application developers.
- **Internal**: workspace metadata, lint policy, and dependency wiring.

## Documentation Placement

Treat the root `README.md`, `crates/gpui-es-fluent/README.md`, and public
rustdocs in `crates/gpui-es-fluent/src/lib.rs` as user-facing. Keep them
concise and example-first when adding usage guidance.

The root `README.md` is the workspace overview. The crate README is the
canonical usage guide for the public crate and is included as the crate-level
rustdoc. Keep item-level rustdocs beside the public items they describe.

This workspace does not currently have examples, book docs, public skills, CI
workflows, `justfile` recipes, or `ARCHITECTURE.md` files. Do not add sync rules
for those surfaces unless the surfaces are added in the same change. If a book,
public skill, or example surface is added later, name its exact path and update
trigger here; put reusable application guidance in the owning surface itself.

## Synchronization Rules

- When public helper behavior, public function names, fallback rendering,
  locale-selection behavior, or supported language bounds change, update
  `crates/gpui-es-fluent/src/lib.rs`, item-level rustdocs, and the affected
  README usage text in the same change.
- When `gpui-component` locale integration changes, keep the `component`
  feature in `crates/gpui-es-fluent/Cargo.toml` aligned with every
  `#[cfg(feature = "component")]` item and related docs in `src/lib.rs`.
- When shared dependency versions, workspace metadata, lint policy, or
  `replace` rules change, update the root `Cargo.toml` first and keep the member
  crate on `workspace = true` for workspace-managed dependencies.
- Treat `.es-fluent/` as an ignored `es-fluent` CLI scratch workspace, not as
  checked-in source. Do not hand-edit it to change public crate behavior.

## Workspace Map

### Public Crate

- `crates/gpui-es-fluent`
  Audience: **User-facing**
  Role: single public crate for GPUI app-global `I18n`, typed language bounds,
  locale selection helpers, fallback rendering, and optional
  `gpui-component` locale integration.

## Validation and Editing Rules

### Validation After Changes

- Run the narrowest command that proves the edited behavior works.
- Use `cargo check -p gpui-es-fluent` for default-feature API and behavior
  changes.
- Use `cargo check -p gpui-es-fluent --all-features` for `component`, dependency,
  workspace metadata, or feature-gate changes.
- Use `cargo doc -p gpui-es-fluent --all-features --no-deps` for rustdoc or
  crate README changes.
- Use `cargo test -p gpui-es-fluent --all-features` when behavior changes are
  covered by tests.
- If validation cannot be run, state why and what remains unvalidated.
- Do not claim a change works unless it was validated or the remaining risk is
  explicitly documented.

### When Editing Rust

- Keep shared dependency versions in the workspace root `Cargo.toml`.
- Use `workspace = true` in `crates/gpui-es-fluent/Cargo.toml` for
  workspace-managed dependencies.
- Keep optional `gpui-component` APIs behind the `component` feature.
