use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

fn save_path() -> PathBuf {
    env::current_exe()
        .expect("Failed to get current exe's path.")
        .with_file_name("did.txt")
}

fn main() {
    let command = env::args()
        .nth(1)
        .expect("Please specify a command or what you did.");

    match &*command {
        "add" => add(&env::args().nth(2).expect("Please specify what you did.")),
        did => add(did),
    }
}

fn add(did: &str) {
    assert!(
        !did.contains(','),
        "You cannot include ',' to what you did."
    );

    let save_path = save_path();

    let mut save_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&save_path)
        .unwrap_or_else(|_| panic!("Failed to open {}.", save_path.display()));

    writeln!(save_file, "{},{}", Local::now().timestamp(), did)
        .expect("Failed to write to the save file.");
}
