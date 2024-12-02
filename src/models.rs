use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}