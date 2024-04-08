use super::*;

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
    pub energy_rate_kwh: Option<f32>,
    pub produced_energy_kwh: Option<f32>,
    pub energy_sold_kwh: Option<f32>,
    pub total_cost: Option<f32>,
    pub org_restricted: u8,
    pub m_p_user_id: Option<String>,
    pub creation_date: DateTime<Utc>,
}
