//! A Rust wrapper for Asciidoctor.js for the WASM target

use wasm_bindgen::JsValue;

pub mod prelude;
pub mod sys;

#[cfg(feature = "yew")]
pub mod yew;

/// Convert the provided Asciidoc content.
pub fn convert<C: AsRef<str>>(content: C, options: &serde_json::Value) -> Result<String, JsValue> {
    Ok(sys::Asciidoctor::convert(
        content.as_ref(),
        serde_wasm_bindgen::to_value(options)?,
    ))
}
