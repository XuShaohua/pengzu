
find_package(Qt5 5.12 COMPONENTS Core REQUIRED)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

find_package(PkgConfig REQUIRED)
pkg_check_modules(PopplerQt5 REQUIRED IMPORTED_TARGET poppler-qt5)
pkg_check_modules(LibMobi REQUIRED IMPORTED_TARGET libmobi)

add_subdirectory(config)
add_subdirectory(third_party/libepub)

set(LINK_LIBS
    PkgConfig::PopplerQt5
    PkgConfig::LibMobi
    Qt5::Core
    libepub
    )

set(SOURCE_FILES
    controllers/shell.cpp
    controllers/shell.h

    formats/cip_record.cpp
    formats/cip_record.h
    formats/epub_parser.cpp
    formats/epub_parser.h
    formats/epub_reader.cpp
    formats/epub_reader.h
    formats/mobi_parser.cpp
    formats/mobi_parser.h
    formats/mobi_reader.cpp
    formats/mobi_reader.h
    formats/pdf_parser.cpp
    formats/pdf_parser.h
    formats/pdf_reader.cpp
    formats/pdf_reader.h
    formats/util.cpp
    formats/util.h
    )

add_executable(cip
    app/cip.cpp
    ${SOURCE_FILES}
    )
target_link_libraries(cip
    ${LINK_LIBS}
    )
target_include_directories(cip
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}
    PRIVATE ${CMAKE_CURRENT_BINARY_DIR}
    )
