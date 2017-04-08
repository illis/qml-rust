project(testresources)

find_package(Qt5Core REQUIRED)

set(${PROJECT_NAME}_SRCS
    testresources.cpp
)

qt5_add_resources(${PROJECT_NAME}_Res_SRCS
    resources.qrc
)

add_library(${PROJECT_NAME} STATIC
    ${${PROJECT_NAME}_SRCS}
    ${${PROJECT_NAME}_Res_SRCS}
)
target_link_libraries(${PROJECT_NAME} PRIVATE Qt5::Core)

set(${PROJECT_NAME}_TEST_SRCS
    main.cpp
)


find_package(Qt5Gui REQUIRED)
find_package(Qt5Quick REQUIRED)
add_executable(${PROJECT_NAME}-test ${${PROJECT_NAME}_TEST_SRCS})
target_link_libraries(${PROJECT_NAME}-test ${PROJECT_NAME} Qt5::Core Qt5::Gui Qt5::Quick)
