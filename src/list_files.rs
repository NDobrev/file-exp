use std::{fs, io, self};
use std::fs::DirEntry;
use std::io::Write;

use super::command::Command;
use super::command_registry::Context;

pub struct ListFilesCommand {

}

impl Command  for ListFilesCommand {
    fn execute(&self, ctx: &mut  Context,  arguments: &str) -> String {
        let mut result = String::new();
        let entries = get_entries_in_folder(".").expect("Get file is not working");
                entries
                .iter()
                .for_each(|entry| {
                    let line = 
                    match entry.is_file(){
                        true => format!("\t{}\t{}\n", entry.get_size(), entry.path().into_os_string().into_string().expect("String conversion error")),
                        false => format!("<DIR>\t\t{}\n", entry.path().into_os_string().into_string().expect("String conversion error")),
                    };
                    result.push_str(&line.to_string());
                });
        return result;
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

#[allow(dead_code)]
fn get_entries_in_folder(path: &str) -> io::Result<Vec<DirEntry>>  {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e))
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(entries)
}

#[allow(dead_code)]
fn get_files_in_folder(path: &str) -> io::Result<Vec<DirEntry>> {
    let entries = get_entries_in_folder(path)?;
    let files = entries.into_iter().filter(|entry| entry.is_file()).collect();
    Ok(files)
}