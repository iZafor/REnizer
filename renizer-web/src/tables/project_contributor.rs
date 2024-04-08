use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ProjectContributor {
    pub c_p_user_id: String,
    pub working_department: Option<String>,
}
