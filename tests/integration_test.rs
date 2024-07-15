use hack_asm::assembler;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

#[test]
fn test_assemble_sample() -> io::Result<()> {
    let input_lines = read_lines("examples/sample.asm")?;

    let expected_output = read_lines("examples/sample.hack")?;

    let output = assembler::assemble(&input_lines).unwrap();

    assert_eq!(output, expected_output);

    Ok(())
}
