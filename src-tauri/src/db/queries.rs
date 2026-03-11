use rusqlite::{Connection, params};
use std::collections::HashMap;
use crate::commands::recipes::{RecipeSummary, RecipeDetail, CategoryInfo};

// ── Recipe entries (sync index) ─────────────────────────────────────────────

pub fn get_local_entry_hashes(conn: &Connection) -> rusqlite::Result<HashMap<String, String>> {
    let mut stmt = conn.prepare("SELECT uid, hash FROM recipe_entries")?;
    let result: rusqlite::Result<HashMap<_, _>> = stmt
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?
        .collect();
    result
}

pub fn get_all_local_uids(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT uid FROM recipe_entries")?;
    let result: rusqlite::Result<Vec<_>> = stmt
        .query_map([], |row| row.get(0))?
        .collect();
    result
}

pub fn upsert_recipe_entry(conn: &Connection, uid: &str, hash: &str) -> rusqlite::Result<()> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    conn.execute(
        "INSERT INTO recipe_entries (uid, hash, synced_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(uid) DO UPDATE SET hash=excluded.hash, synced_at=excluded.synced_at",
        params![uid, hash, now],
    )?;
    Ok(())
}

pub fn delete_recipe_entry(conn: &Connection, uid: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM recipe_entries WHERE uid = ?1", params![uid])?;
    conn.execute("DELETE FROM recipes WHERE uid = ?1", params![uid])?;
    Ok(())
}

// ── Full recipe UPSERT ───────────────────────────────────────────────────────

pub struct RecipeRow {
    pub uid: String,
    pub name: String,
    pub ingredients: String,
    pub directions: String,
    pub notes: String,
    pub nutritional_info: String,
    pub servings: String,
    pub difficulty: String,
    pub prep_time: String,
    pub cook_time: String,
    pub total_time: String,
    pub source: String,
    pub source_url: Option<String>,
    pub rating: i32,
    pub in_trash: bool,
    pub on_favorites: bool,
    pub is_pinned: bool,
    pub photo_url: Option<String>,
    pub photo_hash: String,
    pub hash: String,
    pub description: String,
    pub created: String,
    pub category_uids: Vec<String>,
}

pub fn upsert_recipe(conn: &Connection, r: &RecipeRow) -> rusqlite::Result<()> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    conn.execute(
        "INSERT INTO recipes (uid, name, ingredients, directions, notes, nutritional_info,
         servings, difficulty, prep_time, cook_time, total_time, source, source_url,
         rating, in_trash, on_favorites, is_pinned, photo_url, photo_hash, hash,
         cached_at, description, created)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23)
         ON CONFLICT(uid) DO UPDATE SET
           name=excluded.name, ingredients=excluded.ingredients,
           directions=excluded.directions, notes=excluded.notes,
           nutritional_info=excluded.nutritional_info, servings=excluded.servings,
           difficulty=excluded.difficulty, prep_time=excluded.prep_time,
           cook_time=excluded.cook_time, total_time=excluded.total_time,
           source=excluded.source, source_url=excluded.source_url,
           rating=excluded.rating, in_trash=excluded.in_trash,
           on_favorites=excluded.on_favorites, is_pinned=excluded.is_pinned,
           photo_url=excluded.photo_url, photo_hash=excluded.photo_hash,
           hash=excluded.hash, cached_at=excluded.cached_at,
           description=excluded.description, created=excluded.created",
        params![
            r.uid, r.name, r.ingredients, r.directions, r.notes, r.nutritional_info,
            r.servings, r.difficulty, r.prep_time, r.cook_time, r.total_time,
            r.source, r.source_url, r.rating,
            r.in_trash as i32, r.on_favorites as i32, r.is_pinned as i32,
            r.photo_url, r.photo_hash, r.hash, now, r.description, r.created
        ],
    )?;

    conn.execute("DELETE FROM recipe_categories WHERE recipe_uid = ?1", params![r.uid])?;
    for cat_uid in &r.category_uids {
        let cat_exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM categories WHERE uid = ?1",
            params![cat_uid],
            |row| row.get::<_, i64>(0),
        ).unwrap_or(0) > 0;
        if cat_exists {
            conn.execute(
                "INSERT OR IGNORE INTO recipe_categories (recipe_uid, category_uid) VALUES (?1, ?2)",
                params![r.uid, cat_uid],
            )?;
        }
    }
    Ok(())
}

// ── Photo path update ────────────────────────────────────────────────────────

pub fn update_photo_path(conn: &Connection, uid: &str, path: &str) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE recipes SET photo_cached_path = ?1 WHERE uid = ?2",
        params![path, uid],
    )?;
    Ok(())
}

pub fn get_recipes_needing_photos(
    conn: &Connection,
) -> rusqlite::Result<Vec<(String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT uid, photo_hash, photo_url FROM recipes
         WHERE photo_url IS NOT NULL AND photo_url != ''
           AND photo_cached_path IS NULL
           AND in_trash = 0"
    )?;
    let result: rusqlite::Result<Vec<_>> = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?
    .collect();
    result
}

// ── Categories ───────────────────────────────────────────────────────────────

