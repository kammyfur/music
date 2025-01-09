use crate::models::modal::VersionModal;
use crate::models::modal::PlayerModal;
use crate::models::song::Song;
use web_sys::{Document, HtmlInputElement, Location};

#[derive(Debug, Clone)]
pub struct State {
    pub location: Location,
    pub document: Document,
    pub songs: Vec<Song>,
    pub old_title: String,
    pub version: VersionModal,
    pub player: PlayerModal,
    pub search: HtmlInputElement
}