use super::*;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Organization {
    pub org_id: String,
    pub name: String,
    pub email: Option<String>,
    pub location: Option<String>,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_organization(org_id: String) -> Result<Organization, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Organization_T WHERE org_id = ?")
                .bind(org_id)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  

    pub async fn get_all_organizations() -> Result<Vec<Organization>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Organization_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  

    pub async fn insert_organization(organization: Organization) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Organization_T (org_id, name, email, location) VALUES (SUBSTR(UUID(), 1, 8), ?, ?, ?)")
                .bind(organization.org_id)
                .bind(organization.name)
                .bind(organization.email)
                .bind(organization.location)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  
}

