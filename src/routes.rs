use crate::layout::Layout;
use crate::pages::{blog::Blog, home::Home, not_found::NotFound};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
    #[end_layout]
    #[route("/:..route")]
    NotFound { route:Vec<String> },
}
