#![allow(clippy::all)]
#![allow(warnings)]
// @generated automatically by Diesel CLI.

diesel::table! {
    EmailCcMap (email_id) {
        email_id -> Integer,
        email -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    EmailToMap (email_id) {
        email_id -> Integer,
        email -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    Emails (email_id) {
        email_id -> Integer,
        user_id -> Integer,
        source -> Text,
        subject -> Text,
        sent_at -> BigInt,
        received_at -> BigInt,
        mime_version -> Text,
        content_type -> Text,
        reply_to -> Nullable<Text>,
        reply_to_name -> Nullable<Text>,
        parent -> Nullable<Text>,
        body -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(EmailCcMap, EmailToMap, Emails, Users,);
