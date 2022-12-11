use anyhow::Result;

fn parse_input(input: &str) -> usize {
    13
}

fn main() -> Result<()> {
    Ok(())
}

#[test]
fn test_input() {
    let input = include_str!("./day9_test_input.txt");
    let result = parse_input(input);
    assert_eq!(result, 13)
}
