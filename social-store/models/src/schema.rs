diesel::table! {
    accounts {
        id -> Integer,
        email -> Text,
        password -> Text,
    }
}
