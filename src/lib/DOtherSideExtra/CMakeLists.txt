project(dothersideextra)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/src/lib/DOtherSideExtra/cmake)
find_package(Qt5Core)
find_package(Qt5Gui)
find_package(Qt5Qml)
find_package(Qt5Quick)
find_package(sailfishapp)

if (SAILFISHAPP_FOUND)
    message("Build with sailfishapp")
    add_definitions(-DSAILFISH)
else ()
    message("Build without sailfishapp")
endif ()

set(${PROJECT_NAME}_SRCS
    types.h
    dothersideextra.cpp
)

add_library(${PROJECT_NAME} STATIC ${${PROJECT_NAME}_SRCS})
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 11)
target_link_libraries(${PROJECT_NAME} PUBLIC DOtherSideStatic Qt5::Core Qt5::Gui Qt5::Qml Qt5::Quick)
