use chrono::NaiveDateTime;

pub fn convert_string_to_datetime(datetime_string: &str) -> NaiveDateTime {
    let datetime = NaiveDateTime::parse_from_str(datetime_string, "%Y-%m-%dT%H:%M:%S.%fZ").unwrap();
    datetime
}
