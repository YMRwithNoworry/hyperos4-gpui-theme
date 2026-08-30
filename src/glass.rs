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
    /// Physical thickness of the glass slab in logical pixels.
    pub thickness: Pixels,
    /// Strength of the height-field normal.
    pub normal_strength: f32,
    /// Multiplier applied to lens and Snell displacement.
    pub displacement_scale: f32,
    /// SDF ramp width used to build the liquid dome.
    pub height_transition_width: Pixels,
    /// Depth-dependent blur multiplier.
    pub height_blur_factor: f32,
    /// RGB dispersion amount at the silhouette.
    pub chromatic_aberration: f32,
    /// Red dispersion multiplier.
    pub dispersion_r: f32,
    /// Blue dispersion multiplier.
    pub dispersion_b: f32,
    /// Transmission brightness multiplier.
    pub brightness: f32,
    /// Saturation boost applied to the sampled scene.
    pub vibrancy: f32,
    /// Dual Blinn-Phong specular intensity.
    pub specular_strength: f32,
    /// Blinn-Phong gloss exponent.
    pub shininess: f32,
    /// Border rim intensity.
    pub rim_strength: f32,
    /// Plain top-edge highlight width.
    pub highlight_width: Pixels,
    /// Focused-light caustic amount.
    pub caustic_intensity: f32,
    /// Liquid dome amount (0 = slab, 1 = rounded droplet).
    pub liquid_dome: f32,
    /// Fresnel reflection multiplier.
    pub fresnel_strength: f32,
    /// Final transmission multiplier.
    pub transmittance: f32,
    /// Inner shadow softness.
    pub shadow_softness: f32,
    /// Backdrop parallax multiplier.
    pub parallax_scale: f32,
    /// Pinch applied to backdrop samples while pressing.
    pub backdrop_pinch: f32,
    /// Curved-lens displacement in pixels.
    pub lens_refraction_px: f32,
    /// Radial lens-depth contribution.
    pub lens_depth_effect: f32,
    /// Key-light direction in surface coordinates.
    pub light_direction: [f32; 2],
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
            thickness: px(18.),
            normal_strength: 1.15,
            displacement_scale: 1.15,
            height_transition_width: px(18.),
            height_blur_factor: 1.0,
            chromatic_aberration: if theme.is_dark() { 1.8 } else { 1.35 },
            dispersion_r: 1.0,
            dispersion_b: 1.18,
            brightness: 1.08,
            vibrancy: 1.22,
            specular_strength: 1.52,
            shininess: 88.0,
            rim_strength: 1.22,
            highlight_width: px(4.),
            caustic_intensity: 0.28,
            liquid_dome: 0.72,
            fresnel_strength: 1.3,
            transmittance: 0.96,
            shadow_softness: 0.7,
            parallax_scale: 1.0,
            backdrop_pinch: 0.96,
            lens_refraction_px: 8.0,
            lens_depth_effect: 0.08,
            light_direction: [-0.5, -0.8],
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
    pub thickness: Pixels,
    pub normal_strength: f32,
    pub displacement_scale: f32,
    pub height_transition_width: Pixels,
    pub height_blur_factor: f32,
    pub chromatic_aberration: f32,
    pub dispersion_r: f32,
    pub dispersion_b: f32,
    pub brightness: f32,
    pub vibrancy: f32,
    pub specular_strength: f32,
    pub shininess: f32,
    pub rim_strength: f32,
    pub highlight_width: Pixels,
    pub caustic_intensity: f32,
    pub liquid_dome: f32,
    pub fresnel_strength: f32,
    pub transmittance: f32,
    pub shadow_softness: f32,
    pub parallax_scale: f32,
    pub backdrop_pinch: f32,
    pub lens_refraction_px: f32,
    pub lens_depth_effect: f32,
    pub press_progress: f32,
    pub glow_center: [f32; 2],
    pub glow_strength: f32,
    pub light_direction: [f32; 2],
}

