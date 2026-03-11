use std::path::{Path, PathBuf};
use crate::error::AppError;

pub fn cache_path(photos_dir: &Path, uid: &str, photo_hash: &str) -> PathBuf {
    let hash_prefix = &photo_hash[..8.min(photo_hash.len())];
    photos_dir.join(format!("{}_{}.jpg", uid, hash_prefix))
}

pub fn cached_path_if_exists(photos_dir: &Path, uid: &str, photo_hash: &str) -> Option<PathBuf> {
    let path = cache_path(photos_dir, uid, photo_hash);
    if path.exists() { Some(path) } else { None }
}

pub fn download_and_cache(
    photos_dir: &Path,
    uid: &str,
    photo_hash: &str,
    photo_url: &str,
) -> Result<PathBuf, AppError> {
    let dest = cache_path(photos_dir, uid, photo_hash);
    if dest.exists() {
        return Ok(dest);
    }

    let bytes = reqwest::blocking::get(photo_url)
        .map_err(AppError::from)?
        .bytes()
        .map_err(AppError::from)?;

    std::fs::write(&dest, &bytes)?;
    Ok(dest)
}
