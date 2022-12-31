// @generated automatically by Diesel CLI.

diesel::table! {
    mail (message_id) {
        message_id -> Text,
        recipients -> Nullable<Array<Nullable<Text>>>,
        sent_by -> Nullable<Text>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (email_address) {
        email_address -> Text,
        mailbox -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    mail,
    users,
);
