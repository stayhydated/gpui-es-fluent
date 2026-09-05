#![doc = include_str!("../README.md")]

use es_fluent::{FluentLabel, FluentLocalizerExt as _, FluentMessage};
use gpui_kit::App;
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

    /// Attempts to localize a generated message through the underlying manager.
    pub fn try_localize_message<T>(&self, message: &T) -> Option<String>
    where
        T: FluentMessage + ?Sized,
    {
        self.manager.try_localize_message(message)
    }

    /// Localizes a generated label through the underlying manager.
    pub fn localize_label<T>(&self) -> String
    where
        T: FluentLabel,
    {
        T::localize_label(&self.manager)
    }

    /// Attempts to localize a generated label through the underlying manager.
    pub fn try_localize_label<T>(&self) -> Option<String>
    where
        T: FluentLabel,
    {
        T::try_localize_label(&self.manager)
    }
}

impl gpui_kit::Global for I18n {}

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

impl<L: Language> gpui_kit::Global for CurrentLanguage<L> {}

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
/// Returns `None` when no [`I18n`] global has been installed or the typed
/// message is missing from the active resources.
pub fn try_localize_message<T>(cx: &impl Borrow<App>, message: &T) -> Option<String>
where
    T: FluentMessage + ?Sized,
{
    cx.borrow()
        .try_global::<I18n>()?
        .try_localize_message(message)
}

/// Localizes `message` through the installed [`I18n`] global.
///
/// # Panics
///
/// Panics when no global is installed or when the typed Fluent message is
/// missing from the active resources.
pub fn localize_message<T>(cx: &impl Borrow<App>, message: &T) -> String
where
    T: FluentMessage + ?Sized,
{
    cx.borrow()
        .try_global::<I18n>()
        .map(|i18n| i18n.localize_message(message))
        .unwrap_or_else(|| panic!("gpui-es-fluent I18n global is not installed"))
}

/// Attempts to localize a generated label with the installed [`I18n`] global.
///
/// Returns `None` when no global is installed or the typed label is missing
/// from the active resources.
pub fn try_localize_label<T>(cx: &impl Borrow<App>) -> Option<String>
where
    T: FluentLabel,
{
    cx.borrow().try_global::<I18n>()?.try_localize_label::<T>()
}

/// Localizes a generated label through the installed [`I18n`] global.
///
/// # Panics
///
/// Panics when no global is installed or when the typed Fluent label is
/// missing from the active resources.
pub fn localize_label<T>(cx: &impl Borrow<App>) -> String
where
    T: FluentLabel,
{
    cx.borrow()
        .try_global::<I18n>()
        .map(I18n::localize_label::<T>)
        .unwrap_or_else(|| panic!("gpui-es-fluent I18n global is not installed"))
}

