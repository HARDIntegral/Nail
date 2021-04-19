#ifndef LEXER_HPP
#define LEXER_HPP

#include <string>
#include <vector>

class Lexer {

public:
  enum Type {
	If_Keywords,
    Keywords,
    Types,
    Identifiers,
    Integers,
    Strings,
    Functions,
    Operators,
    End_Of_File
  };

private:
void Tokenize(char *argv[]);
void makeTokens(std::string word);
std::vector<Type> tokens;
};

#endif