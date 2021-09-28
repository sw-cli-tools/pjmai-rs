use crate::ProjectPath;
use log::info;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn read(file_name: ProjectPath) -> Result<String, Box<dyn std::error::Error + 'static>> {
    info!("reading...");
    let contents = fs::read_to_string(&file_name)?;
    info!("...read {} bytes", contents.len());
    Ok(contents)
}
pub fn write(content: &str, target_file: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    info!("writing...");
    let mut file = File::create(target_file).unwrap();
    file.write_all(content.as_bytes())?;
    info!("...wrote {} bytes", content.len());
    Ok(())
}
