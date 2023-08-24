use std::io::{Read, Write};
use std::fs::{self, File};
use dirs::home_dir;

pub fn create_note(note_name: String) {
    if let Some(home_path) = home_dir() {
        let mut note_contents = String::new();
        let folder_path = home_path.join("hw/notes");
        
        let note_path = folder_path.join(note_name);

        if !folder_path.exists() {
            if let Err(e) = fs::create_dir_all(&folder_path) {
                println!("Error creating folder: {}", e);
                return;
            }
        }

        let mut file = match File::create(&note_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error creating file: {}", e);
                return;
            }
        };

        std::io::stdin().read_line(&mut note_contents).expect("Invalid character.");
        if let Err(e) = write!(file, "{}", note_contents) {
                println!("Error writing to file: {}", e);
        }
    } else { 
        println!("Could not find home directory.") 
    }
}

pub fn lcr_note(name: String, list: bool , cat: bool, del: bool) {
    if let Some(home_path) = home_dir() {
        let folder_path: std::path::PathBuf = home_path.join("hw/notes");

        if list {
            list_note(folder_path.clone());
        } else if cat {
            cat_note(folder_path.clone(), name);
        } else if del {
            delete_note(folder_path, name);
        }
        
    } else { 
        println!("Could not find home directory.") 
    }
}

fn delete_note(folder_path: std::path::PathBuf, name: String) {
    let file_path = folder_path.join(name.clone());

    if let Err(e) = fs::remove_file(&file_path) {
        println!("Error deleting file: {}", e);
    } else {
        println!("'{}' deleted.", name);
    }
}

fn cat_note(folder_path: std::path::PathBuf, name: String) {
    let mut contents = String::new();
    let file_path = folder_path.join(name);

    match File::open(file_path) {
        Ok(mut file) => {
            if let Err(err) = file.read_to_string(&mut contents) { eprintln!("Error reading file: {}", err); return; }
        }
        Err(err) => { eprintln!("Error opening file: {}", err); return; }
    }
    println!("{}", contents)
}

fn list_note(folder_path: std::path::PathBuf ) {
    if let Ok(entries) = fs::read_dir(&folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    println!("{}", file_name);
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {:?}", folder_path);
    }
}