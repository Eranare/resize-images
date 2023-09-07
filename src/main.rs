// src/main.rs

extern crate image;
extern crate native_dialog;

use druid::widget::{Button, Flex, Checkbox, ProgressBar};
use druid::{AppLauncher, Widget, WindowDesc, Data, Lens, WidgetExt};

// Import the resize_images function from the resizer module
use crate::resizer::{resize_images, count_images_in_directory};
mod resizer;
mod gui_helpers;

#[derive(Clone, Data, Lens)]
struct AppState {
    input_folder: String,
    output_folder: String,
    include_subfolders: bool,
    progress: f64,
    resize_mode: ResizeMode,
}

#[derive(Clone, Data, PartialEq)]
pub enum ResizeMode {
    Crop,
    Pad,
    Default,
}

fn build_ui() -> impl Widget<AppState> {
    // Create a vertical layout
    let mut col = Flex::column();

    // Add components to the layout
    let choose_input_button = Button::new("Choose Input Folder").on_click(|_, data: &mut AppState, _| {
        data.input_folder = gui_helpers::choose_folder().unwrap_or_default();
    });

    let include_subfolders_check = Checkbox::new("Include Subfolders").lens(AppState::include_subfolders);

    let choose_output_button = Button::new("Choose Output Folder").on_click(|_, data: &mut AppState, _| {
        data.output_folder = gui_helpers::choose_folder().unwrap_or_default();
    });

    use druid::widget::RadioGroup;

    let resize_mode_group = RadioGroup::new(vec![
        ("Default", ResizeMode::Default),
        ("Crop", ResizeMode::Crop),
        ("Pad", ResizeMode::Pad),
    ]).lens(AppState::resize_mode);

    col.add_flex_child(resize_mode_group, 1.0);


    let start_button = Button::new("Start").on_click(|_, data: &mut AppState, _| {
        let total_images = count_images_in_directory(&data.input_folder, data.include_subfolders);
        resize_images(&data.input_folder, &data.output_folder, 256, 256, &mut data.progress, total_images, data.resize_mode.clone());
    });

    let progress_bar = ProgressBar::new().lens(AppState::progress);

    col.add_flex_child(choose_input_button, 1.0);
    col.add_flex_child(include_subfolders_check, 1.0);
    col.add_flex_child(choose_output_button, 1.0);
    col.add_flex_child(progress_bar, 1.0);
    col.add_flex_child(start_button, 1.0);

    col
}

fn main() {
    let main_window = WindowDesc::new(build_ui).title("Image Resizer");
    let state = AppState {
        input_folder: String::new(),
        output_folder: String::new(),
        include_subfolders: false,
        progress: 0.0,
        resize_mode: ResizeMode::Default,
    };
    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}