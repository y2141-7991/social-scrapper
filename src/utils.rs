use chrono::{DateTime, Utc};

pub fn convert_string_to_datetime(datetime_string: &str) -> DateTime<Utc> {
    let datetime = DateTime::parse_from_rfc3339(&datetime_string).unwrap().into();
    datetime
}
