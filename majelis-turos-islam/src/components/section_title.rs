use leptos::prelude::*;

#[component]
pub fn SectionTitle(title: String, sub_title: String, position: String) -> impl IntoView {
    view! {
        <div class=move || format!("section-title d-flex flex-column align-items-{}", position)>
            <p>{sub_title}</p>
            <h2>{title}</h2>
            <img data-trunk src="/img/prayer-head-shp.png" alt="Prayer Head" />
        </div>
    }
}
