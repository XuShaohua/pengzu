
find_package(Qt5 5.12 COMPONENTS Core Gui Svg REQUIRED)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(LINK_LIBS
    Qt5::Gui
    )

set(SOURCE_FILES
    pdf/pdf_reader.cpp
    pdf/pdf_reader.h
    )

add_executable(cip-pdf
    app/main.cpp
    ${SOURCE_FILES}
    )
target_link_libraries(cip-pdf
    ${LINK_LIBS}
    )
target_include_directories(cip-pdf PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})