# stm32-nucleo-l4r5zi/CMakeLists.txt

cmake_minimum_required(VERSION 3.28)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/cmake/arm-none-eabi.cmake")

project(HAL C CXX ASM)

add_library(HAL STATIC
    "src/lib.c"
)

target_include_directories(HAL
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_compile_options(HAL
    PRIVATE
        -mcpu=cortex-m4
        -mthumb
        -Wall
        -Wextra
        -Werror
)



