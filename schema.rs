// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        #[max_length = 25]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 60]
        salted_password -> Varchar,
        account_closed -> Bool,
        punishment_id -> Nullable<Integer>,
    }
}

diesel::table! {
    account_ranked_info (id) {
        id -> Integer,
        account_id -> Integer,
        standing -> Integer,
    }
}

diesel::table! {
    chat (id) {
        id -> Bigint,
        #[max_length = 255]
        chat_text -> Varchar,
        sent_by_id -> Integer,
        room_id -> Bigint,
    }
}

diesel::table! {
    made_matches (id) {
        id -> Bigint,
        gamemode -> Integer,
    }
}

diesel::table! {
    punishment (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Tinyint,
        end_date -> Nullable<Date>,
        #[max_length = 16]
        evidence_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    room (id) {
        id -> Bigint,
    }
}

diesel::table! {
    room_access (id) {
        id -> Bigint,
        account_id -> Integer,
        room_id -> Bigint,
    }
}

diesel::table! {
    tokens (id) {
        id -> Integer,
        #[max_length = 25]
        token -> Varchar,
        account_id -> Integer,
        valid_until -> Timestamp,
        invalidated -> Bool,
    }
}

diesel::joinable!(account -> punishment (punishment_id));
diesel::joinable!(account_ranked_info -> account (account_id));
diesel::joinable!(chat -> account (sent_by_id));
diesel::joinable!(chat -> room (room_id));
diesel::joinable!(room_access -> account (account_id));
diesel::joinable!(room_access -> room (room_id));
diesel::joinable!(tokens -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    account_ranked_info,
    chat,
    made_matches,
    punishment,
    room,
    room_access,
    tokens,
);
