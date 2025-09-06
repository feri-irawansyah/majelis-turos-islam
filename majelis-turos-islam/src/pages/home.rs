use leptos::{prelude::*, task::spawn_local};

use crate::{BACKEND_URL, app::{ArticleData, EventData, KajianData, NewsData, ResultList}, components::section_title::SectionTitle};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {

    let news = RwSignal::<ResultList<NewsData>>::new(ResultList::default());
    let article = RwSignal::<ResultList<ArticleData>>::new(ResultList::default());
    let event = RwSignal::<ResultList<EventData>>::new(ResultList::default());
    let kajian = RwSignal::<ResultList<KajianData>>::new(ResultList::default());

    // fetch artikel
    Effect::new(move |_| {
        let url = format!("{}/news/data", BACKEND_URL);
        spawn_local(async move {
            if let Ok(response) = gloo_net::http::Request::get(&url).send().await {
                match response.json::<ResultList<NewsData>>().await {
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

    Effect::new(move |_| {
        let url = format!("{}/article/data?page=1&pageSize=9", BACKEND_URL);
        spawn_local(async move {
            if let Ok(response) = gloo_net::http::Request::get(&url).send().await {
                match response.json::<ResultList<ArticleData>>().await {
                    Ok(data) => {
                        article.set(data);
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

    Effect::new(move |_| {
        let url = format!("{}/event/data?page=1&pageSize=4", BACKEND_URL);
        spawn_local(async move {
            if let Ok(response) = gloo_net::http::Request::get(&url).send().await {
                match response.json::<ResultList<EventData>>().await {
                    Ok(data) => {
                        event.set(data);
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

    Effect::new(move |_| {
        let url = format!("{}/kajian/data?page=1&pageSize=9", BACKEND_URL);
        spawn_local(async move {
            if let Ok(response) = gloo_net::http::Request::get(&url).send().await {
                match response.json::<ResultList<KajianData>>().await {
                    Ok(data) => {
                        kajian.set(data);
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
        <div class="row justify-content-between">
            <div class="col-md-6 p-1" data-aos="fade-right" data-delay="100">
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
            <div class="col-md-6 p-1" data-aos="fade-left" data-delay="100">
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
        <ArtikelPreview data=article />
        <EventPreview data=event kajian=kajian />
    }
}

#[component]
pub fn ArtikelPreview(data: RwSignal<ResultList<ArticleData>>) -> impl IntoView {
    view! {
        <div class="d-flex flex-column justify-content-center" data-aos="fade-up">
            <SectionTitle
                title="Artikel Terbaru".to_string()
                sub_title="Event & Blog".to_string()
                position="center".to_string()
            />
            <div class="row g-3 my-3">
                <Show
                    when=move || data.get().rows.len() != 0
                    fallback=|| view! { <p>"Loading..."</p> }
                >
                    {move || {
                        data.get()
                            .rows
                            .iter()
                            .enumerate()
                            .map(|(i, article)| {
                                view! {
                                    <a
                                        href=format!("/article/{}", article.slug.clone())
                                        class="col-lg-4 col-md-6 col-12"
                                        data-aos="fade-up"
                                        data-aos-duration="1000"
                                        data-aos-delay=i * 100
                                    >
                                        <div class="card mb-3" style="max-width: 540px;">
                                            <div class="row h-100 g-0">
                                                <div class="col-md-4">
                                                    <img
                                                        src=format!(
                                                            "https://fastly.picsum.photos/id/685/200/200.jpg?hmac=1IjDFMSIa0T_JSvcq79_e2NWPwRJg61Ufbfu4eM4HvA",
                                                        )
                                                        class="img-fluid rounded-start"
                                                        alt="..."
                                                    />
                                                </div>
                                                <div class="col-md-8">
                                                    <div class="card-body">
                                                        <h5 class="card-title">{article.title.clone()}</h5>
                                                        <p class="card-text">{article.author_id}</p>
                                                        <p class="card-text">
                                                            <small class="text-body-secondary">
                                                                {article.created_at.clone()}
                                                            </small>
                                                        </p>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </a>
                                }
                            })
                            .collect_view()
                    }}
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn EventPreview(data: RwSignal<ResultList<EventData>>, kajian: RwSignal<ResultList<KajianData>>) -> impl IntoView {
    view! {
        <div class="d-flex flex-column justify-content-center" data-aos="fade-up">
            <SectionTitle
                title="Kajian Islami".to_string()
                sub_title="".to_string()
                position="center".to_string()
            />
            <div class="row g-3 my-3">
                <div class="col-lg-6 col-12">
                    <Kajian data=kajian />
                </div>
                <div class="col-lg-6 col-12">
                    <div class="row">
                        {move || {
                            data.get()
                                .rows
                                .iter()
                                .take(2)
                                .map(|event| {
                                    view! {
                                        <div class="col-lg-6 col-12">
                                            <div class="card text-bg-dark">
                                                <img src="/img/turos-1.jpg" class="card-img" alt="..." />
                                                <div class="card-img-overlay">
                                                    <h5 class="card-title">{event.title.clone()}</h5>
                                                    <p class="card-text">
                                                        This is a wider card with supporting text below as a natural lead-in to additional content. This content is a little bit longer.
                                                    </p>
                                                    <p class="card-text">
                                                        <small>Last updated 3 mins ago</small>
                                                    </p>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Kajian(data: RwSignal<ResultList<KajianData>>) -> impl IntoView {

    let current_playing = RwSignal::new(None::<String>);

    view! {
        <div id="eventCarousel" class="carousel slide event-slider">
            // <!-- Indicators -->
            <div class="carousel-indicators">
                <Show when=move || !data.get().rows.is_empty() fallback=|| view! {}>
                    {move || {
                        let n = data.get().rows.len();
                        data.get()
                            .rows
                            .iter()
                            .enumerate()
                            .rev()
                            .map(|(orig_i, kajian)| {
                                let display_i = n - 1 - orig_i;
                                let thumb_url = format!(
                                    "https://img.youtube.com/vi/{}/hqdefault.jpg",
                                    kajian.yt_link.split("v=").nth(1).unwrap_or(""),
                                );
                                view! {
                                    <button
                                        type="button"
                                        on:click=move |_| {
                                            current_playing.set(None);
                                        }
                                        data-bs-target="#eventCarousel"
                                        data-bs-slide-to=display_i.to_string()
                                        class=format!(
                                            "card {}",
                                            if display_i == 0 { "active" } else { "" },
                                        )
                                        aria-current=if display_i == 0 { "true" } else { "false" }
                                        aria-label=format!("Slide {}", display_i + 1)
                                    >
                                        <img src=thumb_url alt="YouTube thumbnail" />
                                    </button>
                                }
                            })
                            .collect_view()
                    }}

                </Show>
            </div>

            // <!-- The slideshow/carousel -->
            <div class="carousel-inner">
                <Show
                    when=move || !data.get().rows.is_empty()
                    fallback=|| view! { <p>"Loading..."</p> }
                >
                    {move || {
                        let n = data.get().rows.len();
                        data.get()
                            .rows
                            .iter()
                            .enumerate()
                            .rev()
                            .map(|(orig_i, event)| {
                                let display_i = n - 1 - orig_i;
                                view! {
                                    <div class=format!(
                                        "carousel-item {}",
                                        if display_i == 0 { "active" } else { "" },
                                    )>
                                        <div class="carousel-img-wrapper">
                                            <YoutubeEmbed
                                                url=event.yt_link.clone()
                                                current_playing=current_playing
                                            />
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn YoutubeEmbed(url: String, current_playing: RwSignal<Option<String>>) -> impl IntoView {
    let video_id = url.split("v=").nth(1).unwrap_or("").to_string();
    let thumb_url = format!("https://img.youtube.com/vi/{}/hqdefault.jpg", video_id);
    let embed_url = format!("https://www.youtube.com/embed/{}?autoplay=1", video_id);
    let video_id_clone = video_id.clone(); // copy tambahan

    view! {
        <div
            class="youtube-thumbnail"
            on:click=move |_| {
                current_playing.set(Some(video_id_clone.clone()));
            }
        >
            <Show
                when=move || current_playing.get() == Some(video_id.clone())
                fallback=move || {
                    let url = thumb_url.clone();
                    view! {
                        <img src=url alt="YouTube thumbnail" />
                        <div style="position: absolute; top: 50%; left: 50%; 
                        transform: translate(-50%, -50%); 
                        background: rgba(0,0,0,0.6); 
                        border-radius: 50%; 
                        padding: 20px;">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="40"
                                height="40"
                                fill="white"
                                viewBox="0 0 16 16"
                            >
                                <path d="M6.79 5.093A.5.5 0 0 0 6 5.5v5a.5.5 0 0 0 .79.407l4-2.5a.5.5 0 0 0 0-.814l-4-2.5z" />
                            </svg>
                        </div>
                    }
                }
            >
                <iframe
                    src=embed_url.clone()
                    title="YouTube video player"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen
                ></iframe>
            </Show>
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
