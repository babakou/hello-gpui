struct HelloWorld {
    folder_path: gpui::Entity<gpui_component::input::InputState>,
    sevenzip_exec_path: gpui::Entity<gpui_component::input::InputState>,
}

impl gpui::Render for HelloWorld {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        use gpui::*;
        use gpui_component::StyledExt;
        use gpui_component::input::Input;
        use gpui_component::button::Button;

        let folder_input_view = div()
            .h_flex() //ラベルとフォルダパス入力フォームを横並びにしたいのでh_flex()を使う
            .child("Folder path: ")
            .child(Input::new(&self.folder_path))
            .child(
                Button::new("choose_folder")
                .label("Choose")
            );

        let sevenzip_exec_view = div()
            .h_flex()
            .child("7z Executable: ")
            .child(Input::new(&self.sevenzip_exec_path));
        
        div()
            .v_flex() //組み立てた行パーツを縦並びにしたいので、v_flex()を使う
            .child(folder_input_view)
            .child(sevenzip_exec_view)
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
            //7z実行パスの入力フォームのEntityを作成
            let sevenzip_exec_path_input = cx.new(
                |cx| gpui_component::input::InputState::new(window, cx).placeholder("Enter 7z executable")
            );

            let view = cx.new(|_| HelloWorld {folder_path: folder_path_input, sevenzip_exec_path: sevenzip_exec_path_input});

            // This first level on the window, should be a Root.
            cx.new(|cx| Root::new(view, window, cx))
        }).unwrap();
    });
}