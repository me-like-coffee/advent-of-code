#pragma once

// Locating and opening puzzle inputs, shared by the C++ solutions.
//
// The layout matches the Rust solutions: <root>/year_YYYY/day_DD.txt and
// <root>/year_YYYY/day_DD_example{n}.txt, where <root> is AOC_INPUTS if set
// and not empty, otherwise the repository's inputs/ folder.

#include <expected>
#include <filesystem>
#include <fstream>
#include <string>

namespace aoc {

// The inputs folder: `env_value` (the value of AOC_INPUTS) if set and not
// empty, otherwise the repository's inputs/ folder.
std::filesystem::path inputs_root_from(const char* env_value);

// The inputs folder, from AOC_INPUTS or the repository's inputs/ folder.
std::filesystem::path inputs_root();

// <root>/year_YYYY/day_DD.txt
std::filesystem::path input_path_under(const std::filesystem::path& root, int year, int day);

// <root>/year_YYYY/day_DD_example{n}.txt
std::filesystem::path example_path_under(const std::filesystem::path& root, int year, int day,
                                         int n);

// The puzzle input: argv[1] if given, otherwise the input under inputs_root().
std::filesystem::path input_path(int year, int day, int argc, char** argv);

// Example n under inputs_root().
std::filesystem::path example_path(int year, int day, int n);

// Opens a file for reading, or says why it can't be opened.
std::expected<std::ifstream, std::string> open(const std::filesystem::path& path);

}  // namespace aoc
