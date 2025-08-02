use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {

        div { id: "blog",

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
