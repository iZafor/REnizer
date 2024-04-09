use super::*;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct InvestorInvestProject {
    pub i_user_id: String,
    pub project_id: String,
    pub investment_amount: f32,
    pub investment_date: DateTime<Utc>,
}


#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_investor_invest_project(
        i_user_id: String,
        project_id: String,
        investment_date: DateTime<Utc>
    ) -> Result<InvestorInvestProject, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Investor_Invest_T WHERE i_user_id = ? AND project_id = ? AND investment_date = ?")
                .bind(i_user_id)
                .bind(project_id)
                .bind(investment_date)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn get_all_investor_invest_project() -> Result<Vec<InvestorInvestProject>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Investor_Invest_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn insert_investor_invest_project(investor_invest_project: InvestorInvestProject) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Investor_Invest_Project_T (i_user_id, project_id, investment_amount, investment_date)")
                .bind(investor_invest_project.i_user_id)
                .bind(investor_invest_project.project_id)
                .bind(investor_invest_project.investment_amount)
                .bind(investor_invest_project.investment_date)
                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }
}
