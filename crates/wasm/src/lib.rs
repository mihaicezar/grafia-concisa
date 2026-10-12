//! The core in the browser: `new Engine(bytes)` reads the data file, and
//! `convert` and `revert` return the core's answers as plain objects, which the
//! web page renders (`web/index.html`).
use grafia_concisa::{DataFile, Options};
use wasm_bindgen::prelude::*;

/// The word data and the conversions on it.
#[wasm_bindgen]
pub struct Engine {
    dict: DataFile<Vec<u8>>,
}

#[wasm_bindgen]
impl Engine {
    /// Reads the bytes of `grafia.dat`; throws an Error if they are no valid data file.
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> Result<Engine, JsError> {
        Ok(Engine {
            dict: DataFile::new(bytes)?,
        })
    }

    /// The date of the dexonline data (YYYY-MM-DD), empty if unknown.
    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String {
        self.dict.date().to_string()
    }

    /// Standard -> concise, with the -ee and â -> î switches:
    /// `{text, segments: [{t, alts?, unknown?, obsolete?}], unknown}`, where `obsolete` is
    /// [the obsolete word, then its current forms].
    pub fn convert(&self, text: &str, ee: bool, ai: bool) -> Result<JsValue, JsError> {
        let conversion = grafia_concisa::convert(text, &self.dict, Options { ee, ai });
        Ok(serde_wasm_bindgen::to_value(&conversion)?)
    }

    /// Concise -> standard; with `ee`, the text was written with the -ee rule:
    /// `{text, segments: [{t, alts?}]}`.
    pub fn revert(&self, text: &str, ee: bool) -> Result<JsValue, JsError> {
        let reversion = grafia_concisa::revert(text, &self.dict, ee);
        Ok(serde_wasm_bindgen::to_value(&reversion)?)
    }
}
