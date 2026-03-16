mod constants;
mod models;
use crate::{constants::common_costants::localized, controller::get_issues::api_get_issues};
mod command;
mod controller;
mod utils;



#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
        let text: String = api_get_issues().await?;

        match command::get_issues_api::use_api_get_issues(&text).await {
            Ok(_) => println!(
                "{}",
                localized(constants::common_costants::MsgKey::ApiResponseSuccess)
            ),
            Err(e) => eprintln!(
                "{} {}",
                localized(constants::common_costants::MsgKey::ApiResponseError),
                e
            ),
        }
        Ok(())
}
