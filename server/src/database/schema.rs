#![allow(clippy::all)]
#![allow(warnings)]
// @generated automatically by Diesel CLI.

diesel::table! {
    HmailCcMap (hmail_id, address) {
        hmail_id -> Integer,
        address -> Text,
        username -> Nullable<Text>,
    }
}

diesel::table! {
    HmailRecipientsMap (hmail_id, address) {
        hmail_id -> Integer,
        address -> Text,
        username -> Nullable<Text>,
    }
}

diesel::table! {
    Hmails (hmail_id) {
        hmail_id -> Integer,
        user_id -> Integer,
        sender -> Text,
        subject -> Text,
        sent_at -> BigInt,
        received_at -> BigInt,
        reply_to -> Nullable<Text>,
        reply_to_name -> Nullable<Text>,
        parent -> Nullable<Text>,
        body -> Text,
        hash -> Text,
        pow_classification -> Text,
    }
}

diesel::table! {
    UserWhitelists (user_id, address) {
        user_id -> Integer,
        address -> Text,
        place_in -> Text,
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

diesel::joinable!(HmailCcMap -> Hmails (hmail_id));
diesel::joinable!(HmailRecipientsMap -> Hmails (hmail_id));
diesel::joinable!(Hmails -> Users (user_id));
diesel::joinable!(UserWhitelists -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    HmailCcMap,
    HmailRecipientsMap,
    Hmails,
    UserWhitelists,
    Users,
);
