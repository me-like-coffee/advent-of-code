cmake_minimum_required(VERSION 4.0)
project(advent_of_code LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(AOC_SANITIZE "Build with AddressSanitizer and UndefinedBehaviorSanitizer" OFF)
if(AOC_SANITIZE)
    if(MSVC)
        # PLATFORM: windows (MSVC) — MSVC has AddressSanitizer but no
        # UndefinedBehaviorSanitizer; clang and GCC builds get both below.
        add_compile_options(/fsanitize=address)
    else()
        add_compile_options(-fsanitize=address,undefined -fno-omit-frame-pointer)
        add_link_options(-fsanitize=address,undefined)
    endif()
endif()

enable_testing()
add_subdirectory(cpp)
