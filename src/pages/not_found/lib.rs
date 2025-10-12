use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {

        section { class: "relative min-h-screen bg-gray-950 overflow-hidden flex items-center justify-center  p-6",
            div { class: "absolute inset-0 opacity-20",
                div { class: "absolute top-20 left-10 w-40 h-40 rounded-full bg-purple-600 blur-3xl" }
                div { class: "absolute bottom-20 right-10 w-60 h-60 bg-cyan-600 blur-3xl" }
            }
            div { class: "relative z-10 text-center max-w-2xl mx-auto",
                div { class: "relative mb-12",
                    h1 { class: "text-[120px] md:text-[180px] font-bold text-white tracking-tighter",
                        span { class: "relative",
                            span { class: "absolute inset-0 text-purple-400 opacity-70 animate-glitch-1",
                                "404"
                            }
                            span { class: "absolute inset-0 text-cyan-400 opacity-70 animate-glitch-2",
                                "404"
                            }
                            span { class: "relative", "404" }
                        }
                    }
                }
                h2 { class: "text-3xl md:text-5xl font-bold text-white mb-6", " Page Not Found " }
                p { class: "text-gray-400 text-lg mb-10",
                    " Oops! The page you're looking for has vanished into the digital void. "
                }
                Link {
                    to: Route::Porfolio {},
                    class: " btn relative px-8 py-4 bg-gradient-to-r from-purple-600 to-cyan-600 text-white font-bold rounded-lg overflow-hidden group",
                    span { class: "relative z-10", "Return to Home" }
                    span { class: "absolute inset-0 bg-gradient-to-r from-cyan-600 to-purple-600 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-lg" }
                }
            }
            div { class: " absolute bottom-10 left-1/2 transform -translate-x-1/2 flex gap-4",
                div { class: "w-3 h-3 bg-purple-400 rounded-full animate-float delay-100" }
                div { class: "w-2 h-2 bg-cyan-400 rounded-full animate-float delay-300" }
                div { class: "w-4 h-4 bg-white rounded-full animate-float delay-500" }
            }
        }
    }
}
