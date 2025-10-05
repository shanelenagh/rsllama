cmake_minimum_required(VERSION 3.10)

set(PROJECT_NAME "rsllama")
project(${PROJECT_NAME} VERSION 0.0.1 LANGUAGES CXX)

set(LLAMA_CPP_DIR ${CMAKE_CURRENT_SOURCE_DIR}/llama_cpp)
find_package(CUDAToolkit QUIET)
if(CUDAToolkit_FOUND)
    message(STATUS "CUDA compiler found, so setting build var to have llama.cpp/ggml use CUDA")
    set(GGML_CUDA ON)
else()
    message(STATUS "CUDA compiler not found. Building without CUDA support.")
endif()
add_subdirectory(${LLAMA_CPP_DIR} ${CMAKE_CURRENT_BINARY_DIR}/shared EXCLUDE_FROM_ALL)

add_library(${PROJECT_NAME} SHARED
  ${PROJECT_NAME}.cpp
)
target_link_libraries(${PROJECT_NAME} PUBLIC llama)
target_include_directories(
  ${PROJECT_NAME}
  PRIVATE ${LLAMA_CPP_DIR}/common
  PRIVATE ${LLAMA_CPP_DIR}/include
  PRIVATE ${LLAMA_CPP_DIR}/ggml/include
)
set_target_properties(${PROJECT_NAME} PROPERTIES
  PUBLIC_HEADER ${PROJECT_NAME}.h
  OUTPUT_NAME ${PROJECT_NAME}
)

target_compile_definitions(${PROJECT_NAME} PUBLIC DART_SHARED_LIB)

# Copy llama.cpp libraries to avarex_llama target directory
add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
  COMMAND ${CMAKE_COMMAND} -E copy_if_different 
    $<TARGET_FILE:llama> $<TARGET_FILE:ggml-base> $<TARGET_FILE:ggml> $<TARGET_FILE:ggml-cpu> $<TARGET_FILE_DIR:${PROJECT_NAME}>
)
if(CUDAToolkit_FOUND)
  add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different $<TARGET_FILE:ggml-cuda>  $<TARGET_FILE_DIR:${PROJECT_NAME}>
  )
endif()

# Add test app, if requested
if (DEFINED BUILD_TEST_EXE)
  add_executable(${PROJECT_NAME}_test ${PROJECT_NAME}.cpp)
  target_link_libraries(${PROJECT_NAME}_test PRIVATE llama ${PROJECT_NAME})
endif()

if (ANDROID)
  # Support Android 15 16k page size
  target_link_options(${PROJECT_NAME} PRIVATE "-Wl,-z,max-page-size=16384")
endif()