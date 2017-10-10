cmake_minimum_required(VERSION 3.9)
project(fancyndex)

file(
    GLOB_RECURSE
    sources
    src/*.cpp
    src/*.h
)

add_executable(
    fancyndex
    ${sources}
)

target_compile_options(
    fancyndex
    PUBLIC
    -std=c++14
    -Wall
    -Wextra
    -pedantic-errors
)

target_link_libraries(
    fancyndex
    boost_system
    boost_filesystem
    microhttpd
    yaml-cpp
)

add_custom_target(
    run
    COMMAND fancyndex
)

add_custom_target(
    symbol
    COMMAND iod_generate_symbols ../src/main.cpp ../src/symbols.hh
)
