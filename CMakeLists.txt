cmake_minimum_required(VERSION 3.9)
project(fancyndex)

file(
    GLOB_RECURSE
    sources
    src/*.cpp
    src/*.h
)

add_executable(
    app
    ${sources}
)

target_compile_options(
    app
    PUBLIC
    -std=c++14
    -Wall
    -Wextra
    -pedantic-errors
)

target_link_libraries(
    app
    boost_system
    boost_filesystem
    microhttpd
)

add_custom_target(
    run
    COMMAND app
)

add_custom_target(
    symbol
    COMMAND iod_generate_symbols ../src/*.cpp ../src/symbols.hh
)
