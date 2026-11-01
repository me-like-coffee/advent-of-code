#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
#include <queue>
#include <sstream>
#include <string>
//#include <unordered_set>
#include <vector>

#include <utility>

#include "aoc/input.hpp"
/*
class Compare {
public:
bool operator() (std::vector<size_t>& lh, std::vector<size_t>& rh) {
    return lh[0] < rh[0];
}}
};
*/

int Steps(std::vector<std::vector<size_t>>& height_map, std::vector<int>& start, std::vector<int>& end) {
    std::vector<std::vector<int> > deltas{{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    int num_rows = (int)height_map.size();
    int num_cols = (int)height_map[0].size();
    std::vector<std::vector<bool>> visited(num_rows, std::vector<bool>(num_cols, false));
    std::priority_queue<std::vector<int>, std::vector<std::vector<int> >, std::greater<std::vector<int> > > pq;

    visited[start[0]] [start[1]] = true;
    pq.push({0, start[0], start[1]});

    while (!pq.empty()) {
        int steps = pq.top()[0];
        int row = pq.top()[1];
        int col = pq.top()[2];
        pq.pop();

        if (row == end[0] && col == end[1]) {
            return steps;
        }

        size_t height = height_map[row][col];
        
        for (std::vector<int>& delta : deltas) {
            int new_row = row + delta[0];
            int new_col = col + delta[1];

            if (0 <= new_row && new_row < num_rows &&
                0 <= new_col && new_col < num_cols &&
                !visited[new_row][new_col] &&
                height_map[new_row][new_col] <= (height + 1)) {
                    visited[new_row][new_col] = true;
                    pq.push({steps + 1, new_row, new_col});
            }
        }
    }
    
    return -1;
}

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 12, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    int count = 0;

    std::vector<int> start(2, INT_MIN);
    std::vector<int> end(2, INT_MIN);

    std::vector<std::vector<size_t> > height_map;
    std::vector<std::vector<bool> > visited;
    std::queue<std::vector<int> > start_locations;

    std::priority_queue<std::vector<int>, std::vector<std::vector<int> >, std::greater<std::vector<int> > > pq;

    std::vector<std::vector<int> > deltas{{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    // Process file
    while (std::getline(in_file, line)) {
        ++count;
        int c = line.find_first_of('S');
        if (c != std::string::npos) {
            start[0] = height_map.size();
            start[1] = c;
        }
        c = line.find_first_of('E');
        if (c != std::string::npos) {
            end[0] = height_map.size();
            end[1] = c;
        }

        std::vector<size_t> height_row(line.begin(), line.end());
        std::vector<bool> visited_row(line.size(), false);
        std::for_each(height_row.begin(), height_row.end(), [](size_t& c) {
            if ('S' == c) {
                c = 0;
            } else if ('E' == c) {
                c = 25;
            } else {
                c = c - 'a';
            }
        });
        height_map.push_back(height_row);
        visited.push_back(visited_row);

        for (int c = 0; c < height_row.size(); ++c) {
            if (0 == height_row[c]) {
                start_locations.push({(int)height_map.size() - 1, c});
            }
        }
   }

    int num_rows = (int)height_map.size();
    int num_cols = (int)height_map[0].size();

// TBD this will move to earlier
    pq.push({0, start[0], start[1]});
    visited[start[0]][start[1]] = true;
    
    while (!pq.empty()) {
        int steps = pq.top()[0];
        int row = pq.top()[1];
        int col = pq.top()[2];
        pq.pop();

        if (row == end[0] && col == end[1]) {
            std::cout << "Steps: " << steps << std::endl;
            break;
        }

        size_t height = height_map[row][col];
        
        for (std::vector<int>& delta : deltas) {
            int new_row = row + delta[0];
            int new_col = col + delta[1];

            if (0 <= new_row && new_row < num_rows &&
                0 <= new_col && new_col < num_cols &&
                !visited[new_row][new_col] &&
                height_map[new_row][new_col] <= (height + 1)) {
                    visited[new_row][new_col] = true;
                    pq.push({steps + 1, new_row, new_col});
            }
        }
    }

    std::cout << "Line count: " << count << std::endl;
    std::cout << "Rows: " << height_map.size() << "\tCols: " << height_map[0].size() << std::endl;
    std::cout << "Start: " << start[0] << '\t' << start[1] << std::endl;
    std::cout << "End: " << end[0] << '\t' << end[1] << std::endl;    
    
    int min_steps = INT_MAX;
    std::cout << start_locations.size() << " start locations." << std::endl;
    while (!start_locations.empty()) {
        std::vector<int> s = start_locations.front();
        start_locations.pop();
        int tmp = Steps(height_map, s, end);
        if (-1 != tmp) {
            min_steps = std::min(min_steps, tmp);
        }
    }
    std::cout << "Minimum steps: " << min_steps << std::endl;

    return 0;
}