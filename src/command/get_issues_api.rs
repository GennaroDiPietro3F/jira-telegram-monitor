use crate::constants::common_costants::TICKET;
use crate::controller::send_message_to_bot::send_message_to_bot_func;
use crate::{
    constants::{self, common_costants::localized},
    models::all_open_issues::{
        coverted_issues_response::ConvertedIssues,
        issues_response::{JiraIssue, JiraSearchResponse},
    },
    utils,
};
use constants::common_costants::MsgKey;
use constants::common_costants::NO_SQUAD;
use constants::common_costants::NO_STATUS;
use log::{error, info};
use rfd::MessageLevel;
use tokio::time::sleep;
use std::fs;
use tokio;
use tokio::time::Duration;
/// ### 🍺   Purpose:  
/// Serialize api response
///
///### 🌶️🥵 Description:  
/// This fn serialize the api response into a custom struct "JiraSearchResponse" that contains
///
/// some fields of the response that we need to work with.
///
/// #### *️⃣ Parameters:   
/// - **```&str```**  the response from the API
/// #### 🍗 Returns
/// - **```JiraSearchResponse```**  A struct containing the deserialized response from the API:
pub fn serialize_api_response(text: &str) -> JiraSearchResponse {
    let jira_data_result = serde_json::from_str::<JiraSearchResponse>(text);
    match jira_data_result {
        Ok(data) => data,
        Err(e) => {
            let error_message = localized(constants::common_costants::MsgKey::DeserializationError);
            utils::show_message_dialog(error_message, error_message, MessageLevel::Error);
            panic!("{} {:?}", error_message, e);
        }
    }
}

/// ### 🍺   Purpose:  
/// This function iterates over the issues
///
///### 🌶️🥵 Description:  
/// This function iterates over the issues in the response and extracts the key, status, and squad from each issue.
///
/// #### *️⃣ Parameters:   
/// - **```&JiraSearchResponse```**  Struct containing the response from the API
///
/// #### 🍗 Returns:
/// - **```Vec <ConvertedIssues>```**  A vector of ConvertedIssues structs containing the key, status, and squad of each issue
fn iterate_on_response(jira_data: &JiraSearchResponse) -> Vec<ConvertedIssues> {
    let mut issues_vec: Vec<ConvertedIssues> = Vec::new();
    for issue in &jira_data.issues {
        let issue_key = issue.key.as_deref().unwrap_or("No Key");

        let status = get_status(issue);

        let squad = get_squad(issue);

        let updated = get_updated(issue);
        issues_vec.push(ConvertedIssues::new(
            issue_key.to_string(),
            status.to_string(),
            squad.to_string(),
            updated.to_string(),
        ));
    }

    issues_vec
}

/// ### 🍺   Purpose:  
/// Gets the status of the issue
///
///### 🌶️🥵 Description:  
/// This internal function extracts the status of the issue from the fields of the issue.
///
/// #### *️⃣ Parameters:   
/// - **```&JiraIssue```**  Struct containing a portion of the response from the API
/// #### 🍗 Returns:
/// - **```&str```**  The status of the issue, or "No Status" if not found
fn get_status(issue: &JiraIssue) -> &str {
    issue
        .fields
        .as_ref()
        .and_then(|fields| fields.customfield_10001.as_ref())
        .and_then(|cf| cf.currentStatus.as_ref())
        .and_then(|cs| cs.status.as_deref())
        .unwrap_or(NO_STATUS)
}

/// ### 🍺   Purpose:  
/// Gets the updated time of the issue
/// ///### 🌶️🥵 Description:
///   This internal function extracts the updated time of the issue from the fields of the issue.
/// /// #### *️⃣ Parameters:
/// - **```issue: &JiraIssue```**  Struct containing a portion of the response from the API
/// #### 🍗 Returns:
/// - **```&str```**  The updated time of the issue, or "No Updated" if not found
fn get_updated(issue: &JiraIssue) -> &str {
    issue
        .fields
        .as_ref()
        .and_then(|fields| fields.updated.as_deref())
        .unwrap_or("No Updated")
}
/// ### 🍺   Purpose:
/// Gets the squad of the issue
///
///### 🌶️🥵 Description:
/// This internal function extracts the squad of the issue from the fields of the issue.
///
/// #### *️⃣ Parameters:
/// - **```issue: &JiraIssue```**  Struct containing a portion of the response from the API
fn get_squad(issue: &JiraIssue) -> &str {
    issue
        .fields
        .as_ref()
        .and_then(|fields| fields.customfield_11708.as_ref())
        .and_then(|cf| cf.name.as_deref())
        .unwrap_or(NO_SQUAD)
}

