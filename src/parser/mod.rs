pub mod command;
use command::Address;
use command::Command;

pub fn parse(input: &str) -> Result<Option<Command>, &'static str> {
    let input = input
        .split("//")
        .next()
        .unwrap_or("")
        .replace(char::is_whitespace, "");

    if input.is_empty() {
        Ok(None)
    } else if let Some(address) = input.strip_prefix('@') {
        match address.parse::<u16>() {
            Ok(address) => Ok(Some(Command::ACommand(Address::Numeric(address)))),
            Err(_) => Ok(Some(Command::ACommand(Address::Symbolic(
                address.to_string(),
            )))),
        }
    } else if let Some(symbol_with_post) = input.strip_prefix('(') {
        let symbol = symbol_with_post.split(')').next();
        match symbol {
            Some(symbol) => Ok(Some(Command::LCommand(symbol.to_owned()))),
            None => Err("Invalid symbol declaration"),
        }
    } else {
        let (dest, comp, jump) = parse_c_command(&input);
        Ok(Some(Command::CCommand {
            dest: dest.map(|s| s.to_owned()),
            comp: comp.to_owned(),
            jump: jump.map(|s| s.to_owned()),
        }))
    }
}

fn parse_c_command(input: &str) -> (Option<&str>, &str, Option<&str>) {
    let (dest, input) = if let Some(pos) = input.find('=') {
        (Some(&input[..pos]), &input[pos + 1..])
    } else {
        (None, input)
    };

    let (comp, jump) = if let Some(pos) = input.find(';') {
        (&input[..pos], Some(&input[pos + 1..]))
    } else {
        (input, None)
    };

    (dest, comp, jump)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a_command() {
        let input = "@2";
        let command = parse(input).unwrap().unwrap();
        assert_eq!(command, command::Command::ACommand(Address::Numeric(2)));
    }

    #[test]
    fn test_parse_c_command() {
        let input = "D=A";
        let command = parse(input).unwrap().unwrap();
        assert_eq!(
            command,
            Command::CCommand {
                dest: Some("D".to_string()),
                comp: "A".to_string(),
                jump: None
            }
        );
    }
}
