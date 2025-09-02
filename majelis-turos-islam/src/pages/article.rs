use comrak::{markdown_to_html, ComrakOptions};
use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use serde::Deserialize;

use crate::BACKEND_URL;

#[derive(Deserialize, Clone)]
struct NewsResponse {
    data: NewsArticle,
}

#[derive(Deserialize, Clone)]
struct NewsArticle {
    markdown: String,
}

#[component]
pub fn Article() -> impl IntoView {
    let md = RwSignal::new(String::new());

    // fetch artikel
    Effect::new(move |_| {
        let url = format!("{}/news/article/get/nasab-baklawi-habaib-dalam-gugatan-imam-mursyid-mti-buka-dialog-dengan-ibhrs", BACKEND_URL);
        spawn_local(async move {
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<NewsResponse>().await {
                    md.set(data.data.markdown);
                }
            }
        });
    });

    // bikin computed biar html selalu update pas md berubah
    let html = Memo::new(move |_| {
        markdown_to_html(&md.get(), &ComrakOptions::default())
    });

    view! {
        <leptos_meta::Title text="Artikel - Majelis Turos Islam" />
        <h1>"Artikel"</h1>
        <Show when=move || !html.get().is_empty() fallback=|| view! { <p>"Loading..."</p> }>
            <div class="card-text" inner_html=move || html.get() />
        </Show>
    }
}
