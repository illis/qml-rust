project(dothersideextra-tests)

set(CMAKE_AUTOMOC TRUE)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
find_package(Qt5Core)
find_package(Qt5Gui)
find_package(Qt5Qml)
find_package(Qt5Quick)
find_package(Qt5Test)

include_directories(../src)

set(${PROJECT_NAME}_QMLREGISTER_SRCS
    tst_deqmlregister.cpp
)

qt5_add_resources(${PROJECT_NAME}_Res_SRCS
    resources.qrc
)

add_executable(${PROJECT_NAME}-qmlregister ${${PROJECT_NAME}_QMLREGISTER_SRCS} ${${PROJECT_NAME}_Res_SRCS})
set_property(TARGET ${PROJECT_NAME}-qmlregister PROPERTY CXX_STANDARD 14)
target_link_libraries(${PROJECT_NAME}-qmlregister DOtherSideStatic dothersideextra Qt5::Core Qt5::Gui Qt5::Qml Qt5::Quick Qt5::Test)

set(${PROJECT_NAME}_QLISTMODEL_SRCS
    tst_qlistmodel.cpp
)

add_executable(${PROJECT_NAME}-qlistmodel ${${PROJECT_NAME}_QLISTMODEL_SRCS})
set_property(TARGET ${PROJECT_NAME}-qlistmodel PROPERTY CXX_STANDARD 14)
target_link_libraries(${PROJECT_NAME}-qlistmodel dothersideextra Qt5::Core Qt5::Test)

add_subdirectory(resources)
