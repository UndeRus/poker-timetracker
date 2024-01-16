use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningTask {
    pub priority: u64,
    pub id: String,
}

impl PlanningTask {
    pub fn new(id: &str, priority: u64) -> Self {
        PlanningTask {
            id: id.to_string(),
            priority,
        }
    }
}

#[derive(PartialEq, Debug, Serialize)]
pub struct TimeRecord {
    pub task_id: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
