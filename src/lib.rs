use js_sys::{JsString, Number};
use wasm_bindgen::prelude::{*, JsError};
use zxcvbn::zxcvbn;

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name="passwordScore")]
pub fn password_score(password: JsString) -> Result<Number, JsError> {
    let p = password
        .as_string()
        .ok_or(JsError::new("Invalid or missing argument parameter, 'password'."))?;

    let entropy = zxcvbn(&p, &[])
        .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    let score = entropy.score();

    Ok(Number::from(score))
}
