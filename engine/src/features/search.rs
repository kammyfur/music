use crate::features::state::get_state;
use crate::features::listing::populate_list;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::models::song::Song;
use crate::utils::eval;

fn get_search_results(query: &str) -> Vec<(usize, Song)> {
    let state = get_state();
    let query = query.to_lowercase();
    state.songs.clone().into_iter()
        .enumerate()
        .filter(|x| x.1.track.to_lowercase().contains(&query) || x.1.artist.to_lowercase().contains(&query))
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