#include "aoc/input.hpp"

#include <gtest/gtest.h>

#include <filesystem>
#include <fstream>
#include <string>

namespace {

namespace fs = std::filesystem;

TEST(InputsRoot, UsesEnvironmentValue) {
    EXPECT_EQ(aoc::inputs_root_from("some/where"), fs::path("some/where"));
}

TEST(InputsRoot, DefaultsToRepositoryInputs) {
    const fs::path fallback = aoc::inputs_root_from(nullptr);
    EXPECT_EQ(fallback.filename(), "inputs");
    EXPECT_EQ(aoc::inputs_root_from(""), fallback);
}

TEST(InputPath, LayoutMatchesRustSolutions) {
    const fs::path root("root");
    EXPECT_EQ(aoc::input_path_under(root, 2022, 5), root / "year_2022" / "day_05.txt");
    EXPECT_EQ(aoc::input_path_under(root, 2022, 21), root / "year_2022" / "day_21.txt");
}

TEST(ExamplePath, LayoutMatchesRustSolutions) {
    const fs::path root("root");
    EXPECT_EQ(aoc::example_path_under(root, 2022, 13, 1),
              root / "year_2022" / "day_13_example1.txt");
}

TEST(InputPath, FirstArgumentOverridesTheDefault) {
    std::string program = "aoc";
    std::string argument = "elsewhere.txt";
    char* argv[] = {program.data(), argument.data()};
    EXPECT_EQ(aoc::input_path(2022, 1, 2, argv), fs::path("elsewhere.txt"));
}

TEST(InputPath, WithoutArgumentsUsesInputsRoot) {
    std::string program = "aoc";
    char* argv[] = {program.data()};
    EXPECT_EQ(aoc::input_path(2022, 1, 1, argv),
              aoc::input_path_under(aoc::inputs_root(), 2022, 1));
}

TEST(Open, ReadsAnExistingFile) {
    const fs::path path = fs::temp_directory_path() / "aoc_common_open_test.txt";
    {
        std::ofstream out(path);
        out << "line one\nline two\n";
    }
    auto file = aoc::open(path);
    ASSERT_TRUE(file.has_value()) << file.error();
    std::string line;
    ASSERT_TRUE(std::getline(*file, line));
    EXPECT_EQ(line, "line one");
    file->close();
    fs::remove(path);
}

TEST(Open, ExplainsAMissingFile) {
    const fs::path path = fs::temp_directory_path() / "aoc_no_such_dir" / "day_01.txt";
    auto file = aoc::open(path);
    ASSERT_FALSE(file.has_value());
    EXPECT_NE(file.error().find("no such file"), std::string::npos);
    EXPECT_NE(file.error().find("day_01.txt"), std::string::npos);
}

TEST(Open, RejectsADirectory) {
    EXPECT_FALSE(aoc::open(fs::temp_directory_path()).has_value());
}

}  // namespace
