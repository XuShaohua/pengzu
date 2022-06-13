# - Try to find libzip
# Once done this will define
#
#  LIBZIP_FOUND - system has the zip library
#  LIBZIP_INCLUDE_DIRS - the zip include directories
#  LIBZIP_LIBRARY - Link this to use the zip library
#
# Copyright (c) 2006, Pino Toscano, <toscano.pino@tiscali.it>
#
# Redistribution and use is allowed according to the terms of the BSD license.
# For details see the accompanying COPYING-CMAKE-SCRIPTS file.

if (LIBZIP_LIBRARY AND LIBZIP_INCLUDE_DIRS)
  # in cache already
  set(LIBZIP_FOUND TRUE)
else (LIBZIP_LIBRARY AND LIBZIP_INCLUDE_DIRS)

  find_path(_LIBZIP_INCLUDE_DIR zip.h
    ${GNUWIN32_DIR}/include
  )

  find_library(LIBZIP_LIBRARY NAMES zip
    PATHS
    ${GNUWIN32_DIR}/lib
  )

  if (LIBZIP_LIBRARY)
    get_filename_component(_LIBZIP_LIBRARY_DIR ${LIBZIP_LIBRARY} PATH)

    find_path(_ZIPCONF_INCLUDE_DIR zipconf.h
      PATHS
      ${_LIBZIP_LIBRARY_DIR}/include
      ${_LIBZIP_LIBRARY_DIR}/libzip/include
    )
  endif()

  include(FindPackageHandleStandardArgs)
  FIND_PACKAGE_HANDLE_STANDARD_ARGS(LibZip DEFAULT_MSG LIBZIP_LIBRARY _LIBZIP_INCLUDE_DIR _ZIPCONF_INCLUDE_DIR)

  set(LIBZIP_INCLUDE_DIRS)
  list(APPEND LIBZIP_INCLUDE_DIRS "${_LIBZIP_INCLUDE_DIR}")
  list(APPEND LIBZIP_INCLUDE_DIRS "${_ZIPCONF_INCLUDE_DIR}")
    # ensure that they are cached
    set(LIBZIP_INCLUDE_DIRS ${LIBZIP_INCLUDE_DIRS} CACHE INTERNAL "The libzip include paths")
    set(LIBZIP_LIBRARY ${LIBZIP_LIBRARY} CACHE INTERNAL "The libraries needed to use libzip")

endif (LIBZIP_LIBRARY AND LIBZIP_INCLUDE_DIRS)

mark_as_advanced(LIBZIP_INCLUDE_DIRS LIBZIP_LIBRARY)
