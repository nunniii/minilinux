// lib.rs
use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use filesystem::{ File, get_pahms, ls, cat};
mod filesystem;



#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

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


#[wasm_bindgen]
pub fn call_function(command: String, path: String) -> String {

    let current_dir = get_pahms(&path);
    let parts: Vec<&str> = command.split_whitespace().collect();

    let mut com: String = String::new();

    match parts.get(0).unwrap_or(&"") {
        &"help" => help(),
        &"version" => version(),
        &"cd" => {
            if parts.len() > 1 {
                if parts[1] == "." {
                    return "you are already here uwu".to_string();
                } else {
                    return "??".to_string();
                }
            } else {
                return "No path specified.".to_string();
            }
            format!("{}:::{}", com.to_string(), &path.to_string())
        } 
        &"ls" => {
            if parts.len() > 1 {
                "??".to_string()
            } else {
                ls(&current_dir)
            }
        }
        &"cat" => {
            if parts.len() > 1 {
                cat(parts[1], &current_dir )
            } else {
                "No file specified.".to_string()
            }
        }
        _ => "Command not found.".to_string(),
    }
}


