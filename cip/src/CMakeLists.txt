
find_package(Qt5 5.12 COMPONENTS Core Gui Svg REQUIRED)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

find_package(PkgConfig REQUIRED)
pkg_check_modules(PopplerQt5 REQUIRED IMPORTED_TARGET poppler-qt5)
pkg_check_modules(LibMobi REQUIRED IMPORTED_TARGET libmobi)

add_subdirectory(third_party/libepub)

set(LINK_LIBS
    PkgConfig::PopplerQt5
    PkgConfig::LibMobi
    Qt5::Gui
    libepub
    )

set(SOURCE_FILES
    formats/epub_reader.cpp
    formats/epub_reader.h
    formats/mobi_reader.cpp
    formats/mobi_reader.h
    formats/pdf_reader.cpp
    formats/pdf_reader.h
    )

add_executable(cip
    app/main.cpp
    ${SOURCE_FILES}
    )
target_link_libraries(cip
    ${LINK_LIBS}
    )
target_include_directories(cip PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})