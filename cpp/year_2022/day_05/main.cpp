#include <algorithm>
#include <fstream>
#include <iostream>
#include <stack>
#include <sstream>
#include <string>
#include <vector>

#include <utility>

#include "aoc/input.hpp"

// Reads the starting drawing, which ends at a blank line:
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
// Stack i's crates are in column 1 + 4 * i, listed from the top down.
void ParseStacks(std::istream& in_file, std::vector<std::stack<char> >& stacks) {
    std::vector<std::string> rows;
    std::string line;
    while (std::getline(in_file, line) && !line.empty()) {
        rows.push_back(line);
    }
    if (rows.empty()) {
        return;
    }
    rows.pop_back();  // the stack numbers

    size_t width = 0;
    for (const auto& row : rows) {
        width = std::max(width, row.size());
    }
    stacks.assign((width + 2) / 4, std::stack<char>());
    for (auto row = rows.rbegin(); row != rows.rend(); ++row) {
        for (size_t i = 0; i < stacks.size(); ++i) {
            const size_t column = 1 + 4 * i;
            if (column < row->size() && (*row)[column] != ' ') {
                stacks[i].push((*row)[column]);
            }
        }
    }
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 5, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    std::vector<std::stack<char> > stacks(3);

    int count = 0;

    // Init stacks
    ParseStacks(in_file, stacks);

    // Process file
    while (std::getline(in_file, line)) {
        ++count;
        std::string t0, t1, t2;
        size_t amount, source, dest;
        std::stringstream ss(line);
        ss >> t0 >> amount >> t1 >> source >> t2 >> dest;
        --source; // adjust for 0 index
        --dest;
/*
        // Problem One
        for (size_t i = 0; i < amount; ++i) {
            char c = stacks[source].top();
            stacks[source].pop();
            stacks[dest].push(c);
        }
//*/
//*
        // Problem Two
        std::stack<char> tmp;
        for (size_t i = 0; i < amount; ++i) {
            char c = stacks[source].top();
            stacks[source].pop();
            tmp.push(c);
        }
        while (!tmp.empty()) {
            stacks[dest].push(tmp.top());
            tmp.pop();
        }
//*/
        std::cout << line << std::endl;
        std::cout << amount << '\t' << source << '\t' << dest << std::endl;
    }

    std::cout << "Line count: " << count << std::endl;

    // Print output
    for (int i = 0; i < stacks.size(); ++i) {
        std::cout << stacks[i].top();
    }
    std::cout << std::endl;

    return 0;
}