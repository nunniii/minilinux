use wasm_bindgen::prelude::*;

// Função de teste
#[wasm_bindgen]
pub fn get_message() -> String {
    "Hello from minilinux!".to_string()
}

// Função que simula a execução de comandos
#[wasm_bindgen]
pub fn call_function(command: String) -> String {
    match command.as_str() {
        "hello" => "Hello, World!".to_string(),
        "version" => "Minilinux v1.0".to_string(),
        "help" => "Available commands: hello, version, help".to_string(),
        _ => "Command not found.".to_string(),
    }
}
