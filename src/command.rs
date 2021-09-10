pub trait Command {
    fn execute(&self, arguments: &str) -> String;
}
