use std::{fs, io, self};
use std::fs::DirEntry;
use std::io::Write;

mod command_registry;
mod command;

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


fn main() -> Result<(), ()> {
    loop {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        match command.trim().as_ref()  {
            "exit" => return Ok(()),
            "ls" => {
                let entries = get_entries_in_folder(".").expect("Get file is not working");
                entries
                .iter()
                .for_each(|entry| {
                    if entry.is_file() {
                        println!("\t{}\t{}", entry.get_size(), entry.path().into_os_string().into_string().expect("String conversion error"));
                    } else if entry.is_dir() {
                        println!("<DIR>\t\t{}", entry.path().into_os_string().into_string().expect("String conversion error"));
                    }
                });
                
                io::stdout().flush().expect("Flust sream");
            },
            _ => {
                println!("{}", command.as_str());
                return Err(())
            },
        };
    }
}