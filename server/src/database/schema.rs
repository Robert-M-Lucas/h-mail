#![allow(clippy::all)]
#![allow(warnings)]
// @generated automatically by Diesel CLI.

diesel::table! {
    EmailCcMap (rowid) {
        rowid -> Integer,
        email_id -> Integer,
        email -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    EmailToMap (rowid) {
        rowid -> Integer,
        email_id -> Integer,
        email -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    Emails (email_id) {
        email_id -> Integer,
        user_id -> Integer,
        subject -> Text,
        sent_at -> Integer,
        received_at -> Integer,
        mime_version -> Text,
        content_type -> Text,
        reply_to -> Nullable<Text>,
        parent -> Nullable<Text>,
        hash -> Text,
        pow_classification -> Text,
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

diesel::joinable!(EmailCcMap -> Emails (email_id));
diesel::joinable!(EmailToMap -> Emails (email_id));
diesel::joinable!(Emails -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    EmailCcMap,
    EmailToMap,
    Emails,
    Users,
);
