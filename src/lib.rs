use wasm_bindgen::prelude::*;

// Export the function to JavaScript
#[wasm_bindgen]
pub fn get_message() -> String {
    "Hello from minilinux!".to_string()
}
