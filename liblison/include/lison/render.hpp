#ifndef LISON_RENDER_HPP_INCLUDED_3ZVMANRC7Q5A
#define LISON_RENDER_HPP_INCLUDED_3ZVMANRC7Q5A

#include <lison/lison.hpp>

#include <cairo.h>

namespace lison
{
    void render(const Image &, cairo_t *, double, double);
}

#endif
