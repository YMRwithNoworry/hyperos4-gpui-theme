//! A content-rich showcase for the HyperOS 4 soft-light glass theme.

use gpui::{
    div, linear_color_stop, linear_gradient, point, prelude::*, px, rgba, size, App, Application,
    Context, FontWeight, Hsla, ParentElement, Styled, TitlebarOptions, Window, WindowBounds,
    WindowOptions,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    theme::ThemeMode,
    ActiveTheme, Root, Sizable, StyledExt,
};
use hyperos4_gpui_theme::{
    glass_entrance, glass_interactive, glass_surface, glass_surface_with_backdrop, GlassBackdrop,
    GlassTokens, HyperOs4Theme,
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

fn ambient_pattern() -> impl IntoElement {
    let mut pattern = div().absolute().inset_0().opacity(0.82);
    let colors = [rgba(0x5fd6ffb0), rgba(0xb58cff9c), rgba(0xff9acb98)];
    for i in 0..18 {
        let x = 32. + (i as f32 * 71.) % 1040.;
        let y = 92. + ((i * 53) as f32 % 520.);
        pattern = pattern.child(
            div()
                .absolute()
                .left(px(x))
                .top(px(y))
                .size(px(if i % 3 == 0 { 18. } else { 9. }))
                .rounded_full()
                .bg(colors[i % colors.len()]),
        );
    }
    for i in 0..9 {
        pattern = pattern.child(
            div()
                .absolute()
                .left(px(12. + i as f32 * 132.))
                .top(px(168. + (i % 3) as f32 * 164.))
                .w(px(112.))
                .h(px(3.))
                .rounded_full()
                .bg(rgba(0xe6f5ff82)),
        );
    }
    for i in 0..5 {
        pattern = pattern.child(
            div()
                .absolute()
                .left(px(180. + i as f32 * 190.))
                .top(px(250. + (i % 2) as f32 * 86.))
                .w(px(150.))
                .h(px(22.))
                .rounded(px(11.))
                .bg(if i % 2 == 0 {
                    rgba(0x67d9ff88)
                } else {
                    rgba(0xd69cff78)
                }),
        );
    }
    // High-frequency content makes the same-scene blur and Snell displacement
    // obvious through the glass cards (the shader never samples another
    // native window).
    for i in 0..26 {
        let x = 18. + i as f32 * 44.;
        pattern = pattern.child(
            div()
                .absolute()
                .left(px(x))
                .top(px(76.))
                .w(px(1.))
                .h(px(610.))
                .bg(rgba(if i % 2 == 0 { 0xffffff22 } else { 0x244a9c28 })),
        );
    }
    for i in 0..14 {
        let y = 104. + i as f32 * 42.;
        pattern = pattern.child(
            div()
                .absolute()
                .left(px(12.))
                .top(px(y))
                .w(px(1090.))
                .h(px(1.))
                .bg(rgba(if i % 2 == 0 { 0xffffff2a } else { 0x244a9c24 })),
        );
    }
    // Deliberately sharp backdrop objects. The glass panels above this layer
    // should bend their edges and soften their silhouettes, just like the
    // red/white forms visible through a liquid-glass card.
    pattern = pattern
        .child(
            div()
                .absolute()
                .left(px(54.))
                .top(px(142.))
                .size(px(30.))
                .rounded_full()
                .bg(rgba(0xff3048e8)),
        )
        .child(
            div()
                .absolute()
                .left(px(30.))
                .top(px(220.))
                .w(px(126.))
                .h(px(118.))
                .rounded(px(26.))
                .bg(rgba(0xf5fbff9a)),
        )
        .child(
            div()
                .absolute()
                .left(px(256.))
                .top(px(88.))
                .size(px(238.))
                .rounded_full()
                .bg(rgba(0x27d5c45c)),
        )
        .child(
            div()
                .absolute()
                .right(px(72.))
                .bottom(px(84.))
                .size(px(190.))
                .rounded_full()
                .bg(rgba(0xa982ff55)),
        );
    pattern
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
        .text_color(if active {
            tokens.foreground
        } else {
            tokens.foreground_muted
        });
    if active {
        item = item
            .bg(tokens.fill_hover)
            .border_1()
            .border_color(tokens.border);
    }
    item.child(div().size(px(8.)).rounded_full().bg(if active {
        tokens.highlight
    } else {
        tokens.foreground_muted
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
            .child(
                div()
                    .text_sm()
                    .text_color(tokens.foreground_muted)
                    .child(label),
            )
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

/// A compact player tile used to make the material response visible in the
/// preview. It stays in the same GPUI scene as the workspace, so cards above
/// it can refract and blur these gradients through the DirectX backdrop pass.
fn media_player(tokens: GlassTokens) -> impl IntoElement {
    // Keep this tile transparent: its backdrop is the content painted earlier
    // in the same GPUI scene, rather than an opaque decorative texture.
    let material = GlassBackdrop::from(tokens)
        .with_chromatic_aberration(2.2)
        .with_light_direction([-0.6, -0.75]);
    glass_surface_with_backdrop(
        div()
            .relative()
            .overflow_hidden()
            .w(px(148.))
            .h(px(148.))
            .rounded(px(24.))
            .child(
                div()
                    .absolute()
                    .top(px(14.))
                    .right(px(14.))
                    .text_lg()
                    .text_color(tokens.foreground)
                    .child(">))"),
            )
            .child(
                div()
                    .size_full()
                    .v_flex()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .text_color(tokens.foreground)
                    .child(
                        div()
                            .text_xs()
                            .text_color(tokens.foreground_muted)
                            .child("NOW PLAYING"),
                    )
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("暂无播放"),
                    )
                    .child(
                        div()
                            .h_flex()
                            .items_center()
                            .gap_4()
                            .text_sm()
                            .text_color(tokens.foreground_muted)
                            .child("◀")
                            .child(
                                div()
                                    .size(px(34.))
                                    .h_flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_full()
                                    .bg(tokens.foreground.opacity(0.86))
                                    .text_color(tokens.fill)
                                    .text_lg()
                                    .child("▶"),
                            )
                            .child("▶"),
                    ),
            ),
        tokens,
        material,
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
                .child(
                    div()
                        .text_xs()
                        .text_color(tokens.foreground_muted)
                        .child(time),
                ),
        )
        .child(
            div()
                .text_sm()
                .text_color(tokens.foreground_muted)
                .child("›"),
        )
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
            .bg(theme.background.alpha(0.42))
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
                                            .text_color(tokens.foreground_muted)
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
                                    .text_color(tokens.foreground_muted)
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
                                        .text_color(tokens.foreground_muted)
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
                                                .text_color(tokens.foreground_muted)
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
                                                .text_color(tokens.foreground_muted)
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
                                                    .text_color(tokens.foreground_muted)
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
                                    .relative()
                                    .v_flex()
                                    .gap_3()
                                    .p_5()
                                    // This is ordinary UI content behind the
                                    // player, intentionally painted before the
                                    // transparent glass tile so refraction has
                                    // something sharp to bend and blur.
                                    .child(
                                        div()
                                            .absolute()
                                            .top(px(4.))
                                            .right(px(4.))
                                            .w(px(190.))
                                            .h(px(176.))
                                            .rounded(px(34.))
                                            .opacity(0.72)
                                            .bg(linear_gradient(
                                                132.,
                                                linear_color_stop(theme.cyan.opacity(0.32), 0.),
                                                linear_color_stop(theme.primary.opacity(0.34), 1.),
                                            )),
                                    )
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
                                                            .text_color(tokens.foreground_muted)
                                                            .child("A softer interface helps the important thing stay in focus."),
                                                    ),
                                            )
                                            .child(media_player(tokens)),
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
                                                    .text_color(tokens.foreground_muted)
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
            // Keep the scene translucent so the soft color orbs remain visible
            // through the shell and cards below their tint layers.
            .bg(theme.background.alpha(0.22))
            .child(ambient_pattern())
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
