use crate::global::components::pages::Hero;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
