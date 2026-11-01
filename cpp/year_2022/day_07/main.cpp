#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "aoc/input.hpp"

std::string BuildKey(std::vector<std::string>& curr_dir) {
    std::string result;

    result.push_back('/');
    for (size_t i = 0; i < curr_dir.size(); ++i) {
        result += curr_dir[i];
        result.push_back('/');
    }
    return result;
}

void BuildDirectoryList(std::vector<std::string>& curr_dir, std::vector<std::string>& dir_list) {
    std::string result;

    result.push_back('/');
    dir_list.push_back(result);
    for (size_t i = 0; i < curr_dir.size(); ++i) {
        result += curr_dir[i];
        result.push_back('/');
        dir_list.push_back(result);
    }
}

void UpdateDirSizes(std::vector<std::string>& curr_dir, 
                    std::unordered_map<std::string, size_t>& dir_sizes,
                    size_t file_size) {
    std::vector<std::string> directory_list;
    BuildDirectoryList(curr_dir, directory_list);
    for (size_t i = 0; i < directory_list.size(); ++i) {
        dir_sizes[directory_list[i]] += file_size;
    }
}

void UpdateFileSizes(std::vector<std::string>& curr_dir, 
                    std::unordered_map<std::string, size_t>& file_sizes,
                    std::string& file_name, 
                    size_t file_size) {
    std::string key = BuildKey(curr_dir);
    std::string file_key = key + '/' + file_name;
    file_sizes[file_key] = file_size;
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 7, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    std::vector<std::string> curr_dir;
    std::unordered_map<std::string, size_t> file_sizes;
    std::unordered_map<std::string, size_t> dir_sizes;

    int count = 0;

    // Process file
    while (std::getline(in_file, line)) {
        ++count;

        if ('$' == line[0]) {
            // It's a command
            std::stringstream ss(line);
            std::string prompt, command;
            ss >> prompt >> command;
            if ("cd" == command) {
                std::string operand;
                ss >> operand;
                if (".." == operand) {
                    curr_dir.pop_back();
                } else {
                    if ("/" != operand) {
                        curr_dir.push_back(operand);
                    }
//                    std::cout << BuildKey(curr_dir) << std::endl;
                }
            }
        } else if (isdigit(line[0])) {
            // It's a file
            std::stringstream ss(line);
            size_t file_size;
            std::string file_name;
            ss >> file_size >> file_name;

            UpdateDirSizes(curr_dir, dir_sizes, file_size);
            UpdateFileSizes(curr_dir, file_sizes, file_name, file_size);

        } else {
            // It's a dir
            std::stringstream ss(line);
            std::string dir, dir_name;
            ss >> dir >> dir_name;
        }
//        std::cout << line << std::endl;
    }
    std::cout << "Line count: " << count << std::endl;

    // Process results
    size_t total_dir_size = 0;
    size_t unused_space = 70000000 - dir_sizes["/"];
    size_t min_deletion_space = SIZE_MAX;

    std::cout << "Directories" << std::endl;
    for (std::unordered_map<std::string, size_t>::iterator it = dir_sizes.begin();
        it != dir_sizes.end();
        ++it) {
            std::cout << it->first << '\t' << it->second << std::endl;
            if (it->second <= 100000) {
                total_dir_size += it->second;
            }

            if (30000000 <= unused_space + it->second) {
                min_deletion_space = std::min(min_deletion_space, it->second);
            }
    }
    std::cout << "Total directory size: " << total_dir_size << std::endl;
    std::cout << "Min deletion space: " << min_deletion_space << std::endl;

    return 0;
}