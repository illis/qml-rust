project(qml-rust)
cmake_minimum_required(VERSION 3.0)

include_directories(src/3rdparty/DOtherSide/lib/include src/3rdparty/DOtherSide/lib/include/Qt)
add_subdirectory(src/3rdparty/DOtherSide)
add_subdirectory(src/lib/DOtherSideExtra)
add_subdirectory(tests)
