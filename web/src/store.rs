use crate::library::ColorMode;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct GlobalStore {
    pub locale: String,
    pub color_mode: Option<ColorMode>,
}

impl Default for GlobalStore {
    fn default() -> Self {
        GlobalStore {
            locale: String::from("en"),
            color_mode: None,
        }
    }
}

pub fn set_locale(locale: String, dispatch: Dispatch<GlobalStore>) {
    dispatch.reduce_mut(move |store| {
        store.locale = locale;
    })
}

pub fn set_color_mode(color_mode: Option<ColorMode>, dispatch: Dispatch<GlobalStore>) {
    dispatch.reduce_mut(move |store| {
        store.color_mode = color_mode;
    })
}
