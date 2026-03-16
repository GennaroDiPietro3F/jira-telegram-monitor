use rfd::MessageDialog;
use rfd::MessageLevel;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use std::str;

use crate::constants::common_costants::localized;
use crate::constants::common_costants::MsgKey;

const FILE_NAME: &str = "user_data.json";
const DIR_NAME: &str = "ApiJiraForTickets";

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user: String,
    pub password: String,
}

impl User {
    /// ### 🍺   Purpose:
    /// Reads user data from the JSON file, creating the file if it does not exist.
    ///
    /// ### 🌶️🥵 Description:
    /// If the file does not exist, it is created with default values. Then it deserializes the content into a User struct.
    ///
    /// #### *️⃣ Parameters:
    /// - None
    ///
    /// #### 🍗 Returns:
    /// - **`User`**: The deserialized User struct from the file.
    pub fn get_utente_from_json() -> Self {
        handle_file_creation();
        let contents = fs::read_to_string(Path::new(DIR_NAME).join(FILE_NAME))
            .unwrap_or_else(|_|{
            panic!("{}",localized(MsgKey::DeserializationError))
        });
            serde_json::from_str(&contents).unwrap_or_else(|_|{
            panic!("{}",localized(MsgKey::DeserializationError))
        })
    }
}

//___________________________________________________

/// ### 🍺   Purpose:
/// Creates the directory and user file if they do not exist.
///
/// ### 🌶️🥵 Description:
/// If the directory or file do not exist, they are created with default user data.
///
/// #### *️⃣ Parameters:
/// - None
///
/// #### 🍗 Returns:
/// - *`()`: No return value.
fn handle_file_creation() {
    let dir_path = Path::new(DIR_NAME);
    let file_path = dir_path.join(FILE_NAME);

    // Create the directory if it does not exist
    if !dir_path.exists() {
        fs::create_dir(dir_path).unwrap_or_else(|_|{
            panic!("{}",(localized(MsgKey::ErrorProcessingFile)))
        });
    }

    let mut default_user = None;
    if !file_path.exists() {
        default_user = Some(User {
            user: match get_current_username_by_command() {
                Ok(username) => format!("{}{}", username, "_CHANGE_ME"),
                Err(_) => "UNKNOWN_CHANGE_ME".to_string(),
            },
            password: "INSERT_YOUR_PASSWORD_HERE".to_string(),
        });
    }
    if let Some(user) = default_user {
        let json_string = serde_json::to_string_pretty(&user).unwrap();
        write_in_file(&file_path.to_string_lossy(), &json_string);
    }
}

/// ### 🍺   Purpose:
/// Gets the current username using a system command.
///
/// ### 🌶️🥵 Description:
/// Executes the 'whoami' command and returns the current username.
///
/// #### *️⃣ Parameters:
/// - None
///
/// #### 🍗 Returns:
/// - **`Result<String, io::Error>`**: The current username or an error.
pub fn get_current_username_by_command() -> Result<String, io::Error> {
    let output = Command::new("whoami").output()?; // '?' propagates any I/O error (e.g. command not found)

    if !output.status.success() {
        let error_msg = str::from_utf8(&output.stderr)
            .unwrap_or("Command 'whoami' failed with unknown error code");

        return Err(io::Error::new(ErrorKind::Other, error_msg));
    }

    let output_string = str::from_utf8(&output.stdout)
        .map_err(|e| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("UTF-8 decode failed: {}", e),
            )
        })?
        .trim()
        .to_string();

    Ok(output_string)
}

/// ### 🍺   Purpose:
/// Writes the JSON string to the specified file.
///
/// ### 🌶️🥵 Description:
/// Writes the JSON content to the file, showing an error message if something goes wrong.
///
/// #### *️⃣ Parameters:
/// - **`file_path: &str`**: Path to the file.
/// - **`json_string: &str`**: JSON string to write.
///
/// #### 🍗 Returns:
/// - *`()`: No return value.
fn write_in_file(file_path: &str, json_string: &str) {
    match fs::write(file_path, json_string) {
        Ok(_) => (),
        Err(_e) => {
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title(localized(MsgKey::ErrorWritingFile))
                .set_description(localized(MsgKey::ErrorWritingFile))
                .show();

            panic!("{}", localized(MsgKey::ErrorWritingFile));
        }
    }
}
