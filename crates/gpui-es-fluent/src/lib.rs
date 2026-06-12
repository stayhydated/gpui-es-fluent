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
    pub fn new() -> Result<Self, EmbeddedInitError> {
        Ok(Self {
            manager: EmbeddedI18n::try_new()?,
        })
    }

    /// Creates a localization manager initialized with the requested language.
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
pub fn init(cx: &mut App) -> Result<(), EmbeddedInitError> {
    if cx.try_global::<I18n>().is_none() {
        cx.set_global(I18n::new()?);
    }
    Ok(())
}

/// Installs an [`I18n`] global for `language` if one is not already present.
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
    T::localize_label(&FallbackLocalizer)
}

/// Localizer that renders message and label IDs as readable fallback text.
pub struct FallbackLocalizer;

impl FluentLocalizer for FallbackLocalizer {
    fn localize<'a>(
        &self,
        id: StaticFluentEntryId,
        _args: Option<&FluentArgs<'a>>,
    ) -> Option<String> {
        Some(humanize_key(id.as_ref()))
    }

    fn localize_in_domain<'a>(
        &self,
        _domain: StaticFluentDomain,
        id: StaticFluentEntryId,
        _args: Option<&FluentArgs<'a>>,
    ) -> Option<String> {
        Some(humanize_key(id.as_ref()))
    }
}

/// Converts a Fluent entry ID into display text for fallback rendering.
///
/// The conversion strips a trailing `_label`, splits on `_` and `-`, drops empty
/// segments, and uppercases the first character of each remaining segment.
pub fn humanize_key(id: &str) -> String {
    let id = id.strip_suffix("_label").unwrap_or(id);
    id.split(['_', '-'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(feature = "component")]
/// Reads the current `gpui-component` locale as a language identifier.
///
/// If the component locale is invalid, `fallback` is parsed instead. The
/// fallback string must be a valid language identifier.
pub fn component_language(fallback: &str) -> LanguageIdentifier {
    gpui_component::locale()
        .parse::<LanguageIdentifier>()
        .or_else(|_| fallback.parse::<LanguageIdentifier>())
        .expect("fallback language must be a valid language identifier")
}

#[cfg(feature = "component")]
/// Initializes [`I18n`] from the current `gpui-component` locale.
pub fn init_from_component_locale(cx: &mut App, fallback: &str) -> Result<(), EmbeddedInitError> {
    init_with_language(cx, component_language(fallback))
}

#[cfg(feature = "component")]
/// Sets `gpui-component`'s locale and replaces the [`I18n`] global to match it.
///
/// Invalid `locale` values fall back to `fallback`, which must be a valid
/// language identifier. The selected language is returned.
pub fn set_component_locale(
    cx: &mut App,
    locale: impl AsRef<str>,
    fallback: &str,
) -> Result<LanguageIdentifier, EmbeddedInitError> {
    let language = locale
        .as_ref()
        .parse::<LanguageIdentifier>()
        .unwrap_or_else(|_| {
            fallback
                .parse()
                .expect("fallback language must be a valid language identifier")
        });

    gpui_component::set_locale(&language.to_string());
    replace_with_language(cx, language.clone())?;
    Ok(language)
}

#[cfg(feature = "component")]
/// Syncs the installed [`I18n`] global from the current `gpui-component` locale.
///
/// The parsed language is returned even when no [`I18n`] global is installed.
pub fn sync_component_locale(cx: &impl Borrow<App>, fallback: &str) -> LanguageIdentifier {
    let language = component_language(fallback);
    if let Some(i18n) = cx.borrow().try_global::<I18n>() {
        let _ = i18n.select_language(language.clone());
    }
    language
}
