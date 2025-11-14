use clap::{Command, Parser};

mod core;
mod db;
mod scanner;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cmd = clap::Command::new("crate")
        .bin_name("crate")
        .about("a music cli tool")
        .subcommand_required(true)
        .subcommand(Command::new("import").about("imports a new crate instance"))
        .subcommand(Command::new("ls").about("search for music"))
        .subcommand(Command::new("stats").about("get basic stats about your music collection"));

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            core::init();
        }
        Some(("import", _)) => {
            core::import();
        }
        Some(("stats", _)) => {
            println!("Not implemented yet");
        }
        Some(("ls", _)) => {
            println!("Not implemented yet");
        }
        _ => {
            println!("Unknown command");
        }
    }
}
