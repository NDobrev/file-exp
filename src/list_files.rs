use std::{self, fs, io};
use std::fs::DirEntry;

use super::command::Command;
use super::command_registry::Context;

pub struct ListFilesCommand {}

impl Command for ListFilesCommand {
    fn execute(&self, _ctx: &mut Context, _arguments: &str) -> String {
        get_entries_in_folder(".")
            .unwrap_or_default()
            .iter()
            .fold(String::new(), |result, entry| -> String {
                result + &format_dir_entry(entry)
            })
    }
}

trait FileAuxiliary {
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
    fn get_size(&self) -> u64;
}

impl FileAuxiliary for std::fs::DirEntry {
    fn is_file(&self) -> bool {
        self.metadata().map_or(false, |e| e.is_file())
    }

    fn is_dir(&self) -> bool {
        self.metadata().map_or(false, |e| e.is_dir())
    }

    fn get_size(&self) -> u64 {
        self.metadata().map_or(0, |e| e.len())
    }
}

fn format_dir_entry(entry: &DirEntry) -> String {
    let path = entry
        .path()
        .into_os_string()
        .into_string()
        .expect("String conversion error");
    match entry.is_file() {
        true => format!("\t{}\t{}\n", entry.get_size(), path),
        false => format!("<DIR>\t\t{}\n", path),
    }
}

fn get_entries_in_folder(path: &str) -> io::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(entries)
}

#[allow(dead_code)]
fn get_files_in_folder(path: &str) -> io::Result<Vec<DirEntry>> {
    let entries = get_entries_in_folder(path)?;
    let files = entries
        .into_iter()
        .filter(|entry| entry.is_file())
        .collect();
    Ok(files)
}
