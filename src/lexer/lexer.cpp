#include "../include/lexer.hpp"
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

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

  if (word == "int") {
    Type token = Types;
    tokens.push_back(token);
  }

  if (word == "char") {
    Type token = Types;
    tokens.push_back(token);
  }

  if (word == "bool") {
    Type token = Types;
    tokens.push_back(token);
  }

  if (word == "double") {
    Type token = Types;
    tokens.push_back(token);
  }
}
