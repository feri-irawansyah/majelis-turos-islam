use leptos::prelude::*;
use leptos_router::{hooks::use_location, location::Location};

use crate::components::sholat_hour::SholatView;

#[component]
pub fn Navigation(show_sholat: RwSignal<bool>) -> impl IntoView {
    let location: Location = use_location();

    view! {
        <nav class="navigation navbar navbar-expand-lg bg-secondary sticky-top">
            <div class="container">
                <a class="navbar-brand" href="#">
                    <img src="/img/logo-removebg.png" alt="Logo" class="img-fluid" />
                    <div class="brand-name">
                        <h5 class="fw-bold">Majelis Turos Islam</h5>
                        <p class="fw-bold">Jakarta, Indonesia</p>
                    </div>
                </a>
                <div class="navbar-nav ms-auto">
                    <a
                        class="nav-link"
                        href="/"
                        class:active=move || location.pathname.get() == "/"
                    >
                        <i class="bi bi-house me-2"></i>
                        <span>Beranda</span>
                    </a>
                    <a
                        class="nav-link"
                        href="/mti-news"
                        class:active=move || location.pathname.get() == "/mti-news"
                    >
                        <i class="bi bi-newspaper me-2"></i>
                        <span>MTI News</span>
                    </a>
                    <a
                        class="nav-link"
                        href="/fatwa-maklumat"
                        class:active=move || location.pathname.get() == "/fatwa-maklumat"
                    >
                        <i class="bi bi-journal-arrow-down me-2"></i>
                        <span>Fatwa & Maklumat</span>
                    </a>
                    <a
                        class="nav-link"
                        href="/artikel"
                        class:active=move || location.pathname.get() == "/artikel"
                    >
                        <i class="bi bi-journal-bookmark me-2"></i>
                        <span>Artikel</span>
                    </a>
                    <a
                        class="nav-link"
                        href="/tawawuf"
                        class:active=move || location.pathname.get() == "/tawawuf"
                    >
                        <i class="bi bi-journal-check me-2"></i>
                        <span>Tawawuf</span>
                    </a>
                    <a
                        class="nav-link"
                        href="/tentang"
                        class:active=move || location.pathname.get() == "/tentang"
                    >
                        <i class="bi bi-info-circle me-2"></i>
                        <span>Tentang Kami</span>
                    </a>
                    <SholatView show=show_sholat />
                </div>
            </div>
        </nav>
    }
}
