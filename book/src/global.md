# The GPUI global

`I18n` wraps `es_fluent_manager_embedded::EmbeddedI18n` and implements
`gpui::Global`, making one localization manager available throughout the
application.

## Install the global

Choose the installation helper based on the desired active language:

| Helper | Existing global | Active language |
| --- | --- | --- |
| `init` | Preserved | Embedded default on a new installation |
| `init_with_language` | Preserved | The requested language is selected only when a global is installed |
| `replace_with_language` | Replaced | The requested language is selected on the replacement |

For application startup, install the expected language before opening windows:

```rust,ignore
let language = "fr-FR".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::init_with_language(cx, language)?;
```

Use `init` when the embedded manager's configured default is the desired
startup language:

```rust,ignore
gpui_es_fluent::init(cx)?;
```

Because `init` and `init_with_language` preserve an existing global, neither
helper changes its active language. Use `change_locale` to update that manager
or `replace_with_language` to create a fresh one.

The language-selecting facade helpers use the embedded manager's best-effort
policy. See [Selecting locales](locales.md) when every discovered application
module must support the same locale.

## Choose lookup behavior

Hard-failing helpers require both the installed global and the typed resource:

```rust,ignore
let message = gpui_es_fluent::localize_message(cx, &AppMessage::Save);
let label = gpui_es_fluent::localize_label::<Settings>(cx);
```

They panic when the global or resource is missing. Use them in rendering paths
where a missing translation is an application defect.

Use the `try_*` helpers when the caller has a deliberate missing-state path:

```rust,ignore
if let Some(message) =
    gpui_es_fluent::try_localize_message(cx, &AppMessage::Save)
{
    show_optional_message(message);
}
```

Both fallible helpers return `None` when the global is absent or the typed
resource cannot be localized. They do not distinguish those causes.

## Access the embedded manager

Read the global directly when code needs an embedded-manager API that the
facade does not wrap:

```rust,ignore
let manager = cx.global::<gpui_es_fluent::I18n>().manager();
```

`manager()` returns a shared reference. Use the facade's locale helpers for the
common selection workflows.
