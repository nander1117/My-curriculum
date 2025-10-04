use crate::routes::Route;
use dioxus::prelude::*;

enum Stay {
    Home,
    Blog,
    Settings,
}

#[component]
pub fn DockBarMobile() -> Element {
    let stay = use_signal(|| Stay::Home);
    rsx!(
        div {
            class: "dock | md:hidden border-t border-gray-200 dark:border-gray-700 animation-dock",
            style: "--dock-height:4rem;",

            ButtonHome { stay }
            ButtonBlog { stay }
            button {
                svg {
                    class: "size-[1.2em]",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    g {
                        fill: "currentColor",
                        stroke_linecap: "butt",
                        stroke_linejoin: "miter",
                        circle {
                            cx: "12",
                            cy: "12",
                            fill: "none",
                            r: "3",
                            stroke: "currentColor",
                            stroke_linecap: "square",
                            stroke_miterlimit: "10",
                            stroke_width: "2",
                        }
                        path {
                            d: "m22,13.25v-2.5l-2.318-.966c-.167-.581-.395-1.135-.682-1.654l.954-2.318-1.768-1.768-2.318.954c-.518-.287-1.073-.515-1.654-.682l-.966-2.318h-2.5l-.966,2.318c-.581.167-1.135.395-1.654.682l-2.318-.954-1.768,1.768.954,2.318c-.287.518-.515,1.073-.682,1.654l-2.318.966v2.5l2.318.966c.167.581.395,1.135.682,1.654l-.954,2.318,1.768,1.768,2.318-.954c.518.287,1.073.515,1.654.682l.966,2.318h2.5l.966-2.318c.581-.167,1.135-.395,1.654-.682l2.318.954,1.768-1.768-.954-2.318c.287-.518.515-1.073.682-1.654l2.318-.966Z",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "square",
                            stroke_miterlimit: "10",
                            stroke_width: "2",
                        }
                    }
                }
                span { class: "dock-label", "Settings" }
            }
        }
    )
}

#[component]
fn ButtonHome(stay: Signal<Stay>) -> Element {
    let active = match *stay.read() {
        Stay::Home => "dock-active",
        _ => "",
    };
    rsx!(
        Link {
            onclick: move |_| stay.set(Stay::Home),
            to: Route::Home {},
            class: format!("{active} | my-2 text-gray-700  dark:text-gray-200"),

            svg {
                "data-label": "Home",
                class: "size-[1.2em]",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                g {
                    fill: "currentColor",
                    stroke_linecap: "butt",
                    stroke_linejoin: "miter",
                    polyline {
                        fill: "none",
                        points: "1 11 12 2 23 11",
                        stroke: "currentColor",
                        stroke_miterlimit: "10",
                        stroke_width: "2",
                    }
                    path {
                        d: "m5,13v7c0,1.105.895,2,2,2h10c1.105,0,2-.895,2-2v-7",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "square",
                        stroke_miterlimit: "10",
                        stroke_width: "2",
                    }
                    line {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "square",
                        stroke_miterlimit: "10",
                        stroke_width: "2",
                        x1: "12",
                        x2: "12",
                        y1: "22",
                        y2: "18",
                    }
                }
            }
            span { class: "dock-label", "Home" }
        }
    )
}

#[component]
fn ButtonBlog(stay: Signal<Stay>) -> Element {
    let active = match *stay.read() {
        Stay::Blog => "dock-active",
        _ => "",
    };
    rsx!(
        Link {
            onclick: move |_| stay.set(Stay::Blog),
            to: Route::Blog { id: 1 },
            class: format!("{active} | my-2 text-gray-700  dark:text-gray-200"),
            svg {
                class: "size-[1.2em]",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                g {
                    fill: "currentColor",
                    stroke_linecap: "butt",
                    stroke_linejoin: "miter",
                    polyline {
                        fill: "none",
                        points: "3 14 9 14 9 17 15 17 15 14 21 14",
                        stroke: "currentColor",
                        stroke_miterlimit: "10",
                        stroke_width: "2",
                    }
                    rect {
                        fill: "none",
                        height: "18",
                        rx: "2",
                        ry: "2",
                        stroke: "currentColor",
                        stroke_linecap: "square",
                        stroke_miterlimit: "10",
                        stroke_width: "2",
                        width: "18",
                        x: "3",
                        y: "3",
                    }
                }
            }
            span { class: "dock-label", "Blog" }
        }
    )
}
