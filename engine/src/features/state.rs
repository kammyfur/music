use crate::models::state::State;

static mut APPLICATION_STATE: Option<State> = None;

#[allow(static_mut_refs, clippy::module_name_repetitions)]
pub fn get_state<'a>() -> &'a State {
    unsafe {
        APPLICATION_STATE.as_ref().unwrap()
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn set_state(state: State) {
    unsafe {
        APPLICATION_STATE = Some(state);
    }
}