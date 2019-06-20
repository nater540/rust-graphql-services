table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        uuid -> Varchar,
        email -> Varchar,
        password_digest -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
