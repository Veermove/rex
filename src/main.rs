use std::{str::Chars, rc::Rc, cell::RefCell};

fn main() -> Result<(), String> {
    let example_regex = "([A-Z]{1})".to_string();
    let tokens = tokenize(example_regex)?;
    print_tokens(&tokens);
    Ok(())
}

pub fn print_tokens(next: &ChildNode) {
    if let Some(next_node) = next {
        let nd = next_node.as_ref().borrow();
        println!("{:?}", nd.val);
        print_tokens(&nd.next)
    }
}

pub fn tokenize(message: String) -> Result<ChildNode, String> {
    let root = Rc::new(RefCell::new(Node {
        val: Token::Start, next: None, prev: None,
    }));
    parse_msg_to_tokens(message.chars(), Some(Rc::clone(&root)))?;
    Ok(Some(Rc::clone(&root)))
}

pub fn parse_msg_to_tokens(mut message: Chars, prev_node: ChildNode) -> Result<ChildNode, String> {
    let next_c = message.next();

    if next_c.is_none() {
        if prev_node.is_some() {
            return Ok(prev_node);
        } else {
            return Err("Empty initialization".to_string());
        }
    }

    let cur_token = match next_c {
        Some('^')  => Some(Token::NotOrStringBegin),
        Some('$')  => Some(Token::StringEnd),
        Some('[')  => Some(Token::SetBegin),
        Some(']')  => Some(Token::SetEnd),
        Some('-')  => Some(Token::SetRange),
        Some('(')  => Some(Token::CaptureBegin),
        Some(')')  => Some(Token::CaptureEnd),
        Some('?')  => Some(Token::QzeroOrOne),
        Some('+')  => Some(Token::QoneOrMore),
        Some('*')  => Some(Token::QzeroOrMore),
        Some('{')  => Some(Token::QExactlyBegin),
        Some('}')  => Some(Token::QExactlyEnd),
        Some('.')  => Some(Token::MatchAny),
        Some('\\') => message.next().map(|c| Token::EscapeChar(c.to_string())),
        Some(c)  => Some(Token::Character(c.to_string())),
        None => None
    };


    if let Some(current_token) = cur_token {
        let node = Rc::new(RefCell::new(Node {
            val: current_token,
            prev: None,
            next: None,
        }));

        if let Some(prev) = prev_node {
            prev.as_ref().borrow_mut().next = Some(Rc::clone(&node));
            node.as_ref().borrow_mut().prev = Some(prev);
            return parse_msg_to_tokens(message, Some(Rc::clone(&node)));
        } else {
            return Err("Parsing Error".to_string());
        }
   } else {
        return Err(format!("Syntax error on token: {}", next_c.unwrap_or('~')));
    }
}

type ChildNode = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    pub val: Token,
    pub next: ChildNode,
    pub prev: ChildNode,
}



#[derive(Debug)]
pub enum Token {
    // Control flow
    Start,
    // ^...$
    NotOrStringBegin,
    StringEnd,

    // (...)
    CaptureBegin,
    CaptureEnd,

    // [...]
    SetBegin,
    SetEnd,
    SetRange,

    // \<str>
    EscapeChar(String),

    // Quantifieres
    QzeroOrMore,
    QzeroOrOne,
    QoneOrMore,
    QExactlyBegin,
    QExactlyEnd,

    MatchAny,
    Character(String),
}
