#![doc = include_str!("../README.md")]

use es_fluent::{
    FluentArgs, FluentLabel, FluentLocalizer, FluentLocalizerExt as _, FluentMessage,
    registry::{StaticFluentDomain, StaticFluentEntryId},
};
use gpui::App;
use std::borrow::Borrow;
use strum::IntoEnumIterator;
use unic_langid::LanguageIdentifier;

pub use es_fluent_manager_embedded::{EmbeddedI18n, EmbeddedInitError, LocalizationError};

/// GPUI global wrapper around the embedded `es-fluent` localization manager.
#[derive(Clone)]
pub struct I18n {
    manager: EmbeddedI18n,
}

impl I18n {
    /// Creates a localization manager using the embedded default language.
    ///
    /// # Errors
    ///
    /// Returns [`EmbeddedInitError`] when the embedded localization manager
    /// cannot be initialized.
    pub fn new() -> Result<Self, EmbeddedInitError> {
        Ok(Self {
            manager: EmbeddedI18n::try_new()?,
        })
    }

    /// Creates a localization manager initialized with the requested language.
    ///
    /// # Errors
    ///
    /// Returns [`EmbeddedInitError`] when the embedded localization manager
    /// cannot be initialized for `language`.
    pub fn new_with_language(
        language: impl Into<LanguageIdentifier>,
    ) -> Result<Self, EmbeddedInitError> {
        Ok(Self {
            manager: EmbeddedI18n::try_new_with_language(language)?,
        })
    }

    /// Returns the underlying embedded localization manager.
    pub fn manager(&self) -> &EmbeddedI18n {
        &self.manager
    }

    /// Selects the active language on the underlying manager.
    ///
    /// # Errors
    ///
    /// Returns [`LocalizationError`] when the manager cannot select `language`.
    pub fn select_language(
        &self,
        language: impl Into<LanguageIdentifier>,
    ) -> Result<(), LocalizationError> {
        self.manager.select_language(language)
    }

    /// Localizes a generated message through the underlying manager.
    pub fn localize_message<T>(&self, message: &T) -> String
    where
        T: FluentMessage + ?Sized,
    {
        self.manager.localize_message(message)
    }

    /// Localizes a generated label through the underlying manager.
    pub fn localize_label<T>(&self) -> String
    where
        T: FluentLabel,
    {
        T::localize_label(&self.manager)
    }
}

impl gpui::Global for I18n {}

/// Bounds used by generated language enums that can be stored in GPUI globals.
pub trait Language:
    'static
    + Copy
    + Clone
    + Send
    + Sync
    + IntoEnumIterator
    + TryInto<LanguageIdentifier>
    + TryFrom<LanguageIdentifier>
    + FluentMessage
    + Default
    + std::fmt::Debug
{
}

impl<T> Language for T where
    T: 'static
        + Copy
        + Clone
        + Send
        + Sync
        + IntoEnumIterator
        + TryInto<LanguageIdentifier>
        + TryFrom<LanguageIdentifier>
        + FluentMessage
        + Default
        + std::fmt::Debug
{
}

/// GPUI global wrapper for a typed current-language value.
#[derive(Clone, Copy)]
pub struct CurrentLanguage<L: Language>(pub L);

impl<L: Language> gpui::Global for CurrentLanguage<L> {}

/// Installs an [`I18n`] global if one is not already present.
///
/// # Errors
///
/// Returns [`EmbeddedInitError`] when no [`I18n`] global exists and the embedded
/// localization manager cannot be initialized.
pub fn init(cx: &mut App) -> Result<(), EmbeddedInitError> {
    if cx.try_global::<I18n>().is_none() {
        cx.set_global(I18n::new()?);
    }
    Ok(())
}

/// Installs an [`I18n`] global for `language` if one is not already present.
///
/// # Errors
///
/// Returns [`EmbeddedInitError`] when no [`I18n`] global exists and the embedded
/// localization manager cannot be initialized for `language`.
pub fn init_with_language(
    cx: &mut App,
    language: impl Into<LanguageIdentifier>,
) -> Result<(), EmbeddedInitError> {
    if cx.try_global::<I18n>().is_none() {
        cx.set_global(I18n::new_with_language(language)?);
    }
    Ok(())
}

