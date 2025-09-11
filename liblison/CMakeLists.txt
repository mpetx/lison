
set(LisonSources
  parse.cpp
  render.cpp)
list(TRANSFORM LisonSources PREPEND src/)

set(LisonHeaders
  lison/lison.hpp
  lison/parse.hpp
  lison/render.hpp)
list(TRANSFORM LisonHeaders PREPEND include/)

add_library(Lison
  ${LisonSources})

target_sources(Lison
  PUBLIC
  FILE_SET lison_headers
  TYPE HEADERS
  BASE_DIRS include
  FILES ${LisonHeaders})

target_include_directories(Lison
  PUBLIC
  ${CAIRO_INCLUDE_DIRS})

target_link_libraries(Lison
  PUBLIC
  ${CAIRO_LIBRARIES})

set_target_properties(Lison
  PROPERTIES
  OUTPUT_NAME lison)

install(TARGETS Lison
  EXPORT Lison
  FILE_SET lison_headers)
