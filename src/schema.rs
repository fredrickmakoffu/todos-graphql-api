// @generated automatically by Diesel CLI.

table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        completed -> Bool,
    }
}