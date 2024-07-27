// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        gid -> Int4,
        paid -> Bool,
    }
}