impl From<GlassTokens> for GlassBackdrop {
    fn from(tokens: GlassTokens) -> Self {
        Self {
            blur_radius: tokens.blur_radius,
            distortion_strength: tokens.distortion_strength,
            reflection_strength: tokens.reflection_strength,
            refraction_index: tokens.refraction_index,
            noise_scale: tokens.noise_scale,
            thickness: tokens.thickness,
            normal_strength: tokens.normal_strength,
            displacement_scale: tokens.displacement_scale,
            height_transition_width: tokens.height_transition_width,
            height_blur_factor: tokens.height_blur_factor,
            chromatic_aberration: tokens.chromatic_aberration,
            dispersion_r: tokens.dispersion_r,
            dispersion_b: tokens.dispersion_b,
            brightness: tokens.brightness,
            vibrancy: tokens.vibrancy,
            specular_strength: tokens.specular_strength,
            shininess: tokens.shininess,
            rim_strength: tokens.rim_strength,
            highlight_width: tokens.highlight_width,
            caustic_intensity: tokens.caustic_intensity,
            liquid_dome: tokens.liquid_dome,
            fresnel_strength: tokens.fresnel_strength,
            transmittance: tokens.transmittance,
            shadow_softness: tokens.shadow_softness,
            parallax_scale: tokens.parallax_scale,
            backdrop_pinch: tokens.backdrop_pinch,
            lens_refraction_px: tokens.lens_refraction_px,
            lens_depth_effect: tokens.lens_depth_effect,
            press_progress: 0.0,
            glow_center: [0.5, 0.5],
            glow_strength: 0.0,
            light_direction: tokens.light_direction,
        }
    }
}

impl GlassBackdrop {
    /// Set the spring-driven press state used by the Prismal-style shader.
    pub fn with_press(mut self, progress: f32, center: [f32; 2]) -> Self {
        self.press_progress = progress.clamp(0.0, 1.0);
        self.glow_center = center;
        self.glow_strength = self.press_progress;
        self
    }

    /// Set the key-light direction used by specular, rim and caustic terms.
    pub fn with_light_direction(mut self, direction: [f32; 2]) -> Self {
        self.light_direction = direction;
        self
    }

    /// Enable or tune RGB chromatic dispersion.
    pub fn with_chromatic_aberration(mut self, amount: f32) -> Self {
        self.chromatic_aberration = amount.max(0.0);
        self
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
                thickness: backdrop.thickness,
                normal_strength: backdrop.normal_strength,
                displacement_scale: backdrop.displacement_scale,
                height_transition_width: backdrop.height_transition_width,
                height_blur_factor: backdrop.height_blur_factor,
                chromatic_aberration: backdrop.chromatic_aberration,
                dispersion_r: backdrop.dispersion_r,
                dispersion_b: backdrop.dispersion_b,
                brightness: backdrop.brightness,
                vibrancy: backdrop.vibrancy,
                specular_strength: backdrop.specular_strength,
                shininess: backdrop.shininess,
                rim_strength: backdrop.rim_strength,
                highlight_width: backdrop.highlight_width,
                caustic_intensity: backdrop.caustic_intensity,
                liquid_dome: backdrop.liquid_dome,
                fresnel_strength: backdrop.fresnel_strength,
                transmittance: backdrop.transmittance,
                shadow_softness: backdrop.shadow_softness,
                parallax_scale: backdrop.parallax_scale,
                backdrop_pinch: backdrop.backdrop_pinch,
                lens_refraction_px: backdrop.lens_refraction_px,
                lens_depth_effect: backdrop.lens_depth_effect,
                press_progress: backdrop.press_progress,
                glow_center: backdrop.glow_center,
                glow_strength: backdrop.glow_strength,
                light_direction: backdrop.light_direction,
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
    glass_interactive_with_backdrop(child, tokens, GlassBackdrop::from(tokens))
}

/// Interactive variant that accepts the full Prismal material configuration.
/// The active state raises the rim and shadow while the renderer keeps sampling
/// the same-scene backdrop underneath the panel.
pub fn glass_interactive_with_backdrop(
    child: impl IntoElement,
    tokens: GlassTokens,
    backdrop: GlassBackdrop,
) -> Div {
    glass_surface_with_backdrop(child, tokens, backdrop).hover(move |mut style| {
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

    #[test]
    fn backdrop_builder_exposes_prismal_interaction_controls() {
        let tokens = GlassTokens::from_theme(&Theme::default());
        let backdrop = GlassBackdrop::from(tokens)
            .with_press(1.4, [0.2, 0.8])
            .with_chromatic_aberration(2.0)
            .with_light_direction([-0.25, -0.9]);
        assert_eq!(backdrop.press_progress, 1.0);
        assert_eq!(backdrop.glow_center, [0.2, 0.8]);
        assert_eq!(backdrop.glow_strength, 1.0);
        assert_eq!(backdrop.chromatic_aberration, 2.0);
        assert_eq!(backdrop.light_direction, [-0.25, -0.9]);
        assert!(backdrop.thickness > px(0.));
        assert!(backdrop.liquid_dome > 0.0);
    }
}
