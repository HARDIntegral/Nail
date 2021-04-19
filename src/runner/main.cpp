#include <iostream>
#include <string>
#include <fstream>

#define SUCCESS 1
#define FAILURE 0

int main(int argc, char *argv[]) {
  if (argc < 2 || argc > 2) {
    std::cout << "IMPROPER ARGUMENTS" << std::endl;
    return FAILURE;
  }
  
  std::string inputArgs = argv[1];
  if (inputArgs.substr(inputArgs.length() - 5).compare(".weeb") != 0) {
    std::cout << "IMPROPER FILE TYPE" << std::endl;
    return FAILURE;
  }

  std::fstream inputFile;
  inputFile.open(inputArgs, std::ios::in);
  if (!inputFile) 
    std::cout << "FILE NOT FOUND" << std::endl;
  else {
    // TODO: add the stuff that runs the file

    inputFile.close();
  }

  return SUCCESS;
}