/// Replaces any existing [`I18n`] global with one initialized for `language`.
///
/// # Errors
///
/// Returns [`EmbeddedInitError`] when the embedded localization manager cannot
/// be initialized for `language`.
pub fn replace_with_language(
    cx: &mut App,
    language: impl Into<LanguageIdentifier>,
) -> Result<(), EmbeddedInitError> {
    cx.set_global(I18n::new_with_language(language)?);
    Ok(())
}

/// Changes the active locale on the installed [`I18n`] global.
///
/// This expects [`init`], [`init_with_language`], or [`replace_with_language`] to
/// have installed the global first.
///
/// # Errors
///
/// Returns [`LocalizationError`] when the installed manager cannot select
/// `language`.
///
/// # Panics
///
/// Panics when no [`I18n`] global has been installed.
pub fn change_locale(
    cx: &mut App,
    language: impl Into<LanguageIdentifier>,
) -> Result<(), LocalizationError> {
    cx.global::<I18n>().select_language(language)
}

/// Attempts to localize `message` with the installed [`I18n`] global.
///
/// Returns `None` when no [`I18n`] global has been installed.
pub fn try_localize_message<T>(cx: &impl Borrow<App>, message: &T) -> Option<String>
where
    T: FluentMessage + ?Sized,
{
    Some(cx.borrow().try_global::<I18n>()?.localize_message(message))
}

/// Localizes `message`, falling back to [`fallback_message`] when no global exists.
pub fn localize_message<T>(cx: &impl Borrow<App>, message: &T) -> String
where
    T: FluentMessage + ?Sized,
{
    cx.borrow()
        .try_global::<I18n>()
        .map(|i18n| i18n.localize_message(message))
        .unwrap_or_else(|| fallback_message(message))
}

/// Localizes a generated label, falling back to [`fallback_label`] when no global exists.
pub fn localize_label<T>(cx: &impl Borrow<App>) -> String
where
    T: FluentLabel,
{
    cx.borrow()
        .try_global::<I18n>()
        .map(I18n::localize_label::<T>)
        .unwrap_or_else(fallback_label::<T>)
}

/// Renders a generated message with the fallback localizer.
pub fn fallback_message<T>(message: &T) -> String
where
    T: FluentMessage + ?Sized,
{
    FallbackLocalizer.localize_message(message)
}

/// Renders a generated label with the fallback localizer.
pub fn fallback_label<T>() -> String
where
    T: FluentLabel,
{
    es_fluent::fallback_label::<T>()
}

/// Localizer that renders message and label IDs as readable fallback text.
pub struct FallbackLocalizer;

impl FluentLocalizer for FallbackLocalizer {
    fn localize<'a>(
        &self,
        id: StaticFluentEntryId,
        _args: Option<&FluentArgs<'a>>,
    ) -> Option<String> {
        Some(es_fluent::humanize_fluent_entry_id(id))
    }

    fn localize_in_domain<'a>(
        &self,
        _domain: StaticFluentDomain,
        id: StaticFluentEntryId,
        _args: Option<&FluentArgs<'a>>,
    ) -> Option<String> {
        Some(es_fluent::humanize_fluent_entry_id(id))
    }
}

/// Converts a Fluent entry ID into display text for fallback rendering.
///
/// The conversion strips a trailing `_label`, splits on `_` and `-`, drops empty
/// segments, and uppercases the first character of each remaining segment.
pub fn humanize_key(id: &str) -> String {
    let id = id.strip_suffix("_label").unwrap_or(id);
    let mut output = String::with_capacity(id.len());

    for part in id.split(['_', '-']).filter(|part| !part.is_empty()) {
        if !output.is_empty() {
            output.push(' ');
        }

        let Some(first) = part.chars().next() else {
            continue;
        };

        output.extend(first.to_uppercase());
        output.push_str(&part[first.len_utf8()..]);
    }

    output
}

#[cfg(feature = "component")]
/// Reads the current `gpui-component` locale as a language identifier.
///
/// If the component locale is invalid, the already-parsed `fallback` language
/// is returned instead.
pub fn component_language(fallback: impl Into<LanguageIdentifier>) -> LanguageIdentifier {
    let fallback = fallback.into();
    gpui_component::locale()
        .parse::<LanguageIdentifier>()
        .unwrap_or(fallback)
}

