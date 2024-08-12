use chrono::Utc;
use rpassword;
use std::io::{self, Write};
use std::path::Path;

use crate::commands::add::MemoriaEvent;
use crate::error::{CommemorateError, CommemorateResult};
use crate::utils::encryption::NONCE_LENGTH;
use crate::utils::{decrypt, encrypt, get_memoria_path, read_file, write_file};
use crate::{format_duration, hash_password, verify_password, TimeInfo};

pub fn tell_event(event_identifier: &str) -> CommemorateResult<()> {
    let path = if Path::new(event_identifier).exists() {
        Path::new(event_identifier).to_path_buf()
    } else {
        get_memoria_path(event_identifier)
            .map_err(|_| CommemorateError::EventNotFound(event_identifier.to_string()))?
    };

    if path.extension().and_then(|ext| ext.to_str()) != Some("memoria") {
        return Err(CommemorateError::MemoryInvalid(
            "Ah, maybe this isn't the place for memories, let's look somewhere else".to_string(),
        ));
    }

    let encrypted_data = read_file(&path)?;

    print!("Please enter the password to decrypt this memory: ");

    io::stdout()
        .flush()
        .map_err(|e| CommemorateError::FileReadError(e))?;

    let password = rpassword::read_password().map_err(|e| CommemorateError::FileReadError(e))?;

    let (stored_hashed_password, encrypted_event_data) = encrypted_data.split_at(NONCE_LENGTH + 85);

    let stored_hashed_password_str = String::from_utf8_lossy(stored_hashed_password);

    if !verify_password(&password, stored_hashed_password_str.as_ref())? {
        return Err(CommemorateError::InvalidPassword(
            "Incorrect password".to_string(),
        ));
    }

    let decrypted_data = decrypt(encrypted_event_data, &password)?;

    let event: MemoriaEvent =
        serde_yaml::from_slice(&decrypted_data).map_err(|e| CommemorateError::YamlParseError(e))?;

    let now = Utc::now().timestamp();
    let duration = chrono::Duration::seconds(now - event.time.timestamp);

    println!("\nEvent: {}", event.event);
    println!("Description: {}", event.description);
    println!("Time: {}", event.time.to_original_timezone_string()?);
    println!("Time elapsed: {}", format_duration(duration));
    println!("Last accessed: {}", event.last_time.to_local_string()?);

    // Update last_time
    let mut updated_event = event;

    updated_event.last_time = TimeInfo::now(updated_event.time.timezone.clone());

    let updated_yaml =
        serde_yaml::to_string(&updated_event).map_err(|e| CommemorateError::YamlParseError(e))?;

    let hashed_password = hash_password(&password)?;

    let updated_encrypted_data = encrypt(updated_yaml.as_bytes(), &password)?;

    let mut final_data = hashed_password;

    final_data.extend_from_slice(&updated_encrypted_data);

    write_file(&path, &final_data)?;

    Ok(())
}
