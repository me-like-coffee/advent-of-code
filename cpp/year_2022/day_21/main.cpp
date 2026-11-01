// #include <algorithm>
// #include <numeric>
// #include <functional>
// #include <optional>
// #include <set>
// #include <map>
// #include <assert.h>
// #include <stdio.h>
//#include "timer.hpp"
#include <fstream>
#include <iostream>
#include <string>
#include <sstream>
#include <unordered_map>
#include <vector>

#include "aoc/input.hpp"

struct node_t {
    std::string id;
    bool finished;
    long value;
    char op;
    node_t * left;
    node_t * right;

    node_t() {
        this->id = "";
        this->finished = true;
        this->value = LONG_MIN;
        this->op = '#';
        this->left = nullptr;
        this->right = nullptr;
    }

    long eval(const char previous_op) {
        // BUGBUG: This bugs part 1
        // Move to inverse_eval()
        if ("humn" == this->id) {
            if ('+' == previous_op || '-' == previous_op) {
                return 0;
            } else {
                return 1;
            }
        }

        if (!finished) {
            switch (op) {
                case '+':
                    value = left->eval('+') + right->eval('+');
                    finished = true;
                    break;
                case '-':
                    value = left->eval('-') - right->eval('-');
                    finished = true;
                    break;
                case '*':
                    value = left->eval('*') * right->eval('*');
                    finished = true;
                    break;
                case '/':
                    value = left->eval('/') / right->eval('/');
                    finished = true;
                    break;
            }
        }
        return value;
    }
};

using node_map_t = std::unordered_map<std::string, node_t *>;

node_map_t load_input(const std::string& file){
    node_map_t node_map;
    std::ifstream fs(file);
    std::string line; 
    int human = 0;

    while (std::getline(fs, line)) {
        std::string id = line.substr(0, 4);
        if (node_map.end() == node_map.find(id)) {
            node_map[id] = new node_t();
            node_map[id]->id = id;
        }

        std::stringstream ss(line.substr(6));
        if (isdigit(line[6])) {
            node_map[id]->value = stol(line.substr(6));
            node_map[id]->finished = true;
            // std::cout << id << '\t' << node_map[id]->value << '\t' << node_map[id]->finished << std::endl;
        } else {
            std::string left_id, right_id;
            char op;
            ss >> left_id >> op >> right_id;

            // std::cout << '*' << left_id << "*\t*" << right_id << '*' << std::endl;

            if (right_id == "humn" || left_id == "humn") {
                ++human;
            }

            if (node_map.end() == node_map.find(left_id)) {
                node_map[left_id] = new node_t();
                node_map[left_id]->id = left_id;
            }
            node_map[id]->left = node_map[left_id];

            if (node_map.end() == node_map.find(right_id)) {
                node_map[right_id] = new node_t();
                node_map[right_id]->id = right_id;
            }
            node_map[id]->right = node_map[right_id];

            node_map[id]->op = op;
            node_map[id]->finished = false;
        }
    }
    std::cout << "Human count: " << human << std::endl;

    return node_map;
}

bool GetPath(node_t * root, const std::string& target, std::vector<std::string>& path) {
    if (nullptr == root) {
        return false;
    }

    path.push_back(root->id);

    if (target == root->id) {
        return true;
    }

    if (GetPath(root->left, target, path) || GetPath(root->right, target, path)) {
        return true;
    }

    path.pop_back();

    return false;
}

auto part1(node_map_t& nodes) {  
    // std::cout << nodes.size() << std::endl;
    // for (auto node : nodes) {
    //     std::cout << node.second->id << std::endl;
    // }

    node_t * root = nodes["root"];
    return root->eval('#');
}

auto part2(node_map_t& nodes) {
    std::vector<std::string> path;
    GetPath(nodes["root"], {"humn"}, path);
    for (auto step : path) {
        std::cout << step << '\t';
    }
    std::cout << std::endl;
    
    nodes["humn"]->value = 0;
    nodes["humn"]->finished = true;

    long left_value = nodes["root"]->left->eval('#');
    long right_value = nodes["root"]->right->eval('#');

    std::cout << left_value << '\t' << right_value << std::endl;

    return left_value - right_value;
}

int main(int argc, char** argv)
{
    const auto example = aoc::example_path(2022, 21, 1);
    const auto input = aoc::input_path(2022, 21, argc, argv);
    for (const auto& path : {example, input}) {
        if (auto opened = aoc::open(path); !opened) {
            std::cerr << opened.error() << '\n';
            return 1;
        }
    }
    auto test_values = load_input(example.string());
    auto actual_values = load_input(input.string());

    std::cout << "part1: " << part1(test_values) << std::endl;
    std::cout << "part1: " << part1(actual_values) << std::endl;

    std::cout << "part2: " << part2(test_values) << std::endl;
    std::cout << "part2: " << part2(actual_values) << std::endl;

    return 0;
}