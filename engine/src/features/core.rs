use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use crate::features::directory::get_directory;
use crate::features::listing::populate_list;
use crate::features::state::set_state;
use crate::models::directory::Directory;
use crate::models::modal::{PlayerModal, VersionModal};
use crate::models::song::Song;
use crate::models::state::State;
use crate::process_hash;
use crate::utils::{eval, fella_complete_load};

pub async fn load() {
    let window = web_sys::window().expect("No global `window` exists");
    let document = window.document().expect("Should have a document on window");

    let directory: Directory = get_directory().await;
    document.get_element_by_id("count")
        .unwrap()
        .set_text_content(Some(&format!("{} productions", directory.0.len())));

    let songs: Vec<Song> = (&directory).into();

    set_state(State {
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
            title: document.get_element_by_id("player-title").unwrap(),
            date: document.get_element_by_id("player-date").unwrap()
        },
        search: document.get_element_by_id("search").unwrap().dyn_into().unwrap()
    });

    let songs_enumerated: Vec<(usize, Song)> = songs.into_iter().enumerate().collect();
    populate_list(&songs_enumerated, "js-data-list", false);

    document.get_element_by_id("search")
        .unwrap().dyn_into::<HtmlInputElement>()
        .unwrap().set_value("");
    document.get_element_by_id("search")
        .unwrap().dyn_into::<HtmlElement>()
        .unwrap().focus().unwrap();

    process_hash();
    eval("window.addEventListener('hashchange', () => wasm.process_hash());");
    eval("document.getElementById('player-modal-close').addEventListener('click', () => { wasm.modal_hide(); location.hash = ''; });");
    eval("document.getElementById('app').style.display = '';");
    fella_complete_load();
}