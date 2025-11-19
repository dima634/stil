cmake_minimum_required(VERSION 3.16)

project(eleven VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(ProcessorCount)
ProcessorCount(NUM_CORES)
if(NOT NUM_CORES EQUAL 0)
    set(CMAKE_BUILD_PARALLEL_LEVEL ${NUM_CORES})
endif()

add_subdirectory(eleven)
