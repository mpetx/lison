
#include <lison/parse.hpp>

#include <algorithm>
#include <cstddef>
#include <cmath>
#include <ranges>
#include <string>
#include <unordered_set>
#include <utility>
#include <vector>

#include <picojson.h>

namespace lison
{
    template <class T, class Parse>
    static std::vector<T> parse_array(const picojson::value &, Parse, ParseFailure);

    template <class T, class... Fillers>
    static T parse_object(
	const picojson::value &,
	const std::vector<std::string> &,
	const std::unordered_set<std::string> &,
	ParseFailure,
	Fillers...);

    template <class T, class Parse, class Pred>
    static T parse_sub(const picojson::value &, Parse, Pred, ParseFailure);

    template <class T, class... Fillers>
    static T parse_tuple(const picojson::value &, ParseFailure, Fillers...);

    static double parse_number(const picojson::value &, ParseFailure);
    static double parse_positive_number(const picojson::value &, ParseFailure);
    static double parse_non_negative_number(const picojson::value &, ParseFailure);
    static double parse_channel(const picojson::value &, ParseFailure);
    static std::size_t parse_index(const picojson::value &, ParseFailure);

    static Point parse_point(const picojson::value &, ParseFailure);
    static Color parse_color(const picojson::value &, ParseFailure);

    static Pattern parse_pattern(const picojson::value &, ParseFailure);
    static Monochrome parse_monochrome(const picojson::value &, ParseFailure);
    static LinearGradient parse_linear_gradient(const picojson::value &, ParseFailure);
    static RadialGradient parse_radial_gradient(const picojson::value &, ParseFailure);

    static Pen parse_pen(const picojson::value &);
    static LineCap parse_line_cap(const picojson::value &);
    static LineJoin parse_line_join(const picojson::value &);
    static Brush parse_brush(const picojson::value &);

    static Shape parse_shape(const picojson::value &);
    static Group parse_group(const picojson::value &);
    static Curve parse_curve(const picojson::value &);
    static Region parse_region(const picojson::value &);

    static CurveData parse_curve_data(const picojson::value &);
    static RegionData parse_region_data(const picojson::value &);

    static Segment parse_segment(const picojson::value &);
    static LineSegment parse_line_segment(const picojson::value &);
    static QuadraticBezierSegment parse_quadratic_bezier_segment(const picojson::value &);
    static CubicBezierSegment parse_cubic_bezier_segment(const picojson::value &);

    static Image parse_image(const picojson::value &);

    ParseResult parse(std::string_view text)
    {
	picojson::value val;
	std::string err;

	picojson::parse(val, text.begin(), text.end(), &err);

	if (!err.empty())
	{
	    return ParseFailure::bad_json;
	}

	try
	{
	    return parse_image(val);
	}
	catch (ParseFailure f)
	{
	    return f;
	}
    }

    template <class T, class P>
    static std::vector<T> parse_array(const picojson::value &val, P parse, ParseFailure ctx)
    {
	if (!val.is<picojson::array>())
	{
	    throw ctx;
	}

	auto items = val.get<picojson::array>()
	    | std::ranges::views::transform(parse);

	return std::vector<T> { items.begin(), items.end() };
    }

    template <class T, class... Fillers>
    static T parse_object(
	const picojson::value &val,
	const std::vector<std::string> &required_members,
	const std::unordered_set<std::string> &allowed_members,
	ParseFailure ctx,
	Fillers... fillers)
    {
	if (!val.is<picojson::object>())
	{
	    throw ctx;
	}

	const picojson::object &obj = val.get<picojson::object>();

	auto has_member_named = [&obj](const std::string &member) {
	    return obj.contains(member);
	};

	auto is_member_allowed = [&allowed_members](const auto &member) {
	    return allowed_members.contains(member.first);
	};

	if (!std::ranges::all_of(required_members, has_member_named)
	    || !std::ranges::all_of(obj, is_member_allowed))
	{
	    throw ctx;
	}

	T t;

	auto fill = [&t, &obj](auto filler) {
	    auto i = obj.find(filler.first);

	    if (i != obj.end())
	    {
		filler.second(t, i->second);
	    }
	};

	(fill(fillers), ...);

	return t;
    }

    template <class T, class Parse, class Pred>
    static T parse_sub(const picojson::value &val, Parse parse, Pred pred, ParseFailure ctx)
    {
	T res = parse(val, ctx);

	if (pred(res))
	{
	    return res;
	}
	else
	{
	    throw ctx;
	}
    }

