// src/main.rs

extern crate image;
extern crate native_dialog;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType,CheckButton, Button, Box, ProgressBar, Orientation};
use std::sync::{Arc, Mutex};


// Import the resize_images function from the resizer module

use crate::resizer::{resize_images, count_images_in_directory};
mod resizer;
mod gui_helpers;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Image Resizer");
    window.set_default_size(300, 100);

    let vbox = Box::new(Orientation::Vertical, 5);
    let hbox = Box::new(Orientation::Horizontal, 5);
    let choose_input_button = Button::new_with_label("Choose Input Folder");
    let include_subfolders_check = CheckButton::new_with_label("Include Subfolders");
    let choose_output_button = Button::new_with_label("Choose Output Folder");
    let start_button = Button::new_with_label("Start");
    let progress_bar = ProgressBar::new();

    hbox.pack_start(&choose_input_button, true, true, 0);
    hbox.pack_start(&include_subfolders_check, false, false, 0);
    hbox.pack_start(&choose_output_button, true, true, 0);
    hbox.pack_start(&progress_bar, true, true, 0);
    hbox.pack_start(&start_button, true, true, 0);
    vbox.pack_start(&hbox, true, true, 0);
    window.add(&vbox);

    let input_folder = Arc::new(Mutex::new(String::new()));
    let output_folder = Arc::new(Mutex::new(String::new()));
    let progress = Arc::new(Mutex::new(progress_bar.clone()));
    
    choose_input_button.connect_clicked(move |_| {
        let mut input_folder_lock = input_folder.lock().unwrap();
        *input_folder_lock = gui_helpers::choose_folder().expect("User did not choose an input folder");
    });
    
    choose_output_button.connect_clicked(move |_| {
        let mut output_folder_lock = output_folder.lock().unwrap();
        *output_folder_lock = gui_helpers::choose_folder().expect("User did not choose an output folder");
    });
    
    start_button.connect_clicked(move |_| {
        let input = input_folder.lock().unwrap().clone();
        let output = output_folder.lock().unwrap().clone();
        let include_subfolders = include_subfolders_check.get_active();
        let total_images = count_images_in_directory(&input, include_subfolders);
        resize_images(&input, &output, 256, 256, progress.clone(), total_images);
    });
    

    window.show_all();
    gtk::main();
}
