add_library(aoc_common common/aoc/input.cpp)
target_include_directories(aoc_common PUBLIC common)
target_compile_definitions(aoc_common PRIVATE AOC_DEFAULT_INPUTS="${PROJECT_SOURCE_DIR}/inputs")

# One executable per day: aoc_2022_day_NN reads inputs/year_2022/day_NN.txt,
# or the file given as its first argument.
foreach(day IN ITEMS 01 02 03 04 05 06 07 08 09 10 11 12 13 14 21)
    add_executable(aoc_2022_day_${day} year_2022/day_${day}/main.cpp)
    target_link_libraries(aoc_2022_day_${day} PRIVATE aoc_common)
endforeach()

find_package(GTest CONFIG REQUIRED)
include(GoogleTest)

add_executable(aoc_common_tests common/tests/input_test.cpp)
target_link_libraries(aoc_common_tests PRIVATE aoc_common GTest::gtest_main)
gtest_discover_tests(aoc_common_tests)
