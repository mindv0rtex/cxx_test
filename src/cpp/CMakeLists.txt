cmake_minimum_required(VERSION 3.17)
project(cxx_test)

set(CMAKE_CXX_STANDARD 17)

add_executable(cxx_test main.cpp "${CMAKE_CURRENT_SOURCE_DIR}/../../target/cxxbridge/cxx_test/src/lib.rs.cc")
target_link_directories(cxx_test PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release")
target_link_libraries(cxx_test PRIVATE "libcxx_test.dylib")