#[cfg(feature = "component")]
/// Initializes [`I18n`] from the current `gpui-component` locale.
///
/// # Errors
///
/// Returns [`EmbeddedInitError`] when no [`I18n`] global exists and the embedded
/// localization manager cannot be initialized for the component locale.
pub fn init_from_component_locale(
    cx: &mut App,
    fallback: impl Into<LanguageIdentifier>,
) -> Result<(), EmbeddedInitError> {
    init_with_language(cx, component_language(fallback))
}

#[cfg(feature = "component")]
/// Sets `gpui-component`'s locale and replaces the [`I18n`] global to match it.
///
/// Invalid `locale` values fall back to `fallback`, which must be a valid
/// language identifier. The selected language is returned.
///
/// # Errors
///
/// Returns [`EmbeddedInitError`] when the embedded localization manager cannot
/// be initialized for the selected language.
pub fn set_component_locale(
    cx: &mut App,
    locale: impl AsRef<str>,
    fallback: impl Into<LanguageIdentifier>,
) -> Result<LanguageIdentifier, EmbeddedInitError> {
    let language = locale
        .as_ref()
        .parse::<LanguageIdentifier>()
        .unwrap_or_else(|_| fallback.into());

    gpui_component::set_locale(&language.to_string());
    replace_with_language(cx, language.clone())?;
    Ok(language)
}

