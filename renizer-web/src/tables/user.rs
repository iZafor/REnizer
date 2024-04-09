use super::*;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub password: String,
    pub email: String,
    pub contact_number: Option<String>,
    pub user_type: String,
    pub org_id: Option<String>,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_user_with_id(id: String) -> Result<User, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM User_T WHERE id = ?")
                .bind(id)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn get_user_with_email(email: String) -> Result<User, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM User_T WHERE email = ?")
                .bind(email)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM User_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   

    pub async fn insert_user(user: User) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO User_T (user_id, first_name, last_name, email, contact_number, user_type, org_id) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(user.user_id)
                .bind(user.first_name)
                .bind(user.last_name)
                .bind(user.email)
                .bind(user.contact_number)
                .bind(user.user_type)
                .bind(user.org_id)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   
}