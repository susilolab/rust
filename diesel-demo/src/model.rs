use crate::schema::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[table_name="ommu_conversations"]
pub struct Conversation {
    pub conversation_id: i32,
    pub publish: i32,
    pub buddy_talent_id: Option<i32>,
    pub member_id: Option<i32>,
    pub creator_id: i32,
    pub user_id: i32,
    pub conversation_start: Option<chrono::NaiveDateTime>,
    pub conversation_finish: Option<chrono::NaiveDateTime>,
    pub creation_date: Option<chrono::NaiveDateTime>,
    pub modified_date: Option<chrono::NaiveDateTime>,
    pub modified_id: i32,
    pub updated_date: Option<chrono::NaiveDateTime>,
}

pub struct NewConversation {
    pub conversation_id: i32,
    pub publish: i32,
    pub buddy_talent_id: Option<i32>,
    pub member_id: Option<i32>,
    pub creator_id: i32,
    pub user_id: i32,
    pub creation_date: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, PartialEq)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub published: i32,
}