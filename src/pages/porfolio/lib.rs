use dioxus::prelude::*;

#[component]
pub fn Porfolio() -> Element {
    rsx!(
        section { class: "w-full",
            h1 { class: "text-4xl text-gray-700 dark:text-gray-200 mx-3 ", "Potfolio" }
            hr { class: "text-gray-700" }
            div { class: "grid- " }
        }
    )
}