    template <class T, class... Fillers>
    static T parse_tuple(const picojson::value &val, ParseFailure ctx, Fillers... fillers)
    {
	if (!val.is<picojson::array>())
	{
	    throw ctx;
	}

	const picojson::array &arr = val.get<picojson::array>();

	T t;

	if (arr.size() != sizeof...(fillers))
	{
	    throw ctx;
	}

	std::size_t i = 0;
	(fillers(t, arr[i++]), ...);

	return t;
    }

    static double parse_number(const picojson::value &val, ParseFailure ctx)
    {
	if (!val.is<double>())
	{
	    throw ctx;
	}

	return val.get<double>();
    }

    static double parse_positive_number(const picojson::value &val, ParseFailure ctx)
    {
	auto is_positive = [](double num) { return num > 0; };
	return parse_sub<double>(val, parse_number, is_positive, ctx);
    }

    static double parse_non_negative_number(const picojson::value &val, ParseFailure ctx)
    {
	auto is_non_negative = [](double num) { return num >= 0; };
	return parse_sub<double>(val, parse_number, is_non_negative, ctx);
    }

    static double parse_channel(const picojson::value &val, ParseFailure ctx)
    {
	auto is_channel = [](double num) { return 0 <= num && num <= 1; };
	return parse_sub<double>(val, parse_number, is_channel, ctx);
    }

    static std::size_t parse_index(const picojson::value &val, ParseFailure ctx)
    {
	auto is_index = [](double num) {
	    return num >= 0 &&std::floor(num) == num;
	};
	return parse_sub<double>(val, parse_number, is_index, ctx);
    }

    static Point parse_point(const picojson::value &val, ParseFailure ctx)
    {
	return parse_tuple<Point>(
	    val,
	    ctx,
	    [ctx](Point &p, const picojson::value &val) {
		p.x = parse_number(val, ctx);
	    },
	    [ctx](Point &p, const picojson::value &val) {
		p.y = parse_number(val, ctx);
	    });
    }

    static Color parse_color(const picojson::value &val, ParseFailure ctx)
    {
	if (!val.is<picojson::array>())
	{
	    throw ctx;
	}

	const picojson::array &arr = val.get<picojson::array>();

	if (arr.size() == 3)
	{
	    return Color {
		parse_channel(arr[0], ctx),
		parse_channel(arr[1], ctx),
		parse_channel(arr[2], ctx),
		1
	    };
	}
	else if (arr.size() == 4)
	{
	    return Color {
		parse_channel(arr[0], ctx),
		parse_channel(arr[1], ctx),
		parse_channel(arr[2], ctx),
		parse_channel(arr[3], ctx)
	    };
	}
	else
	{
	    throw ctx;
	}
    }

    static Pattern parse_pattern(const picojson::value &val, ParseFailure ctx)
    {
	if (!val.is<picojson::object>())
	{
	    throw ctx;
	}

	const picojson::object &obj = val.get<picojson::object>();

	auto i = obj.find("type");

	if (i == obj.end() || !i->second.is<std::string>())
	{
	    throw ctx;
	}

	const std::string &type = i->second.get<std::string>();

	if (type == "monochrome")
	{
	    return parse_monochrome(val, ctx);
	}
	else if (type == "linear-gradient")
	{
	    return parse_linear_gradient(val, ctx);
	}
	else if (type == "radial-gradient")
	{
	    return parse_radial_gradient(val, ctx);
	}
	else
	{
	    throw ctx;
	}
    }

    const std::vector<std::string> monochrome_required_members {
	"type", "color"
    };

    const std::unordered_set<std::string> monochrome_allowed_members {
	"type", "color"
    };

    static Monochrome parse_monochrome(const picojson::value &val, ParseFailure ctx)
    {
	return parse_object<Monochrome>(
	    val,
	    monochrome_required_members,
	    monochrome_allowed_members,
	    ctx,
	    std::make_pair(
		"color",
		[ctx](Monochrome &pattern, const picojson::value &val) {
		    pattern.color = parse_color(val, ctx);
		}));
    }

    const std::vector<std::string> linear_gradient_required_members {
	"type", "point-1", "color-1", "point-2", "color-2"
    };

    const std::unordered_set<std::string> linear_gradient_allowed_members {
	"type", "point-1", "color-1", "point-2", "color-2"
    };

