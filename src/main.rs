struct HelloWorld {
    folder_path: gpui::Entity<gpui_component::input::InputState>
}

impl gpui::Render for HelloWorld {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        use gpui::*;
        use gpui_component::StyledExt;
        use gpui_component::input::Input;

        div()
            .h_flex() //ラベルとフォルダパス入力フォームを横並びにしたいのでh_flex()を使う
            .child("Folder path: ")
            .child(Input::new(&self.folder_path))
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
            //フォルダパスの入力フォームのEntityを作成
            let folder_path_input = cx.new(
                |cx| gpui_component::input::InputState::new(window, cx).placeholder("Enter folder path...")
            );

            let view = cx.new(|_| HelloWorld {folder_path: folder_path_input});

            // This first level on the window, should be a Root.
            cx.new(|cx| Root::new(view, window, cx))
        }).unwrap();
    });
}