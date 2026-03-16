use sys_locale::get_locale;

pub const JSON_FILE_NAME: &str = "issues_oldest.json";
pub const NO_SQUAD: &str = "No Squad";
pub const NO_STATUS: &str = "No Status";
pub const TICKET: &str = "Ticket";
pub enum MsgKey {
    SerializationError,
    DeserializationError,
    WritingFileError,
    FileWrittenWithSuccess,
    ApiResponseSuccess,
    ApiResponseError,
    TicketStatusChanged,
    SquadChanged,
    To,
    TicketAdded,
    Removed,
    UpdatedTime,
    ErrorWritingFile,
    ErrorProcessingFile

}


// ITALIANO
pub mod it {
    pub const SERIALIZATION_ERROR: &str = "Errore nella serializzazione del JSON";
    pub const DESERIAZLIZATION_ERROR: &str = "Errore nella deserializzazione del JSON";
    pub const WRITING_FILE_ERROR: &str = "Errore durante la scrittura del file";
    pub const FILE_WRITTEN_WITH_SUCCESS: &str ="File issues.json scritto con successo!";
    pub const API_RESPONSE_SUCCESS: &str = "Risposta API elaborata con successo!";
    pub const API_RESPONSE_ERROR: &str = "Errore nell'elaborazione della risposta API";
    pub const TICKET_STATUS_CHANGED: &str = "Status cambiato da";
    pub const TO : &str = "a";
    pub const SQUAD_CHANGED: &str = "Squadra cambiata da";
    pub const TICKET_ADDED: &str = "Ticket aggiunto";
    pub const REMOVED : &str = "è stato rimosso";
    pub const UPDATED_TIME : &str = "Ultimo aggiornamento cambiato";
    pub const ERROR_WRITING_FILE : &str = "Errore durante la scrittura del file";
    pub const ERROR_PROCESSING_FILE : &str = "Errore durante l'elaborazione del file";
}

pub mod en {
    pub const SERIALIZATION_ERROR: &str = "Error in JSON serialization";
    pub const DESERIAZLIZATION_ERROR: &str = "Error in JSON deserialization";
    pub const WRITING_FILE_ERROR: &str = "Error while writing the file";
    pub const FILE_WRITTEN_WITH_SUCCESS: &str = "File issues.json written successfully!";
    pub const API_RESPONSE_SUCCESS: &str = "API response processed successfully!";
    pub const API_RESPONSE_ERROR: &str = "Error processing API response";
    pub const TICKET_STATUS_CHANGED: &str = "Status changed from";
    pub const SQUAD_CHANGED: &str = "Squad changed from";
    pub const TO: &str = "to";
    pub const TICKET_ADDED: &str = "Ticket added";
    pub const REMOVED: &str = "has been removed";
    pub const UPDATED_TIME : &str = "Last updated changed";
    pub const ERROR_WRITING_FILE : &str = "Error while writing the file";
    pub const ERROR_PROCESSING_FILE : &str = "Error while processing the file";
}


fn get_lang() -> &'static str {
    match get_locale() {
        Some(locale) if locale.starts_with("it") => "it",
        _ => "en",
    }
}


pub fn localized(key: MsgKey) -> &'static str {
    match get_lang() {
        "en" => match key {
            MsgKey::SerializationError => en::SERIALIZATION_ERROR,
            MsgKey::DeserializationError => en::DESERIAZLIZATION_ERROR,
            MsgKey::WritingFileError => en::WRITING_FILE_ERROR,
            MsgKey::FileWrittenWithSuccess => en::FILE_WRITTEN_WITH_SUCCESS,
            MsgKey::ApiResponseSuccess => en::API_RESPONSE_SUCCESS,
            MsgKey::ApiResponseError => en::API_RESPONSE_ERROR,
            MsgKey::TicketStatusChanged => en::TICKET_STATUS_CHANGED,
            MsgKey::SquadChanged => en::SQUAD_CHANGED,
            MsgKey::To => en::TO,
            MsgKey::TicketAdded => en::TICKET_ADDED,
            MsgKey::Removed => en::REMOVED,
            MsgKey::UpdatedTime => en::UPDATED_TIME,
            MsgKey::ErrorWritingFile => en::ERROR_WRITING_FILE,
            MsgKey::ErrorProcessingFile => en::ERROR_PROCESSING_FILE

        },

        "it" => match key {
            MsgKey::SerializationError => it::SERIALIZATION_ERROR,
            MsgKey::DeserializationError => it::DESERIAZLIZATION_ERROR,
            MsgKey::WritingFileError => it::WRITING_FILE_ERROR,
            MsgKey::FileWrittenWithSuccess => it::FILE_WRITTEN_WITH_SUCCESS,
            MsgKey::ApiResponseSuccess => it::API_RESPONSE_SUCCESS,
            MsgKey::ApiResponseError => it::API_RESPONSE_ERROR,
            MsgKey::TicketStatusChanged => it::TICKET_STATUS_CHANGED,
            MsgKey::SquadChanged => it::SQUAD_CHANGED,
            MsgKey::To => it::TO,
            MsgKey::TicketAdded => it::TICKET_ADDED,
            MsgKey::Removed => it::REMOVED,
            MsgKey::UpdatedTime => it::UPDATED_TIME,
            MsgKey::ErrorWritingFile => it::ERROR_WRITING_FILE,
            MsgKey::ErrorProcessingFile => it::ERROR_PROCESSING_FILE
        },
        _ => "Translation not found",
    }
}
        