// src/resizer.rs

extern crate image;
use image::{GenericImageView, ImageFormat};
use std::fs;
use std::sync::{Arc, Mutex};
use gtk::ProgressBar;


pub fn resize_images(input_folder: &str, output_folder: &str, target_width: u32, target_height: u32, progress: Arc<Mutex<ProgressBar>>, total_images: usize) {
    let mut processed_images = 0;
    // Iterate through the input folder and resize each image
    for entry in fs::read_dir(input_folder).expect("Failed to read input directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            let img = image::open(&path).expect("Failed to open image");
            let resized = img.resize(target_width, target_height, image::imageops::FilterType::Lanczos3);

            let output_path = format!("{}/{}", output_folder, path.file_name().unwrap().to_str().unwrap());
            resized.save_with_format(output_path, ImageFormat::Png).expect("Failed to save image");
            
            // Update the progress bar (this is a placeholder, you'd need logic to calculate actual progress)
            processed_images += 1;
            let fraction = processed_images as f64 / total_images as f64;
            let mut progress_lock = progress.lock().unwrap();
            progress_lock.set_fraction(fraction);

        }
    }
}

fn count_images_in_directory(path: &str, include_subfolders: bool) -> usize {
    let mut count = 0;
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() && (path.extension().unwrap_or_default() == "jpg" || path.extension().unwrap_or_default() == "png") {
            count += 1;
        } else if path.is_dir() && include_subfolders {
            count += count_images_in_directory(&path.to_str().unwrap(), true);
        }
    }
    count
}

pub use self::resize_images;
pub use self::count_images_in_directory;