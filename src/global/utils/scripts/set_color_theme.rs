use dioxus::document;

pub enum ColorTheme {
    Light,
    Dark,
    System,
}

pub fn set_color_theme(theme: ColorTheme) {
    match theme {
        ColorTheme::Light => set_light_theme(),
        ColorTheme::Dark => set_dark_theme(),
        ColorTheme::System => set_system_theme(),
    }
}

fn set_light_theme() {
    document::eval(r#"document.documentElement.setAttribute("data-theme", "light");"#);
}
fn set_dark_theme() {
    document::eval(r#"document.documentElement.setAttribute("data-theme", "dark");"#);
}
fn set_system_theme() {
    document::eval(
        r#"
        function setSystemTheme() {
            const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

            const systemTheme = prefersDark ? 'dark' : 'light';

            document.documentElement.setAttribute('data-theme', systemTheme);

        }

        setSystemTheme();

        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', setSystemTheme);
        
    "#,
    );
}
