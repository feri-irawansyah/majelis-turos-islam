use leptos::{prelude::*, task::spawn_local};
use serde::Deserialize;

use crate::{BACKEND_URL, app::{ResultList, format_wib_date}};

#[derive(Deserialize, Clone, Debug, Default)]
pub struct CommentData {
    pub comment_id: i32,
    pub title: String,
    pub stars: i8,
    pub email: String,
    pub name: String,
    pub created_at: String,
}

fn chunk_comments(comments: Vec<CommentData>, size: usize) -> Vec<Vec<CommentData>> {
    comments.chunks(size).map(|c| c.to_vec()).collect()
}

#[component]
pub fn Footer() -> impl IntoView {

    let comments = RwSignal::<ResultList<CommentData>>::new(ResultList::default());

    // fetch artikel
    Effect::new(move |_| {
        let url = format!("{}/comment/data", BACKEND_URL);
        spawn_local(async move {
            if let Ok(response) = gloo_net::http::Request::get(&url).send().await {
                match response.json::<ResultList<CommentData>>().await {
                    Ok(data) => {
                        comments.set(data);
                    }
                    Err(err) => {
                        log::error!("failed to parse response: {:?}", err);
                    }
                }
            } else {
                log::error!("request failed");
            }
        });
    });

    view! {
        <div class="footer mt-5 bg-primary">
            <div class="container mb-3">
                <div class="card">
                    <div class="card-body">This is some text within a card body.</div>
                </div>
            </div>
            <div class="container text-center text-white">
                <div
                    id="carouselExampleAutoplaying"
                    class="carousel slide card"
                    data-bs-ride="carousel"
                >
                    <div class="carousel-indicators">
                        <Show when=move || !comments.get().rows.is_empty() fallback=|| view! {}>
                            {move || {
                                comments
                                    .get()
                                    .rows
                                    .iter()
                                    .enumerate()
                                    .map(|(i, _)| {
                                        view! {
                                            <button
                                                type="button"
                                                data-bs-target="#carouselExampleAutoplaying"
                                                data-bs-slide-to=i.to_string()
                                                class=if i == 0 { "active" } else { "" }
                                                aria-current=if i == 0 { "true" } else { "false" }
                                                aria-label=format!("Slide {}", i + 1)
                                            ></button>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </Show>
                    </div>
                    <div class="carousel-inner">
                        <For
                            each=move || {
                                let rows = comments.get().rows;
                                chunk_comments(rows, 3)
                            }
                            // pakai id pertama biar unique
                            key=|chunk| chunk[0].comment_id
                            children=move |chunk| {
                                let is_first = chunk[0].comment_id != 1;
                                // slide pertama active
                                view! {
                                    <div
                                        class=if is_first {
                                            "carousel-item active"
                                        } else {
                                            "carousel-item"
                                        }
                                        data-bs-interval="5000"
                                    >
                                        <div class="row">
                                            {chunk
                                                .into_iter()
                                                .map(|c| {
                                                    view! {
                                                        <div class="col-md-4">
                                                            <div class="card text-dark">
                                                                <div class="card-body">
                                                                    <h5 class="card-title">{c.title.clone()}</h5>
                                                                    <p class="card-text">{c.email.clone()}</p>
                                                                    <p class="card-text">{c.name.clone()}</p>
                                                                    // ⭐ Render bintang
                                                                    <p>
                                                                        {(1..=5)
                                                                            .map(|i| {
                                                                                let class = if i <= c.stars.into() {
                                                                                    "bi bi-star-fill text-primary me-1"
                                                                                } else {
                                                                                    "bi bi-star-fill text-secondary me-1"
                                                                                };
                                                                                view! { <i class=class></i> }
                                                                            })
                                                                            .collect::<Vec<_>>()}
                                                                    </p>
                                                                    <small>{format_wib_date(&c.created_at)}</small>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
                <hr />
                <p>"Majelis Turos Islam @Snakesystem 2025 - All Rights Reserved"</p>
            </div>
        </div>
    }
}
