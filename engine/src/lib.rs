mod utils;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Element, HtmlElement, Node, Request, RequestInit, RequestMode, Response, Window};
use crate::utils::{log, set_panic_hook};

#[wasm_bindgen]
#[derive(Debug)]
pub struct State {
    window: Window,
    document: Document,
    body: HtmlElement,
    directory: Directory,
    songs: Vec<Song>
}

#[derive(Debug, Clone)]
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
#[derive(Clone)]
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

#[derive(Debug, Clone)]
struct Directory(HashMap<String, Vec<DirectoryEntry>>);

async fn get_directory() -> Directory {
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

    let songs: Vec<Song> = (&directory).into();

    let state = State {
        window: window.clone(),
        document: document.clone(),
        body: body.clone(),
        directory: directory.clone(),
        songs: songs.clone(),
    };

    println!("{:#?}", state);
    println!("Filling HTML...");
    let container = document.get_element_by_id("js-data-list").unwrap();

    let mut songs_enumeration = songs
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &Song)>>();

    songs_enumeration.sort_by(|a, b| a.1.track.to_lowercase().partial_cmp(&b.1.track.to_lowercase()).unwrap());
    songs_enumeration.sort_by(|a, b| a.1.artist.to_lowercase().partial_cmp(&b.1.artist.to_lowercase()).unwrap());
    songs_enumeration.sort_by(|a, b| b.1.year.partial_cmp(&a.1.year).unwrap());
    songs_enumeration.sort_by(|a, b| a.1.ai.partial_cmp(&b.1.ai).unwrap());

    for (id, element) in songs_enumeration {
        container.append_child(&element.html(&document, id)).unwrap();
    }

    state
}

fn hash_text_color(text: &str) -> (u16, u16, u16) {
    let crc = crc::Crc::<u32>::new(&crc::CRC_32_BZIP2);
    let sum = crc.checksum(text.as_bytes());
    let bytes = sum.to_ne_bytes();
    (
        u16::from(bytes[0]) + 64,
        u16::from(bytes[1]) + 64,
        u16::from(bytes[2]) + 64
    )
}

impl From<&Directory> for Vec<Song> {
    fn from(directory: &Directory) -> Vec<Song> {
        directory.0.values()
            .map(|entries| Song {
                versions: (*entries).clone(),
                year: entries.iter()
                    .map(|i| i.year)
                    .max()
                    .unwrap(),
                edition: entries[0].edition
                    .iter()
                    .filter(|i| !i.starts_with('v'))
                    .cloned()
                    .collect(),
                artist: entries[0].artist.clone(),
                track: entries[0].track.clone(),
                original: entries[0].original,
                ai: entries[0].ai,
            })
            .collect()
    }
}

impl Song {
    fn html(&self, document: &Document, id: usize) -> Element {
        let element = document.create_element("a").unwrap();

        element.set_id(&format!("js-data-list-item-{id}"));
        element.class_list().add_3("fella-list-item", "fella-list-link", "fella-list-item-padded")
            .unwrap();

        if !self.ai && !self.original {
            let artist = document.create_element("span").unwrap();
            artist.class_list().add_1("fella-footnotes").unwrap();
            artist.set_text_content(Some(&format!("{} - ", self.artist)));
            element.append_with_node_1(&artist).unwrap();
        }

        let track = document.create_element("span").unwrap();
        track.set_text_content(Some(&self.track));
        element.append_with_node_1(&track).unwrap();

        for ed in &self.edition {
            let edition = document.create_element("span").unwrap();
            edition.class_list().add_1("fella-badge-notice").unwrap();
            edition.set_text_content(Some(ed));
            let hash = hash_text_color(ed);
            edition.set_attribute("style",
                &format!("--fella-badge-notice-rgb: {},{},{} !important;", hash.0, hash.1, hash.2)
            ).unwrap();
            element.append_with_node_1(&edition).unwrap();
        }

        if self.ai {
            let badge = document.create_element("span").unwrap();
            badge.class_list().add_1("fella-badge-notice").unwrap();
            badge.set_text_content(Some("AI generated"));
            badge.set_attribute("style", "--fella-badge-notice-rgb: 255,161,212 !important;").unwrap();
            element.append_with_node_1(&badge).unwrap();
        }

        if self.original && !self.ai {
            let badge = document.create_element("span").unwrap();
            badge.class_list().add_1("fella-badge-notice").unwrap();
            badge.set_text_content(Some("Original"));
            badge.set_attribute("style", "--fella-badge-notice-rgb: 255,132,146 !important;").unwrap();
            element.append_with_node_1(&badge).unwrap();
        }

        if self.versions.len() > 1 {
            let versions = document.create_element("span").unwrap();
            versions.class_list().add_1("fella-badge-notice").unwrap();
            versions.set_text_content(Some(&format!("{} versions", self.versions.len())));
            let hash = hash_text_color(&self.versions.len().to_string());
            versions.set_attribute("style",
                &format!("--fella-badge-notice-rgb: {},{},{} !important;", hash.0, hash.1, hash.2)
            ).unwrap();
            element.append_with_node_1(&versions).unwrap();
        }

        element
    }
}