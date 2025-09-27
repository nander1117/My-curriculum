use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsExclamationCircle;
use dioxus_free_icons::Icon;

#[component]
pub fn ButtonIcon() -> Element {
    rsx!(
        button {
            class: "inline-flex items-center justify-center rounded-xl p-3 text-gray-400 transition-colors duration-300 hover:bg-primary-500/10 hover:text-primary-500",
            r#type: "button",
            span { class: "sr-only", "About this website" }
        }
    )
}
#[component]
fn icon() -> Element {
    rsx!(Icon {
        width: 30,
        height: 30,
        fill: "black",
        icon: BsExclamationCircle,
    })
}
