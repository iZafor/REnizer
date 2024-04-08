use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectAssociate {
    pub p_user_id: String,
    pub hourly_rate: Option<f32>,
    pub working_experience: Option<u32>,
    pub associate_type: String,
}
