#ifndef LISON_LISON_HPP_INCLUDED_J4X2VLAPJCEH
#define LISON_LISON_HPP_INCLUDED_J4X2VLAPJCEH

#include <cstddef>
#include <optional>
#include <variant>
#include <vector>

namespace lison
{
    struct Point
    {
	double x, y;
    };

    struct Color
    {
	double red, green, blue, alpha;
    };

    struct Monochrome
    {
	Color color;
    };

    struct LinearGradient
    {
	Point point1;
	Color color1;
	Point point2;
	Color color2;
    };

    struct RadialGradient
    {
	Point center1;
	double radius1;
	Color color1;
	Point center2;
	double radius2;
	Color color2;
    };

    using Pattern = std::variant<
	Monochrome,
	LinearGradient,
	RadialGradient>;

    enum class LineCap
    {
	butt, round, square
    };

    enum class LineJoin
    {
	miter, round, bevel
    };

    struct Pen
    {
	Pattern pattern;
	double width;
	LineCap cap;
	LineJoin join;
    };

    struct Brush
    {
	Pattern pattern;
    };

    struct Group;
    struct Curve;
    struct Region;

    using Shape = std::variant<
	Group,
	Curve,
	Region>;

    struct Group
    {
	std::vector<Shape> content;
    };

    struct LineSegment
    {
	Point point2;
    };

    struct QuadraticBezierSegment
    {
	Point point2, point3;
    };

    struct CubicBezierSegment
    {
	Point point2, point3, point4;
    };

    using Segment = std::variant<
	LineSegment,
	QuadraticBezierSegment,
	CubicBezierSegment>;

    struct CurveData
    {
	Point start;
	std::vector<Segment> segments;
    };

    struct RegionData
    {
	std::vector<CurveData> curves;
    };

    struct Curve
    {
	std::size_t pen;
	CurveData data;
    };

    struct Region
    {
	std::optional<std::size_t> pen;
	std::optional<std::size_t> brush;
	RegionData data;
    };

    struct Image
    {
	double width, height;
	double unit_per_inch;
	std::vector<Pen> pens;
	std::vector<Brush> brushes;
	std::vector<Shape> shapes;
    };
}

#endif
