// @generated automatically by Diesel CLI.

diesel::table! {
    hmail_cc_map (hmail_id, address) {
        hmail_id -> Int8,
        address -> Text,
        username -> Nullable<Text>,
    }
}

diesel::table! {
    hmail_recipient_map (hmail_id, address) {
        hmail_id -> Int8,
        address -> Text,
        username -> Nullable<Text>,
    }
}

diesel::table! {
    hmails (hmail_id) {
        hmail_id -> Int8,
        user_id -> Int8,
        outbox -> Bool,
        context_for -> Nullable<Int8>,
        sender -> Text,
        sender_name -> Nullable<Text>,
        subject -> Text,
        sent_at -> Int8,
        received_at -> Int8,
        random_id -> Int8,
        reply_to -> Nullable<Text>,
        reply_to_name -> Nullable<Text>,
        parent -> Nullable<Text>,
        body -> Text,
        hash -> Text,
        pow_classification -> Text,
    }
}

diesel::table! {
    user_whitelists (user_id, address) {
        user_id -> Int8,
        address -> Text,
        place_in -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int8,
        username -> Text,
        password_hash -> Text,
        pow_minimum -> Int4,
        pow_accepted -> Int4,
        pow_personal -> Int4,
    }
}

diesel::joinable!(hmail_cc_map -> hmails (hmail_id));
diesel::joinable!(hmail_recipient_map -> hmails (hmail_id));
diesel::joinable!(hmails -> users (user_id));
diesel::joinable!(user_whitelists -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    hmail_cc_map,
    hmail_recipient_map,
    hmails,
    user_whitelists,
    users,
);
