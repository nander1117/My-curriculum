use crate::layout::Layout;
use crate::pages::{blog::Blog, not_found::NotFound, porfolio::Porfolio};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Porfolio {},
        #[route("/blog/:id")]
        Blog { id: i32 },
    #[end_layout]
    #[route("/:..route")]
    NotFound { route:Vec<String> },
}
