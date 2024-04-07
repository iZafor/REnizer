use axum::extract::FromRef;
use leptos::LeptosOptions;
use leptos_router::RouteListing;
use sqlx::MySqlPool;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: MySqlPool,
    pub routes: Vec<RouteListing>,
}
