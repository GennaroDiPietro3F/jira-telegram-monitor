use crate::constants::request_constants::*;
use crate::models::user_data::user::User;
use reqwest::Error;
pub async fn api_get_issues() -> Result<String, Error> {
    let utente = User::get_utente_from_json();
    let client = reqwest::Client::new();
    let response = client
        .get(JIRA_GET_ISSUES_OPEN_URL)
        .query(&QUERY_PARAMS_JIRA_ISSUES)
        .header(ACCEPT_HEADER_KEY, ACCEPT_HEADER_VALUE)
        .header(CONTENT_TYPE_HEADER_KEY, CONTENT_TYPE_HEADER_VALUE)
        .basic_auth(utente.user, Some(utente.password))
        .send()
        .await?;

    let body = response.text().await?; 
    Ok(body)
}