use rusqlite::Connection;

const MIGRATIONS: &[&str] = &[
    // Migration 0 → 1: initial schema
    "
    CREATE TABLE IF NOT EXISTS recipe_entries (
        uid       TEXT PRIMARY KEY NOT NULL,
        hash      TEXT NOT NULL,
        synced_at INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS recipes (
        uid               TEXT PRIMARY KEY NOT NULL,
        name              TEXT NOT NULL,
        ingredients       TEXT NOT NULL DEFAULT '',
        directions        TEXT NOT NULL DEFAULT '',
        notes             TEXT NOT NULL DEFAULT '',
        nutritional_info  TEXT NOT NULL DEFAULT '',
        servings          TEXT NOT NULL DEFAULT '',
        difficulty        TEXT NOT NULL DEFAULT '',
        prep_time         TEXT NOT NULL DEFAULT '',
        cook_time         TEXT NOT NULL DEFAULT '',
        total_time        TEXT NOT NULL DEFAULT '',
        source            TEXT NOT NULL DEFAULT '',
        source_url        TEXT,
        rating            INTEGER NOT NULL DEFAULT 0,
        in_trash          INTEGER NOT NULL DEFAULT 0,
        on_favorites      INTEGER NOT NULL DEFAULT 0,
        is_pinned         INTEGER NOT NULL DEFAULT 0,
        photo_url         TEXT,
        photo_hash        TEXT NOT NULL DEFAULT '',
        photo_cached_path TEXT,
        hash              TEXT NOT NULL,
        cached_at         INTEGER NOT NULL,
        description       TEXT NOT NULL DEFAULT '',
        created           TEXT NOT NULL DEFAULT ''
    );

    CREATE TABLE IF NOT EXISTS categories (
        uid        TEXT PRIMARY KEY NOT NULL,
        name       TEXT NOT NULL,
        order_flag INTEGER NOT NULL DEFAULT 0,
        parent_uid TEXT
    );

    CREATE TABLE IF NOT EXISTS recipe_categories (
        recipe_uid   TEXT NOT NULL REFERENCES recipes(uid) ON DELETE CASCADE,
        category_uid TEXT NOT NULL REFERENCES categories(uid) ON DELETE CASCADE,
        PRIMARY KEY (recipe_uid, category_uid)
    );

    CREATE INDEX IF NOT EXISTS idx_recipes_name ON recipes(name COLLATE NOCASE);
    CREATE INDEX IF NOT EXISTS idx_recipes_trash ON recipes(in_trash);
    CREATE INDEX IF NOT EXISTS idx_recipe_cats ON recipe_categories(category_uid);
    ",
];

pub fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL DEFAULT 0);"
    )?;

    let version: i64 = conn
        .query_row(
            "SELECT COALESCE((SELECT version FROM schema_version LIMIT 1), 0)",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    for (i, migration) in MIGRATIONS.iter().enumerate() {
        if (i as i64) >= version {
            conn.execute_batch(migration)?;
        }
    }

    conn.execute_batch(&format!(
        "DELETE FROM schema_version; INSERT INTO schema_version VALUES ({});",
        MIGRATIONS.len()
    ))?;

    Ok(())
}
