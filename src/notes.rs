//! Note interaction functionality

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


/// Generates the desired filename dynamically using current date.
fn filename() -> String {
    format!("{}{}{}.md", 
            home_dir().unwrap().as_path().display(),
            "/Notes/",
            Local::today().format("%Y-%m-%d"))
}

/// Attempts to open today's notes page
///
/// Checks whether the file for 'today' currently exists, creates it if not,
/// then returns the correct filename; this minimises calls to `filename()` and
/// abstracts the potential complications involved in opening a file away from
/// the program's main logic.
fn open() -> Result<String, NotesError> {
    let target_uri = filename();
    let target_path = Path::new(&target_uri);
    
    if !target_path.exists() { create(&target_uri)?; }

    Ok(target_uri)
}

/// Creates a file with the given name and writes a title to it.
///
/// Creates a file at the given path, then adds an h1 header to it with the
/// current date.
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

/// Adds a note title to today's file.
///
/// Formats the file so that a new header is present at the end with the given
/// `line` as its title. This is then ready for the note contents to be added
/// below.
fn add_title(line: &str) -> Result<(), NotesError> {
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

/// Makes a new note and opens vim for the user to edit it.
///
/// Opens todays file, adds a new note header using `title` as the name, and
/// opens vim for the user to add the content.
pub fn new_note(title: &str) -> Result<(), NotesError> {
    let target_uri = open()?;
    
    add_title(&title)?;

    match Command::new("vim").args(&["+ normal G$", &target_uri]).status() {
        Ok(_x) => return Ok(()),
        Err(_e) => return Err(NotesError::FileNotFound)
    }
}
