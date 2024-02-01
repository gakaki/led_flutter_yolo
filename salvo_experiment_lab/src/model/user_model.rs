use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}
