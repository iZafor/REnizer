use leptos::*;
use crate::tables::User;
#[cfg(feature = "ssr")]
use ssr::auth;

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use axum::async_trait;
    use axum_session::SessionMySqlPool;
    use axum_session_auth::Authentication;
    use sqlx::{MySqlPool, query_as};
    use anyhow::Error;

    pub type AuthSession = axum_session_auth::AuthSession<User, String, SessionMySqlPool, MySqlPool>;

    #[async_trait]
    impl Authentication<User, String, MySqlPool> for User {
        async fn load_user(userid: String, pool: Option<&MySqlPool>) -> Result<User, Error> {
            Ok(query_as("SELECT * FROM User_T WHERE id = ?")
                .bind(userid)
                .fetch_one(pool.unwrap())
                .await?
            )
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing".into()))
    }

    pub fn pool() -> Result<MySqlPool, ServerFnError> {
        use_context::<MySqlPool>()
            .ok_or_else(|| ServerFnError::ServerError("Database connection pool missing".into()))
    }
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    Ok(auth()?.current_user)
}

#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
    Ok(auth()?.logout_user())
}