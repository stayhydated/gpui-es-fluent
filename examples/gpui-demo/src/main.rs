#[cfg(target_family = "wasm")]
use std::{borrow::Cow, cell::RefCell};

use gpui_es_fluent_demo::DemoMessages;
use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::component::{ActiveTheme as _, Root, button::Button};
use gpui_kit::prelude::*;
use gpui_kit::{App, Context, Window, WindowOptions, div};
#[cfg(not(target_family = "wasm"))]
use gpui_kit::{Bounds, WindowBounds, px, size};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

const DEMO_MARKER: &str = "gpui-es-fluent-demo";
#[cfg(not(target_family = "wasm"))]
fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets::new(""))
        .run(launch);
}

#[cfg(target_family = "wasm")]
thread_local! {
    static APPLICATION: RefCell<Option<gpui_kit::ApplicationHandle>> = const { RefCell::new(None) };
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    gpui_kit::platform::web_init();
    let app =
        gpui_kit::platform::single_threaded_web().with_assets(gpui_kit::assets::Assets::new(""));
    APPLICATION.with(|application| {
        *application.borrow_mut() = Some(app.run_embedded(launch));
    });
    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() {}

fn launch(cx: &mut App) {
    gpui_kit::init(cx);

    #[cfg(target_family = "wasm")]
    cx.text_system()
        .add_fonts(vec![Cow::Borrowed(ttf_inter::REGULAR)])
        .expect("the GPUI demo font should load");

    apply_oled_theme(cx);
    gpui_es_fluent_demo::i18n::link();
    gpui_es_fluent::set_component_locale(cx, "en")
        .expect("the English demo locale should initialize");

    #[cfg(not(target_family = "wasm"))]
    let options = {
        let bounds = Bounds::centered(None, size(px(720.), px(500.)), cx);
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        }
    };
    #[cfg(target_family = "wasm")]
    let options = WindowOptions::default();

    cx.open_window(options, |window, cx| {
        let view = cx.new(|_| LocaleDemo::default());
        cx.new(|cx| Root::new(view, window, cx).bg(cx.theme().background))
    })
    .expect("the GPUI demo window should open");
    cx.activate(true);
}

fn apply_oled_theme(cx: &mut App) {
    let black = gpui_kit::rgb(0x000000).into();
    let white = gpui_kit::rgb(0xffffff).into();
    let button_hover = gpui_kit::rgb(0xe5e5e5).into();
    let button_active = gpui_kit::rgb(0xcccccc).into();
    {
        let theme = Theme::global_mut(cx);

        theme.mode = ThemeMode::Dark;
        theme.background = black;
        theme.foreground = white;
        theme.muted = black;
        theme.muted_foreground = white;
        theme.border = white;
        theme.input = black;
        theme.button = white;
        theme.button_foreground = black;
        theme.button_hover = button_hover;
        theme.button_active = button_active;
        theme.tokens.button = white.into();
        theme.tokens.button_foreground = black.into();
        theme.tokens.button_hover = button_hover.into();
        theme.tokens.button_active = button_active.into();

        #[cfg(target_family = "wasm")]
        {
            theme.font_family = "Inter".into();
        }
    }
    Theme::sync_base(cx);
}

#[derive(Default)]
struct LocaleDemo {
    french: bool,
}

impl LocaleDemo {
    fn toggle_locale(&mut self, cx: &mut Context<Self>) {
        self.french = !self.french;
        let locale = if self.french { "fr-FR" } else { "en" };
        gpui_es_fluent::set_component_locale(cx, locale)
            .expect("the selected demo locale should initialize");
        cx.notify();
    }
}

impl Render for LocaleDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let heading = gpui_es_fluent::localize_message(cx, &DemoMessages::Heading);
        let body = gpui_es_fluent::localize_message(cx, &DemoMessages::Body);
        let button = gpui_es_fluent::localize_message(cx, &DemoMessages::ChangeLocale);

        div()
            .id("gpui-es-fluent-demo")
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_5()
            .p_6()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(div().text_xs().child(DEMO_MARKER))
            .child(
                div()
                    .text_3xl()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .child(heading),
            )
            .child(div().max_w(gpui_kit::rems(35.)).text_center().child(body))
            .child(
                Button::new("change-locale")
                    .label(button)
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.toggle_locale(cx);
                    })),
            )
    }
}
