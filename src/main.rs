use std::fs::DirEntry;
use std::io::Write;
use std::{self, fs, io};

mod command;
mod command_registry;
mod list_files;

use command_registry::CommandRegisty;
use list_files::ListFilesCommand;

fn main() -> Result<(), ()> {
    let mut registry = CommandRegisty::new();
    registry.add_command("ls", Box::new(ListFilesCommand {}));
    print!("{}", registry.handle_command("ls").expect("Ok"));
    Ok(())
}
