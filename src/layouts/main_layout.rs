use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <main class="main-layout">
            <div class="jumbotron">
                <img src="/img/banner.jpg" alt="Leptos Logo" />
            </div>
            <Outlet />
        </main>
    }
}
