use crate::code;
use crate::parser;
use crate::parser::command::Command;
use crate::symbol_table::SymbolTable;

pub fn assemble(lines: &[String]) -> Result<Vec<String>, String> {
    let mut output = Vec::new();
    let mut symbol_table = SymbolTable::new();

    let mut commands = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        match parser::parse(line) {
            Ok(Some(command)) => match command {
                Command::LCommand(symbol) => symbol_table.add_entry(symbol, commands.len() as u16),
                _ => commands.push(command),
            },
            Ok(None) => (),
            Err(e) => return Err(format!("Error parsing line {}: {}", index + 1, e)),
        }
    }

    for (index, command) in commands.iter().enumerate() {
        let binary = code::translate(command, &mut symbol_table)
            .map_err(|e| format!("Error parsing line {}: {}", index + 1, e))?;
        output.push(binary);
    }

    Ok(output)
}
