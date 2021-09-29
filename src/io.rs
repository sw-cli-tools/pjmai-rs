use log::info;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

/// Read the serialized configuration from the toml file
pub fn read(file_name: String) -> Result<String, Box<dyn std::error::Error + 'static>> {
    info!("reading...");
    let contents = fs::read_to_string(&file_name)?;
    info!("...read {} bytes", contents.len());
    Ok(contents)
}

/// Write the serialized configuration to the toml file
pub fn write(content: &str, target_file: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    info!("writing...");
    let mut file = File::create(target_file).unwrap();
    file.write_all(content.as_bytes())?;
    info!("...wrote {} bytes", content.len());
    Ok(())
}