    static LinearGradient parse_linear_gradient(const picojson::value &val, ParseFailure ctx)
    {
	return parse_object<LinearGradient>(
	    val,
	    linear_gradient_required_members,
	    linear_gradient_allowed_members,
	    ctx,
	    std::make_pair(
		"point-1",
		[ctx](LinearGradient &pattern, const picojson::value &val) {
		    pattern.point1 = parse_point(val, ctx);
		}),
	    std::make_pair(
		"color-1",
		[ctx](LinearGradient &pattern, const picojson::value &val) {
		    pattern.color1 = parse_color(val, ctx);
		}),
	    std::make_pair(
		"point-2",
		[ctx](LinearGradient &pattern, const picojson::value &val) {
		    pattern.point2 = parse_point(val, ctx);
		}),
	    std::make_pair(
		"color-2",
		[ctx](LinearGradient &pattern, const picojson::value &val) {
		    pattern.color2 = parse_color(val, ctx);
		}));
    }

    const std::vector<std::string> radial_gradient_required_members {
	"type", "center-1", "radius-1", "color-1", "center-2", "radius-2", "color-2"
    };

    const std::unordered_set<std::string> radial_gradient_allowed_members {
	"type", "center-1", "radius-1", "color-1", "center-2", "radius-2", "color-2"	
    };

    static RadialGradient parse_radial_gradient(const picojson::value &val, ParseFailure ctx)
    {
	return parse_object<RadialGradient>(
	    val,
	    radial_gradient_required_members,
	    radial_gradient_allowed_members,
	    ctx,
	    std::make_pair(
		"center-1",
		[ctx](RadialGradient &pattern, const picojson::value &val) {
		    pattern.center1 = parse_point(val, ctx);
		}),
	    std::make_pair(
		"radius-1",
		[ctx](RadialGradient &pattern, const picojson::value &val) {
		    pattern.radius1 = parse_non_negative_number(val, ctx);
		}),
	    std::make_pair(
		"color-1",
		[ctx](RadialGradient &pattern, const picojson::value &val) {
		    pattern.color1 = parse_color(val, ctx);
		}),
	    std::make_pair(
		"center-2",
		[ctx](RadialGradient &pattern, const picojson::value &val) {
		    pattern.center2 = parse_point(val, ctx);
		}),
	    std::make_pair(
		"radius-2",
		[ctx](RadialGradient &pattern, const picojson::value &val) {
		    pattern.radius2 = parse_non_negative_number(val, ctx);
		}),
	    std::make_pair(
		"color-2",
		[ctx](RadialGradient &pattern, const picojson::value &val) {
		    pattern.color2 = parse_color(val, ctx);
		}));
    }

    const std::vector<std::string> pen_required_members {
	"pattern", "width", "cap", "join"
    };

    const std::unordered_set<std::string> pen_allowed_members {
	"pattern", "width", "cap", "join"
    };

    static Pen parse_pen(const picojson::value &val)
    {
	return parse_object<Pen>(
	    val,
	    pen_required_members,
	    pen_allowed_members,
	    ParseFailure::bad_pen,
	    std::make_pair(
		"pattern",
		[](Pen &pen, const picojson::value &val) {
		    pen.pattern = parse_pattern(val, ParseFailure::bad_pen);
		}),
	    std::make_pair(
		"width",
		[](Pen &pen, const picojson::value &val) {
		    pen.width = parse_positive_number(val, ParseFailure::bad_pen);
		}),
	    std::make_pair(
		"cap",
		[](Pen &pen, const picojson::value &val) {
		    pen.cap = parse_line_cap(val);
		}),
	    std::make_pair(
		"join",
		[](Pen &pen, const picojson::value &val) {
		    pen.join = parse_line_join(val);
		}));
    }

    static LineCap parse_line_cap(const picojson::value &val)
    {
	if (!val.is<std::string>())
	{
	    throw ParseFailure::bad_pen;
	}

	const std::string &str = val.get<std::string>();

	if (str == "butt")
	{
	    return LineCap::butt;
	}
	else if (str == "round")
	{
	    return LineCap::round;
	}
	else if (str == "square")
	{
	    return LineCap::square;
	}
	else
	{
	    throw ParseFailure::bad_pen;
	}
    }

    static LineJoin parse_line_join(const picojson::value &val)
    {
	if (!val.is<std::string>())
	{
	    throw ParseFailure::bad_pen;
	}

	const std::string &str = val.get<std::string>();

	if (str == "miter")
	{
	    return LineJoin::miter;
	}
	else if (str == "round")
	{
	    return LineJoin::round;
	}
	else if (str == "bevel")
	{
	    return LineJoin::bevel;
	}
	else
	{
	    throw ParseFailure::bad_pen;
	}
    }

    const std::vector<std::string> brush_required_members {
	"pattern"
    };

