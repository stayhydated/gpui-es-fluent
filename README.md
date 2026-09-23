# gpui-es-fluent

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io: gpui-es-fluent][crate-badge]][crate]

`gpui-es-fluent` gives Rust developers adding localization to GPUI applications
an embedded `es-fluent` manager in GPUI global state, so every view can localize
typed messages and labels through its app context.

## Overview

- `I18n` wraps `es-fluent-manager-embedded` and implements `gpui_kit::Global`.
- Strict and fallible helpers localize generated messages and labels from any
  context that borrows `gpui_kit::App`.
- Locale helpers update the installed manager, while the optional `component`
  feature synchronizes it with GPUI Kit's component locale.

## Example

Install the global with a supported language before opening application windows:

```rust,no_run
fn initialize(cx: &mut gpui_kit::App) -> Result<(), gpui_es_fluent::EmbeddedInitError> {
    let language = "en"
        .parse::<unic_langid::LanguageIdentifier>()
        .expect("the fallback locale should be valid");
    gpui_es_fluent::init_with_language(cx, language)
}
```

Views can then pass generated `es-fluent` message and label types to
`localize_message`, `localize_label`, or their fallible `try_*` counterparts.
After a runtime locale change, notify the owning view so it renders the updated
text.

[ci-badge]: https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml/badge.svg?branch=master
[ci]: https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml
[codecov-badge]: https://codecov.io/gh/stayhydated/gpui-es-fluent/branch/master/graph/badge.svg
[codecov]: https://codecov.io/gh/stayhydated/gpui-es-fluent
[book-badge]: https://img.shields.io/badge/Book-mdBook-blue
[book]: https://stayhydated.github.io/gpui-es-fluent/book/
[crate-badge]: https://img.shields.io/crates/v/gpui-es-fluent.svg?label=gpui-es-fluent
[crate]: https://crates.io/crates/gpui-es-fluent
