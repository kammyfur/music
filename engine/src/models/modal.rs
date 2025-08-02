use web_sys::{Element, HtmlAudioElement};

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct VersionModal {
    pub modal: Element,
    pub list: Element,
    pub title: Element
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct PlayerModal {
    pub modal: Element,
    pub audio: HtmlAudioElement,
    pub title: Element,
    pub date: Element,
    pub author: Element
}