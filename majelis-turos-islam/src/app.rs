use chrono::{DateTime, Datelike, FixedOffset, Utc};
use serde::Deserialize;

#[derive(Deserialize, Clone, Default, Debug)]
pub struct ResultList<T> {
    pub rows: Vec<T>,
    pub total: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

pub fn format_wib_date(date_str: &str) -> String {
    // Parse input sebagai UTC
    let utc_datetime = DateTime::parse_from_rfc3339(date_str).unwrap_or_else(|_| Utc::now().into());

    // Konversi ke WIB (UTC+7)
    let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    let wib_datetime = utc_datetime.with_timezone(&wib_offset);

    let weekdays = ["Min", "Sen", "Sel", "Rab", "Kam", "Jum", "Sab"];
    let months = [
        "Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des",
    ];

    let weekday = weekdays[wib_datetime.weekday().num_days_from_sunday() as usize];
    let month = months[(wib_datetime.month0()) as usize]; // month0 = 0-based

    let day = wib_datetime.day();
    let year = wib_datetime.year();

    format!("{} {}, {} {}", weekday, day, month, year)
}
