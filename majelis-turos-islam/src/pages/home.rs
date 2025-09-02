use leptos::{prelude::*, task::spawn_local};
use serde::Deserialize;

use crate::{BACKEND_URL, app::{ResultList}, components::section_title::SectionTitle};

#[derive(Deserialize, Clone, Debug, Default)]
struct NewsArticle {
    news_id: i32,
    title: String,
    slug: String,
    content: String,
    created_at: String,
}

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {

    let news = RwSignal::<ResultList<NewsArticle>>::new(ResultList::default());

    // fetch artikel
    Effect::new(move |_| {
        let url = format!("{}/news/data", BACKEND_URL);
        spawn_local(async move {
            if let Ok(response) = gloo_net::http::Request::get(&url).send().await {
                match response.json::<ResultList<NewsArticle>>().await {
                    Ok(data) => {
                        news.set(data);
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
        <div class="d-flex flex-row justify-content-between">
            <div
                class="flex-column w-50 p-1"
                data-aos="fade-right"
                data-delay="100"
                data-aos-duration="1500"
            >
                <div id="carouselExampleInterval" class="carousel slide" data-bs-ride="carousel ">
                    // <!-- Indicators -->
                    <div class="carousel-indicators">
                        <Show when=move || !news.get().rows.is_empty() fallback=|| view! {}>
                            {move || {
                                news.get()
                                    .rows
                                    .iter()
                                    .enumerate()
                                    .map(|(i, _)| {
                                        view! {
                                            <button
                                                type="button"
                                                data-bs-target="#carouselExampleInterval"
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

                    // <!-- The slideshow/carousel -->
                    <div class="border-style"></div>
                    <div class="carousel-inner">
                        <Show
                            when=move || !news.get().rows.is_empty()
                            fallback=|| view! { <p>"Loading..."</p> }
                        >
                            {move || {
                                news.get()
                                    .rows
                                    .iter()
                                    .map(|article| {
                                        view! {
                                            <div class="carousel-item active" data-bs-interval="5000">
                                                <div class="carousel-img-wrapper">
                                                    <img
                                                        src="https://fastly.picsum.photos/id/685/200/200.jpg?hmac=1IjDFMSIa0T_JSvcq79_e2NWPwRJg61Ufbfu4eM4HvA"
                                                        class="d-block w-100"
                                                        alt="judul"
                                                    />
                                                    <div class="overlay"></div>
                                                    <div class="text">
                                                        <a
                                                            href=format!("/mti-news/{}", article.slug.clone())
                                                            class="btn btn-primary"
                                                        >
                                                            <span>Baca Selengkapnya</span>
                                                            <i class="ms-1 bi bi-journals"></i>
                                                        </a>
                                                        <a href="/mti-news" class="btn btn-primary">
                                                            <span>Cari Berita</span>
                                                            <i class="ms-1 bi bi-search"></i>
                                                        </a>
                                                    </div>
                                                    <div class="carousel-caption d-none d-md-block">
                                                        <h5>{article.title.clone()}</h5>
                                                        <p>{crate::app::format_wib_date(&article.created_at)}</p>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </Show>
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
            <div
                class="flex-column w-50 p-1"
                data-aos="fade-left"
                data-delay="100"
                data-aos-duration="1500"
            >
                <div class="card px-3">
                    <SectionTitle
                        title="Tentang MTI".to_string()
                        sub_title="Sejarah".to_string()
                        position="left".to_string()
                    />
                    <p class="card-text">
                        "Kalimat turos (التراث) dalam transliterasi Arab-Indonesia yg standart “ al-Turats “. Kita sengaja memakai ejaan Arab melayu menjadi kalimat “ Turos “, bertujuan agar masyarakat Indonesia pada umumnya tidak sulit cara membaca dan mengucapkannya."
                    </p>
                    <p class="card-text">
                        "Turos (تراث) juga bermakna Heritage; warisan, pusaka (إرث ; ميراث ; ترکة , تراث), Tradition (التحدار : إنتقال العادات أو المعتقدات من جيل إلی جيل, تقليد ; عرف ,  نواميس ; تعاليم , ناموس ; تعليم), Legacy (ميراث بوصية , تراث), Patrimony (إرث , ميراث , وقف)."
                    </p>

                    <a href="/tentang" class="btn btn-primary more-about w-50">
                        <span>Selengkapnya</span>
                        <i class="ms-2bi bi-arrow-right"></i>
                    </a>
                </div>
            </div>
        </div>
        <br />
        <br />
        <br />
        <ArtikelPreview />
    }
}

#[component]
pub fn ArtikelPreview() -> impl IntoView {
    view! {
        <div class="d-flex flex-column justify-content-center">
            <SectionTitle
                title="Artikel Terbaru".to_string()
                sub_title="Event & Blog".to_string()
                position="center".to_string()
            />
            <div class="card mb-3" style="max-width: 540px;">
                <div class="row g-0">
                    <div class="col-md-4">
                        <img src="..." class="img-fluid rounded-start" alt="..." />
                    </div>
                    <div class="col-md-8">
                        <div class="card-body">
                            <h5 class="card-title">Card title</h5>
                            <p class="card-text">
                                This is a wider card with supporting text below as a natural lead-in to additional content. This content is a little bit longer.
                            </p>
                            <p class="card-text">
                                <small class="text-body-secondary">Last updated 3 mins ago</small>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
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
