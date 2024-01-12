// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 0]
        id -> Varchar,
        #[max_length = 0]
        first_name -> Varchar,
        #[max_length = 0]
        last_name -> Varchar,
        #[max_length = 0]
        email -> Varchar,
        #[max_length = 0]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
