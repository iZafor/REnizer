use super::*;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct CollaborationTask {
    pub p_user_id: String,
    pub project_id: String,
    pub task_name: String,
    pub assigned_date: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub delivery_date: Option<DateTime<Utc>>,
    pub expected_hour: f32,
    pub hour_taken: Option<f32>,
    pub expected_day: u8,
}
