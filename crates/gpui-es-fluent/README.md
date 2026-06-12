# gpui-es-fluent

Shared GPUI integration helpers for applications that embed `es-fluent`
resources.

## What It Provides

- `I18n`, a GPUI global wrapper around `es_fluent_manager_embedded::EmbeddedI18n`.
- `init`, `init_with_language`, and `replace_with_language` for installing the
  GPUI global.
- `change_locale` for selecting a new language on the installed global.
- `localize_message` and `localize_label`, which use the installed global when
  present and fall back to readable text when it is not.
- `try_localize_message`, which returns `None` instead of using fallback text
  when no global is installed.
- `fallback_message`, `fallback_label`, `FallbackLocalizer`, and `humanize_key`
  for deterministic fallback rendering.
- `Language` and `CurrentLanguage` for typed language state.
- Optional `gpui-component` locale helpers behind the `component` feature.

## Install

```toml
[dependencies]
gpui-es-fluent = "0.1"
```

Enable `gpui-component` integration only when the application uses
`gpui-component` locale state:

```toml
[dependencies]
gpui-es-fluent = { version = "0.1", features = ["component"] }
```

## Basic Usage

Install the `I18n` global during GPUI application setup:

```rust,ignore
fn configure_i18n(cx: &mut gpui::App) -> Result<(), gpui_es_fluent::EmbeddedInitError> {
    gpui_es_fluent::init(cx)?;
    Ok(())
}
```

Use generated `es-fluent` messages and labels through the helper functions:

```rust,ignore
let title = gpui_es_fluent::localize_message(cx, &message);
let label = gpui_es_fluent::localize_label::<SettingsLabel>(cx);
```

`localize_message` and `localize_label` render fallback text when the `I18n`
global is not installed. Fallback rendering strips a trailing `_label`, splits
IDs on `_` and `-`, drops empty segments, and uppercases the first character of
each remaining segment.

Use `try_localize_message` when absence of the global should remain observable:

```rust,ignore
if let Some(text) = gpui_es_fluent::try_localize_message(cx, &message) {
    render(text);
}
```

## Locale Selection

`init_with_language` installs the global with an initial language only when the
global is not already present:

```rust,ignore
gpui_es_fluent::init_with_language(cx, "en-US".parse::<unic_langid::LanguageIdentifier>()?)?;
```

`replace_with_language` always replaces the global:

```rust,ignore
gpui_es_fluent::replace_with_language(cx, "es-ES".parse::<unic_langid::LanguageIdentifier>()?)?;
```

`change_locale` updates the installed global's active language:

```rust,ignore
gpui_es_fluent::change_locale(cx, "fr-FR".parse::<unic_langid::LanguageIdentifier>()?)?;
```

Call `init`, `init_with_language`, or `replace_with_language` before calling
`change_locale`.

## Component Feature

With the `component` feature enabled:

- `component_language(fallback)` reads `gpui_component::locale()` and falls back
  to `fallback` when the component locale is invalid.
- `init_from_component_locale(cx, fallback)` initializes `I18n` from the current
  `gpui-component` locale.
- `set_component_locale(cx, locale, fallback)` sets the `gpui-component` locale,
  replaces `I18n` to match it, and returns the selected language.
- `sync_component_locale(cx, fallback)` reads the current component locale and
  applies it to the installed `I18n` global when one exists.

The fallback string passed to these helpers must be a valid language identifier.
