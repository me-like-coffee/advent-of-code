#include <fstream>
#include <iostream>
#include <string>
#include <sstream>

#include <utility>

#include "aoc/input.hpp"

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 2, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    size_t total_score = 0;
    size_t strategy_score = 0;

    int count = 0;
    while (std::getline(in_file, line)) {
        ++count;

        std::stringstream ss(line);
        char p0;
        char p1;
        ss >> p1 >> p0;

        switch (p0) {
            case 'X':
                total_score += 1;
                switch (p1) {
                    case 'A':
                        total_score += 3;
                        strategy_score += 3;
                        break;
                    case 'B':
                        strategy_score += 1;
                        break;
                    case 'C':
                        total_score += 6;
                        strategy_score += 2;
                        break;
                }
                break;
            case 'Y':
                total_score += 2;
                strategy_score += 3;
                switch (p1) {
                    case 'A':
                        total_score += 6;
                        strategy_score += 1;
                        break;
                    case 'B':
                        total_score += 3;
                        strategy_score += 2;
                        break;
                    case 'C':
                        strategy_score += 3;
                        break;
                }
                break;
            case 'Z':
                total_score += 3;
                strategy_score += 6;
                switch (p1) {
                    case 'A':
                        strategy_score += 2;
                        break;
                    case 'B':
                        total_score += 6;
                        strategy_score += 3;
                        break;
                    case 'C':
                        total_score += 3;
                        strategy_score += 1;
                        break;
                }
                break;
        }
//        std::cout << line << '\t' << p0 << '\t' << p1 << std::endl;
    }

    std::cout << count << std::endl;
    std::cout << "Score: " << total_score << std::endl;
    std::cout << "Strategy score: " << strategy_score << std::endl;
}