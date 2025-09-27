use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {

        div {
            id: "blog",
            class: "w-full p-4 motion-scale-in-[0.5] motion-opacity-in-[20%] motion-blur-in-[20px] motion-ease-in-out",

            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}
