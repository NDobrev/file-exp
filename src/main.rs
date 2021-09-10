use std::{fs, io, self};
use std::fs::DirEntry;
use std::io::Write;

mod command_registry;
mod command;
mod list_files;

use command_registry::CommandRegisty;
use list_files::ListFilesCommand;

fn main() -> Result<(), ()> {
    let mut CommandRegisty = CommandRegisty::new();
    CommandRegisty.add_command("ls", Box::new(ListFilesCommand{}));
    print!("{}", CommandRegisty.handle_command("ls").expect("Ok"));
    Ok(())
}