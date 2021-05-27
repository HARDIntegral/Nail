#include <iostream>
#include <string>
#include <fstream>

int main(int argc, char *argv[]) {
    // Check for valid amount of CLI args    
    if (argc < 2) {
        std::cout << "[ERROR]: NO FILE GIVEN" << std::endl;
    } else if (argc < 3) {
        std::cout << "[WARNING]: NO FLAGS GIVEN" << std::endl;
    }

    // File and flags are handling
    std::string inputFile = argv[1];
    // Future Integral, how the hell are you gonna handle multi-file Nail projects
    // Also future Integral, have fun handling flags
    if (inputFile.substr(inputFile.length() - 4).compare(".nal") != 0) {
        std::cout << "[ERROR]: IMPROPER FILE TYPE GIVEN" << std::endl;
        return 1;
    }
    std::fstream inputMain;
    inputMain.open(inputFile, std::ios::in);
    if (!inputMain)
        std::cout << "[ERROR]: FILE NOT FOUND" << std::endl;
    else {
        std::cout << inputFile << std::endl;
        inputMain.close();
    }

    return 0;
}