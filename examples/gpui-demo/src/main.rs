#[cfg(target_family = "wasm")]
use std::borrow::Cow;

use gpui::prelude::*;
use gpui::{App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, px, size};
use gpui_component::button::Button;
use gpui_component::theme::{Theme, ThemeMode};
use gpui_es_fluent_demo::DemoMessages;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

const DEMO_MARKER: &str = "gpui-es-fluent-demo";
#[cfg(target_family = "wasm")]
const UI_FONT_FAMILY: &str = "Noto Sans SC";
#[cfg(target_family = "wasm")]
const MONO_FONT_FAMILY: &str = "JetBrains Mono";

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_with_app(
        gpui_platform::application().with_assets(gpui_component_assets::Assets::new(
            "https://longbridge.github.io/gpui-component/gallery",
        )),
    );
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    gpui_platform::web_init();
    let app = gpui_platform::single_threaded_web().with_assets(gpui_component_assets::Assets::new(
        "https://longbridge.github.io/gpui-component/gallery",
    ));

    struct WasmApplication(std::rc::Rc<gpui::AppCell>);

    // Keep GPUI's application cell alive while browser callbacks remain queued.
    let app = unsafe {
        let wasm_app = std::mem::transmute::<Application, WasmApplication>(app);
        std::mem::forget(wasm_app.0.clone());
        std::mem::transmute::<WasmApplication, Application>(wasm_app)
    };

    run_with_app(app);
    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() {
    let _ = run();
}

fn run_with_app(app: Application) {
    app.run(|cx: &mut App| {
        gpui_component::init(cx);
        install_web_fonts(cx);
        apply_oled_theme(cx);
        gpui_es_fluent_demo::i18n::link();
        gpui_es_fluent::set_component_locale(cx, "en")
            .expect("the English demo locale should initialize");

        let bounds = Bounds::centered(None, size(px(720.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|_| LocaleDemo::default()),
        )
        .expect("the GPUI demo window should open");
        cx.activate(true);
    });
}

fn install_web_fonts(cx: &mut App) {
    #[cfg(target_family = "wasm")]
    {
        let fonts = vec![
            Cow::Borrowed(
                include_bytes!(concat!(
                    env!("GPUI_COMPONENT_STORY_FONTS_DIR"),
                    "/NotoSansSC-Regular-subset.ttf"
                ))
                .as_slice(),
            ),
            Cow::Borrowed(
                include_bytes!(concat!(
                    env!("GPUI_COMPONENT_STORY_FONTS_DIR"),
                    "/NotoEmoji-Regular.ttf"
                ))
                .as_slice(),
            ),
            Cow::Borrowed(
                include_bytes!(concat!(
                    env!("GPUI_COMPONENT_STORY_FONTS_DIR"),
                    "/JetBrainsMono-Regular.ttf"
                ))
                .as_slice(),
            ),
        ];
        cx.text_system()
            .add_fonts(fonts)
            .expect("gpui-component web fonts should load");
        apply_font_families(cx);
    }

    #[cfg(not(target_family = "wasm"))]
    let _ = cx;
}

#[cfg(target_family = "wasm")]
fn apply_font_families(cx: &mut App) {
    let theme = Theme::global_mut(cx);
    theme.font_family = UI_FONT_FAMILY.into();
    theme.mono_font_family = MONO_FONT_FAMILY.into();
}

fn apply_oled_theme(cx: &mut App) {
    let black = gpui::rgb(0x000000).into();
    let white = gpui::rgb(0xffffff).into();
    let button_hover = gpui::rgb(0xe5e5e5).into();
    let button_active = gpui::rgb(0xcccccc).into();
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
            .bg(gpui::rgb(0x000000))
            .text_color(gpui::rgb(0xffffff))
            .child(div().text_xs().child(DEMO_MARKER))
            .child(
                div()
                    .text_3xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(heading),
            )
            .child(div().max_w(px(560.)).text_center().child(body))
            .child(
                Button::new("change-locale")
                    .label(button)
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.toggle_locale(cx);
                    })),
            )
    }
}
