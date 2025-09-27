use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    let mut count: Signal<usize> = use_signal(|| 0);
    let mut change = use_signal(|| false);
    let mut count_down = use_signal(|| 59);
    fn increment(e: Event<FormData>, count: &mut Signal<usize>) {
        count.set(e.data().value().len());
    }

    rsx! (
        div {
            id: "hero",
            class: "bg-gray-900 p-4 motion-scale-in-[0.5] motion-opacity-in-[20%] motion-blur-in-[20px] motion-ease-in-out",
            calendar-date { class: "cally bg-base-100 border border-base-300 shadow-lg rounded-box",
                calendar-month {}
            }
            img {
                src: HEADER_SVG,
                id: "header",
                alt: "header",
                class: "motion-translate-x-in-[-5%] motion-translate-y-in-[-94%] motion-ease-bounce ",
            }
            label { class: "motion-bounce  peer has-invalid:text-blue-700  shadow cursor-cell  shadow-white p-2 m-5 bg-white border-2 border-amber-300 rounded text-black text-center flex items-center justify-center",
                input {
                    required: true,
                    pattern: "[a-zA-Z0-9_]+",
                    oninput: move |e| increment(e, &mut count),
                    class: " text-black text-center invalid:text-red-800 outline-none select-none motion-translate-x-in-[182%] motion-translate-y-in-[-3%] motion-opacity-in-[0%] motion-blur-in-[5px] ",
                    placeholder: "Type something here...",
                }
                h1 { class: " p-2 motion-scale-in-[2] motion-translate-x-in-[-4%] motion-translate-y-in-[-200%] motion-opacity-in-[0%] motion-duration-[1750ms] motion-duration-[700ms]/translate motion-duration-[700ms]/opacity motion-ease-bounce",
                    "{count}"
                }
            }
            br {}
            div { class: "flex items-center gap-x-2 ",
                img {
                    alt: "",
                    class: "object-cover w-12 h-12 rounded-lg",
                    src: "https://images.unsplash.com/photo-1544005313-94ddf0286df2?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=faceare&facepad=3&w=688&h=688&q=100",
                }
                div {
                    h1 { class: "text-xl font-semibold text-gray-700 capitalize dark:text-white",
                        "Mia John"
                    }
                    p { class: "text-sm text-gray-500 dark:text-gray-400", "miajohn@merakiui.com" }
                }
            }
            button {
                class: "cursor-pointer peer-has-invalid:pointer-events-none m-2 hover:bg-amber-900 peer-has-invalid:bg-blue-400  bg-amber-600 p-2 rounded-2xl border border-amber-300  motion-scale-in-[0] motion-translate-x-in-[185%] motion-translate-y-in-[98%] motion-rotate-in-[2000deg] motion-blur-in-[30px] motion-duration-[0.48s] motion-duration-[0.70s]/scale motion-delay-[0.52s]/rotate motion-duration-[1.00s]/blur motion-ease-spring-bouncier rotate-4",
                onclick: move |_| count.set(0),
                "👋 Hello Dioxus"
            }
            div { id: "links",
                // The RSX macro also supports text nodes surrounded by quotes
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
            div { class: "w-full",

                button {
                    class: "btn btn-xl btn-neutral motion-preset-flomoji-[🎉]",
                    onmouseover: move |_| change.set(true),
                    "hola"
                }
                label { class: "toggle text-base-content",
                    input {
                        class: "theme-controller",
                        r#type: "checkbox",
                        value: "synthwave",
                    }
                    svg {
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        g {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            circle { cx: "12", cy: "12", r: "4" }
                            path { d: "M12 2v2" }
                            path { d: "M12 20v2" }
                            path { d: "m4.93 4.93 1.41 1.41" }
                            path { d: "m17.66 17.66 1.41 1.41" }
                            path { d: "M2 12h2" }
                            path { d: "M20 12h2" }
                            path { d: "m6.34 17.66-1.41 1.41" }
                            path { d: "m19.07 4.93-1.41 1.41" }
                        }
                    }
                    svg {
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        g {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                        }
                    }
                }
                label { class: "toggle text-base-content",
                    input {
                        class: "theme-controller",
                        r#type: "checkbox",
                        value: "synthwave",
                    }
                    svg {
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        g {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            circle { cx: "12", cy: "12", r: "4" }
                            path { d: "M12 2v2" }
                            path { d: "M12 20v2" }
                            path { d: "m4.93 4.93 1.41 1.41" }
                            path { d: "m17.66 17.66 1.41 1.41" }
                            path { d: "M2 12h2" }
                            path { d: "M20 12h2" }
                            path { d: "m6.34 17.66-1.41 1.41" }
                            path { d: "m19.07 4.93-1.41 1.41" }
                        }
                    }
                    svg {
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        g {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                        }
                    }
                }
                span { class: "countdown",
                    span {
                        aria_label: "59",
                        aria_live: "polite",
                        style: "--value:59;",
                        "59"
                    }
                }
            }
            figure { class: "diff aspect-16/9", tabindex: "0",
                div { class: "diff-item-1", role: "img", tabindex: "0",
                    img {
                        alt: "daisy",
                        src: "https://img.daisyui.com/images/stock/photo-1560717789-0ac7c58ac90a.webp",
                    }
                }
                div { class: "diff-item-2", role: "img",
                    img {
                        alt: "daisy",
                        src: "https://img.daisyui.com/images/stock/photo-1560717789-0ac7c58ac90a-blur.webp",
                    }
                }
                div { class: "diff-resizer" }
            }
            div {
                class: "max-w-100 intersect:motion-preset-slide-up-left scale-50 opacity-0 intersect:scale-100 intersect:opacity-100 transition duration-700 ",
                draggable: "true",
                ul { class: "list bg-base-100 rounded-box shadow-md",
                    li { class: "p-4 pb-2 text-xs opacity-60 tracking-wide",
                        "Most played songs this week"
                    }
                    li { class: "list-row",
                        div {
                            img {
                                class: "size-10 rounded-box",
                                src: "https://img.daisyui.com/images/profile/demo/1@94.webp",
                            }
                        }
                        div {
                            div { "Dio Lupa" }
                            div { class: "text-xs uppercase font-semibold opacity-60",
                                "Remaining Reason"
                            }
                        }
                        button {
                            class: "btn btn-square btn-ghost",
                            disabled: change,
                            svg {
                                class: "size-[1.2em]",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                g {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    path { d: "M6 3L20 12 6 21 6 3z" }
                                }
                            }
                        }
                        button {
                            class: "btn btn-square btn-ghost",
                            disabled: change,
                            svg {
                                class: "size-[1.2em]",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                g {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
                                }
                            }
                        }
                    }
                    li { class: "list-row",
                        div {
                            img {
                                class: "size-10 rounded-box",
                                src: "https://img.daisyui.com/images/profile/demo/4@94.webp",
                            }
                        }
                        div {
                            div { "Ellie Beilish" }
                            div { class: "text-xs uppercase font-semibold opacity-60",
                                "Bears of a fever"
                            }
                        }
                        button {
                            class: "btn btn-square btn-ghost",
                            disabled: change,
                            svg {
                                class: "size-[1.2em]",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                g {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    path { d: "M6 3L20 12 6 21 6 3z" }
                                }
                            }
                        }
                        button {
                            class: "btn btn-square btn-ghost",
                            disabled: change,

                            svg {
                                class: "size-[1.2em]",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                g {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
                                }
                            }
                        }
                    }
                    li { class: "list-row ",
                        div {
                            img {
                                class: "size-10 rounded-box",
                                src: "https://img.daisyui.com/images/profile/demo/3@94.webp",
                            }
                        }
                        div {
                            div { "Sabrino Gardener" }
                            div { class: "text-xs uppercase font-semibold opacity-60",
                                "Cappuccino"
                            }
                        }
                        button {
                            class: "btn btn-square btn-ghost",
                            onclick: move |_| change.set(false),
                            svg {
                                class: "size-[1.2em]",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                g {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    path { d: "M6 3L20 12 6 21 6 3z" }
                                }
                            }
                        }
                        label { class: "btn btn-square btn-ghost swap swap-rotate",
                            input {
                                r#type: "checkbox",
                                oninput: move |_| {
                                    let current_value = *change.read();
                                    change.set(!current_value);
                                },
                            }
                            svg {
                                class: "swap-on size-[1.2em] fill-current",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                            }
                            svg {
                                class: "swap-off size-[1.2em] fill-current",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                            }
                        }
                        if *change.read() {
                            svg {
                                class: "swap-off starting:scale-300 duration-3000 motion-duration-3000 size-[1.2em] fill-current  motion-bg-out-amber-400",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                            }
                        }
                    }
                }
            }
            button {
                class: "btn bg-black text-white border-black",
                aria_label: "GitHub logo button",
                svg {
                    height: "16",
                    view_box: "0 0 24 24",
                    width: "16",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M12,2A10,10 0 0,0 2,12C2,16.42 4.87,20.17 8.84,21.5C9.34,21.58 9.5,21.27 9.5,21C9.5,20.77 9.5,20.14 9.5,19.31C6.73,19.91 6.14,17.97 6.14,17.97C5.68,16.81 5.03,16.5 5.03,16.5C4.12,15.88 5.1,15.9 5.1,15.9C6.1,15.97 6.63,16.93 6.63,16.93C7.5,18.45 8.97,18 9.54,17.76C9.63,17.11 9.89,16.67 10.17,16.42C7.95,16.17 5.62,15.31 5.62,11.5C5.62,10.39 6,9.5 6.65,8.79C6.55,8.54 6.2,7.5 6.75,6.15C6.75,6.15 7.59,5.88 9.5,7.17C10.29,6.95 11.15,6.84 12,6.84C12.85,6.84 13.71,6.95 14.5,7.17C16.41,5.88 17.25,6.15 17.25,6.15C17.8,7.5 17.45,8.54 17.35,8.79C18,9.5 18.38,10.39 18.38,11.5C18.38,15.32 16.04,16.16 13.81,16.41C14.17,16.72 14.5,17.33 14.5,18.26C14.5,19.6 14.5,20.68 14.5,21C14.5,21.27 14.66,21.59 15.17,21.5C19.14,20.16 22,16.42 22,12A10,10 0 0,0 12,2Z",
                        fill: "white",
                    }
                }
                " GitHub "
            }
        }
    )
}