pub fn replace_categories(
    conn: &Connection,
    cats: &[(String, String, i32, Option<String>)],
) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM categories", [])?;
    for (uid, name, order_flag, parent_uid) in cats {
        conn.execute(
            "INSERT INTO categories (uid, name, order_flag, parent_uid) VALUES (?1,?2,?3,?4)",
            params![uid, name, order_flag, parent_uid],
        )?;
    }
    Ok(())
}

pub fn get_categories(conn: &Connection) -> rusqlite::Result<Vec<CategoryInfo>> {
    let mut stmt = conn.prepare(
        "SELECT c.uid, c.name, c.order_flag, c.parent_uid,
                COUNT(DISTINCT rc.recipe_uid) as recipe_count
         FROM categories c
         LEFT JOIN recipe_categories rc ON c.uid = rc.category_uid
         LEFT JOIN recipes r ON rc.recipe_uid = r.uid AND r.in_trash = 0
         GROUP BY c.uid
         ORDER BY c.order_flag ASC, c.name COLLATE NOCASE ASC"
    )?;
    let result: rusqlite::Result<Vec<_>> = stmt.query_map([], |row| {
        Ok(CategoryInfo {
            uid: row.get(0)?,
            name: row.get(1)?,
            order_flag: row.get(2)?,
            parent_uid: row.get(3)?,
            recipe_count: row.get(4)?,
        })
    })?
    .collect();
    result
}

// ── Recipe queries ───────────────────────────────────────────────────────────

pub fn get_recipes(
    conn: &Connection,
    category_uid: Option<&str>,
    search_query: Option<&str>,
    include_trash: bool,
) -> rusqlite::Result<Vec<RecipeSummary>> {
    let mut sql = String::from(
        "SELECT DISTINCT r.uid, r.name, r.photo_cached_path, r.rating, r.on_favorites, r.in_trash
         FROM recipes r"
    );

    if category_uid.is_some() {
        sql.push_str(" JOIN recipe_categories rc ON r.uid = rc.recipe_uid");
    }

    sql.push_str(" WHERE 1=1");

    if !include_trash {
        sql.push_str(" AND r.in_trash = 0");
    }

    if category_uid.is_some() {
        sql.push_str(" AND rc.category_uid = ?");
    }

    if search_query.is_some() {
        sql.push_str(" AND LOWER(r.name) LIKE LOWER('%' || ? || '%')");
    }

    sql.push_str(" ORDER BY r.name COLLATE NOCASE ASC");

    let mut stmt = conn.prepare(&sql)?;

    let result: rusqlite::Result<Vec<RecipeSummary>> = match (category_uid, search_query) {
        (Some(cat), Some(q)) => stmt.query_map(params![cat, q], recipe_summary_from_row)?.collect(),
        (Some(cat), None) => stmt.query_map(params![cat], recipe_summary_from_row)?.collect(),
        (None, Some(q)) => stmt.query_map(params![q], recipe_summary_from_row)?.collect(),
        (None, None) => stmt.query_map([], recipe_summary_from_row)?.collect(),
    };

    result
}

fn recipe_summary_from_row(row: &rusqlite::Row) -> rusqlite::Result<RecipeSummary> {
    Ok(RecipeSummary {
        uid: row.get(0)?,
        name: row.get(1)?,
        photo_cached_path: row.get(2)?,
        rating: row.get(3)?,
        on_favorites: row.get::<_, i32>(4)? != 0,
        in_trash: row.get::<_, i32>(5)? != 0,
    })
}

pub fn get_recipe_detail(conn: &Connection, uid: &str) -> rusqlite::Result<Option<RecipeDetail>> {
    let result = conn.query_row(
        "SELECT uid, name, photo_cached_path, rating, on_favorites, is_pinned,
                servings, prep_time, cook_time, total_time, difficulty,
                source, source_url, ingredients, directions, notes,
                nutritional_info, created, description
         FROM recipes WHERE uid = ?1",
        params![uid],
        |row| {
            Ok(RecipeDetail {
                uid: row.get(0)?,
                name: row.get(1)?,
                photo_cached_path: row.get(2)?,
                categories: vec![],
                rating: row.get(3)?,
                on_favorites: row.get::<_, i32>(4)? != 0,
                is_pinned: row.get::<_, i32>(5)? != 0,
                servings: row.get(6)?,
                prep_time: row.get(7)?,
                cook_time: row.get(8)?,
                total_time: row.get(9)?,
                difficulty: row.get(10)?,
                source: row.get(11)?,
                source_url: row.get(12)?,
                ingredients: row.get(13)?,
                directions: row.get(14)?,
                notes: row.get(15)?,
                nutritional_info: row.get(16)?,
                created: row.get(17)?,
                description: row.get(18)?,
            })
        },
    );

    match result {
        Ok(mut detail) => {
            let mut cat_stmt = conn.prepare(
                "SELECT c.name FROM categories c
                 JOIN recipe_categories rc ON c.uid = rc.category_uid
                 WHERE rc.recipe_uid = ?1"
            )?;
            let uid_str = detail.uid.clone();
            let cats: rusqlite::Result<Vec<String>> = cat_stmt
                .query_map(params![uid_str], |row| row.get(0))?
                .collect();
            detail.categories = cats?;
            Ok(Some(detail))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn get_total_recipe_count(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM recipes WHERE in_trash = 0",
        [],
        |row| row.get(0),
    )
}
