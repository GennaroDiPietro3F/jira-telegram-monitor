#![windows_subsystem = "windows"]

mod constants;
mod models;
use crate::{constants::common_costants::localized, controller::get_issues::api_get_issues};
mod command;
mod controller;
mod utils;
use std::io::{stdout, Write};

use log::{info, error, LevelFilter};
use simple_logging::{log_to_file, log_to};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {


    let log_file_path = "app_issues.log"; 

    log_to(std::io::stdout(),LevelFilter::Info);
    info!("Log in console avviato con successo");
    stdout().flush().unwrap();
    
    #[cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
    match log_to_file(log_file_path, LevelFilter::Info) {
        Ok(_) => info!("Logger inizializzato con successo. I messaggi saranno scritti in {}", log_file_path),
        Err(e) => {
            error!("Errore nell'inizializzazione del logger: {}", e);
            return Ok(());
        }
    }

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