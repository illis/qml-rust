project(dothersideextra-tests)

set(CMAKE_AUTOMOC TRUE)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
find_package(Qt5Core)
find_package(Qt5Gui)
find_package(Qt5Qml)
find_package(Qt5Quick)
find_package(Qt5Test)

include_directories(../src/lib/)

set(${PROJECT_NAME}_SRCS
    tst_deqmlregister.cpp
)

qt5_add_resources(${PROJECT_NAME}_Res_SRCS
    resources.qrc
)

add_executable(${PROJECT_NAME} ${${PROJECT_NAME}_SRCS} ${${PROJECT_NAME}_Res_SRCS})
target_link_libraries(${PROJECT_NAME} DOtherSideStatic dothersideextra Qt5::Core Qt5::Gui Qt5::Qml Qt5::Quick Qt5::Test)

add_subdirectory(resources)
