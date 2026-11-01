#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>

#include <utility>

#include "aoc/input.hpp"

std::string Hash(std::vector<int>& v) {
    std::string result;

    result += std::to_string(v[0]);
    result += ',';
    result += std::to_string(v[1]);

    return result;
}

void MoveHead(std::vector<int>& head, char command) {
    switch (command) {
        case 'U':
            ++head[1];
            break;
        case 'D':
            --head[1];
            break;
        case 'L':
            --head[0];
            break;
        case 'R':
            ++head[0];
            break;
    }
}

void MoveTail(std::vector<int>& head, std::vector<int>& tail) {
    int x_delta = tail[0] - head[0];
    int y_delta = tail[1] - head[1];
    if (abs(x_delta) > 1 || abs(y_delta) > 1) {
        // Tail needs to be moved
        if (0 == x_delta) {
            (tail[1] > head[1]) ? --tail[1] : ++tail[1];
        } else if (0 == y_delta) {
            (tail[0] > head[0]) ? --tail[0] : ++tail[0];
        } else {
            (tail[1] > head[1]) ? --tail[1] : ++tail[1];
            (tail[0] > head[0]) ? --tail[0] : ++tail[0];
        }
    }
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 9, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    int count = 0;

    std::vector<std::vector<int> > rope(10, std::vector<int>(2, 0));

    std::unordered_set<std::string> visited;
    visited.insert(Hash(rope.back()));

    // Process file
    while (std::getline(in_file, line)) {
        ++count;

        std::stringstream ss(line);
        char command;
        size_t amount;
        ss >> command >> amount;

        while (amount-- > 0) {
            MoveHead(rope.front(), command);
            for (size_t i = 1; i < rope.size(); ++i) {
                MoveTail(rope[i - 1], rope[i]);
            }
            visited.insert(Hash(rope.back()));
        }
    }
    std::cout << "Line count: " << count << std::endl;
    std::cout << "Visited size: " << visited.size() << std::endl;

    return 0;
}