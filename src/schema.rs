// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Integer,
        original_url -> Text,
        short_code -> Text,
        created_at -> Timestamp,
    }
}
