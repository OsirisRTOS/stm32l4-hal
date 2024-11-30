# stm32-nucleo-l4r5zi/CMakeLists.txt

include(FetchContent)

cmake_minimum_required(VERSION 3.28)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/cmake/arm-none-eabi.cmake")

project(Hal C CXX ASM)

set(Rust_CARGO_TARGET thumbv7em-none-eabi)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml NO_STD)

target_include_directories(hal
    INTERFACE
        include
)

target_sources(hal
    INTERFACE
        include/hal/lib.h
)