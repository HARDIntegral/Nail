use std::collections::LinkedList;

enum TokenTypes {
    ImportStatement,
    FunctionDeclarationKeyword,
    StructDeclarationKeyword,
    EnumDeclarationKeyword,
    ReturnKeyword,

    IfKeyword,
    ElseKeyword,
    ElifKeyword,
    WhileKeyword,
    ForKeyword,

    IntTypeKeyword,
    DoubleTypeKeyword,
    BoolTypeKeyword,
    CharTypeKeyword,
    StringTypeKeyword,

    OpenBracSymbol,
    CloseBracSymbol,

    EndOfLineSymbol,
    CommaSeperSymbol,

    ReturnTypeIdentifier,

    SetOperator,
    AddOperator,
    SubOperator,
    MultOperator,
    DivOperator,
    ModOperator,

    AddSetOperator,
    SubSetOperator,
    MultSetOperator,
    DivSetOperator,
    ModSetOperator,

    Increment,
    Decrement,

    EndOfFileIdentifier,
    OtherIdentfier
}

struct Token {
    type_token: TokenTypes,
    value: String
}


pub fn tokenizer(input_string: String) -> LinkedList<Token> {
    // TODO: create function to tokenize the input file
    let mut token_list: LinkedList<Token> = LinkedList::new();

    let mut raw_tokens: Vec<&str> = Vec::<&str>::new();
    let raw_lines = input_string.lines();
    for line in raw_lines {
        if !line.to_string().trim().starts_with("//") {           // Remove comment lines
            raw_tokens.append(&mut line
                .trim()
                .split(['{', '(', ',', ' ', ')', '}'].as_ref())   // Tokenize input
                .collect()
            );
        }
    }
    raw_tokens.retain(|x| *x != "");                              // Remove whitespace tokens

    let mut token_intermediate: String;
    let mut add_EOL: bool = false;
    let mut add_CS: bool = false;
    for raw_token in raw_tokens.iter() {
        token_intermediate = raw_token.to_string();
        if token_intermediate.len() > 1 {
            if token_intermediate.ends_with(";") {
                let (first, _second) = token_intermediate
                    .split_at(token_intermediate.len() - 1);
                token_intermediate = first.to_string();
                token_intermediate.split(";");
                add_EOL = true;
            } else if token_intermediate.ends_with(",") {
                let (first, _second) = token_intermediate
                    .split_at(token_intermediate.len() - 1);
                token_intermediate = first.to_string();
                add_CS = true;
            }
        }
        let mut clean_token: Token;
        let mut extra_token: Token;
        if add_EOL {
            extra_token.type_token = TokenTypes::EndOfLineSymbol;
            extra_token.value = String::from(";");
        } else if add_CS {
            extra_token.type_token = TokenTypes::CommaSeperSymbol;
            extra_token.value = String::from(",");
        } else if token_intermediate == "import" {
            clean_token.type_token = TokenTypes::ImportStatement;
            clean_token.value = String::from("import");
        } else if token_intermediate == "struct" {
            clean_token.type_token = TokenTypes::StructDeclarationKeyword;
            clean_token.value = String::from("struct");
        } else if token_intermediate == "enum" {
            clean_token.type_token = TokenTypes::EnumDeclarationKeyword;
            clean_token.value = String::from("enum");
        } else if token_intermediate == "return" {
            clean_token.type_token = TokenTypes::ReturnKeyword;
            clean_token.value = String::from("return");
        } else if token_intermediate == "if" {
            clean_token.type_token = TokenTypes::IfKeyword;
            clean_token.value = String::from("if");
        } else if token_intermediate == "else" {
            clean_token.type_token = TokenTypes::ElseKeyword;
            clean_token.value = String::from("elif");
        } else if token_intermediate == "else" {                                                                                                                     
            clean_token.type_token = TokenTypes::ElseKeyword;                                                                                                        
            clean_token.value = String::from("elif");                                                                                                                
        } else if token_intermediate == "while" {
            clean_token.type_token = TokenTypes::WhileKeyword;
            clean_token.value = String::from("while");
        } else if token_intermediate == "for" {
            clean_token.type_token = TokenTypes::ForKeyword;
            clean_token.value = String::from("for");
        } else if token_intermediate == "int" {
            clean_token.type_token = TokenTypes::IntTypeKeyword;
            clean_token.value = String::from("int");
        } else if token_intermediate == "double" {
            clean_token.type_token = TokenTypes::DoubleTypeKeyword;
            clean_token.value = String::from("double");
        } else if token_intermediate == "bool" {
            clean_token.type_token = TokenTypes::BoolTypeKeyword;
            clean_token.value = String::from("bool");
        } else if token_intermediate == "char" {
            clean_token.type_token = TokenTypes::CharTypeKeyword;
            clean_token.value = String::from("char");
        } else if token_intermediate == "String" {
            clean_token.type_token = TokenTypes::StringTypeKeyword;
            clean_token.value = String::from("String");
        } else if token_intermediate == "[" {
            clean_token.type_token = TokenTypes::OpenBracSymbol;
            clean_token.value = String::from("[");
        } else if token_intermediate == "]" {
            clean_token.type_token = TokenTypes::CloseBracSymbol;
            clean_token.value = String::from("]");
        } else if token_intermediate == ";" {
            clean_token.type_token = TokenTypes::EndOfLineSymbol;
            clean_token.value = String::from(";");
        } else if token_intermediate == "," {
            clean_token.type_token = TokenTypes::CommaSeperSymbol;
            clean_token.value = String::from(",")
        } else if token_intermediate == "->" {
            clean_token.type_token = TokenTypes::ReturnTypeIdentifier;
            clean_token.value = String::from("->");
        } else if token_intermediate == "=" {
            clean_token.type_token = TokenTypes::SetOperator;
            clean_token.value = String::from("=");
        } else if token_intermediate == "+" {
            clean_token.type_token = TokenTypes::AddOperator;
            clean_token.value = String::from("+");
        } else if token_intermediate == "-" {
            clean_token.type_token = TokenTypes::SubOperator;
            clean_token.value = String::from("-");
        } else if token_intermediate == "*" {
            clean_token.type_token = TokenTypes::MultOperator;
            clean_token.value = String::from("*");
        } else if token_intermediate == "/" {
            clean_token.type_token = TokenTypes::DivOperator;
            clean_token.value = String::from("/");
        } else if token_intermediate == "%" {
            clean_token.type_token = TokenTypes::ModOperator;
            clean_token.value = String::from("%");
        } else if token_intermediate == "+=" {
            clean_token.type_token = TokenTypes::AddSetOperator;
            clean_token.value = String::from("+=");
        } else if token_intermediate == "-=" {
            clean_token.type_token = TokenTypes::SubSetOperator;
            clean_token.value = String::from("-=");
        } else if token_intermediate == "*=" {
            clean_token.type_token = TokenTypes::MultSetOperator;
            clean_token.value = String::from("*=");
        } else if token_intermediate == "/=" {
            clean_token.type_token = TokenTypes::DivSetOperator;
            clean_token.value = String::from("/=");
        } else if token_intermediate == "%=" {
            clean_token.type_token = TokenTypes::ModSetOperator;
            clean_token.value = String::from("%=");
        } else if token_intermediate == "++" {
            clean_token.type_token = TokenTypes::Increment;
            clean_token.value = String::from("++");
        } else if token_intermediate == "--" {
            clean_token.type_token = TokenTypes::Decrement;
            clean_token.value = String::from("--");
        }  else if token_intermediate == "@@@" {
            clean_token.type_token = TokenTypes::EndOfFileIdentifier;
            clean_token.value = String::from("@@@");
        } else {
            println!("{}", token_intermediate);
            clean_token.type_token = TokenTypes::OtherIdentfier;
            clean_token.value = token_intermediate;
        }

        token_list.push_back(clean_token);
        if add_EOL || add_CS {
            token_list.push_back(extra_token);
        }
   }

    /*
    for token in raw_tokens.iter() {
        println!("{}", token);
    }
    */
    return token_list;
}
