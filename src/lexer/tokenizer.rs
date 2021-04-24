use std::f32::NAN;

use super::{token_enum::TokenTypes, Token};

fn unique_name_tester(register: &Vec<f32>, candidate: f32) -> bool {
    for i in (*register).iter() {
        if *i == candidate {
            return false;
        }
    }
    return true;
}

pub fn tokenize(input_string: String, name_register: &mut Vec<f32>) -> Token {
    let mut clean_token: Token = Token {
        type_token: TokenTypes::NULL,
        value: NAN,
    };
    println!("[{}]", input_string);
    if input_string == "import" {
        clean_token.type_token = TokenTypes::ImportStatement;
        clean_token.value = NAN;
    } else if input_string == "fn" {
        clean_token.type_token = TokenTypes::FunctionDeclarationKeyword;
        clean_token.value = NAN;
    } else if input_string == "struct" {
        clean_token.type_token = TokenTypes::StructDeclarationKeyword;
        clean_token.value = NAN;
    } else if input_string == "enum" {
        clean_token.type_token = TokenTypes::EnumDeclarationKeyword;
        clean_token.value = NAN;
    } else if input_string == "return" {
        clean_token.type_token = TokenTypes::ReturnKeyword;
        clean_token.value = NAN;
    } else if input_string == "if" {
        clean_token.type_token = TokenTypes::IfKeyword;
        clean_token.value = NAN;
    } else if input_string == "else" {
        clean_token.type_token = TokenTypes::ElseKeyword;
        clean_token.value = NAN;
    } else if input_string == "elif" {
        clean_token.type_token = TokenTypes::ElifKeyword;
        clean_token.value = NAN;
    } else if input_string == "while" {
        clean_token.type_token = TokenTypes::WhileKeyword;
        clean_token.value = NAN;
    } else if input_string == "for" {
        clean_token.type_token = TokenTypes::ForKeyword;
        clean_token.value = NAN;
    } else if input_string == "int" {
        clean_token.type_token = TokenTypes::IntTypeKeyword;
        clean_token.value = NAN;
    } else if input_string == "double" {
        clean_token.type_token = TokenTypes::DoubleTypeKeyword;
        clean_token.value = NAN;
    } else if input_string == "bool" {
        clean_token.type_token = TokenTypes::BoolTypeKeyword;
        clean_token.value = NAN;
    } else if input_string == "char" {
        clean_token.type_token = TokenTypes::CharTypeKeyword;
        clean_token.value = NAN;
    } else if input_string == "String" {
        clean_token.type_token = TokenTypes::StringTypeKeyword;
        clean_token.value = NAN;
    } else if input_string == "NULL" {
        clean_token.type_token = TokenTypes::NullTypeKeyword;
        clean_token.value = NAN;
    } else if input_string == "TRUE" {
        clean_token.type_token = TokenTypes::TrueKeyword;
        clean_token.value = NAN;
    } else if input_string == "FALSE" {
        clean_token.type_token = TokenTypes::FalseKeyword;
        clean_token.value = NAN;
    } else if input_string == "[" {
        clean_token.type_token = TokenTypes::OpenBracSymbol;
        clean_token.value = NAN;
    } else if input_string == "]" {
        clean_token.type_token = TokenTypes::CloseBracSymbol;
        clean_token.value = NAN;
    } else if input_string == ";" {
        clean_token.type_token = TokenTypes::EndOfLineSymbol;
        clean_token.value = NAN;
    } else if input_string == "," {
        clean_token.type_token = TokenTypes::CommaSeperSymbol;
        clean_token.value = NAN;
    } else if input_string == ":" {
        clean_token.type_token = TokenTypes::ColonSperSymbol;
        clean_token.value = NAN;
    } else if input_string == "::" {
        clean_token.type_token = TokenTypes::DoubleColonSeperSymbol;
        clean_token.value = NAN;
    } else if input_string == "->" {
        clean_token.type_token = TokenTypes::ReturnTypeIdentifier;
        clean_token.value = NAN;
    } else if input_string == "=" {
        clean_token.type_token = TokenTypes::SetOperator;
        clean_token.value = NAN;
    } else if input_string == "+" {
        clean_token.type_token = TokenTypes::AddOperator;
        clean_token.value = NAN;
    } else if input_string == "-" {
        clean_token.type_token = TokenTypes::SubOperator;
        clean_token.value = NAN;
    } else if input_string == "*" {
        clean_token.type_token = TokenTypes::MultOperator;
        clean_token.value = NAN;
    } else if input_string == "/" {
        clean_token.type_token = TokenTypes::DivOperator;
        clean_token.value = NAN;
    } else if input_string == "%" {
        clean_token.type_token = TokenTypes::ModOperator;
        clean_token.value = NAN;
    } else if input_string == "+=" {
        clean_token.type_token = TokenTypes::AddSetOperator;
        clean_token.value = NAN;
    } else if input_string == "-=" {
        clean_token.type_token = TokenTypes::SubSetOperator;
        clean_token.value = NAN;
    } else if input_string == "*=" {
        clean_token.type_token = TokenTypes::MultSetOperator;
        clean_token.value = NAN;
    } else if input_string == "/=" {
        clean_token.type_token = TokenTypes::DivSetOperator;
        clean_token.value = NAN;
    } else if input_string == "%=" {
        clean_token.type_token = TokenTypes::ModSetOperator;
        clean_token.value = NAN;
    } else if input_string == "++" {
        clean_token.type_token = TokenTypes::Increment;
        clean_token.value = NAN;
    } else if input_string == "--" {
        clean_token.type_token = TokenTypes::Decrement;
        clean_token.value = NAN;
    } else if input_string == "@@@" {
        clean_token.type_token = TokenTypes::EndOfFileIdentifier;
        clean_token.value = NAN;
    } else {
        clean_token.type_token = TokenTypes::OtherIdentfier;
        if input_string == "main" {
            clean_token.value = 1.0;
        } else {
            let mut rand_name_value: f32 = rand::random();
            let mut is_unique: bool = unique_name_tester(name_register, rand_name_value);
            while !is_unique {
                rand_name_value = rand::random();
                is_unique = unique_name_tester(name_register, rand_name_value);
            }
            (*name_register).push(rand_name_value);
            clean_token.value = rand_name_value;
        }
    }
    return clean_token;
}
