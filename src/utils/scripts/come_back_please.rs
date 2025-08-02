use dioxus::prelude::*;
pub fn come_back_please() {
    document::eval(
        r#"
            document.addEventListener("visibilitychange", () => {
                if (document.visibilityState === "hidden") {
                    document.title = "Regresa please!";
                } else {
                    document.title = "My Curriculum";
                }
            });
        "#,
    );
}
