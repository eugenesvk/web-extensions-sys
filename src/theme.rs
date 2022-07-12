use crate::EventTarget;
use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ThemeImages;

    #[wasm_bindgen(method, getter)]
    pub fn theme_frame(this: &ThemeImages) -> Option<String>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ThemeColors;

    #[wasm_bindgen(method, getter)]
    pub fn button_background_active(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn button_background_hover(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn icons(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn icons_attention(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn frame(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn frame_inactive(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn ntp_background(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn ntp_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn popup(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn popup_border(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn popup_highlight(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn popup_highlight_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn popup_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_border(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_highlight(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_highlight_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn tab_background_separator(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn tab_background_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn tab_line(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn tab_loading(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn tab_selected(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn tab_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_bottom_separator(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_border(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_border_focus(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_focus(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_highlight(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_highlight_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_separator(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_text_focus(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_text(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_top_separator(this: &ThemeColors) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_vertical_separator(this: &ThemeColors) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ThemeProperties;

    #[wasm_bindgen(method, getter)]
    pub fn additional_backgrounds_alignment(this: &ThemeProperties) -> Option<Array>;

    #[wasm_bindgen(method, getter)]
    pub fn additional_backgrounds_tiling(this: &ThemeProperties) -> Option<Array>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type Theme;

    #[wasm_bindgen(method, getter)]
    pub fn images(this: &Theme) -> Option<ThemeImages>;

    #[wasm_bindgen(method, getter)]
    pub fn colors(this: &Theme) -> Option<ThemeColors>;

    #[wasm_bindgen(method, getter)]
    pub fn properties(this: &Theme) -> Option<ThemeProperties>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type ThemeUpdateInfo;

    #[wasm_bindgen(method, getter)]
    pub fn theme(this: &ThemeUpdateInfo) -> Theme;

    #[wasm_bindgen(method, getter, js_name = windowId)]
    pub fn window_id(this: &ThemeUpdateInfo) -> Option<i32>;
}

#[wasm_bindgen]
extern "C" {
    pub type BrowserTheme;

    #[wasm_bindgen(method, js_name = getCurrent)]
    pub fn get_current(this: &BrowserTheme, window_id: Option<i32>) -> Promise;

    #[wasm_bindgen(method)]
    pub fn update(this: &BrowserTheme, window_id: Option<i32>, theme: &Theme);

    #[wasm_bindgen(method)]
    pub fn reset(this: &BrowserTheme, window_id: Option<i32>);

    #[wasm_bindgen(method, getter, js_name = onUpdated)]
    pub fn on_updated(this: &BrowserTheme) -> EventTarget;
}
