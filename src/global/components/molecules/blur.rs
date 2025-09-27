use dioxus::prelude::*;
const BLUR_BACKGROUND: Asset = asset!("/assets/images/background/blur-hard.webp");

/// * ### Blur Component
///  A component that adds a blur effect to the background
#[component]
pub fn Blur() -> Element {
    rsx!(
        div {
            style: format!("background-image : url({});", BLUR_BACKGROUND),
            class: "size-full fixed bg-[length:30px]! left-0 pointer-events-none top-0  opacity-[0.05] bg-blend-multiply bg-repeat",
        }
    )
}
