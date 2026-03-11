use keyring::Entry;
use crate::error::AppError;

const SERVICE: &str = "com.paprika-viewer";
const ACCOUNT: &str = "paprika_token";

fn entry() -> Result<Entry, AppError> {
    Entry::new(SERVICE, ACCOUNT).map_err(AppError::from)
}

pub fn store_token(token: &str) -> Result<(), AppError> {
    entry()?.set_password(token).map_err(AppError::from)
}

pub fn load_token() -> Result<String, AppError> {
    entry()?.get_password().map_err(AppError::from)
}

pub fn delete_token() -> Result<(), AppError> {
    entry()?.delete_credential().map_err(AppError::from)
}
