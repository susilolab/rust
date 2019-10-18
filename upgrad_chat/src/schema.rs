table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Integer,
    }
}

table! {
    ommu_users (user_id) {
        user_id -> Integer,
        enabled -> Integer,
        verified -> Integer,
        level_id -> Nullable<Integer>,
        language_id -> Nullable<Integer>,
        email -> Varchar,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        displayname -> Varchar,
        password -> Varchar,
        photos -> Nullable<Varchar>,
        salt -> Varchar,
        deactivate -> Integer,
        search -> Integer,
        invisible -> Integer,
        privacy -> Integer,
        comments -> Integer,
        creation_date -> Timestamp,
        creation_ip -> Varchar,
        modified_date -> Timestamp,
        modified_id -> Nullable<Integer>,
        lastlogin_date -> Timestamp,
        lastlogin_ip -> Varchar,
        lastlogin_from -> Varchar,
        update_date -> Timestamp,
        update_ip -> Varchar,
        auth_key -> Nullable<Varchar>,
        jwt_claims -> Nullable<Varchar>,        
    }
}

table! {
    ommu_conversations (conversation_id) {
        conversation_id -> Integer,
        publish -> Integer,
        buddy_talent_id -> Integer,
        member_id -> Integer,
        creator_id -> Integer,
        user_id -> Integer,
        conversation_start -> Nullable<Datetime>,
        conversation_finish -> Nullable<Datetime>,
        creation_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        modified_id -> Integer,
        updated_date -> Nullable<Timestamp>,
    }
}