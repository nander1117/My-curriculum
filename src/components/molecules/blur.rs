use dioxus::prelude::*;
const BLUR_BACKGROUND: Asset = asset!("/assets/background/blur-hard.webp");

#[component]
pub fn Blur() -> Element {
    rsx!(
        div {
            style: format!("background-image : url({});", BLUR_BACKGROUND),
            class: "size-full fixed bg-[length:130px]! left-0 pointer-events-none top-0  opacity-[0.05] bg-blend-multiply bg-repeat",
        }
    )
}
