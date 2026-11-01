
#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
// #include <numeric>
// #include <stack>
#include <string>
// #include <variant>
#include <unordered_map>
#include <vector>

#include "aoc/input.hpp"

struct point_t {
    int x;
    int y;
};

struct point_hash {
    std::size_t operator() (const point_t& point) const {
        auto int_hash = std::hash<int>();
        return int_hash(point.x) && int_hash(point.y);
    }
};

bool operator==(const point_t& lh, const point_t& rh) {
    return lh.x == rh.x && lh.y == rh.y;
}

using wall_t = std::vector<point_t>;
using walls_t = std::vector<wall_t>;

struct cave_t {
    std::unordered_map<point_t, char, point_hash> points;
    bool has_floor;
    int max_height;

    char& operator() (const int x, const int y) {
        return points[{x, y}];
    } 

    char& operator() (const point_t& pt) {
        return points[pt];
    }

    char operator() (const point_t& pt) const {
        if (points.count(pt)) {
            return points.at(pt);
        } else {
            return '.';
        }
    }
};

cave_t BuildCave(const walls_t& walls, const point_t& start, bool has_floor) {
    cave_t cave;

    auto AddWallSegment = [&](const point_t& a, const point_t& b) {
        if (a.x == b.x) {
            int delta_y = (a.y < b.y) ? 1 : -1;
            for (int y = a.y; y != b.y + delta_y; y += delta_y) {
                cave({a.x, y}) = '#';
            }
        } else if (a.y == b.y) {
            int delta_x = (a.x < b.x) ? 1 : -1;
            for (int x = a.x; x != b.x + delta_x; x += delta_x) {
                cave({x, a.y}) = '#';
            }
        }
    };

    cave(start) = '+';
    cave.has_floor = has_floor;
    cave.max_height = INT_MIN;

    for (const wall_t& wall : walls) {
         for (int i = 0; i < wall.size() - 1; ++i) {
            const point_t& a = wall[i];
            const point_t& b = wall[i + 1];
            AddWallSegment(a, b);
            cave.max_height = std::max(cave.max_height, std::max(a.y, b.y));
         }
    }

    if (cave.has_floor) {
        cave.max_height += 2;
    }

    return cave;
}

walls_t ReadInput(const std::string& filename) {
    std::ifstream fs(filename);
    std::string line;
    walls_t walls;

    while (std::getline(fs, line)) {
        walls.push_back(wall_t());

        const std::string delimiter(" -> ");
        size_t pos = 0;

        auto parse_pos = [&] {
            std::string token = line.substr(0, pos);
            auto comma = token.find(',');
            walls.back().push_back({std::stoi(token.substr(0, comma)), std::stoi(token.substr(comma + 1))});
            line.erase(0, pos + delimiter.length());
        };

        while ((pos = line.find(delimiter)) != std::string::npos) {
            parse_pos();
        }
        parse_pos();
    }

    return walls;
}

bool DropSand(cave_t& cave, int x, int y) {
    auto ValidMove = [&cave] (const int x, const int y) -> bool {
        if (cave.has_floor && y >= cave.max_height) {
            return false;
        }
        return '#' != cave(x, y) && 'o' != cave(x, y);
    };

    bool at_rest = false;
    while (!at_rest) {
        if (!cave.has_floor && (y + 1) > cave.max_height) {
            break;
        }

        if (cave.has_floor && (y + 1) >= cave.max_height) {
            at_rest = true;
            break;
        }

        while (ValidMove(x, y + 1) && y < cave.max_height) {
           ++y;
        }

        if (ValidMove(x - 1, y + 1)) {
            --x;
            ++y;
            continue;
        }

        if (ValidMove(x + 1, y + 1)) {
            ++x;
            ++y;
            continue;
        }

        at_rest = true;
    }

    if (at_rest) {
        cave(x, y) = 'o';
        // std::cout << x << '\t' << y <<std::endl;
        // if (517 == x && 162 == y) {
        //     int i = 0;
        //     while (++i < 10);
        // }
    }

    return at_rest;
}

int CountSand(const cave_t& cave) {
    return std::count_if(cave.points.begin(), cave.points.end(), [] (auto e) -> bool {
        return 'o' == e.second;
    });
}

void PrintWalls(const walls_t& walls) {
    std::cout << walls.size() << " walls." << std::endl;
    for (const wall_t& wall : walls) {
        std::cout << wall.size() << " points." << std::endl;
        for (const point_t& pt : wall) {
            std::cout << pt.x << '\t' << pt.y << std::endl;
        }
        std::cout << std::endl;
    }
}

int Part1(const walls_t& walls) {
    const point_t start{500, 0};
    cave_t cave = BuildCave(walls, start, false);
    while (DropSand(cave, start.x, start.y)) {
        continue;
    }

    return CountSand(cave);
}

int Part2(const walls_t& walls) {
    const point_t start{500, 0};
    cave_t cave = BuildCave(walls, start, true);

    while('+' == cave(start.x, start.y)) {
        DropSand(cave, start.x, start.y);
    }

    return CountSand(cave);
}

int main(int argc, char** argv)
{
    const auto example = aoc::example_path(2022, 14, 1);
    const auto input = aoc::input_path(2022, 14, argc, argv);
    for (const auto& path : {example, input}) {
        if (auto opened = aoc::open(path); !opened) {
            std::cerr << opened.error() << '\n';
            return 1;
        }
    }
    walls_t test_walls = ReadInput(example.string());
    std::cout << "Test part one: " << Part1(test_walls) << std::endl;
    std::cout << "Test part two: " << Part2(test_walls) << std::endl;

    walls_t input_walls = ReadInput(input.string());
    std::cout << "Input part one: " << Part1(input_walls) << std::endl;
    std::cout << "Input part two: " << Part2(input_walls) << std::endl;

    return 0;
}
