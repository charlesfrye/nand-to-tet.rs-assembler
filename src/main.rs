use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut lines = Vec::new();

    for line in handle.lines() {
        match line {
            Ok(content) => lines.push(content),
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    match hack_asm::assembler::assemble(&lines) {
        Ok(output) => {
            for line in output {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error during assembly: {}", e),
    }
}
