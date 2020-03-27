use std::process::Command;
use custom_error::custom_error;
use dirs::home_dir;
use std::path::Path;
use std::fs::{File, OpenOptions};
use chrono::offset::Local;
use std::io::prelude::*;

custom_error!{pub NotesError
    FileNotFound = "File could not be located",
    FileNotTouched = "Failed to create the desired file",
    FailedToWrite = "Failed to write to the desired file",
    Misc{error: String} = "Misc error: {error}"
}

fn filename() -> String {
    format!("{}{}{}.md", 
            home_dir().unwrap().as_path().display(),
            "/Notes/",
            Local::today().format("%Y-%m-%d"))
}

fn open() -> Result<String, NotesError> {
    let target_uri = filename();
    let target_path = Path::new(&target_uri);
    
    if !target_path.exists() { create(&target_uri)?; }

    Ok(target_uri)
}

fn create(uri: &str) -> Result<(), NotesError> {
    let mut header = format!("# {}", Local::today().format("%Y-%m-%d"));

    let mut file = match File::create(uri) {
        Ok(x) => x,
        Err(_e) => return Err(NotesError::FileNotTouched)
    };
    match file.write_all(header.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(NotesError::FailedToWrite)
    }
}

fn add_line(line: &str) -> Result<(), NotesError> {
    let content = format!("\n------\n## {}", line);
    let mut file = match OpenOptions::new().append(true).open(filename()) {
        Ok(x) => x,
        Err(_) => return Err(NotesError::FailedToWrite)
    };
    match file.write_all(content.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(NotesError::FailedToWrite)
    }
}

pub fn new_note(title: &str) -> Result<(), NotesError> {
    let target_uri = open()?;
    
    add_line(&title)?;

    match Command::new("vim").arg(&target_uri).status() {
        Ok(_x) => return Ok(()),
        Err(_e) => return Err(NotesError::FileNotFound)
    }
}
