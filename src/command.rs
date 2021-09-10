use super::command_registry::Context;

pub trait Command {
    fn execute(&self, ctx: &mut Context, arguments: &str) -> String;
}
