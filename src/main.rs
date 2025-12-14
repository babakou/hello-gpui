struct HelloWorld {
}

impl gpui::Render for HelloWorld {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        use gpui::*;

        div()
            .child("hello world")
    }
}

fn main() {
    use gpui::Application;
    use gpui::AppContext;
    use gpui_component::Root;

    let app = Application::new();

    app.run(move |cx: &mut gpui::App| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        let window_options = gpui::WindowOptions {
            window_bounds: Some(gpui::WindowBounds::centered(gpui::size(gpui::px(400.), gpui::px(200.)), cx)),
            ..Default::default()
        };
        cx.open_window(window_options, |window, cx| {
            let view = cx.new(|_| HelloWorld {});

            // This first level on the window, should be a Root.
            cx.new(|cx| Root::new(view, window, cx))
        }).unwrap();
    });
}