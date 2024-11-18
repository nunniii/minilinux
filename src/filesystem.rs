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


// #arch
// -- Ex.: % a|hello.txt|{this is the string of here} a|hi.txt|{string hi} }
//
// -- Legenda:
// % -> indicar início;
// (a, d, p, c) -> (arquivo, diretório, path, comando)
//
// -- Estrutura::     
//      /name
//      |
//      /test  --> hello.txt
//      hi.txt

pub fn get_pahms(input: &str) -> Vec<File> {
    let mut files = Vec::new();
    let mut i = 0;

    while i < input.len() {
        if input[i..].starts_with("a|") {
            // Localiza o nome do arquivo e seu conteúdo
            if let Some(name_end) = input[i + 2..].find('|') {
                let name_start = i + 2;
                let content_start = name_start + name_end + 2;

                if let Some(content_end) = input[content_start..].find('}') {
                    let file_name = &input[name_start..name_start + name_end];
                    let file_content = &input[content_start..content_start + content_end];
                    
                    // Adiciona o arquivo ao vetor
                    files.push(File::new(file_name, file_content));
                    
                    i = content_start + content_end + 1; // Avança após o conteúdo
                } else {
                    break; // Conteúdo inválido, encerra a análise
                }
            } else {
                break; // Nome inválido, encerra a análise
            }
        } else {
            i += 1; // Avança no input
        }
    }

    files
}



pub fn cat(file_name: &str, files: &[File]) -> String {
    match files.iter().find(|file| file.name == file_name) {
        Some(file) => file.read(),
        None => format!("File '{}' not found.", file_name),
    }
}

pub fn ls(items: &[File]) -> String {
    if items.is_empty() {
        return String::from("(Empty)");
    }

    let mut output = String::new();
    for file in items {
        output.push_str(&format!("\t{}\n", file.name));
    }
    output
}
