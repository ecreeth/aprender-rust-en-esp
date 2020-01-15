use orbtk::prelude::*;
use ui::button::styles::{Primary, Secondary}

fn main() {
  Application::new()
    .window(|ctx| {
        Window::create()
          .title("OrbTk - minimal example")
          .position((100.0, 100.0))
          .size(420.0, 730.0)
          .child(
          	TextBlock::create()
          		.text("OrbTk")
          		.bind_style(&mut Primary)
          		// .bind_style(&mut my_button_styles)
          		.build(ctx))
          .build(ctx)
    })
    .run();
}


let mut my_button_styles = style!(
		height: 32.0
    min_width: 80.0
    background: colors::LYNCH_COLOR
    border_radius 2.0
    border_width: 0.0
    border_brush: "transparent"
    padding: (8.0, 0.0, 8.0, 0.0)
    foreground: colors::LINK_WATER_COLOR
    text: ""
    font_size: fonts::FONT_SIZE_12
    font: "Roboto Regular"
    icon: ""
    icon_font: "Material Icons"
    icon_size: fonts::ICON_FONT_SIZE_12
    icon_brush: colors::LINK_WATER_COLOR
    pressed: false
);