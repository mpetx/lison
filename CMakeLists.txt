cmake_minimum_required(VERSION 3.28.1)

project(Lison CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(PkgConfig REQUIRED)

pkg_check_modules(CAIRO REQUIRED cairo)

add_subdirectory(liblison)
add_subdirectory(lison2png)

install(EXPORT Lison
  DESTINATION share/lison/cmake
  NAMESPACE Lison::
  FILE LisonConfig.cmake)
