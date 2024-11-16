// filesystem.rs
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub content: String,
}

impl File {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
        }
    }

    pub fn read(&self) -> String {
        self.content.clone()
    }

    pub fn write(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub name: String,
    pub directories: Vec<Directory>,
    pub files: Vec<File>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn add_directory(&mut self, subdir: Directory) {
        self.directories.push(subdir);
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn list_contents(&self) -> String {
        let mut output = format!("Directory: {}\n", self.name);

        if self.files.is_empty() && self.directories.is_empty() {
            output.push_str("  (Empty)\n");
        } else {
            if !self.files.is_empty() {
                output.push_str("  Files:\n");
                for file in &self.files {
                    output.push_str(&format!("    - {} (content: {})\n", file.name, file.content));
                }
            }

            if !self.directories.is_empty() {
                output.push_str("  Directories:\n");
                for dir in &self.directories {
                    output.push_str(&format!("    - {}\n", dir.name));
                }
            }
        }

        output
    }
}

// Aparato para apontar para o diretório através do nome
pub fn string_to_directory(root: &Directory, name: &str) -> Option<Directory> {
    if root.name == name {
        return Some(root.clone()); 
    }
    for subdir in &root.directories {
        if let Some(found) = string_to_directory(subdir, name) {
            return Some(found);
        }
    }
    None 
}

pub fn ls(dir_name: &str, root: &Directory) -> String {
    // Para identiicar qual diretório o usuário quer listar
    match string_to_directory(root, dir_name) {
        Some(dir) => dir.list_contents(),
        None => format!("Directory '{}' not found.", dir_name),
    }
}


