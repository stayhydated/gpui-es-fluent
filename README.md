# gpui-es-fluent

[![Build Status](https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml)
[![Docs](https://docs.rs/gpui-es-fluent/badge.svg)](https://docs.rs/gpui-es-fluent/)
[![Crates.io](https://img.shields.io/crates/v/gpui-es-fluent.svg)](https://crates.io/crates/gpui-es-fluent)

Store an embedded `es-fluent` manager in GPUI global state, then localize typed
messages and labels from any context that borrows `gpui::App`.

## Install

`gpui-es-fluent` requires Rust 1.96 or newer.

```toml
[dependencies]
gpui-es-fluent = "0.1"
```

Enable the `component` feature when the application also uses
`gpui-component` locale state:

```toml
[dependencies]
gpui-es-fluent = { version = "0.1", features = ["component"] }
```

## Quick start

Install the global before opening windows:

```rust,ignore
let language = "en"
    .parse::<unic_langid::LanguageIdentifier>()
    .expect("the fallback locale should be valid");
gpui_es_fluent::init_with_language(cx, language)?;
```

Localize generated `es-fluent` resources from a render context:

```rust,ignore
let title = gpui_es_fluent::localize_message(cx, &AppMessage::Welcome);
let label = gpui_es_fluent::localize_label::<SettingsLabel>(cx);
```

The strict lookup helpers panic when the global or typed resource is missing.
Use a `try_*` helper when the caller has an explicit missing-state path:

```rust,ignore
if let Some(text) =
    gpui_es_fluent::try_localize_message(cx, &AppMessage::Welcome)
{
    render(text);
}
```

## Documentation

- [User guide](https://stayhydated.github.io/gpui-es-fluent/book/)
- [API reference](https://docs.rs/gpui-es-fluent/)
