use crate::layout::{Navbar, Sidebar, SidebarMobile};
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! (
        SidebarMobile {}
        Navbar {}
        div { class: "flex overflow-hidden max-h-[calc(100dvh-60px)] ",
            Sidebar {}
            main { class: " p-4 overflow-y-auto bg-gray-100 dark:bg-gray-800 w-full motion animation-section",
                div { class: "w-full overflow-hidden shadow rounded-2xl", Outlet::<Route> {} }
            }
        }
    )
}
