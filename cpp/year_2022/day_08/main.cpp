#include <climits>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include <utility>

#include "aoc/input.hpp"

size_t ScenicScore(std::vector<std::vector<int> >& forest, size_t r_start, size_t c_start) {
    size_t n_dist = 1, e_dist = 1, s_dist = 1, w_dist = 1;
    int start_height = forest[r_start][c_start];

    // Up
    for (size_t r = r_start - 1; r >= 1; --r) {
        if (start_height <= forest[r][c_start]) {
            break;
        }
        ++n_dist;
    }

    // Right
    for (size_t c = c_start + 1; c < forest[0].size() - 1; ++c) {
        if (start_height <= forest[r_start][c]) {
            break;
        }
        ++e_dist;
    }

    // Down
    for (size_t r = r_start + 1; r < forest.size() - 1; ++r) {
        if (start_height <= forest[r][c_start]) {
            break;
        }
        ++s_dist;
    }

    // Left
     for (size_t c = c_start - 1; c >= 1; --c) {
        if (start_height <= forest[r_start][c]) {
            break;
        }
        ++w_dist;
    }

   return n_dist * e_dist * s_dist * w_dist;
}

size_t ScenicScore(std::vector<std::vector<int> >& forest) {
    size_t max_scenic_score = 0;

    for (size_t r = 1; r < forest.size() - 1; ++r) {
        for (size_t c = 1; c < forest[0].size() - 1; ++c) {
            max_scenic_score = std::max(max_scenic_score, ScenicScore(forest, r, c));
        }
    }


    return max_scenic_score;
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 8, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;

    std::vector<std::vector<int> > forest;

    int count = 0;

    // Process file
    while (std::getline(in_file, line)) {
        ++count;

        std::vector<int> curr_row;

        for (size_t i = 0; i < line.size(); ++i) {
            int digit = line[i] - '0';
            curr_row.push_back(digit);
        }
        forest.push_back(curr_row);

//        std::cout << line << std::endl;
    }
//    std::cout << "Line count: " << count << std::endl;
/*
    for (size_t r = 0; r < forest.size(); ++r) {
        for (size_t c = 0; c < forest[0].size(); ++c) {
            std::cout << forest[r][c] << '\t';
        }
        std::cout << std::endl;
    }
*/
    size_t num_rows = forest.size();
    size_t num_cols = forest[0].size();
    size_t num_visible = 4;

    std::vector<std::vector<bool> > visibility(num_rows, std::vector<bool>(num_cols, false));

    // Left to right
    for (int r = 1; r < num_rows - 1; ++r) {
        int max_value = INT_MIN;
        for (size_t c = 0; c < num_cols; ++c) {
//            std::cout << forest[r][c] << '\t';
            if (max_value < forest[r][c] && false == visibility[r][c]) {
                visibility[r][c] = true;
                ++num_visible;
            }
            max_value = std::max(max_value, forest[r][c]);
        }
//        std::cout << std::endl;
    }

    // Right to left
    for (int r = 1; r < num_rows - 1; ++r) {
        int max_value = INT_MIN;
        for (int c = num_cols - 1; c >= 0 ; --c) {
            if (max_value < forest[r][c] && false == visibility[r][c]) {
                visibility[r][c] = true;
                ++num_visible;
            }
            max_value = std::max(max_value, forest[r][c]);
        }
    }

    // Top to bottom
    for (int c = 1; c < num_cols - 1; ++c) {
        int max_value = INT_MIN;
        for (int r = 0; r < num_rows; ++r) {
            if (max_value < forest[r][c] && false == visibility[r][c]) {
                visibility[r][c] = true;
                ++num_visible;
            }
            max_value = std::max(max_value, forest[r][c]);
        }
    }

    // Bottom to top
    for (int c = 1; c < num_cols - 1; ++c) {
        int max_value = INT_MIN;
        for (int r = num_rows - 1; r >= 0 ; --r) {
            if (max_value < forest[r][c] && false == visibility[r][c]) {
                visibility[r][c] = true;
                ++num_visible;
            }
            max_value = std::max(max_value, forest[r][c]);
        }
    }
    std::cout << "Number of visible trees: " << num_visible << std::endl;
    std::cout << "Maximum scenic score: " << ScenicScore(forest) << std::endl;
 
//    std::cout << "Test score: " << ScenicScore(forest, 3, 2) << std::endl;
/*
    for (size_t r = 0; r < num_rows; ++r) {
        for (size_t c = 0; c < num_cols; ++c) {
            if (visibility[r][c]) {
                std::cout << forest[r][c];
            }
            std::cout << '\t';
        }
        std::cout << std::endl;
    }
*/    
    return 0;
}