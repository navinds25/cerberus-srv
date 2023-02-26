// @generated automatically by Diesel CLI.

diesel::table! {
    credentials (title) {
        title -> Nullable<Text>,
        collection -> Nullable<Text>,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        url -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    master_security (id) {
        id -> Text,
        root_password -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    credentials,
    master_security,
    users,
);
