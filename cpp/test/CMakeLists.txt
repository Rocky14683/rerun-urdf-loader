cmake_minimum_required(VERSION 3.14)
project(mylib)

set(CMAKE_CXX_STANDARD 17)

add_library(mylib STATIC
        src/mylib.cpp
)

target_include_directories(mylib PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

install(TARGETS mylib
        ARCHIVE DESTINATION lib
        LIBRARY DESTINATION lib
        RUNTIME DESTINATION bin)

install(DIRECTORY include/ DESTINATION include)