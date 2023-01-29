project (EBOOK-TOOLS C)

cmake_minimum_required(VERSION 3.10)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake)

set(LIB_SUFFIX "" CACHE STRING "Define suffix of library directory name (32/64)")

find_package(LibXml2 REQUIRED)
find_package(LibZip REQUIRED)

if(CMAKE_C_COMPILER_ID MATCHES GNU)
  set(CMAKE_C_FLAGS
      "-Wall -W -Wno-long-long -Wundef -Wcast-align -Werror-implicit-function-declaration -Wchar-subscripts -Wpointer-arith -Wwrite-strings -Wformat-security -Wmissing-format-attribute -Wshadow -fno-common ${CMAKE_C_FLAGS}"
      )
endif(CMAKE_C_COMPILER_ID MATCHES GNU)
if(MSVC)
  add_definitions(-D_CRT_SECURE_NO_WARNINGS)
endif(MSVC)

add_library (libepub STATIC
    epub.c
    epub.h
    epub_shared.h
    epub_version.h
    epublib.h
    linklist.c
    linklist.h
    list.c
    ocf.c
    opf.c
    )
target_include_directories(libepub
    PRIVATE ${LIBXML2_INCLUDE_DIR}
    ${LIBZIP_INCLUDE_DIRS}
    )
target_link_libraries (libepub
    ${LIBZIP_LIBRARY}
    ${LIBXML2_LIBRARIES}
    )
set_target_properties (libepub
    PROPERTIES VERSION 0.2.1 SOVERSION 0
    )
