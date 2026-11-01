#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <unordered_set>
#include <vector>

#include <utility>

#include "aoc/input.hpp"

size_t ScoreChar(char c) {
    size_t value;
    if (c >= 'A' && c <= 'Z') {
        value = (c - 'A' + 27);
    } else {
        value = (c - 'a' + 1);
    }
    return value;
}

size_t ProcessLine(std::string& line) {
    size_t mid = line.size() / 2;
    std::unordered_set<char> s(line.begin(), line.begin() + mid);
    char c;
    for (int i = mid; i < line.size(); ++i) {
        if (0 != s.count(line[i])) {
            c = line[i];
            break;
        }
    }

    size_t value = ScoreChar(c);

    std::cout << line << '\t' << c << '\t' << value << std::endl;

    return value;
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 3, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    size_t total_score = 0;
    size_t total_group = 0;

    int count = 0;
    while (std::getline(in_file, line)) {
        ++count;
        total_score += ProcessLine(line);
        std::set<char> s0(line.begin(), line.end());

        std::getline(in_file, line);
        ++count;
        total_score += ProcessLine(line);
        std::set<char> s1(line.begin(), line.end());

        std::getline(in_file, line);
        ++count;
        total_score += ProcessLine(line);
        std::set<char> s2(line.begin(), line.end());

        std::vector<char>::iterator it;
        std::vector<char> i0(52);
        it = std::set_intersection (s0.begin(), s0.end(), s1.begin(), s1.end(), i0.begin());
        i0.resize(it - i0.begin());

        std::vector<char> i1(1);
        std::set_intersection (s2.begin(), s2.end(), i0.begin(), i0.end(), i1.begin());

        total_group += ScoreChar(i1[0]);
    }

    std::cout << count << std::endl;
    std::cout << "a: " << int('a') << '\t' << "A: " << int('A') << std::endl;
    std::cout << "Score: " << total_score << std::endl;
    std::cout << "Group score: " << total_group << std::endl;
}