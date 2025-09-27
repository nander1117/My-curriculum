use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-base-200 text-center p-4",
            h1 { "404 - Página No Encontrada" }
            p { "Lo sentimos, la página que buscas no existe." }
            // Puedes incluir un enlace para regresar a la página de inicio.
            Link { to: Route::Home {}, "Ir a la página de inicio" }
        }
    }
}
