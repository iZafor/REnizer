use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Investor {
    pub i_user_id: String,
    pub investor_type: String,
}
