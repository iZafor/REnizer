use super::*;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Organization {
    pub org_id: String,
    pub name: String,
    pub email: Option<String>,
    pub location: Option<String>,
}
