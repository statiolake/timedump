use chrono::{Duration, Local, NaiveDateTime};
use std::env;
use std::fs;
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
        "show" => show(),
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

fn show() {
    let save_path = save_path();
    let contents = fs::read_to_string(&save_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read `{}`; record things first or check your access rights.",
            save_path.display()
        )
    });
    let contents = contents.lines();

    let mut did_today = Vec::new();
    let now = Local::now().naive_local();
    let ago1d = now - Duration::days(1);

    for content in contents {
        let (datetime, did) = {
            let mut splitted = content.splitn(2, ',');

            let datetime = splitted
                .next()
                .and_then(|x| x.parse().ok())
                .expect("Failed to parse the datetime");
            let datetime = NaiveDateTime::from_timestamp(datetime, 0);
            let did = splitted.next().expect("Failed to parse what you did");

            (datetime, did)
        };

        // if it is done within 24h
        if ago1d <= datetime {
            did_today.push((datetime, did));
        }
    }

    let day_secs = Duration::days(1).num_seconds() as f64;
    let mut current = ago1d;

    for (datetime, did) in did_today {
        let duration = datetime - current;
        let hour = duration.num_hours();
        let mins = duration.num_minutes() % 60;
        let secs = duration.num_seconds() % 60;

        let ratio = duration.num_seconds() as f64 / day_secs * 100.0;
        println!(
            "{:>2}:{:02}:{:02} ({:>5.1}%)  {}",
            hour, mins, secs, ratio, did
        );

        current = datetime;
    }
}
