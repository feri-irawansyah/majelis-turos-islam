use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Timings {
    #[serde(rename = "Fajr")]
    fajr: String,
    #[serde(rename = "Dhuhr")]
    dhuhr: String,
    #[serde(rename = "Asr")]
    asr: String,
    #[serde(rename = "Maghrib")]
    maghrib: String,
    #[serde(rename = "Isha")]
    isha: String,
}

#[derive(Debug, Deserialize)]
struct Data {
    timings: Timings,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Data,
}

#[component]
pub fn SholatHour(show_sholat: RwSignal<bool>) -> impl IntoView {

    view! {
        <nav class="navbar navbar-expand-lg bg-cocolate sticky-top">
            <div class="container">
                <div class="navbar-nav me-auto">
                    <a class="nav-link active" aria-current="page" href="#">
                        <i class="bi bi-map me-2 text-primary"></i>
                        <span class="text-white">Jakarta, Indonesia</span>
                    </a>
                    <a class="nav-link text-white fw-bold" aria-current="page">
                        |
                    </a>
                    <a class="nav-link" href="#">
                        <i class="bi bi-clock me-2 text-primary"></i>
                        <span class="text-white">
                            <Clock />
                        </span>
                    </a>
                </div>
                <div class="navbar-nav ms-auto">
                    <a
                        class="nav-link waktu-sholat"
                        aria-current="page"
                        href="#"
                        on:click=move |_| show_sholat.set(!show_sholat.get())
                    >
                        <i class="bi bi-alarm fs-6 me-2 text-primary"></i>
                        <span class="text-white">Waktu Sholat</span>
                        <i class="bi bi-caret-down-fill ms-1"></i>
                    </a>
                    <a class="nav-link" href="#">
                        Follow Us
                    </a>
                    <a
                        class="nav-link"
                        href="https://www.facebook.com/turosislam.mti"
                        target="_blank"
                    >
                        <i class="bi bi-facebook"></i>
                    </a>
                    <a
                        class="nav-link"
                        href="https://www.youtube.com/channel/UCTaIugJs0Zg2r0O-DzSlmQw"
                        target="_blank"
                    >
                        <i class="bi bi-youtube"></i>
                    </a>
                    <a
                        class="nav-link"
                        href="https://x.com/intent/post?text=Hey%2C+check+out+this+cool+site+I+found%3A+www.yourname.com+%23Topic+via%40my_twitter_name&url=https%3A%2F%2Fmajelisturosislam.org%2F"
                        target="_blank"
                    >
                        <i class="bi bi-twitter"></i>
                    </a>
                    <a class="nav-link">
                        <i class="bi bi-whatsapp"></i>
                    </a>
                </div>
            </div>
        </nav>
    }
}

use chrono::Local;
use gloo_timers::callback::Interval;

#[component]
pub fn Clock() -> impl IntoView {
    // Signal waktu sekarang
    let (time, set_time) = signal(format_now());

    // Update setiap detik
    Effect::new(move || {
        Interval::new(1000, move || {
            set_time.set(format_now());
        })
        .forget(); // keep interval alive
    });

    // View-nya
    view! {
        <div class="clock">
            <strong>{move || time.get()}</strong>
        </div>
    }
}

fn format_now() -> String {
    let now = Local::now();
    now.format("%B %d, %Y at %I:%M:%S %p").to_string()
}

#[component]
pub fn SholatView(show: RwSignal<bool>) -> impl IntoView {
    let (sholat, set_sholat) = signal::<Option<Timings>>(None);

    // fetch jadwal sholat
    Effect::new(move |_| {
        let url = "https://api.aladhan.com/v1/timingsByCity/today?city=Jakarta&country=Indonesia&method=2";

        spawn_local(async move {
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<ApiResponse>().await {
                    set_sholat.set(Some(data.data.timings));
                }
            }
        });
    });

    view! {
        <div class="table-responsive" class:d-none=move || !show.get()>
            <Show when=move || sholat.get().is_some() fallback=|| view! { <p>Loading...</p> }>
                <table class="table table-bordered table-striped table-sm text-center align-middle">
                    <thead class="table-dark">
                        <tr>
                            <th>Sholat</th>
                            <th>Waktu</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            sholat
                                .get()
                                .map(|data| {
                                    view! {
                                        <>
                                            <tr>
                                                <td>Fajr</td>
                                                <td>{data.fajr}</td>
                                            </tr>
                                            <tr>
                                                <td>Dhuhr</td>
                                                <td>{data.dhuhr}</td>
                                            </tr>
                                            <tr>
                                                <td>Asr</td>
                                                <td>{data.asr}</td>
                                            </tr>
                                            <tr>
                                                <td>Maghrib</td>
                                                <td>{data.maghrib}</td>
                                            </tr>
                                            <tr>
                                                <td>Isha</td>
                                                <td>{data.isha}</td>
                                            </tr>
                                        </>
                                    }
                                })
                        }}
                    </tbody>
                </table>
            </Show>
        </div>
    }
}
