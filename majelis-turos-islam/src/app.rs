use chrono::{DateTime, Datelike, FixedOffset, Utc};
use serde::Deserialize;

// Rest Api List
#[derive(Deserialize, Clone, Default, Debug)]
pub struct ResultList<T> {
    pub rows: Vec<T>,
    pub total: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct NewsData {
    pub news_id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub created_at: String,
}
#[derive(Deserialize, Clone, Debug, Default)]
pub struct ArticleData {
    pub article_id: i32,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub author_id: i32,
    pub created_at: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct EventData {
    pub event_id: i32,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub created_at: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct KajianData {
    pub kajian_id: i32,
    pub title: String,
    pub yt_link: String,
    pub created_at: String,
}

pub fn format_wib_date(date_str: &str) -> String {
    // Parse input sebagai UTC
    let utc_datetime = DateTime::parse_from_rfc3339(date_str).unwrap_or_else(|_| Utc::now().into());

    // Konversi ke WIB (UTC+7)
    let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    let wib_datetime = utc_datetime.with_timezone(&wib_offset);

    let weekdays = [
        "Ahad", "Senin", "Selasa", "Rabu", "Kamis", "Jum'at", "Sabtu",
    ];
    let months = [
        "Januari",
        "Februari",
        "Maret",
        "April",
        "Mei",
        "Juni",
        "Juli",
        "Agustus",
        "September",
        "Oktober",
        "November",
        "Desember",
    ];

    let weekday = weekdays[wib_datetime.weekday().num_days_from_sunday() as usize];
    let month = months[(wib_datetime.month0()) as usize]; // month0 = 0-based

    let day = wib_datetime.day();
    let year = wib_datetime.year();

    format!("{}, {} {} {}", weekday, day, month, year)
}
