use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            id: "loading-screen",
            style: "
                display: flex; 
                justify-content: center; 
                align-items: center; 
                position: fixed; 
                top: 0; 
                left: 0; 
                width: 100%; 
                height: 100%; 
                z-index: 9999; 
                background-color: #030712; /* bg-gray-950 */
                animation: fade-out 300ms forwards; /* need for deleted loading */ 
            ",
            // div { style: "display: flex; flex-direction: row; gap: 8px;",
            //     div { style: "width: 16px; height: 16px; border-radius: 9999px; background-color: #1d4ed8; animation: bounce 1s infinite;" }
            //     div { style: "width: 16px; height: 16px; border-radius: 9999px; background-color: #1d4ed8; animation: bounce 1s infinite; animation-delay: -0.3s;" }
            //     div { style: "width: 16px; height: 16px; border-radius: 9999px; background-color: #1d4ed8; animation: bounce 1s infinite; animation-delay: -0.5s;" }
            // }
            span { class: "loading loading-bars w-12 bg-blue-700 " }
        }
    }
}
