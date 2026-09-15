# gpui-es-fluent

[![CI](https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/gpui-es-fluent/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/gpui-es-fluent)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/gpui-es-fluent/book/)
[![Crates.io](https://img.shields.io/crates/v/gpui-es-fluent.svg)](https://crates.io/crates/gpui-es-fluent)

Store an embedded `es-fluent` manager in GPUI global state, then localize typed
messages and labels from any context that borrows `gpui_kit::App`.

## Quick start

Initialize the global with a language supported by the application's embedded
resources before opening windows:

```rust,no_run
fn initialize(cx: &mut gpui_kit::App) -> Result<(), gpui_es_fluent::EmbeddedInitError> {
    let language = "en"
        .parse::<unic_langid::LanguageIdentifier>()
        .expect("the fallback locale should be valid");
    gpui_es_fluent::init_with_language(cx, language)
}
```

Localize generated `es-fluent` resources from a render context:

```rust,ignore
let title = gpui_es_fluent::localize_message(cx, &AppMessage::Welcome);
let label = gpui_es_fluent::localize_label::<SettingsLabel>(cx);
```

Lookups require an initialized global. Under `es-fluent`'s default strict
message policy, a missing typed resource also causes a panic. Use a `try_*`
helper when the caller handles missing output:

```rust,ignore
if let Some(text) =
    gpui_es_fluent::try_localize_message(cx, &AppMessage::Welcome)
{
    render(text);
}
```

## Locale changes

Change the installed manager's language and notify the view from its
`gpui_kit::Context<T>`:

```rust,ignore
let french = "fr-FR".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::change_locale(cx, french)?;
cx.notify();
```

The optional `component` feature adds helpers that synchronize GPUI Kit's
component locale with the embedded manager. Use `set_component_locale` to
update both from a locale string, then notify the view.