/// ### 🍺   Purpose
/// Converts the vector of ConvertedIssues to a JSON string
/// ### 🌶️🥵 Description:
/// This function takes a vector of ConvertedIssues and converts it to a pretty JSON string.
/// #### *️⃣ Parameters:
/// - **```issues_vec: Vec<ConvertedIssues>```**  The vector of ConvertedIssues to convert
/// #### 🍗 Returns:
/// - **```String```**  A pretty JSON string representation of the issues vector
fn convert_my_response_to_json_string(issues_vec: &Vec<ConvertedIssues>) -> String {
    match serde_json::to_string_pretty(&issues_vec) {
        Ok(json) => json,

        Err(e) => {
            let error_message = localized(constants::common_costants::MsgKey::SerializationError);
            utils::show_message_dialog(error_message, error_message, MessageLevel::Error);
            panic!("{} {:?}", error_message, e)
        }
    }
}
/// ### 🍺   Purpose
/// Writes the JSON string to a file
/// ### 🌶️🥵 Description:
/// This function writes the JSON string to a file named "issues.json" in the project root directory.
/// #### *️⃣ Parameters:
/// - **```json_string: &str```**  The JSON string to write to the file
/// #### 🍗 Returns:
/// - *```()```*  Nothing, but prints a success message or panics on error
pub fn write_issues_to_file(json_string: &str) {
    match fs::write(constants::common_costants::JSON_FILE_NAME, json_string) {
        Ok(_) => info!(
            "{}",
            localized(constants::common_costants::MsgKey::FileWrittenWithSuccess)
        ),
        Err(e) => {
            let error_message = localized(constants::common_costants::MsgKey::WritingFileError);
            utils::show_message_dialog(error_message, error_message, MessageLevel::Error);
            panic!("{} {:?}", error_message, e);
        }
    }
}
async fn compare_glbam_tickets(newest_issues_str: &str, newest_issues: &[ConvertedIssues]) {
    let mut changes_in_tickets = Vec::new();

    match fs::read_to_string(constants::common_costants::JSON_FILE_NAME) {
        /* Check if the file exists and reads it otherwise it panics showing an error message at GUI */
        /* -------------------------------------------------------------------------------- */
        Ok(oldest_json) => {
            let oldest_issues = match serde_json::from_str::<Vec<ConvertedIssues>>(&oldest_json) {
                Ok(issues) => issues,
                Err(e) => {
                    let error_message =
                        localized(constants::common_costants::MsgKey::DeserializationError);
                    utils::show_message_dialog(error_message, error_message, MessageLevel::Error);
                    panic!("{} {:?}", error_message, e);
                }
            };
            /* -------------------------------------------------------------------------------- */

            /*  Convert the newest and oldest issues to HashMaps for easier comparison
            This allows us to quickly check if a ticket has been added, removed, or changed */
            /* -------------------------------------------------------------------------------- */
            let map_newest_issues: std::collections::HashMap<_, _> = newest_issues
                .iter()
                .map(|t| (t.glbam_ticket.clone(), t))
                .collect();
            let map_oldest: std::collections::HashMap<_, _> = oldest_issues
                .iter()
                .map(|t| (t.glbam_ticket.clone(), t))
                .collect();
            /*--------------------------------------------------------------------------------*/

            /* For each ticket in the newest issues, check if it exists in the oldest issues
            if not, it means it's a new ticket and we add it to the changes_in_tickets vector so we can print it later in the GUI */
            /* -------------------------------------------------------------------------------- */
            for (newest_ticket, newest_issue) in map_newest_issues.iter() {
                if map_oldest.get(newest_ticket).is_none() {
                    changes_in_tickets.push(format!(
                        "{} {}",
                        newest_issue.glbam_ticket,
                        &localized(MsgKey::TicketAdded)
                    ));
                }
            }
            /* -------------------------------------------------------------------------------- */

            for (oldest_ticket, oldest_issue) in map_oldest.iter() {
                if map_newest_issues.get(oldest_ticket).is_some() {
                    let current_ticket = map_newest_issues.get(oldest_ticket).unwrap();
                    let newest_status = &current_ticket.glbam_status;
                    let newest_squad = &current_ticket.glbam_squad;
                    let newest_updated = &current_ticket.updated;

                    if *newest_status != oldest_issue.glbam_status {
                        changes_in_tickets.push(format!(
                            "{} {} {} {} {}",
                            current_ticket.glbam_ticket,
                            &localized(MsgKey::TicketStatusChanged),
                            oldest_issue.glbam_status,
                            &localized(MsgKey::To),
                            newest_status
                        ));
                    }

                    if *newest_squad != oldest_issue.glbam_squad {
                        changes_in_tickets.push(format!(
                            "{} {} {} {} {}",
                            current_ticket.glbam_ticket,
                            &localized(MsgKey::SquadChanged),
                            oldest_issue.glbam_squad,
                            localized(MsgKey::To),
                            newest_squad
                        ));
                    }

                    if *newest_updated != oldest_issue.updated {
                        changes_in_tickets.push(format!(
                            "{} {} ",
                            current_ticket.glbam_ticket,
                            localized(MsgKey::UpdatedTime)
                        ));
                    }
                } else {
                    info!(
                        "{} {} {}",
                        TICKET,
                        oldest_ticket,
                        localized(MsgKey::Removed)
                    );
                }
            }

            if !changes_in_tickets.is_empty() {
                for change in &changes_in_tickets {
                    info!("{}", change);
                }
                let changes_in_tickets_clone = changes_in_tickets.clone();
                tokio::spawn(async move {
                    utils::show_message_dialog(
                        "Modifiche nei ticket rilevate!",
                        &changes_in_tickets.join("\n"),
                        MessageLevel::Warning,
                    );
                });
                bot_tele(changes_in_tickets_clone).await;
                write_issues_to_file(newest_issues_str);
            }
        }

        Err(e) => {
            info!("Error: {}", e);
            write_issues_to_file(newest_issues_str);
        }
    }
}

