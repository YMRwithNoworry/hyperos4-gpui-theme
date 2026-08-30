# HyperOS 4 Soft Glass for GPUI

一个面向 GPUI Component 的 HyperOS 4 柔光玻璃主题。它把柔和的冷色调、半透明表面、顶部高光和环境阴影封装成语义 token，并提供可选的短时缓动动画。

## 特性

- 浅色和深色 `ThemeConfig`，可直接用于 `Theme::apply_config`
- `GlassTokens::from_theme`：从当前语义主题派生玻璃表面颜色，避免在组件里散落色值
- `glass_surface` / `glass_interactive`：半透明填充、hairline 边框、顶部 specular 高光和双层阴影
- `glass_entrance`：360ms ease-out 入场动画，只在动画期间请求帧
- `ease_in_out_cubic`、`ease_out_back`、`ease_out_quint` 和 `interpolate_hsla` 可用于应用层状态动画
- `examples/preview.rs` 提供可运行的主题预览

GPUI 0.2.2 当前没有跨平台 backdrop blur 绘制 API，因此“柔光玻璃”使用半透明 tint + 高光 + 阴影来保留背景层次；未来接入渲染器模糊能力时，`GlassTokens` 可作为稳定的语义入口。

## 使用

```toml
hyperos4-gpui-theme = { git = "https://github.com/your-name/hyperos4-gpui-theme" }
```

```rust
use gpui::{div, App, Window};
use gpui_component::theme::ThemeMode;
use hyperos4_gpui_theme::{glass_interactive, GlassTokens, HyperOs4Theme};

fn setup(cx: &mut App) {
    gpui_component::init(cx);
    HyperOs4Theme::install(cx, ThemeMode::Light);
}

fn panel(window: &Window, cx: &App) -> impl gpui::IntoElement {
    let glass = GlassTokens::from_theme(cx.theme());
    glass_interactive(div().child("柔光玻璃"), glass)
}
```

运行预览：

```text
cargo run --example preview
```

## 设计说明

玻璃效果是组件级 opt-in，不会给所有控件默认添加动效。入场动画使用 ease-out quint，hover 仅改变表面 tint 和边缘，不依赖 hover 才能发现内容；这与 GPUI Component 的桌面交互和 reduced-motion 友好原则一致。

