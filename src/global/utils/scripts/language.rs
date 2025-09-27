use dioxus::prelude::*;

pub enum Lang {
    En,
    Es,
}
/// * ### Set the language of the document
///
///   @param : `language` |-> Language to set
/// * ### Example
/// ```rust   
///   language(Lang::En) -> Set document to English
///   language(Lang::Es) -> Set document to Spanish
/// ```
pub fn language(language: Lang) {
    match language {
        Lang::En => set_lang_en(),
        Lang::Es => set_lang_es(),
    }
}

fn set_lang_en() {
    document::eval(
        r#"
        document.documentElement.lang = "en";
        document.title = "My Curriculum - Portfolio of [Your Name]";
        document.querySelector('meta[name="description"]').setAttribute("content", "Welcome to my portfolio! I'm [Your Name], a [Your Profession]. Explore my projects, skills, and experience.");
        document.querySelector('meta[name="keywords"]').setAttribute("content", "portfolio, projects, skills, experience, [Your Profession], [Your Name]");
        document.querySelector('meta[name="author"]').setAttribute("content", "[Your Name]");
        "#,
    );
}
fn set_lang_es() {
    document::eval(
        r#"
        document.documentElement.lang = "es";
        document.title = "Mi Currículum - Portafolio de [Tu Nombre]";
        document.querySelector('meta[name="description"]').setAttribute("content", "¡Bienvenido a mi portafolio! Soy [Tu Nombre], un/a [Tu Profesión]. Explora mis proyectos, habilidades y experiencia.");
        document.querySelector('meta[name="keywords"]').setAttribute("content", "portafolio, proyectos, habilidades, experiencia, [Tu Profesión], [Tu Nombre]");
        document.querySelector('meta[name="author"]').setAttribute("content", "[Tu Nombre]");
        "#,
    );
}
