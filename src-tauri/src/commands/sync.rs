use crate::{db::queries, error::AppError, photos, AppState};
use serde::Serialize;
use std::collections::HashSet;
use tauri::{AppHandle, Emitter, State};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncProgress {
    pub total: usize,
    pub done: usize,
    pub phase: String,
}

#[tauri::command]
pub async fn sync_recipes(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<usize, AppError> {
    // Step 1: Get token
    let token = {
        let lock = state.token.lock().await;
        lock.clone().ok_or(AppError::NotLoggedIn)?
    };

    emit_progress(&app, "entries", 0, 0);

    // Step 2: Fetch remote recipe entries (uid + hash)
    let remote_entries = paprika_api::api::get_recipes(&token)
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;

    emit_progress(&app, "entries", 1, 1);

    // Step 3: Fetch remote categories
    let remote_categories = paprika_api::api::get_categories(&token)
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;

    // Step 4: Diff against local hashes
    let local_hashes = state.db.call(|conn| {
        queries::get_local_entry_hashes(conn)
            .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await?;

    let needed_uids: Vec<String> = remote_entries
        .iter()
        .filter(|entry| {
            local_hashes.get(&entry.uid).map_or(true, |h| h != &entry.hash)
        })
        .map(|entry| entry.uid.clone())
        .collect();

    // Sync categories first so recipe_categories FK checks work
    let cat_data: Vec<(String, String, i32, Option<String>)> = remote_categories
        .iter()
        .map(|c| (c.uid.clone(), c.name.clone(), c.order_flag, c.parent_uid.clone()))
        .collect();

    state.db.call(move |conn| {
        queries::replace_categories(conn, &cat_data)
            .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await?;

    // Step 5: Download full recipes for changed/new entries
    let total = needed_uids.len();
    emit_progress(&app, "recipes", 0, total);

    for (i, uid) in needed_uids.iter().enumerate() {
        let recipe = paprika_api::api::get_recipe_by_id(&token, uid)
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        let row = queries::RecipeRow {
            uid: recipe.uid.clone(),
            name: recipe.name.clone(),
            ingredients: recipe.ingredients.clone(),
            directions: recipe.directions.clone(),
            notes: recipe.notes.clone(),
            nutritional_info: recipe.nutritional_info.clone(),
            servings: recipe.servings.clone(),
            difficulty: recipe.difficulty.clone(),
            prep_time: recipe.prep_time.clone(),
            cook_time: recipe.cook_time.clone(),
            total_time: recipe.total_time.clone(),
            source: recipe.source.clone(),
            // source_url is Option<String> in the API
            source_url: recipe.source_url.clone(),
            rating: recipe.rating,
            in_trash: recipe.in_trash,
            on_favorites: recipe.on_favorites,
            is_pinned: recipe.is_pinned,
            // photo_url and photo_hash are Option<String> in the API
            photo_url: recipe.photo_url.clone(),
            photo_hash: recipe.photo_hash.clone().unwrap_or_default(),
            hash: recipe.hash.clone(),
            description: recipe.description.clone(),
            created: recipe.created.clone(),
            category_uids: recipe.categories.clone(),
        };

        let uid_clone = uid.clone();
        let hash_clone = recipe.hash.clone();

        state.db.call(move |conn| {
            queries::upsert_recipe(conn, &row)
                .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))?;
            queries::upsert_recipe_entry(conn, &uid_clone, &hash_clone)
                .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
        }).await?;

        emit_progress(&app, "recipes", i + 1, total);
    }

    // Step 6: Delete recipes no longer in remote list
    let remote_uid_set: HashSet<String> = remote_entries.iter().map(|e| e.uid.clone()).collect();
    let local_uids = state.db.call(|conn| {
        queries::get_all_local_uids(conn)
            .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await?;

    for uid in local_uids {
        if !remote_uid_set.contains(&uid) {
            let uid_clone = uid.clone();
            state.db.call(move |conn| {
                queries::delete_recipe_entry(conn, &uid_clone)
                    .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
            }).await?;
        }
    }

    // Step 8: Download photos
    let photos_needing_download = state.db.call(|conn| {
        queries::get_recipes_needing_photos(conn)
            .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await?;

    let photo_total = photos_needing_download.len();
    emit_progress(&app, "photos", 0, photo_total);

    let photos_dir = state.data_dir.join("photos");

    for (i, (uid, photo_hash, photo_url)) in photos_needing_download.iter().enumerate() {
        let photos_dir_clone = photos_dir.clone();
        let uid_clone = uid.clone();
        let hash_clone = photo_hash.clone();
        let url_clone = photo_url.clone();

        let cached_path = tokio::task::spawn_blocking(move || {
            photos::download_and_cache(&photos_dir_clone, &uid_clone, &hash_clone, &url_clone)
        })
        .await
        .map_err(|e| AppError::Io(e.to_string()))?;

        match cached_path {
            Ok(path) => {
                let path_str = path.to_string_lossy().to_string();
                let uid_clone2 = uid.clone();
                state.db.call(move |conn| {
                    queries::update_photo_path(conn, &uid_clone2, &path_str)
                        .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
                }).await?;
            }
            Err(e) => {
                log::warn!("Failed to download photo for {}: {}", uid, e);
            }
        }

        emit_progress(&app, "photos", i + 1, photo_total);
    }

    // Step 9: Complete
    let total_recipes = state.db.call(|conn| {
        queries::get_total_recipe_count(conn)
            .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
    }).await? as usize;

    emit_progress(&app, "complete", total_recipes, total_recipes);

    Ok(total_recipes)
}

fn emit_progress(app: &AppHandle, phase: &str, done: usize, total: usize) {
    let _ = app.emit("sync_progress", SyncProgress {
        total,
        done,
        phase: phase.to_string(),
    });
}
