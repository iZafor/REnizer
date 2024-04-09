use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectAssociate {
    pub p_user_id: String,
    pub hourly_rate: f32,
    pub working_experience: u32,
    pub associate_type: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_project_associate(p_user_id: String) -> Result<ProjectAssociate, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Project_Associate_T WHERE p_user_id = ?")
                .bind(p_user_id)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  

    pub async fn get_all_project_associates() -> Result<Vec<ProjectAssociate>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM  Project_Associate_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  

    pub async fn insert_project_associate(project_associate: ProjectAssociate) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Project_Associate_T (p_user_id, hourly_rate, working_experience, associate_type) VALUES (?, ?, ?, ?)")
                .bind(project_associate.p_user_id)
                .bind(project_associate.hourly_rate)
                .bind(project_associate.working_experience)
                .bind(project_associate.associate_type)


                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  
}