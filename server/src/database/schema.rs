// @generated automatically by Diesel CLI.

diesel::table! {
    Emails (email_id) {
        email_id -> Nullable<Integer>,
        user_id -> Integer,
        source -> Text,
        email -> Text,
        pow_classification -> Nullable<Text>,
    }
}

diesel::table! {
    Users (user_id) {
        user_id -> Integer,
        username -> Text,
        password_hash -> Text,
        pow_minimum -> Integer,
        pow_accepted -> Integer,
        pow_personal -> Integer,
    }
}

diesel::joinable!(Emails -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(Emails, Users,);
