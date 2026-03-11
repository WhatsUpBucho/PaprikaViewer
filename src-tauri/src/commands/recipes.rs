use crate::{error::AppError, AppState};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSummary {
    pub uid: String,
    pub name: String,
    pub photo_cached_path: Option<String>,
    pub rating: i32,
    pub on_favorites: bool,
    pub in_trash: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeDetail {
    pub uid: String,
    pub name: String,
    pub photo_cached_path: Option<String>,
    pub categories: Vec<String>,
    pub rating: i32,
    pub on_favorites: bool,
    pub is_pinned: bool,
    pub servings: String,
    pub prep_time: String,
    pub cook_time: String,
    pub total_time: String,
    pub difficulty: String,
    pub source: String,
    pub source_url: Option<String>,
    pub ingredients: String,
    pub directions: String,
    pub notes: String,
    pub nutritional_info: String,
    pub created: String,
    pub description: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryInfo {
    pub uid: String,
    pub name: String,
    pub order_flag: i32,
    pub parent_uid: Option<String>,
    pub recipe_count: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFilters {
    pub category_uid: Option<String>,
    pub search_query: Option<String>,
    pub include_trash: Option<bool>,
}

#[tauri::command]
pub async fn get_recipes(
    filters: RecipeFilters,
    state: State<'_, AppState>,
) -> Result<Vec<RecipeSummary>, AppError> {
    let include_trash = filters.include_trash.unwrap_or(false);
    let category_uid = filters.category_uid.clone();
    let search_query = filters.search_query.clone();

    let results = state.db.call(move |conn| {
        crate::db::queries::get_recipes(
            conn,
            category_uid.as_deref(),
            // Treat empty string as no filter
            search_query.as_deref().filter(|s| !s.is_empty()),
            include_trash,
        )
        .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await?;

    Ok(results)
}

#[tauri::command]
pub async fn get_recipe_detail(
    uid: String,
    state: State<'_, AppState>,
) -> Result<RecipeDetail, AppError> {
    let uid_for_error = uid.clone();
    let result = state.db.call(move |conn| {
        crate::db::queries::get_recipe_detail(conn, &uid)
            .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await?;

    result.ok_or_else(|| AppError::NotFound(uid_for_error))
}

#[tauri::command]
pub async fn get_categories(
    state: State<'_, AppState>,
) -> Result<Vec<CategoryInfo>, AppError> {
    let results = state.db.call(|conn| {
        crate::db::queries::get_categories(conn)
            .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await?;

    Ok(results)
}
