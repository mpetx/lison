#ifndef LISON_PARSE_HPP_INCLUDED_MENZQTBYY48P
#define LISON_PARSE_HPP_INCLUDED_MENZQTBYY48P

#include <lison/lison.hpp>

#include <string_view>
#include <variant>

namespace lison
{
    enum class ParseFailure
    {
	bad_json,
	bad_image,
	bad_pen,
	bad_brush,
	bad_shape
    };

    using ParseResult = std::variant<Image, ParseFailure>;

    ParseResult parse(std::string_view);
}

#endif
