# gpui-es-fluent

Shared GPUI integration helpers for `es-fluent`.

This workspace contains one public crate, `gpui-es-fluent`, for GPUI
applications that embed `es-fluent` resources.

The crate owns:

- the `I18n` GPUI global wrapper around `EmbeddedI18n`;
- initialization and locale-selection helpers;
- message and label localization helpers with readable fallback rendering;
- typed language bounds and the `CurrentLanguage` GPUI global wrapper;
- optional `gpui-component` locale integration behind the `component` feature.

See [crates/gpui-es-fluent/README.md](crates/gpui-es-fluent/README.md) for the
crate usage guide. The crate-level rustdoc is generated from that README, with
item-level API docs kept in
[crates/gpui-es-fluent/src/lib.rs](crates/gpui-es-fluent/src/lib.rs).

There are currently no checked-in book docs, public skill docs, examples, or
architecture documents in this workspace.
