# HyperOS 4 Soft Glass for GPUI

一个面向 GPUI Component 的 HyperOS 4 柔光玻璃主题。它把柔和的冷色调、半透明表面、顶部高光和环境阴影封装成语义 token，并提供可选的短时缓动动画。

![HyperOS 4 soft glass preview](assets/preview.png)

播放器卡片与工作区处于同一个 GPUI 场景，透明度和反射层会直接透出其下方的光晕：

![Floating media glass](assets/media-glass.png)

## 特性

- 浅色和深色 `ThemeConfig`，可直接用于 `Theme::apply_config`
- `GlassTokens::from_theme`：从当前语义主题派生玻璃表面颜色，避免在组件里散落色值
- `glass_surface` / `glass_interactive`：半透明填充、hairline 边框、顶部 specular 高光和双层阴影，并插入同场景 backdrop primitive
- `glass_surface_with_backdrop` / `GlassBackdrop`：可调 blur 半径、折射位移、折射率、微表面频率和 Fresnel 反射
- `soft_glass_window_background()`：可选的整窗系统 backdrop blur；组件玻璃不会依赖它
- `glass_entrance`：360ms ease-out 入场动画，只在动画期间请求帧
- `ease_in_out_cubic`、`ease_out_back`、`ease_out_quint` 和 `interpolate_hsla` 可用于应用层状态动画
- `examples/preview.rs` 提供可运行的主题预览

组件级玻璃不通过独立原生窗口，也不模糊 Windows 后方窗口。仓库在 `vendor/gpui` 提供了最小 renderer 扩展：每个 primitive 按 draw order 将此前已绘制的 GPUI scene 复制到 offscreen texture，pixel shader 使用多 tap Gaussian blur，在折射率驱动的 Snell 传输向量上做 UV displacement，再以 Schlick Fresnel 混合反射采样和 tint。这样采样源就是同一窗口、同一 scene 中玻璃下方的 UI。Blade、Metal 等暂未提供可移植的 scene-copy API，会保留相同 primitive 和半透明 tint fallback。

## 使用

```toml
hyperos4-gpui-theme = { git = "https://github.com/YMRwithNoworry/hyperos4-gpui-theme" }
```

```rust
use gpui::{div, App, Window, WindowOptions};
use gpui_component::theme::ThemeMode;
use hyperos4_gpui_theme::{
    glass_interactive, glass_surface_with_backdrop, soft_glass_window_background,
    GlassBackdrop, GlassTokens, HyperOs4Theme,
};

fn setup(cx: &mut App) {
    gpui_component::init(cx);
    HyperOs4Theme::install(cx, ThemeMode::Light);
}

fn panel(window: &Window, cx: &App) -> impl gpui::IntoElement {
    let glass = GlassTokens::from_theme(cx.theme());
    glass_interactive(div().child("柔光玻璃"), glass)
}

fn custom_panel(cx: &App) -> impl gpui::IntoElement {
    let glass = GlassTokens::from_theme(cx.theme());
    let material = GlassBackdrop {
        blur_radius: gpui::px(24.),
        distortion_strength: 4.0,
        reflection_strength: 0.5,
        refraction_index: 1.46,
        noise_scale: 0.018,
    };
    glass_surface_with_backdrop(div().child("可调折射玻璃"), glass, material)
}

fn window_options() -> WindowOptions {
    WindowOptions {
        window_background: soft_glass_window_background(),
        ..Default::default()
    }
}
```

运行预览：

```text
cargo run --example preview
```

## 设计说明

玻璃效果是组件级 opt-in，不会给所有控件默认添加动效。入场动画使用 ease-out quint，hover 仅改变表面 tint 和边缘，不依赖 hover 才能发现内容；这与 GPUI Component 的桌面交互和 reduced-motion 友好原则一致。
