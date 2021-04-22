use logos::Logos;
use std::collections::LinkedList;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Keywords
    #[token("import")]      ImportStatement,
    #[token("fn")]          FunctionDeclarationKeyword,
    #[token("struct")]      StructDeclarationKeyword,
    #[token("enum")]        EnumDeclarationKeyword,
    #[token("return")]      ReturnKeyword,

    #[token("if")]          IfKeyword,
    #[token("else")]        ElseKeyword,
    #[token("elif")]        ElifKeyword,
    #[token("while")]       WhileKeyword,
    #[token("for")]         ForKeyword,

    #[token("int")]         IntTypeKeyword,
    #[token("double")]      DoubleTypeKeyword,
    #[token("bool")]        BoolTypeKeyword,
    #[token("char")]        CharTypeKeyword,

    // Special Symbols
    #[token("(")]           OpenParSymbol,
    #[token(")")]           CloseParSymbol,
    #[token("[")]           OpenBracSymbol,
    #[token("]")]           CloseBracSymbol,
    #[token("{")]           OpenCurlSymbol,
    #[token("}")]           CloseCurlSymbol,

    #[token(",")]           CommaSeperSymbol,
    #[token(";")]           EndOfLineSymbol,

    // Operators
    #[token("=")]           SetOperator,
    #[token("+")]           AddOperator,
    #[token("-")]           SubOperator,
    #[token("*")]           MultOperator,
    #[token("/")]           DivOperator,
    #[token("%")]           ModOperator,

    #[token("+=")]          AddSetOperator,
    #[token("-=")]          SubSetOperator,
    #[token("*=")]          MultSetOperator,
    #[token("/=")]          DivSetOperator,
    #[token("%=")]          ModSetOperator,

    #[token("++")]          Increment,
    #[token("--")]          Decrement,

    // Special Tokens
    #[token("@@@")]         EndOfFileIdentifier,
    #[regex("[a-zA-z]+")]   OtherIdentfier,

    // Error
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn tokenizer() {
    // TODO: create function to tokenize the input file
    let mut token_list: LinkedList<String> = LinkedList::new();
}
