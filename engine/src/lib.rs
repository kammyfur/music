mod utils;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlElement, Request, RequestInit, RequestMode, Response, Window};
use crate::utils::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct State {
    window: Window,
    document: Document,
    body: HtmlElement,
    directory: Directory,
    songs: Vec<Song>
}

#[derive(Debug)]
pub struct Song {
    versions: Vec<DirectoryEntry>,
    year: u32,
    edition: Vec<String>,
    artist: String,
    track: String,
    original: bool,
    ai: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DirectoryEntry {
    id: String,
    #[serde(alias = "cdnId")]
    cdn_id: String,
    file: String,
    edition: Vec<String>,
    year: u32,
    artist: String,
    track: String,
    original: bool,
    ai: bool
}

#[derive(Debug)]
struct Directory(HashMap<String, Vec<DirectoryEntry>>);

pub async fn get_directory() -> Directory {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("https://watercolor-cdn.floo.fi/records/directory.json", &opts)
        .unwrap();

    let window = web_sys::window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await
        .unwrap();
    let response: Response = response.dyn_into().unwrap();

    let json = JsFuture::from(response.text().unwrap()).await
        .unwrap().as_string().unwrap();

    Directory(serde_json::from_str(&json).unwrap(),)
}

#[wasm_bindgen]
pub async fn start() -> State {
    set_panic_hook();
    println!("Hello from Rust!");

    let window = web_sys::window().expect("No global `window` exists");
    let document = window.document().expect("Should have a document on window");
    let body = document.body().expect("Document should have a body");

    println!("Fetching directory...");

    let directory: Directory = get_directory().await;
    println!("Got {} directory entries", directory.0.len());
    document.get_element_by_id("count")
        .unwrap()
        .set_text_content(Some(&format!("{} productions", directory.0.len())));

    let songs = directory.as_songs();

    State {
        window,
        document,
        body,
        directory,
        songs,
    }
}

#[wasm_bindgen]
pub fn debug(state: &State) {
    println!("{:?}", state);
}

#[wasm_bindgen]
pub fn directory(state: &State) -> String {
    serde_json::to_string(&state.directory.0).unwrap()
}

impl Directory {
    pub fn as_songs(&self) -> Vec<Song> {
        todo!()
    }
}