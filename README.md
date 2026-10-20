# gpui-es-fluent

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io: gpui-es-fluent][crate-badge]][crate]

`gpui-es-fluent` gives Rust developers adding localization to GPUI applications
an embedded `es-fluent` manager in GPUI global state, so every view can localize
typed messages and labels through its app context.

Requires Rust 1.99 or newer and GPUI Kit 0.8.0.
Message-owning packages provide their locale configuration, fallback assets,
build-script tracking, and embedded module registration.

## Overview

- `I18n` wraps `es-fluent-manager-embedded` and implements `gpui_kit::Global`.
- Typed message and label helpers work with any context borrowing `gpui_kit::App`.
  Their `try_*` counterparts return `None` when the global or translation is absent.
- `init_with_language` preserves an installed global; `replace_with_language`
  explicitly replaces it. After changing the locale, notify the owning view to
  render the updated text.

## Features

No optional features are enabled by default.

| Feature | Default | Purpose |
| --- | --- | --- |
| `component` | No | Enables GPUI Kit component support and helpers that synchronize component and localization locales; activates the optional `thiserror` dependency. |

`set_component_locale` validates and initializes the replacement manager before
changing either locale state. See the [public API][api] for the component and
embedded-manager helpers.

## Example

The host supplies a GPUI `App`, a supported `LanguageIdentifier`, and an
`es-fluent` message whose owning library registered its embedded assets. Call
this during startup before a global has been installed:

```rust,no_run
use es_fluent::FluentMessage;
use gpui_es_fluent::{EmbeddedInitError, init_with_language, localize_message};
use gpui_kit::App;
use unic_langid::LanguageIdentifier;

fn translated_text(
    cx: &mut App,
    language: LanguageIdentifier,
    message: &impl FluentMessage,
) -> Result<String, EmbeddedInitError> {
    init_with_language(cx, language)?;
    Ok(localize_message(&*cx, message))
}
```

The result is the message rendered in the selected locale. The [maintained
GPUI demo][demo] provides complete registration, typed messages, and a view;
with `component` enabled, it uses `set_component_locale` to keep GPUI Kit's
locale aligned. Its English heading is “One GPUI view, two locales”.

[ci-badge]: https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml/badge.svg?branch=master
[ci]: https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml
[codecov-badge]: https://codecov.io/gh/stayhydated/gpui-es-fluent/branch/master/graph/badge.svg
[codecov]: https://codecov.io/gh/stayhydated/gpui-es-fluent
[book-badge]: https://img.shields.io/badge/Book-mdBook-blue
[book]: https://stayhydated.github.io/gpui-es-fluent/book/
[crate-badge]: https://img.shields.io/crates/v/gpui-es-fluent.svg?label=gpui-es-fluent
[crate]: https://crates.io/crates/gpui-es-fluent
[api]: https://docs.rs/gpui-es-fluent/latest/gpui_es_fluent/
[demo]: https://github.com/stayhydated/gpui-es-fluent/tree/master/examples/gpui-demo
