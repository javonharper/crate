use std::path::Path;

use audiotags::Tag;
use rusqlite::Connection;

pub fn init_db(dir: &String) {
    let conn = Connection::open(dir).expect("Failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS artists (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )",
        [],
    )
    .expect("Failed to create artists table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS artist_tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        artist_id INTEGER NOT NULL,
        key TEXT NOT NULL,
        value TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (artist_id) REFERENCES artists(id)
    )",
        [],
    )
    .expect("Failed to create artist_tags table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS albums (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        artist TEXT NOT NULL,
        artist_id INTEGER NOT NULL,
        title TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (artist_id) REFERENCES artists(id)
    )",
        [],
    )
    .expect("Failed to create albums table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS album_tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        album_id INTEGER NOT NULL,
        key TEXT NOT NULL,
        value TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (album_id) REFERENCES albums(id)
    )",
        [],
    )
    .expect("Failed to create album_tags table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        album TEXT NOT NULL,
        album_id INTEGER NOT NULL,
        artist TEXT NOT NULL,
        artist_id INTEGER NOT NULL,
        path TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (album_id) REFERENCES albums(id),
        FOREIGN KEY (artist_id) REFERENCES artists(id)
    )",
        [],
    )
    .expect("Failed to create tracks table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS track_tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        track_id INTEGER NOT NULL,
        key TEXT NOT NULL,
        value TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (track_id) REFERENCES tracks(id)
    )",
        [],
    )
    .expect("Failed to create track_tags table");
}

pub fn add_track_from_path(db_path: &str, path: &Path) {
    let conn = Connection::open(db_path).expect("Failed to open database");

    let tag = Tag::new().read_from_path(path).expect("Failed to read tag");

    let track_title = match tag.title() {
        Some(title) => title,
        None => "fart----",
    };

    let track_artist = match tag.artist() {
        Some(artist) => artist,
        None => "fart----",
    };

    let track_album = match tag.album() {
        Some(album) => album.title,
        None => "fart----",
    };

    let path = match path.to_str() {
        Some(path) => path,
        None => "fart----",
    };

    let artist = conn
        .query_row(
            "SELECT id FROM artists WHERE name = ?",
            &[track_artist],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| {
            conn.execute("INSERT INTO artists (name) VALUES (?)", &[track_artist])
                .expect("Failed to insert artist");
            conn.last_insert_rowid()
        });

    let album = conn
        .query_row(
            "SELECT id FROM albums WHERE title = ? AND artist = ?",
            &[track_album, track_artist],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| {
            conn.execute(
                "INSERT INTO albums (title, artist, artist_id) VALUES (?, ?, ?)",
                &[track_album, track_artist, artist.to_string().as_str()],
            )
            .expect("Failed to insert album");
            conn.last_insert_rowid()
        });

    conn.execute(
        "INSERT INTO tracks (title, artist, artist_id, album, album_id, path) VALUES (?, ?, ?, ?, ?, ?)",
        &[
            track_title,
            track_artist,
            artist.to_string().as_str(),
            track_album,
            album.to_string().as_str(),
            path,
        ],
    )
    .expect("Failed to insert track");
}
