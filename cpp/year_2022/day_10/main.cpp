#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>

#include <utility>

#include "aoc/input.hpp"

// TBD: This is bugged for the far right column
void Draw(int x, int cycle) {
    if (200 == cycle) {
 //       std::cout << "\tCycle: " << cycle << "\tX: " << x <<std::endl;
    }
    int c = (cycle % 40);
    if (c >= x && c <= (x + 2)) {
        std::cout << '#';
    } else {
        std::cout << '.';
    }

    if (0 == cycle % 40) {
        std::cout << std::endl;
    }
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 10, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    int count = 0;

    std::unordered_set<int> sample_cycles;
    sample_cycles.insert(20);
    sample_cycles.insert(60);
    sample_cycles.insert(100);
    sample_cycles.insert(140);
    sample_cycles.insert(180);
    sample_cycles.insert(220);

    int cycle = 0;
    long x = 1;
    long sample_sum = 0;

    // Process file
    while (std::getline(in_file, line)) {
        ++count;

        std::stringstream ss(line);

        std::string command;
        ss >> command;

        if (command == "addx") {
            long amount;
            ss >> amount;
            ++cycle;
            Draw(x, cycle);
            if (0 != sample_cycles.count(cycle)) {
//                std::cout << "Cycle: " << cycle << "\tX: " << x << "\tStrength: " << (cycle * x) << std::endl;
                sample_sum += (cycle * x);
            }
            ++cycle;
            Draw(x, cycle);
            if (0 != sample_cycles.count(cycle)) {
//                std::cout << "Cycle: " << cycle << "\tX: " << x << "\tStrength: " << (cycle * x) << std::endl;
                sample_sum += (cycle * x);
            }
            x += amount;
        } else {
            ++cycle;
            Draw(x, cycle);
            if (0 != sample_cycles.count(cycle)) {
//                std::cout << "Cycle: " << cycle << "\tX: " << x << "\tStrength: " << (cycle * x) << std::endl;
                sample_sum += (cycle * x);
            }
        }
    }

    std::cout << "Line count: " << count << std::endl;
    std::cout << "Sample: " << sample_sum << std::endl;
    std::cout << "Cycle: " << cycle << std::endl;
    return 0;
}