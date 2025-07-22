use crate::utils::hide_modal;
use crate::utils::show_modal;
use crate::features::state::get_state;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;
use crate::features::modal::modal_hide;
use crate::utils::{eval, initialize_dash};

pub fn register_clicks(base: &str) {
    let state = get_state();
    for index in 0..state.songs.len() {
        let id = &format!("{base}{index}");
        if let Some(el) = state.document.get_element_by_id(id) {
            let real_id = el.get_attribute("data-real-id").unwrap();
            eval(&format!("document.getElementById('{}').addEventListener('click', () => {{ wasm.select_song({real_id}); }});", el.id()));
        }
    }
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
                    .find(|v| v.1.id == parts[0] && v.0.to_string() == parts[1]).map(|e| (s, e.0, e.1))
            })
            .find(Option::is_some);

        if let Some(Some((song, _, version))) = version {
            let mut title = version.track.clone();
            if !version.edition.is_empty() {
                title.push_str(&format!(" ({})", version.edition.join(", ")));
            }
            state.player.title.set_text_content(Some(&title));
            state.document.set_title(&title);

            if let Some(date) = &version.date {
                state.player.date.set_text_content(Some(date));
            } else {
                state.player.date.set_text_content(Some(&version.year.to_string()));
            }

            state.player.quality.set_text_content(Some(&format!("{}-bit {} kHz", version.quality.0, version.quality.1 / 1000)));

            if version.high_res {
                state.player.hires_audio.set_class_name("is_hires");
            } else {
                state.player.hires_audio.set_class_name("");
            }

            if song.original {
                state.player.author.set_text_content(Some(&version.artist));
            } else {
                state.player.author.set_text_content(Some(&format!("{} (Covery by Kammy)", version.artist)));
            }

            initialize_dash(&format!("https://cdn.music.leafia.eu/{}/stream_dash.mpd", version.cdn_id));
            let _ = state.player.audio.play().unwrap();
            hide_modal("version");
            show_modal("player");
            state.player.modal.clone().dyn_into::<HtmlElement>().unwrap().focus().unwrap();
        } else {
            state.location.set_hash("").unwrap();
        }
    }
}