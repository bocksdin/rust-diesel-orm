// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        created_by -> Int4,
        created_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

diesel::joinable!(articles -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    users,
);
