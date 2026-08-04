# gpui-component integration

Enable the `component` feature to keep `gpui-component` locale state and the
embedded `I18n` global aligned:

```toml
[dependencies]
gpui-component = { git = "https://github.com/longbridge/gpui-component" }
gpui-es-fluent = { git = "https://github.com/stayhydated/gpui-es-fluent", features = ["component"] }

[replace]
"https://github.com/zed-industries/zed#gpui@0.2.2" = { git = "https://github.com/zed-industries/zed", rev = "1a246efd7e1b83ab568ec5e3e6c1a43a42e1abba" }
"https://github.com/zed-industries/zed#gpui_macros@0.1.0" = { git = "https://github.com/zed-industries/zed", rev = "1a246efd7e1b83ab568ec5e3e6c1a43a42e1abba" }
```

Use the direct `gpui-component` dependency shown above so the application and
`gpui-es-fluent` share one component locale. The replacement entries align
`gpui-component` with the GPUI revision used in [Getting started](getting_started.md).

Initialize `gpui-component` first during application setup, set its startup
locale, and install `I18n` before opening windows:

```rust,ignore
gpui_component::init(cx);
gpui_component::set_locale("en-US");
gpui_es_fluent::init_from_component_locale(cx)?;
```

`init_from_component_locale` preserves an existing `I18n` global without
resynchronizing it. When a global may already exist, use
`sync_component_locale` to apply the component locale.

To change both systems at runtime, use `set_component_locale` and notify the
owning entity:

```rust,ignore
let language = gpui_es_fluent::set_component_locale(cx, "fr-FR")?;
cx.notify();
```

The returned `LanguageIdentifier` is the canonical parsed locale. A successful
call replaces `I18n`, updates the component locale, and makes subsequent
lookups use the selected language. These helpers use the best-effort module
selection described in [Selecting locales](locales.md#require-every-application-module).

## Synchronize an external change

When another part of the application changes the component locale, apply it to
an installed embedded manager:

```rust,ignore
gpui_component::set_locale("fr-FR");
let language = gpui_es_fluent::sync_component_locale(cx)?;
cx.notify();
```

`sync_component_locale` validates and returns the component language even when
`I18n` is absent; it installs no global in that case. Use
`init_from_component_locale` to install one.

Use `component_language()` when code only needs to read and validate the
component locale.

## Handle synchronization failures

The helpers return `ComponentLocaleError` with distinct cases:

- `InvalidLocale` for a string that is not a Unicode language identifier,
- `Initialization` when a manager cannot initialize the selected language, and
- `Selection` when an installed manager rejects a locale during synchronization.

`set_component_locale` validates the string before changing either system. If
manager initialization later fails, the component locale may already contain
the new value while any previous `I18n` global remains installed. Restore the
previous component locale or retry with a supported locale before rendering.
