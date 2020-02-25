use std::fs::File;
use std::fs;
use std::io::prelude::*;

pub fn read(file_name: String) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let contents = fs::read_to_string(&file_name)?;
    Ok(contents)
}
pub fn write(content: &str, target_file: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut file = File::create(target_file).unwrap();
    file.write(content.as_bytes())?;
    Ok(())
}

