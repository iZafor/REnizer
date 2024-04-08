use super::*;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub contact_number: Option<String>,
    pub user_type: String,
    pub org_id: Option<String>,
}