/// ### 🍺   Purpose
/// Uses the API to get issues and processes the response
/// ### 🌶️🥵 Description:
/// This function calls the API to get issues, serializes the response, iterates over it to extract relevant data,
/// converts it to a JSON string, and writes it to a file.
/// #### *️⃣ Parameters:
/// - **```text: &str```**  The response from the API
/// #### 🍗 Returns:
/// - **```Result<(), String>```**  Ok if successful, Err with an error message if not
pub async fn use_api_get_issues(text: &str) -> Result<(), String> {
    let jira_data = serialize_api_response(text);
    let issues_vec = iterate_on_response(&jira_data);
    let json_string = convert_my_response_to_json_string(&issues_vec);
    compare_glbam_tickets(&json_string, &issues_vec).await;

    Ok(())
}
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
async fn bot_tele (changes_in_tickets_clone:Vec<String>) {
    const MAX_RETRIES: u8 = 3;
let mut attempts = 0;

loop {
    match send_message_to_bot_func(&changes_in_tickets_clone.join("\n")).await {
        Ok(body) => {
            info!("Messaggio Telegram inviato con successo. Risposta: {}", body);
            break; 
        }
        Err(e) => {
            error!("Tentativo fallito ({} di {}): {:?}", attempts + 1, MAX_RETRIES, e);

            if attempts >= MAX_RETRIES - 1 || !e.is_request() {

                error!("ERRORE DEFINITIVO nell'invio del messaggio Telegram: {:?}", e);
                break;
            }

            attempts += 1;
            let wait_time = Duration::from_secs(2u64.pow(attempts as u32));
            error!("Aspetto {:?} prima di riprovare...", wait_time);
            sleep(wait_time).await;
        }
    }
}
}
