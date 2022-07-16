mod parser;
fn main() -> Result<(), String> {
    let example_regex = "([A-Z]{1})".to_string();
    let tokens = parser::tokenize(example_regex)?;
    parser::print_tokens(&tokens);
    Ok(())
}
