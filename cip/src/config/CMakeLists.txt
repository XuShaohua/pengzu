
function(generate_config_file)
  set(CIP_VERSION ${PROJECT_VERSION})

  execute_process(
      COMMAND git rev-parse --short HEAD
      OUTPUT_VARIABLE CIP_BUILD_ID
      OUTPUT_STRIP_TRAILING_WHITESPACE
  )
  execute_process(
      COMMAND git describe --tags
      OUTPUT_VARIABLE CIP_BUILD_TAG
      OUTPUT_STRIP_TRAILING_WHITESPACE
  )
  execute_process(
      COMMAND git show -s --format=%ci
      OUTPUT_VARIABLE CIP_BUILD_DATE
      OUTPUT_STRIP_TRAILING_WHITESPACE
  )
  configure_file(config.h.in ${CMAKE_CURRENT_BINARY_DIR}/config.h
      @ONLY
      )
endfunction()

generate_config_file()
