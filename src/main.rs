use dioxus::{document, prelude::*};
use global::{
    components::molecules::{Blur, Loading},
    utils::scripts::init_scripts_for_app,
};
use routes::Route;

mod global;
mod layout;
mod pages;
mod routes;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    init_scripts_for_app();
    rsx! {
        document::Link { fetchpriority: "high", rel: "icon", href: FAVICON }

        document::Link { fetchpriority: "high", rel: "stylesheet", href: TAILWIND_CSS }
        Loading {}
        Router::<Route> {}
        Blur {}
    }
}
