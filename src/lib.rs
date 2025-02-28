use wasm_bindgen::prelude::*;

/// Parses the given WAT text and returns a vector of bytes on success or a JavaScript error on failure.
///
/// # Arguments
///
/// * `text` - A string slice containing the WAT text to be parsed.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` containing the parsed byte code if parsing is successful.
/// * `Err(JsValue)` if parsing fails, with the error message converted into a JavaScript error value.
#[wasm_bindgen]
pub fn watify(text: &str) -> Result<Vec<u8>, JsValue> {
    wat::parse_str(text).map_err(|e| e.to_string().into())
}
