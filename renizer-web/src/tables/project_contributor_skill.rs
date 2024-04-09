use super::*;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectContributorSkill {
    pub c_p_user_id: String,
    pub skill: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_project_contributor_skills(c_p_user_id: String) -> Result<Vec<ProjectContributorSkill>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Project_Contributor_Skill_T WHERE c_p_user_id = ?")
                .bind(c_p_user_id)
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  

    pub async fn get_all_project_contributor_skills() -> Result<Vec<ProjectContributorSkill>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM  Project_Contributor_Skill_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  

    pub async fn insert_project_contributor_skill(project_contributor_skill: ProjectContributorSkill) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Project_Contributor_Skill_T (c_p_user_id, skill) VALUES (?, ?)")
                .bind(project_contributor_skill.c_p_user_id)
                .bind(project_contributor_skill.skill)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }  
}
