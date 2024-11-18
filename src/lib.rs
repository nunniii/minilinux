// lib.rs
use console_error_panic_hook;

mod filesystem;

use wasm_bindgen::prelude::*;
use filesystem::{ File, Directory, ls, cat};

#[wasm_bindgen]
pub fn get_message() -> String {
    "Hello from MiniLinux!".to_string()
}

#[wasm_bindgen]
pub fn help() -> String {
    "Available commands: help, version, test, cat, ls".to_string()
}

#[wasm_bindgen]
pub fn version() -> String {
    "MiniLinux version 1.0.0".to_string()
}



// #arch
// -- Ex.:  p{/home/uwu} c{Cd}{/path/este/path} % d|name|{ d|test|{  a|hello.txt|{this is the string of here}  } a|hi.txt|{string hi} }
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


// Função para separar as informações de path e comando
#[wasm_bindgen]
pub fn filesystem(input: String) -> String {
    let mut path = String::new();
    if let Some(path_start) = input.find("p{") {
        if let Some(path_end) = input.find("}") {
            path = input[path_start + 2..path_end].to_string(); // Extrai o path
        }
    } 
    format!("{}", path)
}




fn get_pahms(i: &String) -> (String, String) {
    let mut path = String::new();
    let mut command = String::new();
    let mut command_path = String::new();

    // Extrair o path
    if let Some(path_start) = i.find("p{") {
        if let Some(path_end) = i[path_start + 2..].find("}") {
            let path_end_absolute = path_start + 2 + path_end;
            path = i[path_start + 2..path_end_absolute].to_string();
        }
    }

    // Extrair o comando
    if let Some(command_start) = i.find("c{") {
        if let Some(command_end) = i[command_start + 2..].find("}") {
            let command_end_absolute = command_start + 2 + command_end;
            command = i[command_start + 2..command_end_absolute].to_string();
        }
    }

    // Extrair o caminho do comando
    if let Some(command_path_start) = i.find("}{") {
        if let Some(command_path_end) = i[command_path_start + 2..].find("}") {
            let command_path_end_absolute = command_path_start + 2 + command_path_end;
            command_path = i[command_path_start + 2..command_path_end_absolute].to_string();
        }
    }

    // Retorna o path, comando e o caminho do comando
    (format!(
        "--get_pahms:: Path: {}; Comando: {}; Path do Comando: {}",
        path, command, command_path
    ), command )
}





#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}



fn cd(){
    println!("oi");
}




#[wasm_bindgen]
pub fn call_function(command: String, path: String) -> String {



    let parts: Vec<&str> = command.split_whitespace().collect();

    let mut currenty_dir = Directory::new("root");

    currenty_dir.add_file(File::new("file1.txt", "Hello, World!"));
    currenty_dir.add_file(File::new("file2.txt", "Rust is great!"));

    let mut subdir = Directory::new("subdir");
    subdir.add_file(File::new("subfile.txt", "Inside subdir"));

    currenty_dir.add_directory(subdir);

    
    let mut com: String = String::new();

    if get_pahms(&path).1 == "cd" {
        com = String::from("deu");
    }


    match parts.get(0).unwrap_or(&"") {
        &"help" => help(),
        &"version" => version(),
        &"cd" => {
            if parts.len() > 1 {
                if parts[1] == "." {
                    return "you are already here uwu".to_string();
                } else {
                    cd();
                    return "??".to_string();
                }
            } else {
                return "No path specified.".to_string();
            }
            format!("{}:::{}", com.to_string(), &path.to_string())
        } 
        &"ls" => {
            if parts.len() > 1 {
                ls(parts[1], &currenty_dir)
            } else {
                currenty_dir.list_contents()
            }
        }
        &"cat" => {
            if parts.len() > 1 {
                cat(parts[1], &currenty_dir)
            } else {
                "No file specified.".to_string()
            }
        }
        _ => "Command not found.".to_string(),
    }
}
