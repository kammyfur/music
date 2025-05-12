use crate::utils::show_modal;
use crate::features::state::get_state;
use crate::register_clicks;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::models::directory::DirectoryEntry;
use crate::models::song::Song;
use crate::utils::eval;

pub fn populate_list(list: &[(usize, Song)], id: &str, show_year: bool) {
    let state = get_state();
    let document = &state.document;
    let container = document.get_element_by_id(id).unwrap();
    container.set_inner_html("");

    let mut songs_enumeration = list
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &(usize, Song))>>();

    songs_enumeration.sort_by(|a, b| a.1.1.track.to_lowercase().partial_cmp(&b.1.1.track.to_lowercase()).unwrap());
    songs_enumeration.sort_by(|a, b| a.1.1.artist.to_lowercase().partial_cmp(&b.1.1.artist.to_lowercase()).unwrap());
    songs_enumeration.sort_by(|a, b| b.1.1.year.partial_cmp(&a.1.1.year).unwrap());
    songs_enumeration.sort_by(|a, b| b.1.1.ts.partial_cmp(&a.1.1.ts).unwrap());
    songs_enumeration.sort_by(|a, b| a.1.1.ai.partial_cmp(&b.1.1.ai).unwrap());

    let mut last_year = 0;

    for (eid, (rid, element)) in songs_enumeration {
        let show_current_year = if (!show_year && element.year != last_year) || show_year {
            last_year = element.year;
            true
        } else {
            false
        };
        container.append_child(&element.html(eid, *rid, id, show_current_year)).unwrap();
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
        versions.sort_by(|(_, va), (_, vb)| vb.ts.partial_cmp(&va.ts).unwrap());

        for (id, entry) in versions {
            state.version.list.append_child(&entry.html(id)).unwrap();
        }

        for (index, version) in song.versions.iter().enumerate() {
            eval(&format!("document.getElementById('versions-item-{index}').addEventListener('click', () => {{ location.hash = \"#/{}/{index}\"; }});",
                          version.id));
        }

        show_modal("version");
    }
}