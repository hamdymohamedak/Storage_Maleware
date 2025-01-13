use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use rand::Rng;

fn main() -> std::io::Result<()>{
    
    generate_files();
    Ok(())
}

fn generate_files () -> std::io::Result<()>{


    let startup_folder = env::var("APPDATA")
        .map(|appdata| Path::new(&appdata).join("Microsoft\\Windows\\Start Menu\\Programs\\Startup"))
        .unwrap_or_else(|_| Path::new("C:\\").to_path_buf());

    // Generate a random string for the file name
    let random_filename = generate_random_filename();
    
    // Set the random file path in the Startup folder
    let file_path = startup_folder.join(random_filename);

    // Create the file and write to it
    let mut file = File::create(file_path)?;
    file.write_all(b"Hello, world!")?;
    
    Ok(())

}
// Function to generate a random filename
fn generate_random_filename() -> String {
    let mut rng = rand::thread_rng();
    let random_str: String = (0..10)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    format!("{}.{}", random_str,random_str)
}
