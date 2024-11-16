// lib.rs
mod filesystem;

use wasm_bindgen::prelude::*;
use filesystem::{ File, Directory, ls};

#[wasm_bindgen]
pub fn get_message() -> String {
    "Hello from MiniLinux!".to_string()
}

#[wasm_bindgen]
pub fn help() -> String {
    "Available commands: help, version, hello, ls".to_string()
}

#[wasm_bindgen]
pub fn version() -> String {
    "MiniLinux version 1.0.0".to_string()
}

#[wasm_bindgen]
pub fn call_function(command: String) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    let mut currenty_dir = Directory::new("root");

    currenty_dir.add_file(File::new("file1.txt", "Hello, World!"));
    currenty_dir.add_file(File::new("file2.txt", "Rust is great!"));

    let mut subdir = Directory::new("subdir");
    subdir.add_file(File::new("subfile.txt", "Inside subdir"));

    currenty_dir.add_directory(subdir);

    match parts.get(0).unwrap_or(&"") {
        &"help" => help(),
        &"version" => version(),
        &"ls" => {
            if parts.len() > 1 {
                ls(parts[1], &currenty_dir)
            } else {
                currenty_dir.list_contents()
            }
        }
        _ => "Command not found.".to_string(),
    }
}
