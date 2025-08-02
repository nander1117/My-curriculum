use components::Blur;
use dioxus::prelude::*;
use routes::Route;
use utils::scripts::come_back_please;

mod body;
mod components;
mod navbar;
mod routes;
mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    come_back_please();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
        Blur {}
    }
}