    const std::unordered_set<std::string> brush_allowed_members {
	"pattern"
    };

    static Brush parse_brush(const picojson::value &val)
    {
	return parse_object<Brush>(
	    val,
	    brush_required_members,
	    brush_allowed_members,
	    ParseFailure::bad_brush,
	    std::make_pair(
		"pattern",
		[](Brush &brush, const picojson::value &val) {
		    brush.pattern = parse_pattern(val, ParseFailure::bad_brush);
		}));
    }

    static Shape parse_shape(const picojson::value &val)
    {
	if (!val.is<picojson::object>())
	{
	    throw ParseFailure::bad_shape;
	}

	const picojson::object &obj = val.get<picojson::object>();

	auto i = obj.find("type");

	if (i == obj.end() || !i->second.is<std::string>())
	{
	    throw ParseFailure::bad_shape;
	}

	const std::string &type = i->second.get<std::string>();

	if (type == "group")
	{
	    return parse_group(val);
	}
	else if (type == "curve")
	{
	    return parse_curve(val);
	}
	else if (type == "region")
	{
	    return parse_region(val);
	}
	else
	{
	    throw ParseFailure::bad_shape;
	}
    }

    const std::vector<std::string> group_required_members {
	"type", "content"
    };

    const std::unordered_set<std::string> group_allowed_members {
	"type", "content", "edit-annot"
    };

    static Group parse_group(const picojson::value &val)
    {
	return parse_object<Group>(
	    val,
	    group_required_members,
	    group_allowed_members,
	    ParseFailure::bad_shape,
	    std::make_pair(
		"content",
		[](Group &shape, const picojson::value &val) {
		    shape.content = parse_array<Shape>(val, parse_shape, ParseFailure::bad_shape);
		}));
    }

    const std::vector<std::string> curve_required_members {
	"type", "pen", "data"
    };

    const std::unordered_set<std::string> curve_allowed_members {
	"type", "pen", "data"
    };

    static Curve parse_curve(const picojson::value &val)
    {
	return parse_object<Curve>(
	    val,
	    curve_required_members,
	    curve_allowed_members,
	    ParseFailure::bad_shape,
	    std::make_pair(
		"pen",
		[](Curve &shape, const picojson::value &val) {
		    shape.pen = parse_index(val, ParseFailure::bad_shape);
		}),
	    std::make_pair(
		"data",
		[](Curve &shape, const picojson::value &val) {
		    shape.data = parse_curve_data(val);
		}));
    }

    const std::vector<std::string> region_required_members {
	"type", "data"
    };

    const std::unordered_set<std::string> region_allowed_members {
	"type", "pen", "brush", "data"
    };

    static Region parse_region(const picojson::value &val)
    {
	return parse_object<Region>(
	    val,
	    region_required_members,
	    region_allowed_members,
	    ParseFailure::bad_shape,
	    std::make_pair(
		"pen",
		[](Region &shape, const picojson::value &val) {
		    shape.pen = parse_index(val, ParseFailure::bad_shape);
		}),
	    std::make_pair(
		"brush",
		[](Region &shape, const picojson::value &val) {
		    shape.brush = parse_index(val, ParseFailure::bad_shape);
		}),
	    std::make_pair(
		"data",
		[](Region &shape, const picojson::value &val) {
		    shape.data = parse_region_data(val);
		}));
    }

    static CurveData parse_curve_data(const picojson::value &val)
    {
	if (!val.is<picojson::array>())
	{
	    throw ParseFailure::bad_shape;
	}

	const picojson::array &arr = val.get<picojson::array>();

	if (arr.size() == 0)
	{
	    throw ParseFailure::bad_shape;
	}

	auto segs = arr
	    | std::ranges::views::drop(1)
	    | std::ranges::views::transform(parse_segment);
	    
	return CurveData {
	    parse_point(arr[0], ParseFailure::bad_shape),
	    std::vector<Segment> { segs.begin(), segs.end() }
	};
    }

    static RegionData parse_region_data(const picojson::value &val)
    {
	return RegionData {
	    parse_array<CurveData>(val, parse_curve_data, ParseFailure::bad_shape)
	};
    }

    static Segment parse_segment(const picojson::value &val)
    {
	if (!val.is<picojson::array>())
	{
	    throw ParseFailure::bad_shape;
	}

	const picojson::array &arr = val.get<picojson::array>();

	if (arr.size() == 0 || !arr[0].is<std::string>())
	{
	    throw ParseFailure::bad_shape;
	}

	const std::string &type = arr[0].get<std::string>();

	if (type == "L")
	{
	    return parse_line_segment(val);
	}
	else if (type == "Q")
	{
	    return parse_quadratic_bezier_segment(val);
	}
	else if (type == "C")
	{
	    return parse_cubic_bezier_segment(val);
	}
	else
	{
	    throw ParseFailure::bad_shape;
	}
    }

