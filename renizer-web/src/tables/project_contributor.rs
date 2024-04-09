use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectContributor {
    pub c_p_user_id: String,
    pub working_department: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_project_contributor(c_p_user_id: String) -> Result<ProjectContributor, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Project_Contributor_T WHERE c_p_user_id = ?")
                .bind(c_p_user_id)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   

    pub async fn get_all_project_contributors() -> Result<Vec<ProjectContributor>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM  Project_Contributor_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   

    pub async fn insert_project_contributor(project_contributor: ProjectContributor) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Project_Contributor_Skill_T (c_p_user_id, working_department) VALUES (?, ?)")
                .bind(project_contributor.c_p_user_id)
                .bind(project_contributor.working_department)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   
}