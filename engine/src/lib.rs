mod utils;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Element, HtmlAudioElement, HtmlElement, HtmlInputElement, Location, Request, RequestInit, RequestMode, Response};
use crate::utils::{initialize_dash, set_panic_hook, eval, fella_complete_load};

static mut APPLICATION_STATE: Option<State> = None;

#[derive(Debug, Clone)]
pub struct State {
    location: Location,
    document: Document,
    songs: Vec<Song>,
    old_title: String,
    version: VersionModal,
    player: PlayerModal,
    search: HtmlInputElement
}

#[derive(Debug, Clone)]
pub struct VersionModal {
    modal: Element,
    list: Element,
    title: Element
}

#[derive(Debug, Clone)]
pub struct PlayerModal {
    modal: Element,
    audio: HtmlAudioElement,
    title: Element
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

#[wasm_bindgen]
pub fn modal_hide() {
    let state = get_state();
    state.document.set_title(&state.old_title);
    let _ = state.player.audio.pause();
    state.player.modal.class_list().remove_1("show").unwrap();
}

#[allow(static_mut_refs)]
fn get_state<'a>() -> &'a State {
    unsafe {
        APPLICATION_STATE.as_ref().unwrap()
    }
}

async fn get_directory() -> Directory {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("https://cdn.floo.fi/watercolor/records/directory.json", &opts)
        .unwrap();

    let window = web_sys::window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await
        .unwrap();
    let response: Response = response.dyn_into().unwrap();

    let json = JsFuture::from(response.text().unwrap()).await
        .unwrap().as_string().unwrap();

    Directory(serde_json::from_str(&json).unwrap())
}

#[wasm_bindgen]
pub fn process_hash() {
    let state = get_state();
    let hash = state.location.hash().unwrap();
    modal_hide();
    state.player.modal.class_list().remove_1("show").unwrap();
    state.version.modal.class_list().remove_1("show").unwrap();

    let parts: Vec<&str> = hash.split("#/").collect();

    if parts.len() > 1 {
        let parts: Vec<&str> = parts[1].split('/').collect();
        let version = state.songs.iter()
            .map(|s| {
                s.versions.iter()
                    .enumerate()
                    .find(|v| v.1.id == parts[0] && v.0.to_string() == parts[1])
            })
            .find(Option::is_some);

        if let Some(Some((_, version))) = version {
            let mut title = format!("{} - {}", version.artist, version.track);
            if !version.edition.is_empty() {
                title.push_str(&format!(" ({})", version.edition.join(", ")));
            }
            title.push_str(&format!(" [{}]", version.year));
            state.player.title.set_text_content(Some(&title));
            state.document.set_title(&title);

            initialize_dash(&format!("https://cdn.floo.fi/watercolor/records/{}/stream_dash.mpd", version.cdn_id));
            let _ = state.player.audio.play().unwrap();
            state.player.modal.class_list().add_1("show").unwrap();
            state.player.modal.clone().dyn_into::<HtmlElement>().unwrap().focus().unwrap();
        } else {
            state.location.set_hash("").unwrap();
        }
    }
}

#[wasm_bindgen(start)]
pub async fn start() {
    set_panic_hook();

    let window = web_sys::window().expect("No global `window` exists");
    let document = window.document().expect("Should have a document on window");

    let directory: Directory = get_directory().await;
    document.get_element_by_id("count")
        .unwrap()
        .set_text_content(Some(&format!("{} productions", directory.0.len())));

    let songs: Vec<Song> = (&directory).into();

    unsafe {
        APPLICATION_STATE = Some(State {
            location: window.location(),
            document: document.clone(),
            songs: songs.clone(),
            old_title: document.title(),
            version: VersionModal {
                modal: document.get_element_by_id("versions").unwrap(),
                list: document.get_element_by_id("versions-list").unwrap(),
                title: document.get_element_by_id("versions-title").unwrap()
            },
            player: PlayerModal {
                modal: document.get_element_by_id("player").unwrap(),
                audio: document.get_element_by_id("player-el").unwrap().dyn_into().unwrap(),
                title: document.get_element_by_id("player-title").unwrap()
            },
            search: document.get_element_by_id("search").unwrap().dyn_into().unwrap()
        });
    }

    populate_list(&songs, "js-data-list");

    document.get_element_by_id("search")
        .unwrap().dyn_into::<HtmlInputElement>()
        .unwrap().set_value("");
    document.get_element_by_id("search")
        .unwrap().dyn_into::<HtmlElement>()
        .unwrap().focus().unwrap();

    process_hash();
    eval("window.addEventListener('hashchange', () => wasm.process_hash());");
    eval("document.getElementById('player-modal-close').addEventListener('click', () => { wasm.modal_hide(); location.hash = ''; });");
    eval("document.getElementById('app').style.display = '';"); // TODO: There has to be a better way to do this
    fella_complete_load();
}

fn register_clicks(base: &str) {
    let state = get_state();
    for index in 0..state.songs.len() {
        let id = &format!("{base}{index}");
        if let Some(el) = state.document.get_element_by_id(id) {
            eval(&format!("document.getElementById('{}').addEventListener('click', () => {{ wasm.select_song({index}); }});", el.id()));
        }
    }
}

fn populate_list(list: &[Song], id: &str) {
    let state = get_state();
    let document = &state.document;
    let container = document.get_element_by_id(id).unwrap();
    container.set_inner_html("");

    let mut songs_enumeration = list
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &Song)>>();

