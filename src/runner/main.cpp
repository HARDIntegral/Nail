#include <iostream>
#include <string>
#include <fstream>

#define SUCCESS 1
#define FAILURE 0

using namespace std;

int main(int argc, char *argv[]) {
  if (argc < 2 || argc > 2) {
    cout << "IMPROPER ARGUMENTS" << endl;
    return FAILURE;
  }
  
  string inputArgs = argv[1];
  if (inputArgs.substr(inputArgs.length() - 3).compare(".wl") != 0) {
    cout << "IMPROPER FILE TYPE" << endl;
    return FAILURE;
  }

  fstream inputFile;
  cout << inputArgs << endl;
  inputFile.open(inputArgs, ios::in);
  if (!inputFile) 
    cout << "FILE NOT FOUND" << endl;
  else {
    cout << inputArgs << endl;

    // TODO: add the stuff that runs the file

    inputFile.close();
  }

  return SUCCESS;
}
