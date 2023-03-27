use clap::Parser as ClapParser;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEvent, DebouncedEventKind};
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

#[derive(ClapParser, Debug)]
#[command(
    author = "Jonxslays",
    version = "0.1.0",
    about = "A ziglings file watcher."
)]
struct Parser {
    /// The path to your ziglings directory
    #[arg(short, long)]
    path: String,
}

fn handle_file_events(events: Vec<DebouncedEvent>) {
    let mut paths = Vec::new();

    for event in events {
        match event.kind {
            DebouncedEventKind::Any => {
                if should_rebuild(&event.path, &paths) {
                    paths.push(event.path)
                }
            }
            _ => {}
        }
    }

    for path in &mut paths {
        rebuild_file(path);
    }
}

fn should_rebuild(path: &PathBuf, paths: &Vec<PathBuf>) -> bool {
    if let Some(ext) = path.extension() {
        // Only zig files that exist and haven't already
        // changed this debounce cycle
        ext.eq("zig") && !paths.contains(&path) && path.exists()
    } else {
        false
    }
}

fn rebuild_file(path: &mut PathBuf) {
    let target = path
        .file_name()
        .unwrap_or_else(|| {
            eprintln!("Invalid file name for path {}", path.to_string_lossy());
            std::process::exit(1);
        })
        .to_string_lossy();

    let number = target.split('_').collect::<Vec<&str>>()[0];
    let exercise = number.parse::<usize>().unwrap_or_else(|err| {
        eprintln!("Failed to convert exercise number: {}", err.to_string());
        std::process::exit(1);
    });

    path.pop();  // Yes
    path.pop();  // Also, yes

    Command::new("zig")
        .arg("build")
        .arg("--build-file")
        .arg(&format!("{}/{}", path.to_string_lossy(), "build.zig"))
        .arg(exercise.to_string())
        .spawn()  // :) have a great day!
        .unwrap_or_else(|err| {
            eprintln!("Error: Do you have zig installed? - {}", err.to_string());
            std::process::exit(1);
        });
}

fn main() -> notify::Result<()> {
    let parser = Parser::parse();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut path = Path::new(&parser.path)
        .canonicalize()
        .unwrap_or_else(|err| {
            println!("Error: {}", err.to_string());
            println!("Targeting: {}", parser.path);
            std::process::exit(1);
        });

    path.extend(["exercises"]);
    if !path.exists() {
        path.pop();

        eprintln!(
            "Cannot find ziglings exercises directory in {}",
            path.to_string_lossy(),
        );

        std::process::exit(1);
    }

    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;
    println!("Watching for file changes in {}", path.to_string_lossy());

    #[rustfmt::skip]
    debouncer.watcher().watch(path.as_path(), RecursiveMode::NonRecursive)?;

    for message in rx {
        match message {
            Ok(events) => handle_file_events(events),
            Err(errs) => {
                let message = errs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");

                eprintln!("Error: {}", message);
            }
        }
    }

    Ok(())
}
