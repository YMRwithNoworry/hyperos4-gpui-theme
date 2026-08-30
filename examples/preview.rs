use gpui::{div, prelude::*, App, Application, Context, Window, WindowOptions};
use gpui_component::{button::Button, theme::ThemeMode, ActiveTheme, Root, StyledExt};
use hyperos4_gpui_theme::{glass_entrance, glass_interactive, GlassTokens, HyperOs4Theme};

struct Preview;

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tokens = GlassTokens::from_theme(cx.theme());
        div()
            .size_full()
            .v_flex()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(cx.theme().background)
            .child(glass_entrance(
                "hyperos4-preview-card",
                div()
                    .v_flex()
                    .gap_2()
                    .p_6()
                    .child("HyperOS 4")
                    .child("Soft light glass for GPUI"),
                tokens,
            ))
            .child(glass_interactive(
                Button::new("toggle-theme")
                    .label("Interactive glass")
                    .on_click(|_, _, _| println!("glass action")),
                tokens,
            ))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        HyperOs4Theme::install(cx, ThemeMode::Light);
        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Preview);
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
