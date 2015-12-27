cmake_minimum_required(VERSION 2.8)
project(RustAlgebloat NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
find_package(cargo)
include(Rust)

cargo_dependency(cargo
                 OTHER_CARGO_FLAGS --release
                 PACKAGE_NAMES chrono
                 PACKAGE_VERSIONS =0.2.17)

set(ALGEBLOAT_ROOT algebloat/src/lib.rs)
set(ALGEBLOAT_MACROS_ROOT algebloat_macros/src/lib.rs)
set(EXAMPLE_ROOT algebloat/examples/example.rs)
set(CUSTOM_CSS_SRC "${CMAKE_SOURCE_DIR}/doc/main.css")
set(CUSTOM_CSS_DEST "${CMAKE_BINARY_DIR}/doc/main.css")
set(STOCK_CSS "${CMAKE_BINARY_DIR}/doc/main.css;${CMAKE_BINARY_DIR}/doc/rustdoc.css;")
set(CARGO_DEPENDENCY_DIR "${CMAKE_BINARY_DIR}/cargo/target/release/deps")

set(RUSTC_FLAGS -L ${CMAKE_BINARY_DIR}/lib -L ${CARGO_DEPENDENCY_DIR} -C opt-level=3)
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
add_custom_target("DELETE_STOCK_CSS"
                  DEPENDS ${GNUPLOT_DOC_FULL_TARGET}
                  COMMAND "${CMAKE_COMMAND}" -E remove -f ${STOCK_CSS}
                  COMMENT "Deleting stock CSS")

add_custom_command(OUTPUT "${CUSTOM_CSS_DEST}"
                   DEPENDS "DELETE_STOCK_CSS"
                   DEPENDS "${CUSTOM_CSS_SRC}"
                   COMMAND "${CMAKE_COMMAND}" -E copy "${CUSTOM_CSS_SRC}" "${CUSTOM_CSS_DEST}")

add_custom_target(doc
                  DEPENDS ${ALGEBLOAT_DOC_FULL_TARGET} ${CUSTOM_CSS_DEST})

# Install library
install(FILES ${ALGEBLOAT_ARTIFACTS} ${ALGEBLOAT_MACROS_ARTIFACTS}
        DESTINATION lib)
