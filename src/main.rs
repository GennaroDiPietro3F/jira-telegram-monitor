#![windows_subsystem = "windows"]

mod constants;
mod models;
use crate::{constants::common_costants::localized, controller::get_issues::api_get_issues};
mod command;
mod controller;
mod utils;
mod config;
use config::logger_initializer::setup_logger;
use log::{info, error, warn};





#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

     if let Err(e) = setup_logger() {
        eprintln!("Errore inizializzazione logger: {}", e);
        return Ok(());
    }

    info!("Logger inizializzato — stdout + file attivi");

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
    loop {
        let text = api_get_issues().await?;

        match command::get_issues_api::use_api_get_issues(&text).await {
            Ok(_) => {
                info!(
                    "{}",
                    localized(constants::common_costants::MsgKey::ApiResponseSuccess)
                );
            }
            Err(e) => {
                error!(
                    "{} {}",
                    localized(constants::common_costants::MsgKey::ApiResponseError),
                    e
                );
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}