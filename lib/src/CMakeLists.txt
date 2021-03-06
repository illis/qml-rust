project(dothersideextra)

set(CMAKE_AUTOMOC TRUE)
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
    detypes.h
    deqml.cpp
    deqmlregister.fwd.h
    deqmlregister.h
    deqmlregisterhelper.h
    deqobject.cpp
    deqobjectwrapper.h
    deslotexecutor.h
    deqbaselistmodel.cpp
    deqlistmodel.cpp
    deqlistmodelmetaobject.cpp
    dothersideextra.cpp
)

add_library(${PROJECT_NAME} STATIC ${${PROJECT_NAME}_SRCS})
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 14)
target_link_libraries(${PROJECT_NAME} PUBLIC DOtherSideStatic Qt5::Core Qt5::Gui Qt5::Qml Qt5::Quick)

if (SAILFISHAPP_FOUND)
    target_include_directories(${PROJECT_NAME} PUBLIC ${SAILFISHAPP_INCLUDE_DIRS})
    target_link_libraries(${PROJECT_NAME} PUBLIC sailfishapp)
endif ()
