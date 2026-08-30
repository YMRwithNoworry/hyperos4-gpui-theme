//! A content-rich showcase for the HyperOS 4 soft-light glass theme.

use gpui::{
    div, point, prelude::*, px, size, App, Application, Context, FontWeight, Hsla, ParentElement,
    Styled, TitlebarOptions, Window, WindowBounds, WindowOptions,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    theme::ThemeMode,
    ActiveTheme, Root, Sizable, StyledExt,
};
use hyperos4_gpui_theme::{
    glass_entrance, glass_interactive, glass_surface, soft_glass_window_background, GlassTokens,
    HyperOs4Theme,
};

struct Preview;

fn orb(color: Hsla, size: f32, x: f32, y: f32) -> impl IntoElement {
    div()
        .absolute()
        .left(px(x))
        .top(px(y))
        .size(px(size))
        .rounded_full()
        .bg(color)
}

fn nav_item(label: &'static str, active: bool, tokens: GlassTokens) -> impl IntoElement {
    let mut item = div()
        .h(px(42.))
        .w_full()
        .h_flex()
        .items_center()
        .gap_3()
        .px_3()
        .rounded(tokens.radius)
        .text_sm()
        .font_weight(if active {
            FontWeight::SEMIBOLD
        } else {
            FontWeight::NORMAL
        })
        .text_color(if active { tokens.border } else { tokens.shadow });
    if active {
        item = item
            .bg(tokens.fill_hover)
            .border_1()
            .border_color(tokens.border);
    }
    item.child(div().size(px(8.)).rounded_full().bg(if active {
        tokens.highlight
    } else {
        tokens.shadow
    }))
    .child(label)
}

fn metric(
    label: &'static str,
    value: &'static str,
    accent: Hsla,
    tokens: GlassTokens,
) -> impl IntoElement {
    glass_interactive(
        div()
            .v_flex()
            .gap_2()
            .p_4()
            .flex_1()
            .child(div().text_sm().text_color(tokens.shadow).child(label))
            .child(
                div()
                    .h_flex()
                    .items_end()
                    .justify_between()
                    .child(div().text_2xl().font_weight(FontWeight::BOLD).child(value))
                    .child(div().size(px(10.)).rounded_full().bg(accent)),
            ),
        tokens,
    )
}

