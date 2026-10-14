//! The engine in WASM, under Node, on the data file the site ships: the
//! answers the page renders, as JSON. The rules are tested in crates/core.
//!
//! Run: cargo test -p grafia-wasm --target wasm32-unknown-unknown
//! (needs wasm-bindgen-test-runner: see README.md, *Tests*)
#![cfg(target_arch = "wasm32")]
use grafia_wasm::Engine;
use wasm_bindgen::{JsError, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

const DATA: &[u8] = include_bytes!("../../../web/grafia.dat");

fn engine() -> Engine {
    Engine::new(DATA.to_vec())
        .map_err(JsValue::from)
        .expect("web/grafia.dat")
}

fn json(value: Result<JsValue, JsError>) -> String {
    let value = value.map_err(JsValue::from).expect("an answer");
    js_sys::JSON::stringify(&value).expect("JSON").into()
}

fn text(value: Result<JsValue, JsError>) -> String {
    let value = value.map_err(JsValue::from).expect("an answer");
    js_sys::Reflect::get(&value, &"text".into())
        .expect("text")
        .as_string()
        .expect("a string")
}

#[wasm_bindgen_test]
fn convert() {
    let engine = engine();
    // alts and unknown only where they apply: the page tests them for truth
    assert_eq!(
        json(engine.convert("Soare, ia zzoa", false, true)),
        r#"{"text":"Sōre, ė zzō","segments":[{"t":"Sōre, "},{"t":"ė","alts":["ė","ia"]},{"t":" "},{"t":"zzō","unknown":"zzoa"}],"unknown":["zzoa"]}"#
    );
    // an obsolete form: the current spelling first, the obsolete word and its current form
    assert_eq!(
        json(engine.convert("Alineare", false, true)),
        r#"{"text":"Aliniere","segments":[{"t":"Aliniere","alts":["Aliniere","Alineare"],"obsolete":["Alineare","Aliniere"]}],"unknown":[]}"#
    );
    // a [ ] span is a segment of its own, left as it is; a Latin name stays as it is
    assert_eq!(
        json(engine.convert("Soare [fier], Traian", false, true)),
        r#"{"text":"Sōre fier, Traian","segments":[{"t":"Sōre "},{"t":"fier","literal":true},{"t":", Traian"}],"unknown":[]}"#
    );
    // the switches, in order: ee, then ai
    assert_eq!(text(engine.convert("idee câine", true, false)), "ideẹ câine");
    assert_eq!(text(engine.convert("idee câine", false, true)), "idee cîine");
}

#[wasm_bindgen_test]
fn revert() {
    let engine = engine();
    assert_eq!(
        json(engine.revert("Bordeẹ, aleẹ", true)),
        r#"{"text":"Bordeie, alee","segments":[{"t":"Bordeie","alts":["Bordeie","Bordee"]},{"t":", alee"}]}"#
    );
    assert_eq!(text(engine.revert("Bordeẹ, aleẹ", false)), "Bordeie, aleie");
}

#[wasm_bindgen_test]
fn data_file() {
    let date = engine().date();
    assert!(
        date.len() == 10 && date.chars().all(|c| c.is_ascii_digit() || c == '-'),
        "{date}"
    );
    let error = Engine::new(b"not a data file".to_vec())
        .err()
        .map(JsValue::from)
        .expect("an error");
    let message = js_sys::Reflect::get(&error, &"message".into()).expect("message");
    assert_eq!(message.as_string().as_deref(), Some("not a grafia concisă data file"));
}
