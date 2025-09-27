use dioxus::prelude::*;

/// * ### Change the document title when the user switches tabs in browser
pub fn come_back_please() {
    document::eval(
        r#"
            document.addEventListener("visibilitychange", () => {
                if (document.visibilityState === "hidden") {
                    document.title = "Come back please!";
                } else {
                    document.title = "My Curriculum";
                }
            });
        "#,
    );
}
