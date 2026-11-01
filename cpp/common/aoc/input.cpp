#include "aoc/input.hpp"

#include <cstdlib>
#include <format>
#include <system_error>

#ifndef AOC_DEFAULT_INPUTS
#error "AOC_DEFAULT_INPUTS must name the repository's inputs folder"
#endif

namespace aoc {

namespace {

std::filesystem::path year_dir(const std::filesystem::path& root, int year) {
    return root / std::format("year_{}", year);
}

}  // namespace

std::filesystem::path inputs_root_from(const char* env_value) {
    if (env_value != nullptr && *env_value != '\0') {
        return env_value;
    }
    return AOC_DEFAULT_INPUTS;
}

std::filesystem::path inputs_root() { return inputs_root_from(std::getenv("AOC_INPUTS")); }

std::filesystem::path input_path_under(const std::filesystem::path& root, int year, int day) {
    return year_dir(root, year) / std::format("day_{:02}.txt", day);
}

std::filesystem::path example_path_under(const std::filesystem::path& root, int year, int day,
                                         int n) {
    return year_dir(root, year) / std::format("day_{:02}_example{}.txt", day, n);
}

std::filesystem::path input_path(int year, int day, int argc, char** argv) {
    if (argc > 1) {
        return argv[1];
    }
    return input_path_under(inputs_root(), year, day);
}

std::filesystem::path example_path(int year, int day, int n) {
    return example_path_under(inputs_root(), year, day, n);
}

std::expected<std::ifstream, std::string> open(const std::filesystem::path& path) {
    std::error_code error;
    if (!std::filesystem::is_regular_file(path, error)) {
        return std::unexpected(std::format("cannot read {}: no such file", path.string()));
    }
    std::ifstream file(path);
    if (!file) {
        return std::unexpected(std::format("cannot read {}", path.string()));
    }
    return file;
}

}  // namespace aoc
