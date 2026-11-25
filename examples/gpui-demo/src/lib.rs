use es_fluent::EsFluent;

pub mod i18n;

#[derive(Clone, Copy, Debug, EsFluent)]
#[fluent(namespace = "demo")]
pub enum DemoMessages {
    Heading,
    Body,
    ChangeLocale,
}

#[cfg(test)]
mod tests {
    use super::*;
    use es_fluent::unic_langid::LanguageIdentifier;
    use gpui_es_fluent::I18n;

    #[test]
    fn typed_demo_messages_follow_locale_switches() {
        i18n::link();
        let i18n = I18n::new().expect("the embedded demo catalogs should load");
        for (locale, expected) in [
            (
                "en",
                [
                    "One GPUI view, two locales",
                    "The text below comes from typed Fluent messages stored in GPUI global state.",
                    "Lire en français",
                ],
            ),
            (
                "fr-FR",
                [
                    "Une vue GPUI, deux langues",
                    "Le texte ci-dessous vient de messages Fluent typés stockés dans l’état global de GPUI.",
                    "Read in English",
                ],
            ),
            (
                "en",
                [
                    "One GPUI view, two locales",
                    "The text below comes from typed Fluent messages stored in GPUI global state.",
                    "Lire en français",
                ],
            ),
        ] {
            i18n.select_language(locale.parse::<LanguageIdentifier>().unwrap())
                .expect("the selected demo locale should load");
            for (message, expected) in [
                DemoMessages::Heading,
                DemoMessages::Body,
                DemoMessages::ChangeLocale,
            ]
            .into_iter()
            .zip(expected)
            {
                assert_eq!(
                    i18n.try_localize_message(&message).as_deref(),
                    Some(expected),
                    "{locale}: {message:?}"
                );
            }
        }
    }

    #[test]
    fn typed_demo_messages_use_parent_locale_and_preserve_it_after_rejection() {
        i18n::link();
        let i18n = I18n::new_with_language("fr-FR-fonipa".parse::<LanguageIdentifier>().unwrap())
            .expect("the French regional catalog should serve a variant");
        let french_heading = "Une vue GPUI, deux langues";
        assert_eq!(
            i18n.try_localize_message(&DemoMessages::Heading).as_deref(),
            Some(french_heading)
        );

        let unsupported = "de".parse::<LanguageIdentifier>().unwrap();
        assert!(matches!(
            i18n.select_language(unsupported.clone()),
            Err(gpui_es_fluent::LocalizationError::LanguageNotSupported(rejected))
                if rejected == unsupported
        ));
        assert_eq!(
            i18n.try_localize_message(&DemoMessages::Heading).as_deref(),
            Some(french_heading)
        );
    }
}
