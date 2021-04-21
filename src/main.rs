use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("=")]
    EQUAL,
    #[regex(r"-?\d")]
    INT,

    // error
    #[error]
    // skip
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn main() {
    let lex = Token::lexer("let a: int = 1 * (1 + 3)");
    lex.for_each(|i| println!("{:?}", i));
    // println!("{:?}", lex.next())
}
