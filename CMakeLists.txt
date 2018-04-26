cmake_minimum_required(VERSION "3.0")
cmake_policy(SET CMP0048 NEW)

execute_process(COMMAND git describe --abbrev=0
    OUTPUT_VARIABLE GIT_REPO_VERSION OUTPUT_STRIP_TRAILING_WHITESPACE)

project(i3switch VERSION "${GIT_REPO_VERSION}")
message("Building ${CMAKE_PROJECT_NAME} v${GIT_REPO_VERSION}")

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(PkgConfig)
pkg_check_modules(ZEROMQ libzmq REQUIRED)
find_package(Threads REQUIRED)

file(GLOB_RECURSE SOURCES RELATIVE ${CMAKE_SOURCE_DIR} src/*.cpp)
include_directories(${ZEROMQ_INCLUDE_DIRS})

add_executable(${CMAKE_PROJECT_NAME} ${SOURCES})
target_link_libraries(${CMAKE_PROJECT_NAME} ${ZEROMQ_LIBRARIES} ${CMAKE_THREAD_LIBS_INIT})

install(TARGETS ${CMAKE_PROJECT_NAME} DESTINATION /usr/bin/)
