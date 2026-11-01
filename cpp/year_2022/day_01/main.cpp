#include <iostream>

#include <fstream>
#include <queue>
#include <string>
#include <sstream>
#include <vector>

#include <utility>

#include "aoc/input.hpp"

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 1, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
//    std::stringstream ss;
    size_t max_total = 0;
    size_t curr_total = 0;
    size_t value = 0;
    std::priority_queue<size_t> q;


    int count = 0;

    size_t min_length = INT_MAX;
    std::string min_line;
    size_t min_count = 0;

    size_t max_length = 0;
    std::string max_line;
    size_t max_count = 0;

    while (std::getline(in_file, line)) {
        if (0 < line.size()) {
            std::stringstream ss(line);
            ss >> value;
            curr_total += value;

            std::cout << line << '\t' << value << std::endl;
        } else {
            if (curr_total > max_total) {
                max_total = curr_total;
            }
            q.push(curr_total);
            curr_total = 0;
            std::cout << line << std::endl;
        }

        ++count;
        if (line.size() <= min_length) {
            min_length = line.size();
            ++min_count;
            min_line = line;
        }
        if (line.size() >= max_length) {
            max_length = line.size();
            ++max_count;
            max_line = line;
        }
    }
    std::cout << count << " lines read." << std::endl;
    std::cout << "min: " << min_length << '\t' << min_count << '\t' << min_line <<std::endl;
    std::cout << "max: " << max_length << '\t' << max_count << '\t' << max_line <<std::endl;
    std::cout << "Max calories: " << max_total << std::endl;

    size_t three_elves = 0;
    for (int i = 0; i < 3; ++i) {
        three_elves += q.top();
        q.pop();
    }
    std::cout << three_elves << std::endl;
}