#include <iostream>
#include <string>
#include <fstream>



int main(int argc, char *argv[]) {
  if (argc < 2 || argc > 2) {
    std::cout << "IMPROPER ARGUMENTS" << std::endl;
    return 1;
  }

  std::string inputArgs = argv[1];
  if (inputArgs.substr(inputArgs.length() - 3).compare(".wl") != 0) {
    std::cout << "IMPROPER FILE TYPE" << std::endl;
    return 1;
  }

  std::fstream inputFile;
  std::cout << inputArgs << std::endl;
  inputFile.open(inputArgs, std::ios::in);
  if (!inputFile)
    std::cout << "FILE NOT FOUND" << std::endl;
  else {
    std::cout << inputArgs << std::endl;

    // TODO: add the stuff that runs the file
    inputFile.close();
  }

  return 0;
}
