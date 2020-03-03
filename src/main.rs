extern crate chrono;

use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind, Write};
use std::fs::OpenOptions;
use chrono::Local;

const CONFIG_NAME: &str = "notes-cli";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    notes_path: String,
    delimiter: String,
}

impl ::std::default::Default for Config {
    fn default() -> Config {
        Config {
            notes_path: "".into(),
            delimiter: " ~ ".into(),
        }
    }
}

/// Notes CLI arguments
#[derive(StructOpt, Debug)]
#[structopt(name = "notes-cli", about = "A simple note taker that stores notes with a timestamp and optional tags")]
struct Cli {
    /// The note to record
    #[structopt(name = "note")]
    note: String,
    /// Optional tags to associate with the note
    #[structopt(name = "tags", short = "t", long = "tags")]
    tags: Option<Vec<String>>,
    /// Optional path indicating the file to write the notes to
    #[structopt(name = "path", short = "p", long = "path")]
    path: Option<String>,
    /// Delimiter to use between date, note, and tags
    #[structopt(name = "delimiter", short = "d", long = "delimiter")]
    delimiter: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg: Config = confy::load(CONFIG_NAME)?;
    let args = Cli::from_args();

    if let Some(path) = args.path.clone() {
        if !path.is_empty() {
            cfg.notes_path = path;
            confy::store(CONFIG_NAME, &cfg)?;
        }
    }
    if let Some(delim) = args.delimiter.clone() {
        if !delim.is_empty() {
            cfg.delimiter = delim;
            confy::store(CONFIG_NAME, &cfg)?;
        }
    }
    if cfg.notes_path.is_empty() {
        return Err(Box::new(Error::new(ErrorKind::NotFound, "Path to notes has not been configured. \
            Please set it with the -p or --path option. The path will be saved for future use.")));
    }
    if args.note.contains(cfg.delimiter.as_str()) {
        return Err(Box::new(Error::new(ErrorKind::InvalidInput,
            "Note cannot contain delimiter.")));
    }

    let dt = Local::now();
    let note = format!("{}{}{}{}{}\n",
        dt.format("%F %H:%M:%S"),
        cfg.delimiter,
        args.note,
        cfg.delimiter,
        args.tags.unwrap_or(vec![]).join(", "));
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(cfg.notes_path)?;
    file.write(note.as_bytes()).expect("Failed to append note to file.");
    print!("{}", note);
    Ok(())
}