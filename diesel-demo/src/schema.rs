table! {
    ommu_conversations (conversation_id) {
        conversation_id -> Integer,
        publish -> Integer,
        buddy_talent_id -> Nullable<Integer>,
        member_id -> Nullable<Integer>,
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
