project(game-of-life)
cmake_minimum_required(VERSION 3.5)

add_definitions(-std=c++11 -O3)
include_directories(include)

file(GLOB SOURCES src/*.cpp)

set(CMAKE_VERBOSE_MAKEFILE on)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/front/pkg)
add_executable(backend ${SOURCES})

set_target_properties(backend PROPERTIES LINK_FLAGS "--bind -s WASM=1 -s EXPORT_NAME=\"'backend'\"")

