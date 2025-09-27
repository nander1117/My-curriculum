use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! (
        nav { class: "relative bg-white shadow dark:bg-gray-800 animation-navbar",
            div { class: "container px-6 py-4 mx-auto md:flex md:justify-between md:items-center",
                div { class: "flex items-center justify-between",
                    a { href: "#", aria_label: "home",
                        img {
                            alt: "",
                            class: "w-auto h-6 sm:h-7",
                            src: "https://merakiui.com/images/full-logo.svg",
                        }
                        ""
                    }
                    div { class: "flex ",
                        label {
                            class: "btn btn-circle swap swap-rotate *:size-4 size-5",
                            r#for: "open-sidebar",

                            input {
                                r#type: "radio",
                                class: "peer",
                                id: "close-sidebar",
                                name: "sidebar",
                                checked: "true",
                            }
                            svg {
                                class: "swap-on fill-current ",
                                "aria-label": "sun",
                                view_box: "0 0 512 512",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z" }
                            }
                            svg {
                                class: "swap-off fill-current",
                                view_box: "0 0 512 512",
                                xmlns: "http://www.w3.org/2000/svg",
                                polygon { points: "400 145.49 366.51 112 256 222.51 145.49 112 112 145.49 222.51 256 112 366.51 145.49 400 256 289.49 366.51 400 400 366.51 289.49 256 400 145.49" }
                            }
                        }
                        label {
                            class: "btn btn-circle swap swap-rotate *:size-4 size-5",
                            r#for: "close-sidebar-desktop",

                            input {
                                r#type: "radio",
                                class: "peer",
                                id: "open-sidebar-desktop",
                                name: "sidebar-desktop",
                                checked: "true",
                            }
                            svg {
                                class: "swap-on fill-current ",
                                "aria-label": "sun",
                                view_box: "0 0 512 512",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z" }
                            }
                            svg {
                                class: "swap-off fill-current",
                                view_box: "0 0 512 512",
                                xmlns: "http://www.w3.org/2000/svg",
                                polygon { points: "400 145.49 366.51 112 256 222.51 145.49 112 112 145.49 222.51 256 112 366.51 145.49 400 256 289.49 366.51 400 400 366.51 289.49 256 400 145.49" }
                            }
                        }
                    }
                }
                div { class: "flex flex-col md:flex-row md:mx-6",
                    Link {
                        to: Route::Home {},
                        class: "my-2 text-gray-700 transition-colors duration-300 transform dark:text-gray-200 hover:text-blue-500 dark:hover:text-blue-400 md:mx-4 md:my-0",
                        "Home"
                    }

                    Link {
                        to: Route::Blog { id: 1 },
                        class: "my-2 text-gray-700 transition-colors duration-300 transform dark:text-gray-200 hover:text-blue-500 dark:hover:text-blue-400 md:mx-4 md:my-0",
                        "Blog"
                    }
                    a {
                        class: "my-2 text-gray-700 transition-colors duration-300 transform dark:text-gray-200 hover:text-blue-500 dark:hover:text-blue-400 md:mx-4 md:my-0",
                        href: "#",
                        "Contact"
                    }
                    a {
                        class: "my-2 text-gray-700 transition-colors duration-300 transform dark:text-gray-200 hover:text-blue-500 dark:hover:text-blue-400 md:mx-4 md:my-0",
                        href: "#",
                        "About"
                    }
                }
                div { class: "flex justify-center md:block " }
            }

        }
    )
}
