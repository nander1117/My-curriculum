use body::{Blog, Home};
use dioxus::prelude::*;
use navbar::Navbar;
mod body;
mod components;
mod navbar;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
}
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const BLUR_BACKGROUND: Asset = asset!("/assets/background/blur-hard.webp");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    come_back_please();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
        Blur {}
    }
}
fn come_back_please() {
    document::eval(
        r#"
            document.addEventListener("visibilitychange", () => {
                if (document.visibilityState === "hidden") {
                    document.title = "Regresa please!";
                } else {
                    document.title = "My Curriculum";
                }
            });
        "#,
    );
}
#[component]
fn Blur() -> Element {
    rsx!(
        div {
            style: format!("background-image : url({});", BLUR_BACKGROUND),
            class: "size-full fixed bg-[length:130px]! left-0 pointer-events-none top-0  opacity-[0.05] bg-blend-multiply bg-repeat",
        }
    )
}
