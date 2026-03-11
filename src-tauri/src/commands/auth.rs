use crate::{error::AppError, keychain, AppState};
use tauri::State;

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let token_struct = paprika_api::api::login(&email, &password)
        .await
        .map_err(|e| AppError::Auth(e.to_string()))?;

    let token = token_struct.token;
    keychain::store_token(&token)?;

    let mut lock = state.token.lock().await;
    *lock = Some(token);

    Ok(())
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<(), AppError> {
    let _ = keychain::delete_token();

    let mut lock = state.token.lock().await;
    *lock = None;

    Ok(())
}

#[tauri::command]
pub async fn check_auth(state: State<'_, AppState>) -> Result<bool, AppError> {
    let lock = state.token.lock().await;
    Ok(lock.is_some())
}