    songs_enumeration.sort_by(|a, b| a.1.track.to_lowercase().partial_cmp(&b.1.track.to_lowercase()).unwrap());
    songs_enumeration.sort_by(|a, b| a.1.artist.to_lowercase().partial_cmp(&b.1.artist.to_lowercase()).unwrap());
    songs_enumeration.sort_by(|a, b| b.1.year.partial_cmp(&a.1.year).unwrap());
    songs_enumeration.sort_by(|a, b| a.1.ai.partial_cmp(&b.1.ai).unwrap());

    for (eid, element) in songs_enumeration {
        container.append_child(&element.html(eid, id)).unwrap();
    }

    register_clicks(&format!("{id}-item-"));
}

#[wasm_bindgen]
pub fn select_song(index: usize) {
    let state = get_state();
    let song = &state.songs[index];

    if song.versions.len() < 2 {
        let version = &song.versions[0];
        state.location.set_hash(&format!("#/{}/0", version.id)).unwrap();
    } else {
        let mut title = song.track.clone();
        if !song.edition.is_empty() {
            title.push_str(&format!(" ({})", song.edition.join(", ")));
        }
        state.version.title.set_text_content(Some(&title));
        state.version.list.set_inner_html("");

        let versions = song.versions.clone();
        let mut versions: Vec<(usize, &DirectoryEntry)> = versions
            .iter()
            .enumerate()
            .collect();
        versions.sort_by(|(_, va), (_, vb)| va.file.partial_cmp(&vb.file).unwrap());
        versions.sort_by(|(_, va), (_, vb)| va.edition.len().partial_cmp(&vb.edition.len()).unwrap());
        versions.sort_by(|(_, va), (_, vb)| vb.year.partial_cmp(&va.year).unwrap());

        for (id, entry) in versions {
            state.version.list.append_child(&entry.html(id)).unwrap();
        }

        for (index, version) in song.versions.iter().enumerate() {
            eval(&format!("document.getElementById('versions-item-{index}').addEventListener('click', () => {{ location.hash = \"#/{}/{index}\"; }});",
                version.id));
        }

        state.version.modal.class_list().add_1("show").unwrap();
    }
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

fn get_search_results(query: &str) -> Vec<Song> {
    let state = get_state();
    let query = query.to_lowercase();
    state.songs.clone().into_iter()
        .filter(|x| x.track.to_lowercase().contains(&query) || x.artist.to_lowercase().contains(&query))
        .collect()
}

#[wasm_bindgen]
pub fn search() {
    let state = get_state();
    let query = state.search.value();

    if query.is_empty() {
        eval("document.getElementById('js-data-list').style.display = '';");
        eval("document.getElementById('js-data-results').style.display = 'none';");
    } else {
        let results = get_search_results(&query);
        populate_list(&results, "js-data-results");
        eval("document.getElementById('js-data-list').style.display = 'none';");
        eval("document.getElementById('js-data-results').style.display = '';");
    }
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
    fn html(&self, id: usize, prefix: &str) -> HtmlElement {
        let state = get_state();
        let document = &state.document;
        let element = document.create_element("a").unwrap();

        element.set_id(&format!("{prefix}-item-{id}"));
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

        let element: HtmlElement = element.dyn_into().unwrap();
        element
    }
}

impl DirectoryEntry {
    fn html(&self, id: usize) -> HtmlElement {
        let state = get_state();
        let document = &state.document;
        let element = document.create_element("a").unwrap();

        element.class_list().add_3("fella-list-item", "fella-list-link", "fella-list-item-padded")
            .unwrap();
        element.set_id(&format!("versions-item-{id}"));

        let year = document.create_element("span").unwrap();
        year.set_text_content(Some(&self.year.to_string()));
        year.class_list().add_1("fella-badge-notice").unwrap();
        let hash = hash_text_color(&self.year.to_string());
        year.set_attribute("style",
            &format!("--fella-badge-notice-rgb: {},{},{} !important;", hash.0, hash.1, hash.2)
        ).unwrap();
        element.append_with_node_1(&year).unwrap();

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

        let element: HtmlElement = element.dyn_into().unwrap();
        element
    }
}