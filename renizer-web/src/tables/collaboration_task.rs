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

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_collaboration_task(
        p_user_id: String,
        project_id: String,
        task_name: String, 
        assigned_date: DateTime<Utc>) -> Result<CollaborationTask, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Collaboration_Task_T WHERE p_user_id = ? AND project_id = ? AND task_name = ? AND assigned_date = ?")
                .bind(p_user_id)
                .bind(project_id)
                .bind(task_name)
                .bind(assigned_date)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    } 

    pub async fn get_all_collaboration_tasks(
        p_user_id: String,
        project_id: String,
        task_name: String, 
        assigned_date: DateTime<Utc>) -> Result<Vec<CollaborationTask>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Collaboration_Task_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    } 

    pub async fn insert_collaboration_task(collaboration_task: CollaborationTask) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Collaboration_Task_T (p_user_id, project_id, task_name, assigned_date, start_date, delivery_date, expected_hour, hour_taken, expected_day) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(collaboration_task.p_user_id)
                .bind(collaboration_task.project_id)
                .bind(collaboration_task.task_name)
                .bind(collaboration_task.assigned_date)
                .bind(collaboration_task.start_date)
                .bind(collaboration_task.delivery_date)
                .bind(collaboration_task.expected_hour)
                .bind(collaboration_task.hour_taken)
                .bind(collaboration_task.expected_day)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    } 
}

