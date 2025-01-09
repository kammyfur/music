use crate::features::state::get_state;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[allow(clippy::module_name_repetitions)]
pub fn modal_hide() {
    let state = get_state();
    state.document.set_title(&state.old_title);
    let _ = state.player.audio.pause();
    state.player.modal.class_list().remove_1("show").unwrap();
}

#[wasm_bindgen]
pub fn version_hide() {
    let state = get_state();
    state.version.modal.class_list().remove_1("show").unwrap();
}