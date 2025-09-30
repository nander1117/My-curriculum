use dioxus::prelude::*;

/// * ### Add intersection observer to elements with class intersect for scroll animation in tailwindcss
///   more info: [tailwindcss-intersect](https://github.com/heidkaemper/tailwindcss-intersect)
///
///   it's equivalent to adding the following script in html:
///
///   ```html
///   <script src="https://unpkg.com/tailwindcss-intersect@2.x.x/dist/observer.min.js" defer></script>
///   ```
#[component]
pub fn ActiveIntersetForScrollAnimation() -> Element {
    rsx!(script {
        src: "https://unpkg.com/tailwindcss-intersect@2.x.x/dist/observer.min.js",
        defer: "true",
    })
}
