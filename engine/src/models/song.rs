use crate::features::state::get_state;
use crate::utils::hash_text_color;
use web_sys::HtmlElement;
use crate::models::directory::DirectoryEntry;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Song {
    pub versions: Vec<DirectoryEntry>,
    pub year: u32,
    pub ts: u64,
    pub date: Option<String>,
    pub edition: Vec<String>,
    pub artist: String,
    pub track: String,
    pub original: bool
}

impl Song {
    pub fn html(&self, id: usize, real_id: usize, prefix: &str, show_year: bool) -> HtmlElement {
        let state = get_state();
        let document = &state.document;
        let element = document.create_element("a").unwrap();
        element.set_attribute("style", "display: grid; grid-template-columns: 48px 1fr max-content;").unwrap();

        let year = document.create_element("div").unwrap();
        let content = document.create_element("div").unwrap();
        let badges = document.create_element("div").unwrap();

        if show_year {
            let year_span = document.create_element("span").unwrap();
            //year_span.class_list().add_1("fella-footnotes").unwrap();
            year_span.set_text_content(Some(&self.year.to_string()));
            year.append_with_node_1(&year_span).unwrap();
        }

        element.set_attribute("data-real-id", &real_id.to_string()).unwrap();
        element.set_id(&format!("{prefix}-item-{id}"));
        element.class_list().add_2("list-group-item", "list-group-item-action")
            .unwrap();

        if !self.original {
            let artist = document.create_element("span").unwrap();
            //artist.class_list().add_1("fella-footnotes").unwrap();
            artist.set_text_content(Some(&format!("{} – ", self.artist)));
            content.append_with_node_1(&artist).unwrap();
        }

        let track = document.create_element("span").unwrap();
        track.set_text_content(Some(&self.track));
        content.append_with_node_1(&track).unwrap();

        for ed in &self.edition {
            let edition = document.create_element("span").unwrap();
            edition.class_list().add_1("badge").unwrap();
            edition.set_text_content(Some(ed));
            let hash = hash_text_color(ed);
            edition.set_attribute("style",
                                  &format!("background-color: rgba({},{},{},.5) !important;", hash.0, hash.1, hash.2)
            ).unwrap();
            badges.append_with_node_1(&edition).unwrap();
        }

        if self.original {
            let badge = document.create_element("span").unwrap();
            badge.class_list().add_1("badge").unwrap();
            badge.set_text_content(Some("Original"));
            badge.set_attribute("style", "background-color: rgba(255,132,146,.5) !important;").unwrap();
            badges.append_with_node_1(&badge).unwrap();
        } else {
            let badge = document.create_element("span").unwrap();
            badge.class_list().add_1("badge").unwrap();
            badge.set_text_content(Some("Cover"));
            badge.set_attribute("style", "background-color: rgba(133,255,241,.5) !important;").unwrap();
            badges.append_with_node_1(&badge).unwrap();
        }

        if self.versions.len() > 1 {
            let versions = document.create_element("span").unwrap();
            versions.class_list().add_1("badge").unwrap();
            versions.set_text_content(Some(&format!("{} versions", self.versions.len())));
            let hash = hash_text_color(&self.versions.len().to_string());
            versions.set_attribute("style",
                                   &format!("background-color: rgba({},{},{},.5) !important;", hash.0, hash.1, hash.2)
            ).unwrap();
            badges.append_with_node_1(&versions).unwrap();
        }

        element.append_with_node_3(&year, &content, &badges).unwrap();

        let element: HtmlElement = element.dyn_into().unwrap();
        element
    }
}