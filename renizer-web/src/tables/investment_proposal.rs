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
