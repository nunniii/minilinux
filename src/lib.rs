use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn get_message() -> String {
    "Hello from MiniLinux!".to_string()
}


#[wasm_bindgen]
pub fn help() -> String {
    "Available commands: help, version, hello".to_string()
}


#[wasm_bindgen]
pub fn version() -> String {
    "MiniLinux version 1.0.0".to_string()
}


#[wasm_bindgen]
pub fn hello(s: String) -> String {
    format!("Test called:: {}. \n{}", s, get_message())
}

// Função principal para execução de comandos
#[wasm_bindgen]
pub fn call_function(command: String) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts.get(0).unwrap_or(&"") {
        &"help" => help(),
        &"version" => version(),
        &"hello" => {
            if parts.len() > 1 {
                hello(parts[1].to_string())
            } else {
                "Usage: hello <some string>".to_string()
            }
        }
        _ => "Command not found.".to_string(),
    }
}
