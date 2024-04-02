use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Organization {
    pub org_id: String,
    pub name: String,
    pub email: Option<String>,
    pub location: Option<String>
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct OrganizationContact {
    pub org_id: String,
    pub contact: String
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub contact_number: Option<String>,
    pub user_type: char,
    pub org_id: Option<String>
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Investor {
    pub i_user_id: String,
    pub investor_type: String
}

#[derive(PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectAssociate {
    pub p_user_id: String,
    pub hourly_rate: Option<f32>,
    pub working_experience: Option<u32>,
    pub associate_type: char
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectManager {
    pub m_p_user_id: String
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectContributor {
    pub c_p_user_id: String,
    pub working_department: Option<String>
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectContributorSkill {
    pub c_p_user_id: String,
    pub skill: String
}

#[derive(PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub energy_rate_kwh: Option<f32>,
    pub produced_energy_kwh: Option<f32>,
    pub energy_sold_kwh: Option<f32>,
    pub total_cost: Option<f32>,
    pub org_restricted: u8,
    pub m_p_user_id: Option<String>,
    pub creation_date: DateTime<Utc>
}

#[derive(PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct InvestmentProposal {
    pub i_user_id: String,
    pub project_id: String,
    pub investment_amount: f32,
    pub proposal_date: DateTime<Utc>,
    pub proposal_status: String
}

#[derive(PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct InvestorInvest {
    pub i_user_id: String,
    pub project_id: String,
    pub investment_amount: f32,
    pub investment_date: DateTime<Utc>
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Collaboration {
    pub p_user_id: String,
    pub project_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub role: String
}

#[derive(PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct CollaborationTask {
    pub p_user_id: String,
    pub project_id: String,
    pub task_name: String,
    pub assigned_date: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub delivery_date: Option<DateTime<Utc>>,
    pub expected_hour: f32,
    pub hour_taken: Option<f32>,
    pub expected_day: u8
}
