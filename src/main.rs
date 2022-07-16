mod parser;
fn main() -> Result<(), String> {
    let example_regex = "([A-Z]{1})".to_string();
    let mut tokens = parser::tokenize(example_regex)?;
    // parser::print_tokens(&tokens);
    let (cur1, next) = parser::head_tail(tokens);
    let (cur2, next1) = parser::head_tail(next);
    println!("HERE1: {:?},\nHERE2: {:?}", cur1, cur2);
    Ok(())
}
