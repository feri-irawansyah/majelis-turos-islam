use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="d-flex flex-row justify-content-between">
            <div class="flex-column">
                <div id="carouselExampleInterval" class="carousel slide" data-bs-ride="carousel">
                    <div class="carousel-inner">
                        <div class="carousel-item active" data-bs-interval="10000">
                            <img src="..." class="d-block w-100" alt="..." />
                        </div>
                        <div class="carousel-item" data-bs-interval="2000">
                            <img src="..." class="d-block w-100" alt="..." />
                        </div>
                        <div class="carousel-item">
                            <img src="..." class="d-block w-100" alt="..." />
                        </div>
                    </div>
                    <button
                        class="carousel-control-prev"
                        type="button"
                        data-bs-target="#carouselExampleInterval"
                        data-bs-slide="prev"
                    >
                        <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                        <span class="visually-hidden">Previous</span>
                    </button>
                    <button
                        class="carousel-control-next"
                        type="button"
                        data-bs-target="#carouselExampleInterval"
                        data-bs-slide="next"
                    >
                        <span class="carousel-control-next-icon" aria-hidden="true"></span>
                        <span class="visually-hidden">Next</span>
                    </button>
                </div>
            </div>
            <div class="flex-column">dan okeh</div>
        </div>
    }
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
