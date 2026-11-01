#include <cstdint>
#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
#include <queue>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "aoc/input.hpp"

enum operation {mult_old, mult_const, add, unknown};
struct Monkey {
    size_t id;
    std::queue<size_t> items;
    enum operation operation;  // "enum": GCC rejects a member that renames its type
    size_t operand;
    size_t test_divisor;
    size_t true_dest;
    size_t false_dest;
};

void ParseLine(std::string& line, std::vector<std::string>& tokens) {
    tokens.clear();
    std::stringstream ss(line);
    while (!ss.eof()) {
        std::string token;
        ss >> token;
        tokens.push_back(token);
    }
}

size_t BuildMonkey(std::vector<std::string>& lines, Monkey& monkey) {
    std::vector<std::string> tokens;

    // Id 
    ParseLine(lines[0], tokens);
    monkey.id = (size_t)std::stoi(tokens[1]);

    // Items
    ParseLine(lines[1], tokens);
    for (size_t i = 2; i < tokens.size(); ++i) {
        monkey.items.push((size_t)std::stoi(tokens[i]));
    }

    // Operation
    ParseLine(lines[2], tokens);
    if ("*" == tokens[4] && "old" == tokens[5]) {
        monkey.operation = mult_old;
        monkey.operand = SIZE_MAX;
    } else if ("*" == tokens[4]) {
        monkey.operation = mult_const;
        monkey.operand = (size_t)std::stoi(tokens[5]);
    } else if ("+" == tokens[4]) {
        monkey.operation = add;
        monkey.operand = (size_t)std::stoi(tokens[5]);
    } else {
        monkey.operation = unknown;
        monkey.operand = SIZE_MAX;
    }

    // Test divisor
    ParseLine(lines[3], tokens);
    monkey.test_divisor = (size_t)std::stoi(tokens[3]);

    // Test true destination
    ParseLine(lines[4], tokens);
    monkey.true_dest = (size_t)std::stoi(tokens[5]);

    // False true destination
    ParseLine(lines[5], tokens);
    monkey.false_dest = (size_t)std::stoi(tokens[5]);

    return monkey.test_divisor;
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 11, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    int count = 0;

    std::vector<Monkey> monkeys;
    size_t modulus = 1;

    // Process file
    while (std::getline(in_file, line)) {
        ++count;

        std::vector<std::string> lines;
        lines.push_back(line);

        for (int i = 0; i < 5; ++i) {
            std::getline(in_file, line);
            lines.push_back(line);
        }

        Monkey monkey;
        modulus *= BuildMonkey(lines, monkey);
        monkeys.push_back(monkey);

        // Consume blank line
        std::getline(in_file, line);

//    std::cout << tokens[0] << '\t' << tokens[1] << '\t' << tokens.size() << std::endl;
    }

    std::vector<size_t> inspection_count(monkeys.size(), 0);
    for (size_t i = 0; i < 10000; ++i) {
        for (size_t j = 0; j < monkeys.size(); ++j) {
            Monkey& monkey = monkeys[j];
            while (!monkey.items.empty()) {
                size_t worry_level = monkey.items.front();
                monkey.items.pop();
                ++inspection_count[j];

                switch (monkey.operation) {
                    case add:
                        worry_level += monkey.operand;
                        break;
                    case mult_const:
                        worry_level *= monkey.operand;
                        break;
                    case mult_old:
                        worry_level *= worry_level;
                        break;
                    case unknown:
                        std::cout << "Unknown operator" << std::endl;
                        break;
                }

//                worry_level /= 3;
                worry_level %= modulus;
                if (0 == worry_level % monkey.test_divisor) {
                    monkeys[monkey.true_dest].items.push(worry_level);
                } else {
                    monkeys[monkey.false_dest].items.push(worry_level);
                }
            }

        }
    }

    std::sort(inspection_count.begin(), inspection_count.end(), std::greater<size_t>());
//    std::reverse(inspection_count.begin(), inspection_count.end());
    
    // for (size_t i = 0; i < monkeys.size(); ++i) {
    //     std::cout << "Monkey " << i << ": ";
    //     std::cout << inspection_count[i] << std::endl;
    // }

    // for (size_t i = 0; i < monkeys.size(); ++i) {
    //     while (!monkeys[i].items.empty()) {
    //         std::cout << monkeys[i].items.front() << ",\t";
    //         monkeys[i].items.pop();
    //     }    
    //     std::cout << std::endl;
    // }

    std::cout << "Monkey business: " << inspection_count[0] * inspection_count[1] << std::endl;
//    std::cout << "Line count: " << count << std::endl;
    return 0;
}