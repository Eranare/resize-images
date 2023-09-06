// src/gui_helpers.rs

extern crate native_dialog;
use native_dialog::FileDialog;

pub fn choose_folder() -> Option<String> {
    FileDialog::new()
        .show_open_single_dir()
        .unwrap()
        .map(|path| path.to_str().unwrap().to_string())
}