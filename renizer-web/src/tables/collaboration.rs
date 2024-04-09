use super::*;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Collaboration {
    pub p_user_id: String,
    pub project_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub role: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_collaboration(
        p_user_id: String, 
        project_id: String, 
        start_date: DateTime<Utc>, 
        role: String) -> Result<Collaboration, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Collaboration_T WHERE p_user_id = ? AND project_id = ? start_date = ? AND role = ?")
                .bind(p_user_id)
                .bind(project_id)
                .bind(start_date)
                .bind(role)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn get_all_collaborations() -> Result<Vec<Collaboration>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Collaboration_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn insert_collaboration(collaboration: Collaboration) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Collaboration_T (p_user_id, project_id, start_date, end_date, role)")
                .bind(collaboration.p_user_id)
                .bind(collaboration.project_id)
                .bind(collaboration.start_date)
                .bind(collaboration.end_date)
                .bind(collaboration.role)

                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }       
    }
}


