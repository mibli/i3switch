cmake_minimum_required(VERSION "3.22")
cmake_policy(SET CMP0048 NEW)

# Project name and version
# ------------------------

if (NOT GIT_REPO_VERSION)
    execute_process(COMMAND git describe --abbrev=0
        OUTPUT_VARIABLE GIT_REPO_VERSION OUTPUT_STRIP_TRAILING_WHITESPACE)
endif()

# Project configuration
# ---------------------

project(i3switch VERSION "${GIT_REPO_VERSION}")
message("Building ${CMAKE_PROJECT_NAME} v${GIT_REPO_VERSION}")

file(GLOB_RECURSE SOURCES RELATIVE ${CMAKE_SOURCE_DIR} src/*.cpp)
add_executable(${CMAKE_PROJECT_NAME} ${SOURCES})

# C++ 11 standard is used to ensure wide compatibility
target_compile_features(${CMAKE_PROJECT_NAME} PUBLIC cxx_std_11)
if("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
    add_definitions(-DDEBUG)
    target_compile_options(${CMAKE_PROJECT_NAME} PUBLIC -g -ggdb)
endif()

# Linking libraries
# -----------------

# Threads are required for asynchronous listening on i3 IPC socket
find_package(Threads REQUIRED)
if (NOT Threads_FOUND)
    message(FATAL_ERROR "Threads not found. Please install the pthreads library.")
endif()
target_link_libraries(${CMAKE_PROJECT_NAME} Threads::Threads)

# docopt is used for command line argument parsing
find_package(docopt CONFIG REQUIRED)
if (NOT docopt_FOUND)
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(DOCOPT docopt)
    if (NOT DOCOPT_FOUND)
        message(FATAL_ERROR "docopt not found. Please install the docopt library.")
    endif()
    target_link_libraries(${CMAKE_PROJECT_NAME} ${DOCOPT_LIBRARIES})
else()
    target_link_libraries(${CMAKE_PROJECT_NAME} docopt)
endif()

# Install rules
# -------------

install(TARGETS ${CMAKE_PROJECT_NAME} DESTINATION ${CMAKE_INSTALL_PREFIX}/bin)
