use super::{token_enum::TokenTypes, Token};
use std::i32::MIN;

const INTEGER_TYPE: i32 = 0;
const FLOATING_POINT_TYPE: i32 = 1;

fn try_parse(candidate: String) -> i32 {
    let chars_list: Vec<char> = candidate.chars().collect();
    let mut decimal_point_count: i32 = 0;
    for i in chars_list.iter() {
        if i.is_digit(10) {
            if i == &'.' {
                decimal_point_count += 1;
            }
        } else {
            return MIN;
        }
    }
    if decimal_point_count == 0 {
        return INTEGER_TYPE;
    } else if decimal_point_count == 1 {
        return FLOATING_POINT_TYPE;
    } else {
        return MIN;
    }
}

pub fn tokenize(input_string: String) -> Token {
    let mut clean_token: Token = Token {
        type_token: TokenTypes::NULL,
        value: String::new(),
    };
    println!("[{}]", input_string);
    if input_string == "import" {
        clean_token.type_token = TokenTypes::ImportStatement;
    } else if input_string == "fn" {
        clean_token.type_token = TokenTypes::FunctionDeclarationKeyword;
    } else if input_string == "struct" {
        clean_token.type_token = TokenTypes::StructDeclarationKeyword;
    } else if input_string == "enum" {
        clean_token.type_token = TokenTypes::EnumDeclarationKeyword;
    } else if input_string == "return" {
        clean_token.type_token = TokenTypes::ReturnKeyword;
    } else if input_string == "if" {
        clean_token.type_token = TokenTypes::IfKeyword;
    } else if input_string == "else" {
        clean_token.type_token = TokenTypes::ElseKeyword;
    } else if input_string == "elif" {
        clean_token.type_token = TokenTypes::ElifKeyword;
    } else if input_string == "while" {
        clean_token.type_token = TokenTypes::WhileKeyword;
    } else if input_string == "for" {
        clean_token.type_token = TokenTypes::ForKeyword;
    } else if input_string == "in" {
        clean_token.type_token = TokenTypes::InLoopRangeKeyword;
    } else if input_string == ".." {
        clean_token.type_token = TokenTypes::RangeSymbol;
    } else if input_string == "break" {
        clean_token.type_token = TokenTypes::BreakKeyword;
    } else if input_string == "continue" {
        clean_token.type_token = TokenTypes::ContinueKeyword;
    } else if input_string == "let" {
        clean_token.type_token = TokenTypes::VarInitKeyword;
    } else if input_string == "const" {
        clean_token.type_token = TokenTypes::ConstInitKeyword;
    } else if input_string == "int" {
        clean_token.type_token = TokenTypes::IntTypeKeyword;
    } else if input_string == "double" {
        clean_token.type_token = TokenTypes::DoubleTypeKeyword;
    } else if input_string == "bool" {
        clean_token.type_token = TokenTypes::BoolTypeKeyword;
    } else if input_string == "char" {
        clean_token.type_token = TokenTypes::CharTypeKeyword;
    } else if input_string == "String" {
        clean_token.type_token = TokenTypes::StringTypeKeyword;
    } else if input_string == "NULL" {
        clean_token.type_token = TokenTypes::NullTypeKeyword;
    } else if input_string == "TRUE" {
        clean_token.type_token = TokenTypes::TrueKeyword;
    } else if input_string == "FALSE" {
        clean_token.type_token = TokenTypes::FalseKeyword;
    } else if input_string == "[" {
        clean_token.type_token = TokenTypes::OpenBracSymbol;
    } else if input_string == "]" {
        clean_token.type_token = TokenTypes::CloseBracSymbol;
    } else if input_string == "{" {
        clean_token.type_token = TokenTypes::OpenCurlSymbol;
    } else if input_string == "}" {
        clean_token.type_token = TokenTypes::CloseCurlSymbol;
    } else if input_string == "(" {
        clean_token.type_token = TokenTypes::OpenParenSymbol;
    } else if input_string == ")" {
        clean_token.type_token = TokenTypes::CloseParenSymbol;
    } else if input_string == ";" {
        clean_token.type_token = TokenTypes::EndOfLineSymbol;
    } else if input_string == "," {
        clean_token.type_token = TokenTypes::CommaSeperSymbol;
    } else if input_string == ":" {
        clean_token.type_token = TokenTypes::ColonSperSymbol;
    } else if input_string == "::" {
        clean_token.type_token = TokenTypes::DoubleColonSeperSymbol;
    } else if input_string == "->" {
        clean_token.type_token = TokenTypes::ReturnTypeIdentifier;
    } else if input_string == "=" {
        clean_token.type_token = TokenTypes::SetOperator;
    } else if input_string == "==" {
        clean_token.type_token = TokenTypes::IsEqualOperator;
    } else if input_string == "<" {
        clean_token.type_token = TokenTypes::LessThanOperator;
    } else if input_string == ">" {
        clean_token.type_token = TokenTypes::GreaterThanOperator;
    } else if input_string == "<=" {
        clean_token.type_token = TokenTypes::LessEqualOperator;
    } else if input_string == ">=" {
        clean_token.type_token = TokenTypes::GreaterEqualOperator;
    } else if input_string == "+" {
        clean_token.type_token = TokenTypes::AddOperator;
    } else if input_string == "-" {
        clean_token.type_token = TokenTypes::SubOperator;
    } else if input_string == "*" {
        clean_token.type_token = TokenTypes::MultOperator;
    } else if input_string == "/" {
        clean_token.type_token = TokenTypes::DivOperator;
    } else if input_string == "%" {
        clean_token.type_token = TokenTypes::ModOperator;
    } else if input_string == "+=" {
        clean_token.type_token = TokenTypes::AddSetOperator;
    } else if input_string == "-=" {
        clean_token.type_token = TokenTypes::SubSetOperator;
    } else if input_string == "*=" {
        clean_token.type_token = TokenTypes::MultSetOperator;
    } else if input_string == "/=" {
        clean_token.type_token = TokenTypes::DivSetOperator;
    } else if input_string == "%=" {
        clean_token.type_token = TokenTypes::ModSetOperator;
    } else if input_string == "++" {
        clean_token.type_token = TokenTypes::Increment;
    } else if input_string == "--" {
        clean_token.type_token = TokenTypes::Decrement;
    } else if input_string == "@@@" {
        clean_token.type_token = TokenTypes::EndOfFileIdentifier;
    } else {
        let is_numeric: i32 = try_parse((&*input_string).to_string());
        if is_numeric == INTEGER_TYPE {
            clean_token.type_token = TokenTypes::IntegerLiteral;
            clean_token.value = input_string.to_string();
        } else if is_numeric == FLOATING_POINT_TYPE {
            clean_token.type_token = TokenTypes::FloatLiteral;
            clean_token.value = input_string.to_string();
        } else {
            clean_token.type_token = TokenTypes::OtherIdentfier;
            clean_token.value = input_string.to_string();
        }
    }
    return clean_token;
}
