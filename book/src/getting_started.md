# Getting started

This tutorial defines an embedded Fluent message, installs the localization
global with an active language, and renders the message from a GPUI context.

## Prerequisites

Start with:

- Rust 1.96 or newer,
- an existing GPUI application,
- the `cargo es-fluent` command, and
- the current Git dependencies:

```toml
[dependencies]
es-fluent = { git = "https://github.com/stayhydated/es-fluent" }
es-fluent-manager-embedded = { git = "https://github.com/stayhydated/es-fluent" }
gpui = { git = "https://github.com/zed-industries/zed", rev = "1a246efd7e1b83ab568ec5e3e6c1a43a42e1abba" }
gpui-es-fluent = { git = "https://github.com/stayhydated/gpui-es-fluent" }
unic-langid = "0.9"

[build-dependencies]
es-fluent-build = { git = "https://github.com/stayhydated/es-fluent" }
```

Keep the shared `es-fluent` and GPUI dependencies on these sources. Cargo treats
registry and Git builds as distinct crate identities, even when their version
numbers match.

## 1. Configure the locale assets

Create `i18n.toml` next to the application's `Cargo.toml`:

```toml
fallback_language = "en"
assets_dir = "assets/i18n"
```

Create the fallback locale directory at `assets/i18n/en`. Locale directory
names use canonical BCP-47 language tags such as `en` and `fr-FR`.

## 2. Expose the messages and manager

Keep localizable types and the embedded manager module reachable from the
application's library target. For example, `src/lib.rs` can contain:

```rust,ignore
use es_fluent::EsFluent;

pub mod i18n;

#[derive(Clone, Copy, Debug, EsFluent)]
pub enum AppMessage {
    Welcome,
    Save,
}
```

Define the embedded manager module in `src/i18n.rs`:

```rust,ignore
es_fluent_manager_embedded::define_i18n_module!();
```

Track locale assets from `build.rs` so changing an FTL file or locale directory
reruns the build script:

```rust,ignore
fn main() {
    es_fluent_build::track_i18n_assets();
}
```

After adding or changing a derived message, generate and validate the FTL:

```sh
cargo es-fluent generate
cargo es-fluent check --all-locales
```

Review the generated fallback values and add translations before starting the
application.

## 3. Install an active language

Install the GPUI global during application setup, before opening windows:

```rust,ignore
app.run(|cx| {
    let language = "en"
        .parse::<unic_langid::LanguageIdentifier>()
        .expect("the fallback locale should be valid");
    gpui_es_fluent::init_with_language(cx, language)
        .expect("the fallback locale should initialize");

    // Open application windows after globals are ready.
});
```

`init_with_language` selects the requested locale when it installs the global.
Use `init` instead when the embedded manager's configured default language is
the desired startup language. Both helpers preserve an existing `I18n` global.

## 4. Render a typed message

Use typed messages from a render context:

```rust,ignore
let text = gpui_es_fluent::localize_message(cx, &AppMessage::Welcome);
```

The setup is working when the view renders the translated `Welcome` value. A
fallible lookup also returns `Some` after the global and resource are available:

```rust,ignore
assert!(
    gpui_es_fluent::try_localize_message(cx, &AppMessage::Welcome).is_some()
);
```

## Troubleshooting

- **A `localize_*` lookup says the global is not installed:** run an
  initialization helper before opening the application's windows.
- **The global exists but a hard-failing lookup still panics:** select a
  supported locale with `init_with_language` or `change_locale`, then check
  that the generated FTL contains the message.
- **A message trait bound or `gpui::App` type does not match:** run
  `cargo tree -d` from the application root and align duplicate `es-fluent` or
  GPUI packages with the dependency sources above.
- **`cargo es-fluent` cannot find the derived type:** keep the type and
  `pub mod i18n;` reachable from a library target rather than only from
  `src/main.rs`, then rerun `cargo es-fluent generate`.
