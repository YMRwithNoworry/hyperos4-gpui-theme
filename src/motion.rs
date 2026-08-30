use gpui::Hsla;

/// Cubic ease-in-out. It starts and ends gently while keeping the middle
/// responsive, which works well for glass surfaces appearing in a window.
pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = -2.0 * t + 2.0;
        1.0 - f * f * f / 2.0
    }
}

/// A restrained overshoot used for a panel's first appearance.
pub fn ease_out_back(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0) - 1.0;
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * t * t * t + c1 * t * t
}

/// Quintic ease-out, matching GPUI's built-in easing style while being exposed
/// as a named token for application-level transitions.
pub fn ease_out_quint(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(5)
}

/// Interpolate two HSLA colors in a predictable, clamped range.
pub fn interpolate_hsla(from: Hsla, to: Hsla, t: f32) -> Hsla {
    let t = t.clamp(0.0, 1.0);
    Hsla {
        h: from.h + (to.h - from.h) * t,
        s: from.s + (to.s - from.s) * t,
        l: from.l + (to.l - from.l) * t,
        a: from.a + (to.a - from.a) * t,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::hsla;

    #[test]
    fn easing_is_bounded_at_endpoints() {
        for easing in [ease_in_out_cubic, ease_out_quint] {
            assert!((easing(0.0) - 0.0).abs() < f32::EPSILON);
            assert!((easing(1.0) - 1.0).abs() < f32::EPSILON);
        }
        assert!((ease_out_back(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((ease_out_back(1.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn interpolation_clamps_progress() {
        let from = hsla(0.0, 0.2, 0.3, 0.4);
        let to = hsla(1.0, 0.8, 0.7, 0.9);
        assert_eq!(interpolate_hsla(from, to, -1.0), from);
        assert_eq!(interpolate_hsla(from, to, 2.0), to);
    }
}
