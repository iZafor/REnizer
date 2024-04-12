use super::*;

#[cfg(feature = "ssr")]
use sqlx::*;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub energy_rate_kwh: Option<Decimal>,
    pub produced_energy_kwh: Option<Decimal>,
    pub energy_sold_kwh: Option<Decimal>,
    pub total_cost: Option<Decimal>,
    pub org_restricted: bool,
    pub m_p_user_id: Option<String>,
    pub creation_date: DateTime<Utc>,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_project(project_id: String) -> Result<Option<Project>, ServerFnError> {
        // if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Project_T WHERE project_id = ?")
                .bind(project_id)
                .fetch_optional(&pool()?)
                .await?
            )
        // }else {
        //     Err(ServerFnError::ServerError("User is not logged in!".into()))
        // }
    }   

    pub async fn get_all_projects() -> Result<Vec<Project>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM  Project_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   

    pub async fn insert_project_contributor(project: Project) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Project_Contributor_Skill_T (project_id, name, description, location, start_date, end_date, status, energy_rate_kwh, produced_energy_kwh, energy_sold_kwh, total_cost, org_restricted, m_p_user_id, creation_date) VALUES (SUBSTR(UUID(), 1, 8), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(project.name)
                .bind(project.description)
                .bind(project.location)
                .bind(project.start_date)
                .bind(project.end_date)
                .bind(project.status)
                .bind(project.energy_rate_kwh)
                .bind(project.produced_energy_kwh)
                .bind(project.energy_sold_kwh)
                .bind(project.total_cost)
                .bind(project.org_restricted)
                .bind(project.m_p_user_id)
                .bind(project.creation_date)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }   
}