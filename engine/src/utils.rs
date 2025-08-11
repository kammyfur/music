use wasm_bindgen::prelude::wasm_bindgen;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen]
    pub fn eval(code: &str);

    #[wasm_bindgen]
    pub fn l(message: &str) -> String;

    #[wasm_bindgen]
    pub fn alert(msg: &str);

    #[wasm_bindgen(js_name = completeLoad)]
    pub fn complete_load();

    #[wasm_bindgen(js_name = showModal)]
    pub fn show_modal(modal: &str);

    #[wasm_bindgen(js_name = hideModal)]
    pub fn hide_modal(modal: &str);

    #[wasm_bindgen(js_name = navigate)]
    pub fn navigate(page: &str);

    #[wasm_bindgen(js_name = changeURL)]
    pub fn change_url(url: &str, replace: bool);
}

pub fn hash_text_color(text: &str) -> (u16, u16, u16) {
    let crc = crc::Crc::<u32>::new(&crc::CRC_32_BZIP2);
    let sum = crc.checksum(text.as_bytes());
    let bytes = sum.to_ne_bytes();
    (
        u16::from(bytes[0]) + 64,
        u16::from(bytes[1]) + 64,
        u16::from(bytes[2]) + 64
    )
}