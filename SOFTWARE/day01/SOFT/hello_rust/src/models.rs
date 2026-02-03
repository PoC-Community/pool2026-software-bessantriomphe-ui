use serde::{Serialize, Deserialize};
use chrono::{DateTime,Utc};

#[derive(Serialize, Deserialize, Debug)]
pub enum Priority{
    Low,
    Medium,
    High
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub priority: Priority,
    pub tags: Vec<String>
}