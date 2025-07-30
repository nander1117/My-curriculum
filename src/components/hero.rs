use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    let mut count: Signal<usize> = use_signal(|| 0);
    fn increment(e: Event<FormData>, count: &mut Signal<usize>) {
        count.set(e.data().value().len());
    }
    rsx! {
        div { id: "hero", class: "bg-gray-900",
            img { src: HEADER_SVG, id: "header" }
            label { class: "has-invalid:text-blue-700 shadow  shadow-white p-2 m-5 bg-white border-2 border-amber-300 rounded text-black text-center flex items-center justify-center",
                input {
                    required: true,
                    pattern: "[a-zA-Z0-9_]+",
                    oninput: move |e| increment(e, &mut count),
                    class: "text-black text-center invalid:text-red-500 outline-none select-none",
                    placeholder: "Type something here...",
                }
                h1 { class: "p-2", "{count}" }
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
        }
    }
}
