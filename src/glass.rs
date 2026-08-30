use std::time::Duration;

use gpui::InteractiveElement;
use gpui::{
    canvas, div, linear_color_stop, linear_gradient, point, px, transparent_black, Animation,
    AnimationElement, AnimationExt, BoxShadow, Corners, Div, ElementId, Hsla, IntoElement,
    PaintBackdropBlur, ParentElement, Pixels, Styled, WindowBackgroundAppearance,
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
    /// Backdrop Gaussian blur radius in logical pixels.
    pub blur_radius: Pixels,
    /// Maximum refractive displacement in logical pixels.
    pub distortion_strength: f32,
    /// Fresnel reflection intensity.
    pub reflection_strength: f32,
    /// Index of refraction for the glass medium.
    pub refraction_index: f32,
    /// Micro-surface normal frequency in pixels.
    pub noise_scale: f32,
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
            blur_radius: px(if theme.is_dark() { 22. } else { 18. }),
            distortion_strength: if theme.is_dark() { 3.4 } else { 2.6 },
            reflection_strength: if theme.is_dark() { 0.52 } else { 0.42 },
            refraction_index: 1.46,
            noise_scale: 0.018,
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

/// Select the native window backdrop used by HyperOS 4 glass previews.
///
/// GPUI maps [`WindowBackgroundAppearance::Blurred`] to the platform's
/// compositor (acrylic on Windows, a visual-effect view on macOS, and the
/// available Wayland blur protocol). Platforms that do not expose a backdrop
/// blur keep the translucent tint and highlight treatment as a graceful
/// fallback.
pub const fn soft_glass_window_background() -> WindowBackgroundAppearance {
    WindowBackgroundAppearance::Blurred
}

/// Build a soft-light glass surface around any GPUI element.
pub fn glass_surface(child: impl IntoElement, tokens: GlassTokens) -> Div {
    glass_surface_with_backdrop(child, tokens, GlassBackdrop::from(tokens))
}

/// Parameters for a refractive HyperOS 4 glass layer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassBackdrop {
    pub blur_radius: Pixels,
    pub distortion_strength: f32,
    pub reflection_strength: f32,
    pub refraction_index: f32,
    pub noise_scale: f32,
}

impl From<GlassTokens> for GlassBackdrop {
    fn from(tokens: GlassTokens) -> Self {
        Self {
            blur_radius: tokens.blur_radius,
            distortion_strength: tokens.distortion_strength,
            reflection_strength: tokens.reflection_strength,
            refraction_index: tokens.refraction_index,
            noise_scale: tokens.noise_scale,
        }
    }
}

/// Build a glass surface with an in-scene backdrop sample and refractive
/// compositing. The canvas is an absolute child, so its primitive is inserted
/// exactly where the surface appears in the GPUI scene and samples only pixels
/// painted below it in that same scene.
pub fn glass_surface_with_backdrop(
    child: impl IntoElement,
    tokens: GlassTokens,
    backdrop: GlassBackdrop,
) -> Div {
    let reflection = linear_gradient(
        145.,
        linear_color_stop(tokens.highlight.opacity(0.46), 0.),
        linear_color_stop(tokens.highlight.opacity(0.0), 0.42),
    );
    let highlight = div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .h(px(1.))
        .bg(tokens.highlight);

    let backdrop_layer = canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            window.paint_backdrop_blur(PaintBackdropBlur {
                bounds,
                corner_radii: Corners::all(tokens.radius),
                blur_radius: backdrop.blur_radius,
                distortion_strength: backdrop.distortion_strength,
                reflection_strength: backdrop.reflection_strength,
                refraction_index: backdrop.refraction_index,
                noise_scale: backdrop.noise_scale,
                tint: tokens.fill.alpha(0.28),
            });
        },
    )
    .absolute()
    .inset_0();

    div()
        .relative()
        .overflow_hidden()
        .rounded(tokens.radius)
        // The backdrop primitive is painted before this translucent fallback
        // tint. On renderers without scene sampling the tint still preserves
        // the soft glass appearance.
        .bg(transparent_black())
        .border_1()
        .border_color(tokens.border)
        .shadow(tokens.shadows(false))
        .child(backdrop_layer)
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded(tokens.radius)
                .bg(tokens.fill.alpha(0.42)),
        )
        // A low-alpha directional reflection gives every surface a gentle
        // Fresnel-like response while the native window backdrop supplies the
        // actual blur behind transparent pixels.
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded(tokens.radius)
                .bg(reflection),
        )
        .child(highlight)
        .child(child)
}

/// Build a glass surface with a visible hover state and a stronger ambient
/// shadow. GPUI keeps the interaction state keyboard-safe and does not require
/// hover to discover the panel's content.
pub fn glass_interactive(child: impl IntoElement, tokens: GlassTokens) -> Div {
    glass_surface(child, tokens).hover(move |mut style| {
        // Keep the parent quad transparent so the backdrop primitive still
        // samples the content beneath the surface on hover.
        style.background = Some(transparent_black().into());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_surface_keeps_translucency_and_contrast() {
        let tokens = GlassTokens::from_theme(&Theme::default());
        assert!(tokens.fill.a < 1.0);
        assert!(tokens.fill_hover.a <= 1.0);
        assert!(tokens.border.a > 0.0);
        assert!(tokens.radius > px(0.));
        assert_eq!(tokens.shadows(false).len(), 1);
        assert_eq!(tokens.shadows(true).len(), 1);
    }

    #[test]
    fn window_background_uses_native_blur() {
        assert_eq!(
            soft_glass_window_background(),
            WindowBackgroundAppearance::Blurred
        );
    }
}
