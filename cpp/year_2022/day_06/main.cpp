#include <fstream>
#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <string>

#include <utility>

#include "aoc/input.hpp"

size_t ParseLine(std::string& line, const size_t offset) {
    size_t left = 0;
    while (left + offset < line.size()) {
        std::string ss = line.substr(left, offset);
        std::unordered_set<char> s(ss.begin(), ss.end());
        if (offset == s.size()) {
            return left + offset;
        }
        ++left;
    }

    return 0;
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 6, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;

    int count = 0;

    // Process file
    while (std::getline(in_file, line)) {
        ++count;

        size_t result_1 = ParseLine(line, 4);
        size_t result_2 = ParseLine(line, 14);
        std::cout << line << '\t' << line.size() << '\t' << result_1 << '\t' << result_2 << std::endl;
    }
    
    std::cout << "Line count: " << count << std::endl;

    return 0;
}