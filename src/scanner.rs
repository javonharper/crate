use walkdir::WalkDir;

use crate::db::add_track_from_path;

pub fn scan(db_path: &str, path: &str) {
    let mut count = 0;
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if !is_music_file(entry.path().to_str().unwrap()) {
                    continue;
                }

                let _path = entry.path().strip_prefix(path).unwrap_or(entry.path());

                add_track_from_path(db_path, entry.path());

                count += 1;
            }

            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    println!("Total music files found: {}", count);
}

pub fn is_music_file(path: &str) -> bool {
    path.ends_with(".mp3") || path.ends_with(".wav")
}
