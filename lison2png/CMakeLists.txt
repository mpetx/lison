
set(LisonToPngSources
  main.cpp)
list(TRANSFORM LisonToPngSources PREPEND src/)

add_executable(LisonToPng
  ${LisonToPngSources})

target_link_libraries(LisonToPng
  PRIVATE
  Lison)

set_target_properties(LisonToPng
  PROPERTIES
  OUTPUT_NAME lison2png)

install(TARGETS LisonToPng)
