use es_fluent::EsFluent;

pub mod i18n;

#[derive(Clone, Copy, Debug, EsFluent)]
pub enum DemoMessages {
    Heading,
    Body,
    ChangeLocale,
}
