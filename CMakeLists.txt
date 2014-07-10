cmake_minimum_required(VERSION 2.8)
project(RustAlgebloat NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(ALGEBLOAT_ROOT src/algebloat/lib.rs)
set(ALGEBLOAT_MACROS_ROOT src/algebloat_macros/lib.rs)
set(EXAMPLE_ROOT examples/example.rs)
set(CUSTOM_CSS_SRC "${CMAKE_SOURCE_DIR}/doc/main.css")
set(CUSTOM_CSS_DEST "${CMAKE_BINARY_DIR}/doc/main.css")

set(RUSTC_FLAGS -L ${CMAKE_BINARY_DIR}/lib --opt-level 3)
set(RUSTDOC_FLAGS -L ${CMAKE_BINARY_DIR}/lib)

# Compile the macros as part of the config process
get_rust_deps(${ALGEBLOAT_MACROS_ROOT} ALGEBLOAT_MACROS_DEPS
              COMPILE
              DESTINATION lib)
get_rust_deps(${ALGEBLOAT_ROOT} ALGEBLOAT_DEPS)

# Build the library
rust_crate(${ALGEBLOAT_MACROS_ROOT}
           TARGET_NAME ALGEBLOAT_MACROS
           DESTINATION lib
           DEPENDS "${ALGEBLOAT_MACROS_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

rust_crate(${ALGEBLOAT_ROOT}
           TARGET_NAME ALGEBLOAT
           DESTINATION lib
           DEPENDS "${ALGEBLOAT_DEPS}" "${ALGEBLOAT_MACROS_FULL_TARGET}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

add_custom_target(library_target
                  ALL
                  DEPENDS ${ALGEBLOAT_FULL_TARGET})

# Build tests
rust_crate(${ALGEBLOAT_ROOT}
           TARGET_NAME ALGEBLOAT_TEST
           DESTINATION test
           DEPENDS "${ALGEBLOAT_DEPS}" "${ALGEBLOAT_MACROS_FULL_TARGET}"
           OTHER_RUSTC_FLAGS --test)

add_custom_target(test
                  COMMAND ./algebloat
                  WORKING_DIRECTORY "${CMAKE_BINARY_DIR}/test"
                  DEPENDS ${ALGEBLOAT_TEST_FULL_TARGET})

# Build examples
rust_crate_auto(${EXAMPLE_ROOT}
                TARGET_NAME EXAMPLE
                DESTINATION examples
                DEPENDS "${ALGEBLOAT_FULL_TARGET}")

add_custom_target(examples_target
                  ALL
                  DEPENDS ${EXAMPLE_FULL_TARGET})


# Build documentation
rust_doc(${ALGEBLOAT_ROOT}
         TARGET_NAME ALGEBLOAT_DOC
         DESTINATION doc
         DEPENDS "${ALGEBLOAT_DEPS}")

# Copy the custom CSS
file(RELATIVE_PATH rel_dest "${CMAKE_BINARY_DIR}" "${CUSTOM_CSS_DEST}")
add_custom_target("DELETE_STOCK_CSS"
                  DEPENDS ${ALGEBLOAT_DOC_FULL_TARGET}
                  COMMAND "${CMAKE_COMMAND}" -E remove "${CUSTOM_CSS_DEST}"
                  COMMENT "Deleting stock ${rel_dest}")

add_custom_command(OUTPUT "${CUSTOM_CSS_DEST}"
                   DEPENDS "DELETE_STOCK_CSS"
                   DEPENDS "${CUSTOM_CSS_SRC}"
                   COMMAND "${CMAKE_COMMAND}" -E copy "${CUSTOM_CSS_SRC}" "${CUSTOM_CSS_DEST}")

add_custom_target(doc
                  DEPENDS ${ALGEBLOAT_DOC_FULL_TARGET} ${CUSTOM_CSS_DEST})

# Install library
install(FILES ${ALGEBLOAT_ARTIFACTS} ${ALGEBLOAT_MACROS_ARTIFACTS}
        DESTINATION lib)
