// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        premission_level -> Nullable<Tinyint>,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}
