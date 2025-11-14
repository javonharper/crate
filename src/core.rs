use std::path::Path;

use crate::{db::init_db, scanner::scan};

pub fn init() {
    println!("[{}]", "command.init");
    // FIXME: lol
    if !Path::new("/Users/javon/.config/crate/config.yaml").exists() {
        println!("Config file does not exist");
    }
}

struct Config {
    pub directory: String,
    pub db: String,
}

pub fn import() {
    println!("[{}]", "command.import");
    // if !Path::new("/Users/javon/.config/crate/config.yaml").exists() {
    //     println!("Config file does not exist");
    // }

    // FIXME: lol
    let config = Config {
        directory: String::from("/Users/javon/Music/Music/Media.localized"),
        db: String::from("/Users/javon/.config/crate/library.db"),
    };

    if !Path::new(&config.db).exists() {
        println!("[{}] initializing database.", "command.import");
        init_db(&config.db);

        println!(
            "[{}] scanning directory. this may take a while.",
            "command.import"
        );

        scan(&config.db, &config.directory);
    }
}
