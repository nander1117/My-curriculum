use dioxus::{document, prelude::*};
use global::{components::molecules::Blur, utils::scripts::init_scripts_for_app};
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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // deleted al agregar fullstack
        script {
            src: "https://unpkg.com/tailwindcss-intersect@2.x.x/dist/observer.min.js",
            defer: "true",
        }
        Router::<Route> {}
        Blur {}
    }
}
