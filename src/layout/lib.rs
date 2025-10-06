use crate::layout::{DockBarMobile, Navbar, Sidebar, SidebarMobile};
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! (
        SidebarMobile {}
        Navbar {}
        div { class: "flex overflow-hidden md:max-h-[calc(100dvh-72.60px)] sm:max-h-[calc(100dvh-61px-4rem)] max-h-[calc(100dvh-56px-4rem)] h-full",
            Sidebar {}
            main { class: " flex overflow-y-auto bg-white dark:bg-gray-950 w-full motion animation-section ",
                div { class: "flex w-full h-fit min-h-full",
                    Block {}
                    Outlet::<Route> {}
                    Block {}
                }
            }
        }
        DockBarMobile {}
    )
}

#[component]
fn Block() -> Element {
    rsx!(
        div {
            id: "Blok",
            class: " w-7 grow border-x border-x-(--pattern-fg) bg-[image:repeating-linear-gradient(315deg,_var(--pattern-fg)_0,_var(--pattern-fg)_1px,_transparent_0,_transparent_50%)] bg-[size:10px_10px] bg-fixed [--pattern-fg:var(--color-gray-200)] md:block dark:[--pattern-fg:var(--color-gray-700)] md:min-h-[calc(100dvh-72.60px)] sm:min-h-[calc(100dvh-61px-4rem)] min-h-[calc(100dvh-56px-4rem)]",
        }
    )
}