fn note_row(
    title: &'static str,
    time: &'static str,
    accent: Hsla,
    tokens: GlassTokens,
) -> impl IntoElement {
    div()
        .h_flex()
        .items_center()
        .gap_3()
        .py_2()
        .child(div().size(px(9.)).rounded_full().bg(accent))
        .child(
            div()
                .v_flex()
                .gap_1()
                .flex_1()
                .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(title))
                .child(div().text_xs().text_color(tokens.shadow).child(time)),
        )
        .child(div().text_sm().text_color(tokens.shadow).child("›"))
}

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let tokens = GlassTokens::from_theme(theme);

        let shell = div()
            .relative()
            .v_flex()
            .w(px(1120.))
            .h(px(700.))
            .rounded(px(28.))
            // Keep the window surface translucent so the compositor can show
            // its blurred backdrop underneath the workspace shell.
            .bg(theme.background.alpha(0.68))
            .border_1()
            .border_color(theme.border.alpha(0.72))
            .shadow(vec![gpui::BoxShadow {
                color: tokens.shadow_strong,
                offset: point(px(0.), px(20.)),
                blur_radius: px(54.),
                spread_radius: px(0.),
            }])
            .child(
                div()
                    .h_flex()
                    .h(px(76.))
                    .items_center()
                    .justify_between()
                    .px_6()
                    .border_b_1()
                    .border_color(theme.border.alpha(0.62))
                    .child(
                        div()
                            .h_flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .size(px(36.))
                                    .h_flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_full()
                                    .bg(theme.primary)
                                    .text_color(theme.primary_foreground)
                                    .font_weight(FontWeight::BOLD)
                                    .child("✦"),
                            )
                            .child(
                                div()
                                    .v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .child("Lumen"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(tokens.shadow)
                                            .child("soft glass workspace"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .h_flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .h_flex()
                                    .items_center()
                                    .gap_2()
                                    .px_3()
                                    .h(px(32.))
                                    .rounded_full()
                                    .bg(tokens.fill)
                                    .border_1()
                                    .border_color(tokens.border)
                                    .text_xs()
                                    .text_color(tokens.shadow)
                                    .child("⌘ K  Search"),
                            )
                            .child(
                                Button::new("new-note")
                                    .primary()
                                    .small()
                                    .label("New note")
                                    .on_click(|_, _, _| println!("new note")),
                            ),
                    ),
            )
            .child(
                div()
                    .h_flex()
                    .flex_1()
                    .p_5()
                    .gap_5()
                    .child(
                        glass_surface(
                            div()
                                .v_flex()
                                .gap_2()
                                .p_3()
                                .w(px(206.))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(tokens.shadow)
                                        .px_3()
                                        .child("WORKSPACE"),
                                )
                                .child(nav_item("Overview", true, tokens))
                                .child(nav_item("Notes", false, tokens))
                                .child(nav_item("Calendar", false, tokens))
                                .child(nav_item("Insights", false, tokens))
                                .child(div().flex_1())
                                .child(
                                    div()
                                        .v_flex()
                                        .gap_2()
                                        .p_3()
                                        .rounded(tokens.radius)
                                        .bg(theme.primary.opacity(0.1))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(tokens.shadow)
                                                .child("TODAY"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("3 focus blocks"),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(tokens.shadow)
                                                .child("Next up at 14:30"),
                                        ),
                                ),
                            tokens,
                        ),
                    )
                    .child(
                        div()
                            .v_flex()
                            .flex_1()
                            .gap_4()
                            .child(
                                div()
                                    .h_flex()
                                    .items_end()
                                    .justify_between()
                                    .child(
                                        div()
                                            .v_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_2xl()
                                                    .font_weight(FontWeight::BOLD)
                                                    .child("Good evening, Mia"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(tokens.shadow)
                                                    .child("Your calm space for the next small step."),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .h_flex()
                                            .items_center()
                                            .gap_2()
                                            .px_3()
                                            .h(px(28.))
                                            .rounded_full()
                                            .bg(theme.success.opacity(0.18))
                                            .text_xs()
                                            .text_color(theme.success_foreground)
                                            .child("●")
                                            .child("All synced"),
                                    ),
                            )
                            .child(glass_entrance(
                                "hero-card",
                                div()
                                    .v_flex()
                                    .gap_3()
                                    .p_5()
                                    .child(
                                        div()
                                            .h_flex()
                                            .items_start()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .v_flex()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(tokens.highlight)
                                                            .child("FOCUS MODE"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xl()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Make room for good work"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(tokens.shadow)
                                                            .child("A softer interface helps the important thing stay in focus."),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .size(px(54.))
                                                    .h_flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .rounded_full()
                                                    .bg(theme.primary.opacity(0.2))
                                                    .text_2xl()
                                                    .text_color(theme.primary)
                                                    .child("◌"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .h_flex()
                                            .items_center()
                                            .justify_between()
                                            .pt_2()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(tokens.shadow)
                                                    .child("Deep work · 48 min"),
                                            )
                                            .child(
                                                Button::new("start-focus")
                                                    .outline()
                                                    .small()
                                                    .label("Start focus"),
                                            ),
                                    ),
                                tokens,
                            ))
                            .child(
                                div()
                                    .h_flex()
                                    .gap_4()
                                    .child(metric("Notes captured", "128", theme.primary, tokens))
                                    .child(metric("Focus time", "6h 24m", theme.success, tokens))
                                    .child(metric("Calm streak", "12 days", theme.warning, tokens)),
                            )
                            .child(
                                glass_surface(
                                    div()
                                        .v_flex()
                                        .gap_3()
                                        .p_5()
                                        .child(
                                            div()
                                                .h_flex()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .text_lg()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child("Recent notes"),
                                                )
                                                .child(
                                                    Button::new("view-all")
                                                        .ghost()
                                                        .small()
                                                        .label("View all"),
                                                ),
                                        )
                                        .child(note_row(
                                            "Design principles",
                                            "Updated 8 min ago",
                                            theme.primary,
                                            tokens,
                                        ))
                                        .child(note_row(
                                            "A quieter morning",
                                            "Yesterday",
                                            theme.success,
                                            tokens,
                                        ))
                                        .child(note_row(
                                            "Ideas for the weekend",
                                            "Mon, 09:42",
                                            theme.warning,
                                            tokens,
                                        )),
                                    tokens,
                                ),
                            ),
                    ),
            );

        div()
            .relative()
            .size_full()
            .h_flex()
            .items_center()
            .justify_center()
            // This alpha is intentional: WindowBackgroundAppearance::Blurred
            // supplies the native acrylic/visual-effect backdrop, while the
            // soft color orbs remain visible through the shell and cards.
            .bg(theme.background.alpha(0.22))
            .child(orb(theme.primary.opacity(0.13), 420., -120., -100.))
            .child(orb(theme.cyan.opacity(0.11), 360., 860., 430.))
            .child(orb(theme.magenta.opacity(0.09), 250., 760., -80.))
            .child(shell)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        HyperOs4Theme::install(cx, ThemeMode::Light);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1120.), px(700.)), cx)),
            window_background: soft_glass_window_background(),
            titlebar: Some(TitlebarOptions {
                title: Some("HyperOS 4 · Soft Glass Preview".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        if let Err(error) = cx.open_window(options, |window, cx| {
            let view = cx.new(|_| Preview);
            cx.new(|cx| Root::new(view, window, cx))
        }) {
            eprintln!("failed to open preview window: {error:#}");
        }
    });
}
