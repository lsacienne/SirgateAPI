// @generated automatically by Diesel CLI.

diesel::table! {
    achievement (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    client (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password -> Text,
        salt -> Text,
        role_id -> Int4,
        rank_id -> Int4,
    }
}

diesel::table! {
    client_achievement (client_id, achievement_id) {
        client_id -> Uuid,
        achievement_id -> Uuid,
    }
}

diesel::table! {
    friend (client1_id, client2_id) {
        client1_id -> Uuid,
        client2_id -> Uuid,
    }
}

diesel::table! {
    rank (id) {
        id -> Int4,
        rank_name -> Text,
    }
}

diesel::table! {
    role (role_id) {
        role_id -> Int4,
        #[max_length = 50]
        role_name -> Varchar,
    }
}

diesel::joinable!(client -> rank (rank_id));
diesel::joinable!(client -> role (role_id));
diesel::joinable!(client_achievement -> achievement (achievement_id));
diesel::joinable!(client_achievement -> client (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    achievement,
    client,
    client_achievement,
    friend,
    rank,
    role,
);