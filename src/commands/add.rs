use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::error::{CommemorateError, CommemorateResult};
use crate::hash_password;
use crate::utils::{encrypt, get_memoria_path, parse_time, write_file, TimeInfo};

#[derive(Serialize, Deserialize)]
pub struct MemoriaEvent {
    pub event: String,
    pub description: String,
    pub time: TimeInfo,
    pub store_time: TimeInfo,
    pub last_time: TimeInfo,
}

pub fn add_event(
    name: &str,
    description: &str,
    time: Option<&str>,
    area: &str,
    path: Option<&str>,
) -> CommemorateResult<()> {
    let time_info = if let Some(time_str) = time {
        parse_time(time_str, area)?
    } else {
        TimeInfo::now(area.to_string())
    };

    let event = MemoriaEvent {
        event: name.to_string(),
        description: description.to_string(),
        time: time_info.clone(),
        store_time: TimeInfo::now(area.to_string()),
        last_time: TimeInfo::now(area.to_string()),
    };

    let yaml = serde_yaml::to_string(&event).map_err(|e| CommemorateError::YamlParseError(e))?;

    print!("Please enter a password to encrypt this memory: ");
    io::stdout()
        .flush()
        .map_err(|e| CommemorateError::FileReadError(e))?;
    let password = rpassword::read_password().map_err(|e| CommemorateError::FileReadError(e))?;

    let hashed_password = hash_password(&password)?;

    let encrypted_data = encrypt(yaml.as_bytes(), &password)?;

    let mut final_data = hashed_password;
    final_data.extend_from_slice(&encrypted_data);

    let file_path = if let Some(custom_path) = path {
        std::path::PathBuf::from(custom_path)
    } else {
        get_memoria_path(name)?
    };

    write_file(&file_path, &final_data)?;

    println!(
        "Event '{}' has been successfully added and encrypted.",
        name
    );
    Ok(())
}
