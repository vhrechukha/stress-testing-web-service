// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        age -> Int4,
        #[max_length = 100]
        position -> Varchar,
    }
}
