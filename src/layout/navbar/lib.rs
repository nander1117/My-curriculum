use crate::{global::components::atomos::ToggleTheme, Route};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! (
        nav { class: "relative bg-white shadow  dark:bg-gray-950 animation-navbar border-b border-gray-200 dark:border-gray-700",
            div { class: "container px-6 py-4 mx-auto md:flex md:justify-between md:items-center",
                div { class: "flex items-center justify-between",
                    Logo {}
                    div { class: "flex ", OCSidebarMobileButton {} }
                }
                div { class: "*:btn *:btn-ghost | hidden md:flex gap-6 *:my-2 *:text-gray-700 *:transition-colors *:duration-300 *:transform *:dark:text-gray-200 *:hover:text-blue-500 *:dark:hover:text-blue-400  *:md:my-0",

                    ButtonHome {}
                    ButtonBlog {}
                    a { href: "#", "Contact" }
                    a { href: "#", "About" }
                }
                div { class: "flex gap-2 items-center",
                    // ToggleTheme {}
                    OCSidebarButton {}
                }
            }
        }
    )
}
#[component]
fn ButtonHome() -> Element {
    rsx!(
        Link { to: Route::Home {}, "Home" }
    )
}
#[component]
fn ButtonBlog() -> Element {
    rsx!(
        Link { to: Route::Blog { id: 1 }, "Blog" }
    )
}

/// * ### this button is responsible for opening and closing the sidebar-desktop
/// It uses the `swap` class from daisyui to toggle between two states
///
/// OC stands for `Open` `Close`
///
/// the `|` pipeline in class means that on one side I use DaisyUI and on the other TailwinCSS
///
/// I use input type radio to be able to close the sidebar when
/// clicking the input within himself = ` OCSidebarButton`
///
/// label is used instead of button to be able to click on it and toggle the input
///
/// label is associated with the input inside of sidebar-desktop with `for="close-sidebar-desktop"`
#[component]
fn OCSidebarButton() -> Element {
    rsx!(
        label {
            class: " swap swap-rotate btn btn-ghost btn-circle  | relative size-8 *:size-6  md:grid hidden transition-colors duration-300 hover:text-blue-500 dark:hover:text-blue-400 ",
            r#for: "close-sidebar-desktop",

            input {
                r#type: "radio",
                class: "size-8! checked:hidden absolute z-10 cursor-pointer ",
                id: "open-sidebar-desktop",
                name: "sidebar-desktop",
                checked: "true",
            }
            svg {
                class: "swap-on | fill-current  ml-0.5",
                "aria-label": "close menu",
                fill: "none",
                view_box: "0 0 15 15",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    clip_rule: "evenodd",
                    d: "M3.49949 14.9C3.7204 14.9 3.89949 14.7209 3.89949 14.5L3.89949 10.4657L5.21664 11.7829C5.37285 11.9391 5.62612 11.9391 5.78233 11.7829C5.93854 11.6267 5.93854 11.3734 5.78233 11.2172L3.78233 9.21718C3.70732 9.14217 3.60557 9.10002 3.49949 9.10002C3.3934 9.10002 3.29166 9.14217 3.21664 9.21718L1.21664 11.2172C1.06043 11.3734 1.06043 11.6267 1.21664 11.7829C1.37285 11.9391 1.62612 11.9391 1.78233 11.7829L3.09949 10.4657L3.09949 14.5C3.09949 14.7209 3.27857 14.9 3.49949 14.9ZM7.99998 10.5C7.99998 10.7762 8.22383 11 8.49998 11H14.5C14.7761 11 15 10.7762 15 10.5C15 10.2239 14.7761 10 14.5 10H8.49998C8.22383 10 7.99998 10.2239 7.99998 10.5ZM7.99998 7.50002C7.99998 7.77617 8.22383 8.00002 8.49998 8.00002H14.5C14.7761 8.00002 15 7.77617 15 7.50002C15 7.22388 14.7761 7.00002 14.5 7.00002H8.49998C8.22383 7.00002 7.99998 7.22388 7.99998 7.50002ZM8.49998 5.00002C8.22383 5.00002 7.99998 4.77617 7.99998 4.50002C7.99998 4.22388 8.22383 4.00002 8.49998 4.00002H14.5C14.7761 4.00002 15 4.22388 15 4.50002C15 4.77617 14.7761 5.00002 14.5 5.00002H8.49998ZM3.89949 0.500025C3.89949 0.279111 3.7204 0.100025 3.49949 0.100025C3.27857 0.100025 3.09949 0.279111 3.09949 0.500025L3.09949 4.53434L1.78233 3.21718C1.62612 3.06097 1.37285 3.06097 1.21664 3.21718C1.06043 3.37339 1.06043 3.62666 1.21664 3.78287L3.21664 5.78287C3.29166 5.85788 3.3934 5.90002 3.49949 5.90002C3.60557 5.90002 3.70732 5.85788 3.78233 5.78287L5.78233 3.78287C5.93854 3.62666 5.93854 3.37339 5.78233 3.21718C5.62612 3.06097 5.37285 3.06097 5.21664 3.21718L3.89949 4.53434L3.89949 0.500025Z",
                    fill: "currentColor",
                    fill_rule: "evenodd",
                }
            }
            svg {
                class: "swap-off | fill-current ml-[1px]",
                "aria-label": "menu",
                view_box: "0 0 512 512",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z" }
            }
        }
    )
}

