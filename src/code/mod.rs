use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{
    parser::command::{Address, Command},
    symbol_table::SymbolTable,
};

pub fn translate(command: &Command, symbols: &mut SymbolTable) -> Result<String, &'static str> {
    match command {
        Command::ACommand(Address::Symbolic(symbol)) => {
            Ok(format!("{:016b}", symbols.get_address(symbol)))
        }
        Command::ACommand(Address::Numeric(address)) => Ok(format!("{:016b}", address)),
        Command::CCommand { dest, comp, jump } => Ok("111".to_string()
            + &translate_comp(comp.to_string())
            + &translate_dest(dest)
            + &translate_jump(jump)),
        Command::LCommand(_) => Err("Label command not parseable to binary"),
    }
}

fn translate_dest(dest: &Option<String>) -> String {
    let mut code = 0;
    if let Some(dest) = dest {
        if dest.contains('M') {
            code += 1;
        }
        if dest.contains('D') {
            code += 2;
        }
        if dest.contains('A') {
            code += 4
        }
    }
    format!("{:03b}", code)
}

fn translate_jump(jump: &Option<String>) -> String {
    let mut code = 0;
    if let Some(jump) = jump {
        if jump.contains('G') {
            code += 1;
        }
        if jump.contains('E') {
            code += 2;
        }
        if jump.contains('L') {
            code += 4;
        }
        if jump.contains('N') {
            code += 3; // JNE == 5 == 0b101
        }
        if jump.contains('M') {
            code += 7;
        }
    }
    format!("{:03b}", code)
}

fn translate_comp(comp: String) -> String {
    let (normalized, is_a) = normalize_and_extract(comp);
    let mut prefix = "".to_string();
    if is_a {
        prefix += "1";
    } else {
        prefix += "0";
    }
    prefix
        + COMP_TO_BITS
            .get(normalized.as_str())
            .copied()
            .unwrap_or("000000")
}

fn normalize_and_extract(comp: String) -> (String, bool) {
    if comp.contains('M') {
        (comp.replace('M', "X"), true)
    } else {
        (comp.replace('A', "X"), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::command::Command;

    #[test]
    fn test_translate_a_command() {
        let command = Command::ACommand(Address::Numeric(2));
        let binary = translate(&command, &mut SymbolTable::new()).unwrap();
        assert_eq!(binary, "0000000000000010");
    }

    #[test]
    fn test_translate_c_command() {
        let command = Command::CCommand {
            dest: Some("D".to_string()),
            comp: "A".to_string(),
            jump: Some("JLE".to_string()),
        };
        let binary = translate(&command, &mut SymbolTable::new()).unwrap();
        assert_eq!(binary, "1110110000010110");
    }
}

lazy_static! {
    static ref COMP_TO_BITS: HashMap<&'static str, &'static str> = {
        let mut table = HashMap::new();
        table.insert("0", "101010");
        table.insert("1", "111111");
        table.insert("-1", "111010");
        table.insert("D", "001100");
        table.insert("X", "110000");
        table.insert("!D", "001101");
        table.insert("!X", "110001");
        table.insert("-D", "001111");
        table.insert("-X", "110011");
        table.insert("D+1", "011111");
        table.insert("X+1", "110111");
        table.insert("D-1", "001110");
        table.insert("X-1", "110010");
        table.insert("D+X", "000010");
        table.insert("D-X", "010011");
        table.insert("X-D", "000111");
        table.insert("D&X", "000000");
        table.insert("D|X", "010101");
        table
    };
}
