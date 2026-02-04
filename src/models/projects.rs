use serde::{Deserialize,Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime,Utc};


#[derive(Debug,Serialize,Deserialize)]
pub struct Project{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:Option<ObjectId>,
    pub title:String,
    pub description:String,
    pub tech_stack:Vec<String>,
    pub github_url:Option<String>,
    pub live_url:Option<String>,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>
}