use dioxus::prelude::*;
const BLUR_BACKGROUND: Asset = asset!("/assets/images/background/blur-hard.webp");

/// * ### Blur Component
///  A component that adds a blur effect to the background
#[component]
pub fn Blur() -> Element {
    rsx!(
        div {
            style: format!("background-image : url({});", BLUR_BACKGROUND),
            class: "size-full fixed bg-[length:30px]! left-0 pointer-events-none top-0  opacity-[0.05] bg-blend-multiply bg-repeat z-20",
        }
        div { class: "hidden dark:block absolute inset-0 opacity-20 pointer-events-none z-20",
            div { class: "absolute top-20 left-10 w-40 h-40 rounded-full bg-purple-600 blur-3xl" }
            div { class: "absolute bottom-20 right-10 w-60 h-60 bg-cyan-600 blur-3xl" }
        }
    )
}
