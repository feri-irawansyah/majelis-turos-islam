use crate::components::counter_btn::Button;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! { <Button /> }
}

// <picture>
//     <source
//         srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
//         media="(prefers-color-scheme: dark)"
//     />
//     <img
//         src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
//         alt="Leptos Logo"
//         height="200"
//         width="400"
//     />
// </picture>
