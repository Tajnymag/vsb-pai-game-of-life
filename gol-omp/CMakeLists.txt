cmake_minimum_required(VERSION 3.21)
project(gol_omp)

set(CMAKE_CXX_STANDARD 20)

find_package(SDL2 REQUIRED)
find_package(OpenMP REQUIRED)

add_executable(gol_omp src/main.cpp src/RuleLengthEncoded.cpp src/utils.cpp src/patterns.cpp)

target_link_libraries(gol_omp PRIVATE SDL2::SDL2 OpenMP::OpenMP_CXX)