    static LineSegment parse_line_segment(const picojson::value &val)
    {
	return parse_tuple<LineSegment>(
	    val,
	    ParseFailure::bad_shape,
	    [](LineSegment &seg, const picojson::value &val) {
	    },
	    [](LineSegment &seg, const picojson::value &val) {
		seg.point2 = parse_point(val, ParseFailure::bad_shape);
	    });
    }

    static QuadraticBezierSegment parse_quadratic_bezier_segment(const picojson::value &val)
    {
	return parse_tuple<QuadraticBezierSegment>(
	    val,
	    ParseFailure::bad_shape,
	    [](QuadraticBezierSegment &seg, const picojson::value &val) {
	    },
	    [](QuadraticBezierSegment &seg, const picojson::value &val) {
		seg.point2 = parse_point(val, ParseFailure::bad_shape);
	    },
	    [](QuadraticBezierSegment &seg, const picojson::value &val) {
		seg.point3 = parse_point(val, ParseFailure::bad_shape);
	    });
    }

    static CubicBezierSegment parse_cubic_bezier_segment(const picojson::value &val)
    {
	return parse_tuple<CubicBezierSegment>(
	    val,
	    ParseFailure::bad_shape,
	    [](CubicBezierSegment &seg, const picojson::value &val) {
	    },
	    [](CubicBezierSegment &seg, const picojson::value &val) {
		seg.point2 = parse_point(val, ParseFailure::bad_shape);
	    },
	    [](CubicBezierSegment &seg, const picojson::value &val) {
		seg.point3 = parse_point(val, ParseFailure::bad_shape);
	    },
	    [](CubicBezierSegment &seg, const picojson::value &val) {
		seg.point4 = parse_point(val, ParseFailure::bad_shape);
	    });
    }

    const std::vector<std::string> image_required_members {
	"width", "height", "unit-per-inch", "pens", "brushes", "shapes"
    };

    const std::unordered_set<std::string> image_allowed_members {
	"width", "height", "unit-per-inch", "editor", "pens", "brushes", "shapes"
    };

    namespace detail
    {
	struct ShapeIndexChecker
	{
	    const Image &image;

	    bool operator()(const Group &group) const
	    {
		return std::ranges::all_of(group.content, [*this](const Shape &shape) {
		    return std::visit(*this, shape);
		});
	    }

	    bool operator()(const Curve &curve) const
	    {
		return curve.pen < image.pens.size();
	    }

	    bool operator()(const Region &region) const
	    {
		return (!region.pen.has_value()
			|| region.pen.value() < image.pens.size())
		    && (!region.brush.has_value()
			|| region.brush.value() < image.brushes.size());
	    }
	};
    }

    static Image parse_image(const picojson::value &val)
    {
	auto im = parse_object<Image>(
	    val,
	    image_required_members,
	    image_allowed_members,
	    ParseFailure::bad_image,
	    std::make_pair(
		"width",
		[](Image &im, const picojson::value &val) {
		    im.width = parse_positive_number(val, ParseFailure::bad_image);
		}),
	    std::make_pair(
		"height",
		[](Image &im, const picojson::value &val) {
		    im.height = parse_positive_number(val, ParseFailure::bad_image);
		}),
	    std::make_pair(
		"unit-per-inch",
		[](Image &im, const picojson::value &val) {
		    im.unit_per_inch = parse_positive_number(val, ParseFailure::bad_image);
		}),
	    std::make_pair(
		"pens",
		[](Image &im, const picojson::value &val) {
		    im.pens = parse_array<Pen>(val, parse_pen, ParseFailure::bad_image);
		}),
	    std::make_pair(
		"brushes",
		[](Image &im, const picojson::value &val) {
		    im.brushes = parse_array<Brush>(val, parse_brush, ParseFailure::bad_image);
		}),
	    std::make_pair(
		"shapes",
		[](Image &im, const picojson::value &val) {
		    im.shapes = parse_array<Shape>(val, parse_shape, ParseFailure::bad_image);
		}));

	detail::ShapeIndexChecker checker { im };
	if (!std::ranges::all_of(im.shapes, [&checker](const Shape &shape) {
	    return std::visit(checker, shape);
	}))
	{
	    throw ParseFailure::bad_shape;
	}

	return im;
    }
}
