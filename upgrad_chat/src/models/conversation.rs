use crate::schema::*;

#[derive(Debug, Queryable,Serialize, Deserialize, Insertable)]
#[table_name="ommu_conversations"]
pub struct Conversation {
    pub conversation_id: i32,
    publish: i32,
    buddy_talent_id: i32,
    member_id: i32,
    creator_id: i32,
    user_id: i32,
    conversation_start: Option<chrono::NaiveDateTime>,
    conversation_finish: Option<chrono::NaiveDateTime>,
    creation_date: Option<chrono::NaiveDateTime>,
    modified_date: Option<chrono::NaiveDateTime>,
    modified_id: i32,
    updated_date: Option<chrono::NaiveDateTime>,
}
