use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectManager {
    pub m_p_user_id: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_project_manager(m_p_user_id: String) -> Result<ProjectManager, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Project_Manager_T WHERE m_p_user_id = ?")
                .bind(m_p_user_id)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   

    pub async fn get_all_project_managers() -> Result<Vec<ProjectManager>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM  Project_Manager_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   

    pub async fn insert_project_manager(project_manager: ProjectManager) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Project_Manager_T (m_p_user_id) VALUES (?)")
                .bind(project_manager.m_p_user_id)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   
}