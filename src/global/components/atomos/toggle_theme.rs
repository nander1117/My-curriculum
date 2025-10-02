use crate::global::utils::scripts::{set_color_theme, ColorTheme};
use dioxus::prelude::*;

#[component]
pub fn ToggleTheme() -> Element {
    rsx! (
        label { class: "toggle text-base-content hidden md:inline-grid",
            input {
                class: "theme-controller",
                r#type: "checkbox",
                value: "synthwave",
                oninput: move |event: FormEvent| {
                    match event.data().checked() {
                        true => set_color_theme(ColorTheme::Dark),
                        false => set_color_theme(ColorTheme::Light),
                    };
                },
                checked: true,
            }
            svg { view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
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
            svg { view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
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
    )
}
