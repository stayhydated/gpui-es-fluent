# gpui-es-fluent

[![Build Status](https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/gpui-es-fluent/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/github/stayhydated/gpui-es-fluent/graph/badge.svg)](https://codecov.io/github/stayhydated/gpui-es-fluent)
[![Docs](https://docs.rs/gpui-es-fluent/badge.svg)](https://docs.rs/gpui-es-fluent/)
[![Crates.io](https://img.shields.io/crates/v/gpui-es-fluent.svg)](https://crates.io/crates/gpui-es-fluent)

Shared GPUI integration helpers for applications that embed `es-fluent`
resources.

## What It Provides

- `I18n`, a GPUI global wrapper around `es_fluent_manager_embedded::EmbeddedI18n`.
- `init`, `init_with_language`, and `replace_with_language` for installing the
  GPUI global.
- `change_locale` for selecting a new language on the installed global.
- `localize_message` and `localize_label`, which require an installed global
  and fail hard when a typed Fluent resource is missing.
- `try_localize_message` and `try_localize_label`, which return `None` when the
  global or typed resource is missing and the caller wants to handle that state.
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

`localize_message` and `localize_label` panic when the `I18n` global is not
installed or the typed resource is missing. This keeps untranslated keys out of
user-facing output.

Use a `try_*` helper only when absence is an expected state:

```rust,ignore
if let Some(text) = gpui_es_fluent::try_localize_message(cx, &message) {
    render(text);
}

let maybe_label = gpui_es_fluent::try_localize_label::<SettingsLabel>(cx);
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

- `component_language()` reads and validates `gpui_component::locale()`,
  returning a typed `ComponentLocaleError` for invalid state.
- `init_from_component_locale(cx)` initializes `I18n` from the current
  `gpui-component` locale.
- `set_component_locale(cx, locale)` sets the `gpui-component` locale,
  replaces `I18n` to match it, and returns the selected language.
- `sync_component_locale(cx)` reads the current component locale and
  applies it to the installed `I18n` global when one exists, returning a
  localization error when the installed manager rejects the selected language.

Invalid component locale strings are explicit errors:

```rust,ignore
gpui_component::set_locale("en-US");
let language = gpui_es_fluent::component_language()?;
```