#[cfg(feature = "component")]
/// Syncs the installed [`I18n`] global from the current `gpui-component` locale.
///
/// The parsed language is returned even when no [`I18n`] global is installed.
///
/// # Errors
///
/// Returns [`LocalizationError`] when an installed manager cannot select the
/// parsed language.
pub fn sync_component_locale(
    cx: &impl Borrow<App>,
    fallback: impl Into<LanguageIdentifier>,
) -> Result<LanguageIdentifier, LocalizationError> {
    let language = component_language(fallback);
    if let Some(i18n) = cx.borrow().try_global::<I18n>() {
        i18n.select_language(language.clone())?;
    }
    Ok(language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use es_fluent::registry::StaticFluentArgumentName;
    use es_fluent_manager_embedded::__manager_core::{
        FluentArgumentMap, I18nModule, I18nModuleDescriptor, I18nModuleRegistration, Localizer,
        ModuleData,
    };
    use std::sync::{Mutex, Once};
    use unic_langid::langid;

    const TEST_DOMAIN: &str = "gpui-es-fluent-test";

    static TEST_SUPPORTED_LANGUAGES: &[LanguageIdentifier] = &[langid!("en-US"), langid!("fr")];
    static TEST_MODULE_DATA: ModuleData = ModuleData {
        name: TEST_DOMAIN,
        domain: es_fluent_manager_embedded::__manager_core::__macro::static_domain(TEST_DOMAIN),
        supported_languages: TEST_SUPPORTED_LANGUAGES,
        namespaces: &[],
    };
    static TEST_MODULE: TestModule = TestModule;
    static INVENTORY_ONCE: Once = Once::new();

    es_fluent_manager_embedded::__inventory::submit!(&TEST_MODULE as &dyn I18nModuleRegistration);

    struct TestModule;

    impl I18nModuleDescriptor for TestModule {
        fn data(&self) -> &'static ModuleData {
            &TEST_MODULE_DATA
        }
    }

    impl I18nModule for TestModule {
        fn create_localizer(&self) -> Box<dyn Localizer> {
            Box::new(TestLocalizer {
                selected: Mutex::new(langid!("en-US")),
            })
        }
    }

    struct TestLocalizer {
        selected: Mutex<LanguageIdentifier>,
    }

    impl Localizer for TestLocalizer {
        fn select_language(&self, lang: &LanguageIdentifier) -> Result<(), LocalizationError> {
            if TEST_SUPPORTED_LANGUAGES
                .iter()
                .any(|candidate| candidate == lang)
            {
                *self.selected.lock().unwrap() = lang.clone();
                Ok(())
            } else {
                Err(LocalizationError::LanguageNotSupported(lang.clone()))
            }
        }

        fn localize<'a>(
            &self,
            id: StaticFluentEntryId,
            _args: Option<&FluentArgumentMap<'a>>,
        ) -> Option<String> {
            let selected = self.selected.lock().unwrap().to_string();
            let value = match (selected.as_str(), id.as_str()) {
                ("en-US", "test_message") => "Hello from test",
                ("en-US", "test_label") => "Test label",
                ("fr", "test_message") => "Bonjour du test",
                ("fr", "test_label") => "Etiquette de test",
                _ => return None,
            };

            Some(value.to_string())
        }
    }

    struct TestMessage;

    impl FluentMessage for TestMessage {
        fn to_fluent_string_with(
            &self,
            localize: &mut es_fluent::FluentMessageLookup<'_>,
        ) -> String {
            localize(
                static_domain(TEST_DOMAIN),
                static_entry("test_message"),
                None,
            )
        }
    }

    struct TestLabel;

    impl FluentLabel for TestLabel {
        fn fluent_label_domain() -> StaticFluentDomain {
            static_domain(TEST_DOMAIN)
        }

        fn fluent_label_id() -> StaticFluentEntryId {
            static_entry("test_label")
        }
    }

    fn force_inventory_link() {
        INVENTORY_ONCE.call_once(|| {
            let _ = &TEST_MODULE;
        });
    }

    fn static_domain(value: &'static str) -> StaticFluentDomain {
        StaticFluentDomain::try_new(value).unwrap()
    }

    fn static_entry(value: &'static str) -> StaticFluentEntryId {
        StaticFluentEntryId::try_new(value).unwrap()
    }

    fn language(value: &str) -> LanguageIdentifier {
        value.parse().unwrap()
    }

    fn with_test_app(test: impl FnOnce(&mut gpui::TestAppContext)) {
        let mut cx = gpui::TestAppContext::single();
        test(&mut cx);
        cx.quit();
    }

    #[test]
    fn humanize_key_strips_label_suffix_and_title_cases_segments() {
        assert_eq!(
            humanize_key("sales-order_status_label"),
            "Sales Order Status"
        );
        assert_eq!(humanize_key("__line-item__"), "Line Item");
    }

    #[test]
    fn fallback_helpers_render_readable_message_and_label_text() {
        let localizer = FallbackLocalizer;

        assert_eq!(fallback_message(&TestMessage), "Test Message");
        assert_eq!(fallback_label::<TestLabel>(), "Test");
        assert_eq!(
            localizer.localize(static_entry("primary-action_label"), None),
            Some("Primary Action".to_string())
        );
        assert_eq!(
            localizer.localize_in_domain(
                static_domain(TEST_DOMAIN),
                static_entry("secondary_action"),
                Some(&FluentArgs::new()),
            ),
            Some("Secondary Action".to_string())
        );
    }

    #[test]
    fn i18n_facade_delegates_to_embedded_manager() {
        force_inventory_link();
        let i18n = I18n::new_with_language(language("en-US")).unwrap();

        assert_eq!(
            i18n.manager().localize_message(&TestMessage),
            "Hello from test"
        );
        assert_eq!(i18n.localize_message(&TestMessage), "Hello from test");
        assert_eq!(i18n.localize_label::<TestLabel>(), "Test label");

        i18n.select_language(language("fr")).unwrap();

        assert_eq!(i18n.localize_message(&TestMessage), "Bonjour du test");
        assert_eq!(i18n.localize_label::<TestLabel>(), "Etiquette de test");
    }

    #[test]
    fn i18n_can_initialize_before_a_language_is_selected() {
        force_inventory_link();
        let i18n = I18n::new().unwrap();

        assert_eq!(i18n.localize_message(&TestMessage), "test_message");
        assert_eq!(i18n.localize_label::<TestLabel>(), "test_label");
    }

    #[test]
    fn app_helpers_use_fallbacks_without_i18n_global() {
        with_test_app(|cx| {
            cx.update(|cx| {
                assert_eq!(try_localize_message(&*cx, &TestMessage), None);
                assert_eq!(localize_message(&*cx, &TestMessage), "Test Message");
                assert_eq!(localize_label::<TestLabel>(&*cx), "Test");
            })
        });
    }

    #[test]
    fn init_installs_i18n_once_and_change_locale_updates_it() {
        force_inventory_link();

        with_test_app(|cx| {
            cx.update(|cx| {
                init(cx).unwrap();
                assert_eq!(
                    try_localize_message(&*cx, &TestMessage),
                    Some("test_message".to_string())
                );

                let initial = cx.global::<I18n>().manager() as *const EmbeddedI18n;
                init(cx).unwrap();
                assert_eq!(
                    cx.global::<I18n>().manager() as *const EmbeddedI18n,
                    initial
                );

                change_locale(cx, language("en-US")).unwrap();
                assert_eq!(
                    try_localize_message(&*cx, &TestMessage),
                    Some("Hello from test".to_string())
                );
            })
        });
    }

    #[test]
    fn language_initialization_preserves_existing_global_until_replace() {
        force_inventory_link();

        with_test_app(|cx| {
            cx.update(|cx| {
                init_with_language(cx, language("en-US")).unwrap();
                assert_eq!(localize_message(&*cx, &TestMessage), "Hello from test");

                init_with_language(cx, language("fr")).unwrap();
                assert_eq!(localize_message(&*cx, &TestMessage), "Hello from test");

                replace_with_language(cx, language("fr")).unwrap();
                assert_eq!(localize_message(&*cx, &TestMessage), "Bonjour du test");
            })
        });
    }

    #[test]
    fn language_selection_errors_are_returned_by_wrappers() {
        force_inventory_link();
        let i18n = I18n::new_with_language(language("en-US")).unwrap();

        assert!(i18n.select_language(language("de")).is_err());
        assert!(I18n::new_with_language(language("de")).is_err());
    }

    #[test]
    fn fluent_args_can_be_passed_through_fallback_localizer() {
        let mut args = FluentArgs::new();
        args.insert(
            StaticFluentArgumentName::try_new("count").unwrap(),
            es_fluent::FluentValue::from(3),
        );

        assert_eq!(
            FallbackLocalizer.localize(static_entry("items-total"), Some(&args)),
            Some("Items Total".to_string())
        );
    }

    #[cfg(feature = "component")]
    static COMPONENT_LOCALE_LOCK: Mutex<()> = Mutex::new(());

    #[cfg(feature = "component")]
    fn component_locale_lock() -> std::sync::MutexGuard<'static, ()> {
        COMPONENT_LOCALE_LOCK.lock().unwrap()
    }

    #[cfg(feature = "component")]
    #[test]
    fn component_language_parses_component_locale_or_uses_fallback() {
        let _guard = component_locale_lock();
        let fallback = language("en-US");

        gpui_component::set_locale("fr");
        assert_eq!(component_language(fallback.clone()), language("fr"));

        gpui_component::set_locale("not a locale");
        assert_eq!(component_language(fallback.clone()), fallback);
    }

    #[cfg(feature = "component")]
    #[test]
    fn init_from_component_locale_uses_current_component_locale() {
        let _guard = component_locale_lock();
        force_inventory_link();
        gpui_component::set_locale("en-US");

        with_test_app(|cx| {
            cx.update(|cx| {
                init_from_component_locale(cx, language("fr")).unwrap();
                assert_eq!(localize_message(&*cx, &TestMessage), "Hello from test");
            })
        });
    }

    #[cfg(feature = "component")]
    #[test]
    fn set_component_locale_updates_component_and_i18n() {
        let _guard = component_locale_lock();
        force_inventory_link();

        with_test_app(|cx| {
            cx.update(|cx| {
                assert_eq!(
                    set_component_locale(cx, "fr", language("en-US")).unwrap(),
                    language("fr")
                );
                assert_eq!(&*gpui_component::locale(), "fr");
                assert_eq!(localize_message(&*cx, &TestMessage), "Bonjour du test");

                assert_eq!(
                    set_component_locale(cx, "not a locale", language("en-US")).unwrap(),
                    language("en-US")
                );
                assert_eq!(&*gpui_component::locale(), "en-US");
                assert_eq!(localize_message(&*cx, &TestMessage), "Hello from test");
            })
        });
    }

    #[cfg(feature = "component")]
    #[test]
    fn sync_component_locale_updates_i18n_when_global_exists() {
        let _guard = component_locale_lock();
        force_inventory_link();

        with_test_app(|cx| {
            cx.update(|cx| {
                init_with_language(cx, language("en-US")).unwrap();
                gpui_component::set_locale("fr");

                assert_eq!(
                    sync_component_locale(&*cx, language("en-US")).unwrap(),
                    language("fr")
                );
                assert_eq!(localize_message(&*cx, &TestMessage), "Bonjour du test");
            })
        });
    }

    #[cfg(feature = "component")]
    #[test]
    fn sync_component_locale_returns_language_without_i18n_global() {
        let _guard = component_locale_lock();
        gpui_component::set_locale("fr");

        with_test_app(|cx| {
            cx.update(|cx| {
                assert_eq!(
                    sync_component_locale(&*cx, language("en-US")).unwrap(),
                    language("fr")
                );
            })
        });
    }
}
