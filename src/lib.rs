//! HyperOS 4 inspired soft-light glass theme for GPUI.
//!
//! The theme is intentionally built on top of GPUI Component's semantic theme
//! tokens. Surfaces use translucent fills, a cool specular edge, restrained
//! shadows, and the native blurred window backdrop where the platform supports
//! it. Per-surface blur is not exposed by GPUI yet, so translucent tinting is
//! retained as a portable fallback.
//!
//! ```no_run
//! use gpui::{App, Application, WindowBackgroundAppearance, WindowOptions};
//! use gpui_component::theme::ThemeMode;
//! use hyperos4_gpui_theme::HyperOs4Theme;
//!
//! let app = Application::new();
//! app.run(|cx: &mut App| {
//!     gpui_component::init(cx);
//!     HyperOs4Theme::install(cx, ThemeMode::Light);
//!     let _options = WindowOptions {
//!         window_background: WindowBackgroundAppearance::Blurred,
//!         ..Default::default()
//!     };
//! });
//! ```

mod glass;
mod motion;
mod theme;

pub use glass::{
    glass_entrance, glass_interactive, glass_surface, soft_glass_window_background, GlassTokens,
};
pub use motion::{ease_in_out_cubic, ease_out_back, ease_out_quint, interpolate_hsla};
pub use theme::HyperOs4Theme;