#[cfg(feature = "component")]
/// Errors produced while synchronizing GPUI Kit component locale state.
#[derive(Debug, thiserror::Error)]
pub enum ComponentLocaleError {
    /// The component locale is not a valid Unicode language identifier.
    #[error("invalid GPUI Kit component locale `{locale}`")]
    InvalidLocale {
        /// The rejected locale string.
        locale: String,
        /// The language identifier parse error.
        #[source]
        source: unic_langid::LanguageIdentifierError,
    },
    /// The embedded localization manager could not be initialized.
    #[error("failed to initialize gpui-es-fluent localization")]
    Initialization(#[from] EmbeddedInitError),
    /// The installed localization manager rejected the selected language.
    #[error("failed to select GPUI Kit component locale")]
    Selection(#[from] LocalizationError),
}

#[cfg(feature = "component")]
/// Reads the current GPUI Kit component locale as a language identifier.
///
/// # Errors
///
/// Returns [`ComponentLocaleError::InvalidLocale`] when GPUI Kit
/// contains an invalid language identifier.
pub fn component_language() -> Result<LanguageIdentifier, ComponentLocaleError> {
    let locale = gpui_kit::component::locale().to_string();
    locale
        .parse::<LanguageIdentifier>()
        .map_err(|source| ComponentLocaleError::InvalidLocale { locale, source })
}

#[cfg(feature = "component")]
/// Initializes [`I18n`] from the current GPUI Kit component locale.
///
/// # Errors
///
/// Returns [`ComponentLocaleError`] when the component locale is invalid or
/// the embedded localization manager cannot initialize it.
pub fn init_from_component_locale(cx: &mut App) -> Result<(), ComponentLocaleError> {
    init_with_language(cx, component_language()?)?;
    Ok(())
}

#[cfg(feature = "component")]
/// Sets GPUI Kit's component locale and replaces the [`I18n`] global to match it.
///
/// # Errors
///
/// Returns [`ComponentLocaleError`] when `locale` is invalid or the embedded
/// localization manager cannot initialize it.
pub fn set_component_locale(
    cx: &mut App,
    locale: impl AsRef<str>,
) -> Result<LanguageIdentifier, ComponentLocaleError> {
    let locale = locale.as_ref();
    let language = locale.parse::<LanguageIdentifier>().map_err(|source| {
        ComponentLocaleError::InvalidLocale {
            locale: locale.to_owned(),
            source,
        }
    })?;

    gpui_kit::component::set_locale(&language.to_string());
    replace_with_language(cx, language.clone())?;
    Ok(language)
}

#[cfg(feature = "component")]
/// Syncs the installed [`I18n`] global from the current GPUI Kit component locale.
///
/// The parsed language is returned even when no [`I18n`] global is installed.
///
/// # Errors
///
/// Returns [`ComponentLocaleError`] when the component locale is invalid or an
/// installed manager cannot select the parsed language.
pub fn sync_component_locale(
    cx: &impl Borrow<App>,
) -> Result<LanguageIdentifier, ComponentLocaleError> {
    let language = component_language()?;
    if let Some(i18n) = cx.borrow().try_global::<I18n>() {
        i18n.select_language(language.clone())?;
    }
    Ok(language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use es_fluent::registry::StaticFluentMessageKey;
    use es_fluent_manager_embedded::__manager_core::{
        FluentArgumentMap, I18nModule, I18nModuleDescriptor, I18nModuleRegistration, Localizer,
        ModuleData, ModuleDomain,
    };
    use std::sync::{Mutex, Once};
    use unic_langid::langid;

    const TEST_DOMAIN: &str = "gpui-es-fluent-test";

    static TEST_SUPPORTED_LANGUAGES: &[LanguageIdentifier] = &[langid!("en-US"), langid!("fr")];
    static TEST_MODULE_DATA: ModuleData = ModuleData {
        name: TEST_DOMAIN,
        owner: es_fluent_manager_embedded::__manager_core::__macro::static_domain(TEST_DOMAIN),
        supported_languages: TEST_SUPPORTED_LANGUAGES,
        domains: &[ModuleDomain {
            domain: es_fluent_manager_embedded::__manager_core::__macro::static_domain(TEST_DOMAIN),
            namespaces: &[],
        }],
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
            key: StaticFluentMessageKey,
            _args: Option<&FluentArgumentMap<'a>>,
        ) -> Option<String> {
            if key.owner() != TEST_DOMAIN || key.domain() != TEST_DOMAIN {
                return None;
            }
            let selected = self.selected.lock().unwrap().to_string();
            let value = match (selected.as_str(), key.id().as_str()) {
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
            localize(static_key("test_message"), None)
        }
    }

    struct TestLabel;

    impl FluentLabel for TestLabel {
        fn fluent_label_key() -> StaticFluentMessageKey {
            static_key("test_label")
        }
    }

    fn force_inventory_link() {
        INVENTORY_ONCE.call_once(|| {
            let _ = &TEST_MODULE;
        });
    }

    fn static_key(id: &'static str) -> StaticFluentMessageKey {
        es_fluent::registry::__macro::static_message_key(
            TEST_DOMAIN,
            es_fluent::registry::__macro::static_domain(TEST_DOMAIN),
            es_fluent::registry::__macro::static_entry_id(id),
        )
    }

    fn language(value: &str) -> LanguageIdentifier {
        value.parse().unwrap()
    }

    fn with_test_app(test: impl FnOnce(&mut gpui_kit::TestAppContext)) {
        let mut cx = gpui_kit::TestAppContext::single();
        test(&mut cx);
        cx.quit();
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

        assert_eq!(i18n.try_localize_message(&TestMessage), None);
        assert_eq!(i18n.try_localize_label::<TestLabel>(), None);
    }

    #[test]
    fn app_try_helpers_report_a_missing_i18n_global() {
        with_test_app(|cx| {
            cx.update(|cx| {
                assert_eq!(try_localize_message(&*cx, &TestMessage), None);
                assert_eq!(try_localize_label::<TestLabel>(&*cx), None);
            })
        });
    }

    #[test]
    #[should_panic(expected = "gpui-es-fluent I18n global is not installed")]
    fn app_localize_message_panics_without_i18n_global() {
        with_test_app(|cx| {
            cx.update(|cx| {
                localize_message(&*cx, &TestMessage);
            })
        });
    }

    #[test]
    fn init_installs_i18n_once_and_change_locale_updates_it() {
        force_inventory_link();

        with_test_app(|cx| {
            cx.update(|cx| {
                init(cx).unwrap();
                assert_eq!(try_localize_message(&*cx, &TestMessage), None);

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

    #[cfg(feature = "component")]
    static COMPONENT_LOCALE_LOCK: Mutex<()> = Mutex::new(());

    #[cfg(feature = "component")]
    fn component_locale_lock() -> std::sync::MutexGuard<'static, ()> {
        COMPONENT_LOCALE_LOCK
            .lock()
            .unwrap_or_else(|error| error.into_inner())
    }

    #[cfg(feature = "component")]
    #[test]
    fn component_language_parses_component_locale_and_rejects_invalid_state() {
        let _guard = component_locale_lock();

        gpui_kit::component::set_locale("fr");
        assert_eq!(component_language().unwrap(), language("fr"));

        gpui_kit::component::set_locale("not a locale");
        assert!(matches!(
            component_language(),
            Err(ComponentLocaleError::InvalidLocale { locale, .. }) if locale == "not a locale"
        ));
        gpui_kit::component::set_locale("en-US");
    }

    #[cfg(feature = "component")]
    #[test]
    fn init_from_component_locale_uses_current_component_locale() {
        let _guard = component_locale_lock();
        force_inventory_link();
        gpui_kit::component::set_locale("en-US");

        with_test_app(|cx| {
            cx.update(|cx| {
                init_from_component_locale(cx).unwrap();
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
                assert_eq!(set_component_locale(cx, "fr").unwrap(), language("fr"));
                assert_eq!(&*gpui_kit::component::locale(), "fr");
                assert_eq!(localize_message(&*cx, &TestMessage), "Bonjour du test");

                assert!(matches!(
                    set_component_locale(cx, "not a locale"),
                    Err(ComponentLocaleError::InvalidLocale { locale, .. })
                        if locale == "not a locale"
                ));
                assert_eq!(&*gpui_kit::component::locale(), "fr");
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
                gpui_kit::component::set_locale("fr");

                assert_eq!(sync_component_locale(&*cx).unwrap(), language("fr"));
                assert_eq!(localize_message(&*cx, &TestMessage), "Bonjour du test");
            })
        });
    }

    #[cfg(feature = "component")]
    #[test]
    fn sync_component_locale_returns_language_without_i18n_global() {
        let _guard = component_locale_lock();
        gpui_kit::component::set_locale("fr");

        with_test_app(|cx| {
            cx.update(|cx| {
                assert_eq!(sync_component_locale(&*cx).unwrap(), language("fr"));
            })
        });
    }
}
