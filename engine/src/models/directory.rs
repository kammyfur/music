use crate::features::state::get_state;
use crate::utils::hash_text_color;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use web_sys::HtmlElement;
use crate::models::song::Song;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct DirectoryEntry {
    pub id: String,
    #[serde(alias = "cdnId")]
    pub cdn_id: String,
    pub file: String,
    pub edition: Vec<String>,
    pub year: u32,
    pub ts: u64,
    pub date: Option<String>,
    pub artist: String,
    pub track: String,
    pub original: bool,
    pub ai: bool
}

#[derive(Debug, Clone)]
pub struct Directory(pub HashMap<String, Vec<DirectoryEntry>>);

impl DirectoryEntry {
    pub fn html(&self, id: usize) -> HtmlElement {
        let state = get_state();
        let document = &state.document;
        let element = document.create_element("a").unwrap();

        element.class_list().add_2("list-group-item", "list-group-item-action")
            .unwrap();
        element.set_id(&format!("versions-item-{id}"));

        let year = document.create_element("span").unwrap();
        let mut text = &self.year.to_string();

        if let Some(date) = &self.date {
            text = date;
        }

        year.set_text_content(Some(text));
        year.class_list().add_1("badge").unwrap();
        let hash = hash_text_color(text);
        year.set_attribute("style",
                           &format!("background-color: rgba({},{},{},.5) !important;", hash.0, hash.1, hash.2)
        ).unwrap();
        element.append_with_node_1(&year).unwrap();

        let track = document.create_element("span").unwrap();
        track.set_text_content(Some(&self.track));
        element.append_with_node_1(&track).unwrap();

        for ed in &self.edition {
            let edition = document.create_element("span").unwrap();
            edition.class_list().add_1("badge").unwrap();
            edition.set_text_content(Some(ed));
            let hash = hash_text_color(ed);
            edition.set_attribute("style",
                                  &format!("background-color: rgba({},{},{},.5) !important;", hash.0, hash.1, hash.2)
            ).unwrap();
            element.append_with_node_1(&edition).unwrap();
        }

        let element: HtmlElement = element.dyn_into().unwrap();
        element
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
                date: entries.iter()
                    .map(|i| i.date.clone())
                    .max()
                    .unwrap(),
                ts: entries.iter()
                    .map(|i| i.ts)
                    .max()
                    .unwrap(),
            })
            .collect()
    }
}