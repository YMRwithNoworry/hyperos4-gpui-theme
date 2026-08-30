use std::rc::Rc;

use gpui::App;
use gpui_component::theme::{Theme, ThemeConfig, ThemeMode};

const LIGHT_CONFIG: &str = include_str!("../assets/hyperos4-light.json");
const DARK_CONFIG: &str = include_str!("../assets/hyperos4-dark.json");

/// HyperOS 4 soft-light theme entry points.
pub struct HyperOs4Theme;

impl HyperOs4Theme {
    /// Parse the light theme configuration shipped with this crate.
    pub fn light() -> ThemeConfig {
        serde_json::from_str(LIGHT_CONFIG).expect("bundled HyperOS4 light theme is valid")
    }

    /// Parse the dark theme configuration shipped with this crate.
    pub fn dark() -> ThemeConfig {
        serde_json::from_str(DARK_CONFIG).expect("bundled HyperOS4 dark theme is valid")
    }

    /// Return the configuration matching a theme mode.
    pub fn config(mode: ThemeMode) -> ThemeConfig {
        if mode.is_dark() {
            Self::dark()
        } else {
            Self::light()
        }
    }

    /// Apply HyperOS4 colors to an initialized GPUI Component application.
    ///
    /// `gpui_component::init(cx)` is called automatically when the global theme
    /// is not present. Calling it explicitly at the application entry point is
    /// still recommended because it initializes the complete component catalog.
    pub fn install(cx: &mut App, mode: ThemeMode) {
        if !cx.has_global::<Theme>() {
            gpui_component::init(cx);
        }
        let config = Rc::new(Self::config(mode));
        let theme = Theme::global_mut(cx);
        // `Theme::apply_config` updates the palette snapshot but intentionally
        // leaves the active mode to its caller (the registry uses this to keep
        // light and dark configurations side by side).
        theme.mode = mode;
        theme.apply_config(&config);
    }

    /// The raw JSON source for consumers that want to register the theme in a
    /// `ThemeRegistry` or persist it alongside other application themes.
    pub const fn light_source() -> &'static str {
        LIGHT_CONFIG
    }

    /// The raw JSON source for consumers that want to register the theme in a
    /// `ThemeRegistry` or persist it alongside other application themes.
    pub const fn dark_source() -> &'static str {
        DARK_CONFIG
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_configs_parse_with_expected_modes() {
        let light = HyperOs4Theme::light();
        let dark = HyperOs4Theme::dark();
        assert_eq!(light.name.as_ref(), "HyperOS 4 Soft Glass Light");
        assert_eq!(dark.name.as_ref(), "HyperOS 4 Soft Glass Dark");
        assert!(!light.mode.is_dark());
        assert!(dark.mode.is_dark());
        assert!(light.colors.background.is_some());
        assert!(dark.colors.background.is_some());
    }
}
