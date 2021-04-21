#include "../include/lexer.hpp"
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

Lexer::Lexer(){}
Lexer::~Lexer() = default;


void Lexer::Tokenize(char *argv[]) {

  std::string filename = argv[1];

  std::fstream file(filename);

  if (!file.is_open()) {
    std::cerr << "UNABLE TO OPEN FILE" << std::endl;
    return;
  }

  std::string line;
  while (std::getline(file, line)) {

    std::istringstream stringStream(line);
    std::string word;

    while (stringStream >> word) {
      makeTokens(word);
    }
  }
}

void Lexer::makeTokens(std::string word) {

    // Variable Types
  if (word.find("int") != std::string::npos) {
    Type token = Types;
    tokens.push_back(token);
  }

  if (word.find("char") != std::string::npos) {
    Type token = Types;
    tokens.push_back(token);
  }

  if (word.find("bool") != std::string::npos) {
    Type token = Types;
    tokens.push_back(token);
  }

  if (word.find("double") != std::string::npos) {
    Type token = Types;
    tokens.push_back(token);
  }

  // If statements
  if (word.find("if") != std::string::npos) {
      Type token = If_Keywords;
      tokens.push_back(token);
  }

  if (word.find("else") != std::string::npos) {
      Type token = If_Keywords;
      tokens.push_back(token);
  }

  if (word.find("elif") != std::string::npos) {
      Type token = If_Keywords;
      tokens.push_back(token);
  }


  // Functions
  if (word.find("func") != std::string::npos) {
    Type token = Functions;
    tokens.push_back(token);
  }
}
