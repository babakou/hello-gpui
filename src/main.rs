use gpui_component::table::Table;
use std::future::Future;
use std::path::Path;

mod compressed_files_table;
mod util;

struct HelloWorld {
    folder_path: gpui::Entity<gpui_component::input::InputState>,
    sevenzip_exec_path: gpui::Entity<gpui_component::input::InputState>,
    extension: gpui::Entity<gpui_component::input::InputState>,
    files: gpui::Entity<gpui_component::table::TableState<compressed_files_table::CompressedFilesTable>>,
    is_delete_originals: bool,
}

impl HelloWorld {
    fn on_choose_clicked(
        &mut self,
        _event: &gpui::ClickEvent,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) {
        let input = self.folder_path.clone();
        let picker = rfd::AsyncFileDialog::new().pick_folder();

        self.set_input_from_picker(input, window, cx, picker);
    }

    fn choose_sevenzip(
            &mut self,
            _event: &gpui::ClickEvent,
            window: &mut gpui::Window,
            cx: &mut gpui::Context<Self>,
    ) {
        let input = self.sevenzip_exec_path.clone();
        let picker = rfd::AsyncFileDialog::new()
            .add_filter("7-Zip executable", &["exe"])
            .pick_file();

        self.set_input_from_picker(input, window, cx, picker);
    }

    fn set_input_from_picker<F>(
        &mut self,
        input: gpui::Entity<gpui_component::input::InputState>,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
        picker: F,
    )
    where
        F: Future<Output = Option<rfd::FileHandle>> + 'static,
    {
        cx.spawn_in(window, async move |this, cx| {
            let Some(file) = picker.await else {
                // キャンセル時は入力欄を変更しない
                return;
            };

            let path = file.path().to_string_lossy().into_owned();

            this.update_in(cx, |_, window, cx| {
                input.update(cx, |state, cx| {
                    state.set_value(path, window, cx);
                });
            })
            .ok();
        })
        .detach();
    }

    fn scan_files(
        &mut self,
        _event: &gpui::ClickEvent,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) {
        let root = self.folder_path.read(cx).value().to_string();
        let extension = self.extension.read(cx).value().to_string();

        let rows = match util::scan_files(Path::new(&root), &extension) {
            Ok(rows) => rows,
            Err(error) => {
                eprintln!("ファイル走査に失敗しました: {error}");
                return;
            }
        };

        self.files.update(cx, |table, cx| {
            table.delegate_mut().replace_rows(rows);
            cx.notify();
        });
    }
}

impl gpui::Render for HelloWorld {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        use gpui::*;
        use gpui_component::StyledExt;
        use gpui_component::input::Input;
        use gpui_component::button::Button;
        use gpui_component::checkbox::Checkbox;

        let folder_input_view = div()
            .h_flex() //ラベルとフォルダパス入力フォームを横並びにしたいのでh_flex()を使う
            .child("Folder path: ")
            .child(Input::new(&self.folder_path))
            .child(
                Button::new("choose_folder")
                    .label("Choose root folder")
                    .on_click(cx.listener(Self::on_choose_clicked))
            );

        let sevenzip_exec_view = div()
            .h_flex()
            .child("7z Executable: ")
            .child(Input::new(&self.sevenzip_exec_path))
            .child(
                Button::new("choose_7zip")
                    .label("Choose 7z.exe")
                    .on_click(cx.listener(Self::choose_sevenzip))
            );

        let extension_view = div()
            .h_flex()
            .child("Extension: ")
            .child(Input::new(&self.extension))
            .child(
                Button::new("scan_files")
                    .label("Scan")
                    .on_click(cx.listener(Self::scan_files))
            );

        let compress_button = div()
            .child(
                Button::new("compress")
                    .label("Compress")
                    .w(gpui::px(150.))
            );

        let delete_after_compression_control = div()
            .child(
                Checkbox::new("delete_after_compression_control")
                    .label("Delete after the compression")
                    .checked(self.is_delete_originals)
                    .on_click(cx.listener(|this: &mut HelloWorld, checked: &bool, _, cx| {
                        this.is_delete_originals = *checked;
                        cx.notify();
                    }))
            );

        let compress_view = div()
            .h_flex()
            .child(compress_button)
            .gap_2()
            .child(delete_after_compression_control);

        let table = div()
            .flex_1()
            .child(Table::new(&self.files));

        div()
            .size_full()
            .v_flex() //組み立てた行パーツを縦並びにしたいので、v_flex()を使う
            .child(folder_input_view)
            .child(sevenzip_exec_view)
            .child(extension_view)
            .child(compress_view)
            .child(table)
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
            window_bounds: Some(gpui::WindowBounds::centered(gpui::size(gpui::px(800.), gpui::px(600.)), cx)),
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
            //圧縮対象拡張子の入力フォームのEntityを作成
            let extension_input = cx.new(
                |cx| gpui_component::input::InputState::new(window, cx).placeholder("Extension")
            );
            //圧縮ファイルテーブルのEntityを作成
            let file_table_entity = compressed_files_table::CompressedFilesTable::new(window, cx);

            let view = cx.new(
                |_|
                HelloWorld {
                    folder_path: folder_path_input,
                    sevenzip_exec_path: sevenzip_exec_path_input,
                    extension: extension_input,
                    files: file_table_entity,
                    is_delete_originals: false,
                });

            // This first level on the window, should be a Root.
            cx.new(|cx| Root::new(view, window, cx))
        }).unwrap();
    });
}