#[component]
fn Logo() -> Element {
    rsx!(
        Link { to: Route::Home {}, aria_label: "home",
            img {
                alt: "logo",
                class: "w-auto h-6 sm:h-7",
                src: "https://merakiui.com/images/full-logo.svg",
            }
        }
    )
}
/// * ### this button is responsible for opening and closing the sidebar-mobile
/// It uses the `swap` class from daisyui to toggle between two states
///
/// OC stands for `Open` `Close`
///
/// the `|` pipeline in class means that on one side I use DaisyUI and on the other TailwinCSS
///
/// I use input type radio to be able to close the sidebar when clicking the backdrop of sidebar-mobile
///
/// I added `name="sidebar-mobile"` to the inputs to group them together and make them work as expected
///
/// label is used instead of button to be able to click on it and toggle the input
///
/// label is associated with the input inside of sidebar-mobile with `for="open-sidebar-mobile"`
#[component]
fn OCSidebarMobileButton() -> Element {
    rsx!(
        label {
            class: " swap swap-rotate btn btn-ghost btn-circle | size-6 *:size-5 md:hidden scale-150",
            r#for: "open-sidebar-mobile",

            input {
                r#type: "radio",
                id: "close-sidebar-mobile",
                name: "sidebar-mobile",
                checked: "true",
            }
            svg {
                class: "swap-off | fill-current ",
                "aria-label": "close menu",
                fill: "none",
                view_box: "0 0 15 15",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    clip_rule: "evenodd",
                    d: "M3.49949 14.9C3.7204 14.9 3.89949 14.7209 3.89949 14.5L3.89949 10.4657L5.21664 11.7829C5.37285 11.9391 5.62612 11.9391 5.78233 11.7829C5.93854 11.6267 5.93854 11.3734 5.78233 11.2172L3.78233 9.21718C3.70732 9.14217 3.60557 9.10002 3.49949 9.10002C3.3934 9.10002 3.29166 9.14217 3.21664 9.21718L1.21664 11.2172C1.06043 11.3734 1.06043 11.6267 1.21664 11.7829C1.37285 11.9391 1.62612 11.9391 1.78233 11.7829L3.09949 10.4657L3.09949 14.5C3.09949 14.7209 3.27857 14.9 3.49949 14.9ZM7.99998 10.5C7.99998 10.7762 8.22383 11 8.49998 11H14.5C14.7761 11 15 10.7762 15 10.5C15 10.2239 14.7761 10 14.5 10H8.49998C8.22383 10 7.99998 10.2239 7.99998 10.5ZM7.99998 7.50002C7.99998 7.77617 8.22383 8.00002 8.49998 8.00002H14.5C14.7761 8.00002 15 7.77617 15 7.50002C15 7.22388 14.7761 7.00002 14.5 7.00002H8.49998C8.22383 7.00002 7.99998 7.22388 7.99998 7.50002ZM8.49998 5.00002C8.22383 5.00002 7.99998 4.77617 7.99998 4.50002C7.99998 4.22388 8.22383 4.00002 8.49998 4.00002H14.5C14.7761 4.00002 15 4.22388 15 4.50002C15 4.77617 14.7761 5.00002 14.5 5.00002H8.49998ZM3.89949 0.500025C3.89949 0.279111 3.7204 0.100025 3.49949 0.100025C3.27857 0.100025 3.09949 0.279111 3.09949 0.500025L3.09949 4.53434L1.78233 3.21718C1.62612 3.06097 1.37285 3.06097 1.21664 3.21718C1.06043 3.37339 1.06043 3.62666 1.21664 3.78287L3.21664 5.78287C3.29166 5.85788 3.3934 5.90002 3.49949 5.90002C3.60557 5.90002 3.70732 5.85788 3.78233 5.78287L5.78233 3.78287C5.93854 3.62666 5.93854 3.37339 5.78233 3.21718C5.62612 3.06097 5.37285 3.06097 5.21664 3.21718L3.89949 4.53434L3.89949 0.500025Z",
                    fill: "currentColor",
                    fill_rule: "evenodd",
                }
            }
            svg {
                class: "swap-on | fill-current ",
                "aria-label": "menu",
                view_box: "0 0 512 512",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z" }
            }
        }
    )
}
