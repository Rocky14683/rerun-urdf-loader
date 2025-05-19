cmake_minimum_required(VERSION 3.10)
project(urdfparser)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_C_STANDARD 17)

# Find Boost
find_package(Boost REQUIRED)

# Include paths
include_directories(
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${Boost_INCLUDE_DIRS}
)

# URDF source files
set(URDF_SRCS
        src/common.cpp
        src/joint.cpp
        src/geometry.cpp
        src/link.cpp
        src/model.cpp
        src/tinyxml.cpp
        src/tinyxmlerror.cpp
        src/tinyxmlparser.cpp
        src/ffi_binder/parse_urdf.cpp
)

# Create static library
add_library(urdfparser STATIC ${URDF_SRCS})

# Ensure position-independent code for linking with Rust
set_target_properties(urdfparser PROPERTIES POSITION_INDEPENDENT_CODE ON)

# Installation (optional)
install(TARGETS urdfparser
        ARCHIVE DESTINATION lib
        LIBRARY DESTINATION lib
        RUNTIME DESTINATION bin)

install(DIRECTORY include/ DESTINATION include)
