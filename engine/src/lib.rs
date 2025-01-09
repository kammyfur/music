mod utils;
mod models;
mod features;

use crate::features::core::load;
use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;

pub use crate::features::modal::*;
pub use crate::features::navigation::*;

#[wasm_bindgen(start)]
pub async fn start() {
    set_panic_hook();
    load().await;
}