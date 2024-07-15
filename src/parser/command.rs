#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Command {
    ACommand(Address),
    CCommand {
        dest: Option<String>,
        comp: String,
        jump: Option<String>,
    },
    LCommand(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Address {
    Numeric(u16),
    Symbolic(String),
}
