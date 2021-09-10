use std::collections::HashMap;

use super::command::Command;

pub struct CommandRegisty {
    register_commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegisty {
    pub fn new() -> CommandRegisty {
        CommandRegisty { register_commands: HashMap::new()}
    }

    pub fn add_command(&mut self, name: &str, command_handler: Box<dyn Command>) {
        self.register_commands.insert(String::from(name), command_handler);
    }

    pub fn handle_command(&self, line: &str) -> Result<String, String> { 
        match self.register_commands.get(line) {
            Some(c) => Ok(c.execute(line)),
            _ => Err(String::from("Bad command")),
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;

struct IdentityCommand {
    output: String,
}

impl IdentityCommand {
    fn new(output: &str) -> IdentityCommand {
        IdentityCommand { output: String::from(output) }
    }
}

impl Command for IdentityCommand {
    fn execute(&self, _arguments: &str) -> String {
        self.output.clone()
    }
}

#[test]
fn empty_factor() {
    let f = CommandRegisty::new();
    drop(f);
}

#[test]
fn add_single_command() {
    let mut f = CommandRegisty::new();
    f.add_command("hello", Box::new(IdentityCommand::new("Hello")));
    drop(f);
}

#[test]
fn execute_single_command() {
    let mut f = CommandRegisty::new();
    f.add_command("hello", Box::new(IdentityCommand::new("Hello")));
    assert!(f.handle_command("hello").expect("Expect hello").eq("Hello"));
    drop(f);
}

#[test]
fn execute_bad_command() {
    let f = CommandRegisty::new();
    assert!(f.handle_command("hello").is_err());
    drop(f);
}

}