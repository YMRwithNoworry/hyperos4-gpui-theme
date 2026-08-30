use std::time::Duration;

use gpui::InteractiveElement;
use gpui::{
    div, point, px, Animation, AnimationElement, AnimationExt, BoxShadow, Div, ElementId, Hsla,
    IntoElement, ParentElement, Pixels, Styled,
};
use gpui_component::theme::Theme;

use crate::ease_out_quint;

/// Semantic tokens for a soft-light glass surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassTokens {
    /// Translucent surface fill. The alpha is intentionally below 1 to retain
    /// the depth of the content behind the panel.
    pub fill: Hsla,
    /// Slightly brighter fill for hover/active affordance.
    pub fill_hover: Hsla,
    /// Hairline edge color.
    pub border: Hsla,
    /// One-pixel specular highlight along the top edge.
    pub highlight: Hsla,
    /// Ambient shadow below the panel.
    pub shadow: Hsla,
    /// Focused/raised shadow used by interactive panels.
    pub shadow_strong: Hsla,
    /// Corner radius in logical pixels.
    pub radius: Pixels,
}

impl GlassTokens {
    /// Derive glass tokens from the active semantic theme.
    pub fn from_theme(theme: &Theme) -> Self {
        let tint = if theme.is_dark() {
            theme.primary.opacity(0.16)
        } else {
            theme.primary.opacity(0.09)
        };
        let fill = theme
            .background
            .blend(tint)
            .alpha(if theme.is_dark() { 0.76 } else { 0.82 });
        Self {
            fill,
            fill_hover: theme
                .background
                .blend(theme.primary.opacity(0.2))
                .alpha(0.9),
            border: theme.border.blend(theme.primary.opacity(0.36)).alpha(0.72),
            highlight: theme
                .primary_foreground
                .alpha(if theme.is_dark() { 0.22 } else { 0.62 }),
            shadow: theme
                .foreground
                .alpha(if theme.is_dark() { 0.28 } else { 0.12 }),
            shadow_strong: theme
                .primary
                .alpha(if theme.is_dark() { 0.34 } else { 0.2 }),
            radius: theme.radius_lg,
        }
    }

    fn shadows(self, strong: bool) -> Vec<BoxShadow> {
        let (color, blur, y) = if strong {
            (self.shadow_strong, 24.0, 8.0)
        } else {
            (self.shadow, 18.0, 5.0)
        };
        vec![BoxShadow {
            color,
            offset: point(px(0.), px(y)),
            blur_radius: px(blur),
            spread_radius: px(0.),
        }]
    }
}

/// Build a soft-light glass surface around any GPUI element.
pub fn glass_surface(child: impl IntoElement, tokens: GlassTokens) -> Div {
    let highlight = div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .h(px(1.))
        .bg(tokens.highlight);

    div()
        .relative()
        .overflow_hidden()
        .rounded(tokens.radius)
        .bg(tokens.fill)
        .border_1()
        .border_color(tokens.border)
        .shadow(tokens.shadows(false))
        .child(highlight)
        .child(child)
}

/// Build a glass surface with a visible hover state and a stronger ambient
/// shadow. GPUI keeps the interaction state keyboard-safe and does not require
/// hover to discover the panel's content.
pub fn glass_interactive(child: impl IntoElement, tokens: GlassTokens) -> Div {
    glass_surface(child, tokens).hover(move |mut style| {
        style.background = Some(tokens.fill_hover.into());
        style.border_color = Some(tokens.highlight);
        style.box_shadow = Some(tokens.shadows(true));
        style
    })
}

/// Fade a glass surface into the window using a short ease-out transition.
///
/// The wrapper requests animation frames only while the transition is active;
/// it therefore has no ambient cost after settling.
pub fn glass_entrance(
    id: impl Into<ElementId>,
    child: impl IntoElement,
    tokens: GlassTokens,
) -> AnimationElement<Div> {
    glass_surface(child, tokens).with_animation(
        id,
        Animation::new(Duration::from_millis(360)).with_easing(ease_out_quint),
        |element, progress| element.opacity(0.78 + progress * 0.22),
    )
}
