// src/resizer.rs
use crate::ResizeMode;

use image::{ImageFormat};
use image::GenericImageView;
use std::fs;

pub fn resize_images(input_folder: &str, output_folder: &str, target_width: u32, target_height: u32, progress: &mut f64, total_images: usize, resize_mode: ResizeMode) {
    let mut processed_images = 0;
    // Iterate through the input folder and resize each image
    for entry in fs::read_dir(input_folder).expect("Failed to read input directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() && is_supported_image(&path) {
            let img = image::open(&path).expect("Failed to open image");
            
            let processed_img = match resize_mode {
                ResizeMode::Crop => {
                    let (width, height) = img.dimensions();
                    let target_aspect_ratio = target_width as f64 / target_height as f64;
                    
                    let (crop_width, crop_height) = if (width as f64 / height as f64) > target_aspect_ratio {
                        (height as f64 * target_aspect_ratio, height as f64)
                    } else {
                        (width as f64, width as f64 / target_aspect_ratio)
                    };
            
                    let crop_x = (width as f64 - crop_width) / 2.0;
                    let crop_y = (height as f64 - crop_height) / 2.0;
            
                    img.crop_imm(crop_x as u32, crop_y as u32, crop_width as u32, crop_height as u32);
                    let cropped_img = img.crop_imm(crop_x as u32, crop_y as u32, crop_width as u32, crop_height as u32);
                    cropped_img.resize(target_width, target_height, image::imageops::FilterType::Lanczos3)
                },
                ResizeMode::Pad => {
                    let (width, height) = img.dimensions();
                    let resized = if width > target_width || height > target_height {
                        img.resize(target_width, target_height, image::imageops::FilterType::Lanczos3)
                    } else {
                        img
                    };
            
                    let (new_width, new_height) = resized.dimensions();
                    let pad_x = (target_width.saturating_sub(new_width)) / 2;
                    let pad_y = (target_height.saturating_sub(new_height)) / 2;
            
                    image::DynamicImage::ImageRgba8(image::ImageBuffer::from_fn(target_width, target_height, |x, y| {
                        if x >= pad_x && x < new_width + pad_x && y >= pad_y && y < new_height + pad_y {
                            resized.get_pixel(x - pad_x, y - pad_y)
                        } else {
                            image::Rgba([255, 255, 255, 0])
                        }
                    }))
                },
                
                ResizeMode::Default => {
                    img.resize(target_width, target_height, image::imageops::FilterType::Lanczos3)
                },
            };

            let output_path = format!("{}/{}", output_folder, path.file_name().unwrap().to_str().unwrap());
            processed_img.save_with_format(output_path, ImageFormat::Png).expect("Failed to save image");
            
            // Update the progress value
            processed_images += 1;
            *progress = processed_images as f64 / total_images as f64;
        }
    }
}

pub fn count_images_in_directory(path: &str, include_subfolders: bool) -> usize {
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

fn is_supported_image(file_path: &std::path::Path) -> bool {
    match file_path.extension().and_then(|s| s.to_str()) {
        Some("jpg") | Some("jpeg") | Some("png") => true,
        _ => false,
    }
}
