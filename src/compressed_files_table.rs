use gpui::AppContext;

pub struct CompressedFiles {
    filename: String,
    size: usize,
}

pub struct CompressedFilesTable {
    data: Vec<CompressedFiles>,
    columns: Vec<gpui_component::table::Column>,
}


impl CompressedFilesTable {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::Entity<gpui_component::table::TableState<CompressedFilesTable>> {
        use gpui_component::table::Column;
        let delegate = Self {
            data: vec![
                CompressedFiles { filename: "aaa.txt".to_string(), size: 10 },
                CompressedFiles { filename: "bbb.txt".to_string(), size: 20 },
            ],
            columns: vec![
                Column::new("filename", "File name"),
                Column::new("size", "Size"),
            ]
        };

        cx.new(
            |cx| gpui_component::table::TableState::new(delegate, window, cx)
        )
    }

    pub fn push_row(&mut self, filename: &str, size: usize) {
        self.data.push( CompressedFiles {
            filename: filename.to_string(),
            size,
        })
    }
}

impl gpui_component::table::TableDelegate for CompressedFilesTable {
    fn columns_count(&self, _: &gpui::App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &gpui::App) -> usize {
        self.data.len()
    }

    fn column(&self, col_ix: usize, _: &gpui::App) -> &gpui_component::table::Column {
        &self.columns[col_ix]
    }

    fn render_td(
            &mut self,
            row_ix: usize,
            col_ix: usize,
            _window: &mut gpui::Window,
            _cx: &mut gpui::Context<gpui_component::table::TableState<Self>>,
        ) -> impl gpui::IntoElement {
        use gpui::*;

        let row = &self.data[row_ix];
        let col = &self.columns[col_ix];

        //このkeyはColumn::new()のkeyに渡したもの。構造体のメンバ名は関係ない
        match col.key.as_ref() {
            "filename" => div().child(row.filename.clone()),
            "size" => div().child(row.size.to_string()),
            _ => div(),
        }
    }
}
