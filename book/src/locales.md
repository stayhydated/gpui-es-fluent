# Selecting locales

Select the initial language during startup, then change the installed manager
and notify the affected entity when the user chooses another locale.

## Select the startup locale

```rust,ignore
let initial = "en".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::init_with_language(cx, initial)?;
```

`init_with_language` preserves an existing global. Use
`replace_with_language` when the application must create a fresh manager even
if one has already been installed:

```rust,ignore
let restored = "fr-FR".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::replace_with_language(cx, restored)?;
```

To change the language on an existing manager without replacing the global,
use `change_locale` as described below.

Both helpers return an initialization error when module discovery fails, when
no discovered application module supports the requested language, or when its
resources cannot form a usable Fluent bundle.

## Require every application module

The facade's language-selection helpers use `EmbeddedI18n`'s best-effort
policy. A locale switch succeeds when at least one discovered application
module accepts the locale; modules that reject it are left out of the active
manager, so lookups for their resources remain missing.

When every discovered module must support the locale, install the global
without a language and select through the underlying manager's strict API:

```rust,ignore
let initial = "en".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::init(cx)?;
cx.global::<gpui_es_fluent::I18n>()
    .manager()
    .select_language_strict(initial)?;
```

At runtime, call `select_language_strict` through the same manager before
notifying the entity. A failed strict switch leaves the previously active
manager resources unchanged.

## Change the runtime locale

`change_locale` selects a language on the installed manager. It panics when the
global is missing and returns a localization error when no application module
supports the language or a module reports another selection failure:

```rust,ignore
let french = "fr-FR".parse::<unic_langid::LanguageIdentifier>()?;
gpui_es_fluent::change_locale(cx, french)?;
cx.notify();
```

GPUI globals do not automatically rerender entities. Call `cx.notify()` on the
owning `gpui::Context<T>` after a successful change, as shown above.

## Track a typed language choice

Applications with `es-fluent-lang` and `strum` can generate a
supported-language enum that meets the `CurrentLanguage<L>` bounds:

```toml
[dependencies]
es-fluent-lang = { git = "https://github.com/stayhydated/es-fluent" }
strum = { version = "0.28", features = ["derive"] }
```

```rust,ignore
use es_fluent_lang::es_fluent_language;
use strum::EnumIter;

#[es_fluent_language]
#[derive(EnumIter)]
pub enum Languages {}
```

Install the typed value as a second GPUI global:

```rust,ignore
cx.set_global(gpui_es_fluent::CurrentLanguage(Languages::default()));
```

`CurrentLanguage` is application-managed state. Locale helpers do not update it,
so keep both globals synchronized in the locale-selection handler:

```rust,ignore
let selected = Languages::FrFr;
let language: unic_langid::LanguageIdentifier = selected.into();

gpui_es_fluent::change_locale(cx, language)?;
cx.set_global(gpui_es_fluent::CurrentLanguage(selected));
cx.notify();
```

The `Language` trait captures the bounds implemented by generated language
enums. The active embedded manager remains the source used by localization
lookups.
