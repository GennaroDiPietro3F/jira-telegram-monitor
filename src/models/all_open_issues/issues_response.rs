use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JiraIssue {
    pub key: Option<String>,
    pub fields: Option<Fields>,
}
#[derive(Debug, Deserialize)]
pub struct Fields {
    
    pub customfield_10001: Option<Customfield10001>,
    pub customfield_11708: Option<Customfield11708>,
    pub updated: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct Customfield10001 {
    pub currentStatus: Option<CurrentStatus>,
}
#[derive(Debug, Deserialize)]
pub struct Customfield11708 {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentStatus {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JiraSearchResponse {
    pub issues: Vec<JiraIssue>
}
