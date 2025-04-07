use blurhash::encode;
use image::{GenericImageView, ImageFormat};
use std::path::Path;
use std::fs;
use std::env;
use rayon::prelude::*;
use log::{info, warn, error};
use serde_json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::BufReader;
use std::fs::File;

fn process_image(file_path: &Path, counter: &AtomicUsize, total: usize) -> Option<(String, String)> {
    let mut file_path = file_path.to_path_buf();
    
    // Check if file has an image extension
    let is_image = file_path.extension()
        .map(|ext| matches!(ext.to_str(), Some("jpg" | "jpeg" | "png")))
        .unwrap_or(false);
    
    if !is_image {
        let new_file_path = file_path.with_extension("jpg");
        if let Err(e) = fs::rename(&file_path, &new_file_path) {
            error!("Failed to rename {} to {}: {}", file_path.display(), new_file_path.display(), e);
            counter.fetch_add(1, Ordering::Relaxed);
            return None;
        }
        file_path = new_file_path;
    }
    
    // Process the image with optimized loading
    if let Ok(file) = File::open(&file_path) {
        let reader = BufReader::new(file);
        if let Ok(img) = image::load(reader, ImageFormat::from_path(&file_path).unwrap_or(ImageFormat::Jpeg)) {
            let (width, height) = img.dimensions();
            
            // Skip very large images
            if width * height > 10_000_000 {
                warn!("Skipping large image: {} ({}x{})", file_path.display(), width, height);
                counter.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            
            if let Ok(blurhash) = encode(4, 4, width, height, &img.to_rgba8().into_vec()) {
                counter.fetch_add(1, Ordering::Relaxed);
                info!("Progress: {}/{}", counter.load(Ordering::Relaxed), total);
                return Some((file_path.display().to_string(), blurhash));
            } else {
                error!("Failed to generate blurhash for {}", file_path.display());
            }
        } else {
            error!("Failed to open image: {}", file_path.display());
        }
    } else {
        error!("Failed to open file: {}", file_path.display());
    }
    counter.fetch_add(1, Ordering::Relaxed);
    info!("Progress: {}/{}", counter.load(Ordering::Relaxed), total);
    None
}

fn main() {
    // Initialize logger
    env_logger::init();
    info!("Starting blurhash generator");

    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let mut folder_path = "";
    let mut output_json_path = "";
    let mut sample_size: Option<usize> = None;
    let mut chunk_size: usize = 100; // Process images in chunks to manage memory
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--sample" => {
                if i + 1 < args.len() {
                    if let Ok(size) = args[i + 1].parse::<usize>() {
                        sample_size = Some(size);
                        i += 2;
                        continue;
                    }
                }
                error!("--sample requires a numeric value");
                std::process::exit(1);
            },
            "--chunk" => {
                if i + 1 < args.len() {
                    if let Ok(size) = args[i + 1].parse::<usize>() {
                        chunk_size = size;
                        i += 2;
                        continue;
                    }
                }
                error!("--chunk requires a numeric value");
                std::process::exit(1);
            },
            arg => {
                if folder_path.is_empty() {
                    folder_path = arg;
                } else if output_json_path.is_empty() {
                    output_json_path = arg;
                } else {
                    error!("Too many arguments");
                    std::process::exit(1);
                }
                i += 1;
            }
        }
    }
    
    if folder_path.is_empty() || output_json_path.is_empty() {
        error!("Usage: {} <folder_path> <output_path> [--sample <number>] [--chunk <number>]", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(folder_path);
    info!("Processing directory: {}", folder_path);

    if !path.exists() || !path.is_dir() {
        error!("Error: '{}' is not a valid directory", folder_path);
        std::process::exit(1);
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            // Collect all paths first
            let mut paths: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect();

            // if sample size is provided, slice the vec
            if let Some(size) = sample_size {
                paths = paths.into_iter().take(size).collect();
            }
            
            let total_files = paths.len();
            info!("Found {} files to process", total_files);

            // Create a counter for tracking progress
            let counter = AtomicUsize::new(0);
            let mut all_results = Vec::new();

            // Process images in chunks to manage memory
            for chunk in paths.chunks(chunk_size) {
                let results: Vec<_> = chunk.par_iter()
                    .filter_map(|path| process_image(path, &counter, total_files))
                    .collect();
                
                all_results.extend(results);
            }

            info!("Successfully processed {} images", all_results.len());
            
            // Create JSON output with streaming serialization
            let mut json_writer = std::fs::File::create(&output_json_path).unwrap();
            serde_json::to_writer_pretty(&mut json_writer, &serde_json::json!({
                "results": all_results.iter().map(|(file_path, blurhash)| {
                    serde_json::json!({
                        "file": file_path,
                        "blurhash": blurhash
                    })
                }).collect::<Vec<_>>()
            })).unwrap();
            
            info!("Results saved to {}", output_json_path);
        }
        Err(e) => error!("Error reading directory: {}", e),
    }
}