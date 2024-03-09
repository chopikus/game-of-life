cmake_minimum_required(VERSION 3.14)
project(game-of-life)

#google test
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
include(FetchContent)
cmake_policy(SET CMP0135 NEW) 
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)


#output executable
add_definitions(-O3)
file(GLOB SOURCES src/hashlife/*.cpp src/rle_parser/*.cpp src/units/*.cpp src/universe/*.cpp)
set(CMAKE_VERBOSE_MAKEFILE on)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/front/pkg)
add_executable(backend ${SOURCES})
set_target_properties(backend PROPERTIES LINK_FLAGS "--bind -s TOTAL_MEMORY=2048MB -s WASM=1 -s EXPORT_NAME=\"'backend'\" -s WASM_BIGINT")

#test target
enable_testing()
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/build)
file(GLOB_RECURSE TEST_SOURCES ${SOURCES} src/tests/*.cpp)
add_executable(
  backend_test
  ${TEST_SOURCES}
)

target_link_options(backend_test PRIVATE --bind -s TOTAL_MEMORY=2048MB --profiling)
target_link_libraries(
  backend_test
  GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(backend_test)