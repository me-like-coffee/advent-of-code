#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#include <utility>

#include "aoc/input.hpp"

int main(int argc, char** argv) {
    auto opened = aoc::open(aoc::input_path(2022, 4, argc, argv));
    if (!opened) {
        std::cerr << opened.error() << '\n';
        return 1;
    }
    std::ifstream in_file = std::move(*opened);
    std::string line;
    size_t total_score = 0;
    size_t foo_count = 0;
    size_t overlap_count = 0;
    
    size_t min_s = SIZE_T_MAX;
    size_t max_e = 0;
    int count = 0;

    while (std::getline(in_file, line)) {
        ++count;
        size_t s_0, e_0, s_1, e_1;
        char t_0, t_1, t_2;
        std::stringstream ss(line);
        ss >> s_0 >> t_0 >> e_0 >> t_1 >> s_1 >> t_2 >> e_1;

        min_s = std::min(s_1, std::min(min_s, s_0));
        max_e = std::max(e_1, std::max(max_e, e_0));

        std::set<size_t> st0;
        for (size_t i = s_0; i <= e_0; ++i) {
            st0.insert(i);
        }
        std::set<size_t> st1;
        for (size_t i = s_1; i <= e_1; ++i) {
            st1.insert(i);
        }
        std::vector<size_t> foo(100);
        std::vector<size_t>::iterator it;
        it = std::set_intersection(st0.begin(), st0.end(), st1.begin(), st1.end(), foo.begin());
        foo.resize(it - foo.begin());
        if (foo.size() == st0.size() || foo.size() == st1.size()) {
            ++foo_count;
        }
        if (foo.size() > 0) {
            ++overlap_count;
        }

        if (s_0 <= s_1) {
            if (s_1 <= e_0 && e_1 <= e_0) {
                ++total_score;
            }
        } else {
            if (s_0 <= e_1 && e_0 <= e_1) {
                ++total_score;
            }
        }

        std::cout << s_0 << '\t' << e_0 << '\t' << s_1 << '\t' << e_1 << std::endl;
        std::cout << line << std::endl;
    }

    std::cout << "Line count: " << count << std::endl;
    std::cout << "Min: " << min_s << '\t' << "Max: " << max_e << std::endl;
    std::cout << "Total score: " << total_score << std::endl;
    std::cout << "Foo score: " << foo_count << std::endl;
    std::cout << "Overlap score: " << overlap_count << std::endl;

    return 0;
}