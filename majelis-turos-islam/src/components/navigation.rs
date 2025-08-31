use leptos::prelude::*;

use crate::components::sholat_hour::SholatView;

#[component]
pub fn Navigation(show_sholat: RwSignal<bool>) -> impl IntoView {
    view! {
        <nav class="navigation navbar navbar-expand-lg bg-body-tertiary sticky-top">
            <div class="container">
                <a class="navbar-brand" href="#">
                    <img src="/img/favicon.ico" alt="Logo" class="img-fluid" />
                    <div class="brand-name">
                        <h5 class="fw-bold">Majelis Turos Islam</h5>
                        <p class="fw-bold">Jakarta, Indonesia</p>
                    </div>
                </a>
                <div class="navbar-nav ms-auto">
                    <a class="nav-link" href="/">
                        MTI News
                    </a>
                    <a class="nav-link" href="/fatwa-maklumat">
                        Fatwa & Maklumat
                    </a>
                    <a class="nav-link" href="/artikel">
                        Artikel
                    </a>
                    <a class="nav-link" href="/tawawuf">
                        Tasawuf
                    </a>
                    <a class="nav-link" href="/tentang">
                        Tentang Kami
                    </a>
                    <SholatView show=show_sholat />
                </div>
            </div>
        </nav>
    }
}
