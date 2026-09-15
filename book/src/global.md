# The GPUI global

`I18n` wraps `es_fluent_manager_embedded::EmbeddedI18n` and implements
`gpui_kit::Global`, making one localization manager available throughout the
application.

## Install the global

Choose the installation helper based on the desired active language:

| Helper | Existing global | Active language |
| --- | --- | --- |
| `init` | Preserved | Select separately after installation |
| `init_with_language` | Preserved | Requested language on a new installation |
| `replace_with_language` | Replaced | Requested language on the replacement |

For application startup, install the expected language before opening windows:

```rust,ignore
let language = "fr-FR".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::init_with_language(cx, language)?;
```

Use `init` when setup and language selection need separate steps:

```rust,ignore
gpui_es_fluent::init(cx)?;
let language = "en".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::change_locale(cx, language)?;
```

Because `init` and `init_with_language` preserve an existing global, neither
helper changes its active language. Use `change_locale` to update that manager
or `replace_with_language` to create a fresh one.

The language-selecting facade helpers use the embedded manager's best-effort
policy. See [Selecting locales](locales.md) when every discovered application
module must support the same locale.

## Choose lookup behavior

Use `localize_message` and `localize_label` after initializing the global:

```rust,ignore
let message = gpui_es_fluent::localize_message(cx, &AppMessage::Save);
let label = gpui_es_fluent::localize_label::<Settings>(cx);
```

Both helpers panic when the global is missing. With `es-fluent`'s default strict
message policy, they also panic when a typed resource cannot be resolved after
locale fallback. A package configured with `missing_message_policy = "fallback-str"`
returns the generated source-name fallback for missing resources.

Use the `try_*` helpers when the caller handles missing output:

```rust,ignore
if let Some(message) =
    gpui_es_fluent::try_localize_message(cx, &AppMessage::Save)
{
    show_optional_message(message);
}
```

Both fallible helpers return `None` when the global is absent or the typed
resource cannot be localized, including under `fallback-str`. They do not
distinguish those causes.

## Access the embedded manager

Read the global directly when code needs an embedded-manager API that the
facade does not wrap:

```rust,ignore
let manager = cx.global::<gpui_es_fluent::I18n>().manager();
```

`manager()` returns a shared reference. Use the facade's locale helpers for the
common selection workflows.
