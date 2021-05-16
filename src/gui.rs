use::orbtk::prelude::*;

pub fn run_gui(title: &'static str) {
    Application::new()
    .window(move |ctx| {
        Window::new()
            .title(title)
            .position((100.0, 100.0))
            .size(400.0, 400.0)
            .child(TextBlock::new().text("Close this to run game!").build(ctx))
            .build(ctx)
    })
    .run();
}