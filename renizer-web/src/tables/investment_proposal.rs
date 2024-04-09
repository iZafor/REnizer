use super::*;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct InvestmentProposal {
    pub i_user_id: String,
    pub project_id: String,
    pub investment_amount: f32,
    pub proposal_date: DateTime<Utc>,
    pub proposal_status: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use leptos::leptos_server::ServerFnError;
    use sqlx::mysql::MySqlQueryResult;
    use crate::auth::{ssr::pool, get_user};

    pub async fn get_investment_proposal(
        i_user_id: String, 
        project_id: String, 
        proposal_date: DateTime<Utc>) -> Result<InvestmentProposal, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Investment_Proposal_T WHERE i_user_id = ? AND project_id = ? AND proposal_date = ?")
                .bind(i_user_id)
                .bind(project_id)
                .bind(proposal_date)
                .fetch_one(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn get_all_investment_proposals() -> Result<Vec<InvestmentProposal>, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query_as("SELECT * FROM Investment_Proposal_T")
                .fetch_all(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }

    pub async fn insert_investment_proposal(investment_proposal: InvestmentProposal) -> Result<MySqlQueryResult, ServerFnError> {
        if let Ok(Some(_)) = get_user().await {
            Ok(sqlx::query("INSERT INTO Investment_Proposal_T (i_user_id, project_id, investment_amount, proposal_date, proposal_status) VALUES (?, ?, ?, ?, ?)")
                .bind(investment_proposal.i_user_id)
                .bind(investment_proposal.project_id)
                .bind(investment_proposal.investment_amount)
                .bind(investment_proposal.proposal_date)
                .bind(investment_proposal.proposal_status)

                .execute(&pool()?)
                .await?)
        }else {
            Err(ServerFnError::ServerError("User is not logged in!".into()))
        }
    }
}

