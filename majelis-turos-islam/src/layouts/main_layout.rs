use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{footer::Footer, navigation::Navigation, sholat_hour::SholatHour};

#[component]
pub fn MainLayout() -> impl IntoView {
    let show_sholat = RwSignal::new(false);

    view! {
        <main class="main-layout">
            <SholatHour show_sholat />
            <Navigation show_sholat />
            <div class="card text-bg-dark" data-aos="fade-in" data-aos-duration="2000">
                <img data-trunk src="/img/banner.jpg" alt="Leptos Logo" class="" />
                // <img data-trunk src="/img/bismillah.png" alt="Bismillah" class="card-logo" />
                <div class="card-img-overlay"></div>
            </div>
            <img data-trunk src="/img/bg-vector-1.png" alt="Vector Image" class="left-vector" />
            <div class="container main-content" data-aos="fade-in" data-aos-duration="1000">
                <Outlet />
            </div>
            <img data-trunk src="/img/bg-vector-2.png" alt="Vector Image" class="right-vector" />
            <Footer />
        </main>
    }
}
