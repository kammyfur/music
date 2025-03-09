use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::models::directory::Directory;
use wasm_bindgen::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub async fn get_directory() -> Directory {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("https://media.music.floo.fi/directory.json", &opts)
        .unwrap();

    let window = web_sys::window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await
        .unwrap();
    let response: Response = response.dyn_into().unwrap();

    let json = JsFuture::from(response.text().unwrap()).await
        .unwrap().as_string().unwrap();

    Directory(serde_json::from_str(&json).unwrap())
}