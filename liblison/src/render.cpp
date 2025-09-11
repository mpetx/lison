
#include <lison/render.hpp>

#include <algorithm>
#include <ranges>
#include <variant>

namespace lison
{
    namespace detail
    {
	struct CoordTransformer
	{
	    double scale;

	    CoordTransformer(const Image &im, double ppi, double s)
		: scale { ppi / im.unit_per_inch * s }
	    {
	    }

	    double transform(double x) const
	    {
		return x * scale;
	    }

	    Point transform(const Point &p) const
	    {
		return Point {
		    transform(p.x),
		    transform(p.y)
		};
	    }
	};
    }

    static void render_shape(const Shape &, cairo_t *, const Image &, const detail::CoordTransformer &);

    namespace detail
    {
	struct PatternSetter
	{
	    cairo_t * const cr;
	    const CoordTransformer &transformer;

	    void operator()(const Monochrome &pattern) const
	    {
		cairo_set_source_rgba(
		    cr,
		    pattern.color.red,
		    pattern.color.green,
		    pattern.color.blue,
		    pattern.color.alpha);
	    }

	    void operator()(const LinearGradient &pattern) const
	    {
		Point p1 = transformer.transform(pattern.point1);
		Point p2 = transformer.transform(pattern.point2);
		cairo_pattern_t *pat = cairo_pattern_create_linear(
		    p1.x, p1.y,
		    p2.x, p2.y);
		cairo_pattern_add_color_stop_rgba(
		    pat, 0,
		    pattern.color1.red,
		    pattern.color1.green,
		    pattern.color1.blue,
		    pattern.color1.alpha);
		cairo_pattern_add_color_stop_rgba(
		    pat, 1,
		    pattern.color2.red,
		    pattern.color2.green,
		    pattern.color2.blue,
		    pattern.color2.alpha);

		cairo_set_source(cr, pat);

		cairo_pattern_destroy(pat);
	    }

	    void operator()(const RadialGradient &pattern) const
	    {
		Point c1 = transformer.transform(pattern.center1);
		Point c2 = transformer.transform(pattern.center2);
		cairo_pattern_t *pat = cairo_pattern_create_radial(
		    c1.x, c1.y, transformer.transform(pattern.radius1),
		    c2.x, c2.y, transformer.transform(pattern.radius2));
		cairo_pattern_add_color_stop_rgba(
		    pat, 0,
		    pattern.color1.red,
		    pattern.color1.green,
		    pattern.color1.blue,
		    pattern.color1.alpha);
		cairo_pattern_add_color_stop_rgba(
		    pat, 1,
		    pattern.color2.red,
		    pattern.color2.green,
		    pattern.color2.blue,
		    pattern.color2.alpha);

		cairo_set_source(cr, pat);

		cairo_pattern_destroy(pat);
	    }
	};

	struct SegmentPutter
	{
	    cairo_t * const cr;
	    const CoordTransformer &transformer;

	    void operator()(const LineSegment &seg) const
	    {
		Point p2 = transformer.transform(seg.point2);

		cairo_line_to(cr, p2.x, p2.y);
	    }

	    void operator()(const QuadraticBezierSegment &seg) const
	    {
		Point qp1;
		cairo_get_current_point(cr, &qp1.x, &qp1.y);
		Point qp2 = transformer.transform(seg.point2);
		Point qp3 = transformer.transform(seg.point3);

		Point cp2 = Point {
		    qp1.x + 2 * (qp2.x - qp1.x) / 3.0,
		    qp1.y + 2 * (qp2.y - qp1.y) / 3.0,
		};
		Point cp3 = Point {
		    qp3.x + 2 * (qp2.x - qp3.x) / 3.0,
		    qp3.y + 2 * (qp2.y - qp3.y) / 3.0,
		};
		Point cp4 = qp3;

		cairo_curve_to(cr, cp2.x, cp2.y, cp3.x, cp3.y, cp4.x, cp4.y);
	    }

	    void operator()(const CubicBezierSegment &seg) const
	    {
		Point p2 = transformer.transform(seg.point2);
		Point p3 = transformer.transform(seg.point3);
		Point p4 = transformer.transform(seg.point4);

		cairo_curve_to(cr, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y);
	    }
	};

	struct ShapeRenderer
	{
	    cairo_t * const cr;
	    const Image &image;
	    const CoordTransformer &transformer;

	    void set_pattern(const Pattern &pattern) const
	    {
		PatternSetter setter { cr, transformer };
		std::visit(setter, pattern);
	    }

	    void set_pen(const Pen &pen) const
	    {
		set_pattern(pen.pattern);
		cairo_set_line_width(cr, transformer.transform(pen.width));

		cairo_set_line_cap(
		    cr,
		    pen.cap == LineCap::butt ? CAIRO_LINE_CAP_BUTT
		    : pen.cap == LineCap::round ? CAIRO_LINE_CAP_ROUND
		    : CAIRO_LINE_CAP_SQUARE);

		cairo_set_line_join(
		    cr,
		    pen.join == LineJoin::miter ? CAIRO_LINE_JOIN_MITER
		    : pen.join == LineJoin::round ? CAIRO_LINE_JOIN_ROUND
		    : CAIRO_LINE_JOIN_BEVEL);
	    }

	    void set_brush(const Brush &brush) const
	    {
		set_pattern(brush.pattern);
	    }

	    void put_path(const CurveData &data, bool closed) const
	    {
		Point start = transformer.transform(data.start);
		cairo_move_to(cr, start.x, start.y);

		SegmentPutter putter { cr, transformer };

		for (const Segment &seg : data.segments)
		{
		    std::visit(putter, seg);
		}

		if (closed)
		{
		    cairo_close_path(cr);
		}
	    }

	    void operator()(const Group &group) const
	    {
		for (const Shape &shape : group.content)
		{
		    render_shape(shape, cr, image, transformer);
		}
	    }

	    void operator()(const Curve &curve) const
	    {
		put_path(curve.data, false);

		set_pen(image.pens.at(curve.pen));
		cairo_stroke(cr);
	    }

	    void operator()(const Region &region) const
	    {
		put_path(region.data.curves.at(0), true);

		for (std::size_t i = 1; i < region.data.curves.size(); ++i)
		{
		    cairo_new_sub_path(cr);
		    put_path(region.data.curves[i], true);
		}

		if (region.brush.has_value())
		{
		    set_brush(image.brushes.at(region.brush.value()));
		    cairo_fill_preserve(cr);
		}

		if (region.pen.has_value())
		{
		    set_pen(image.pens.at(region.pen.value()));
		    cairo_stroke(cr);
		}
		else
		{
		    cairo_new_path(cr);
		}		    
	    }
	};
    }

    static void render_shape(const Shape &shape, cairo_t *cr, const Image &im, const detail::CoordTransformer &trans)
    {
	detail::ShapeRenderer renderer { cr, im, trans };
	std::visit(renderer, shape);
    }

    void render(const Image &im, cairo_t *cr, double ppi, double scale)
    {
	detail::CoordTransformer trans { im, ppi, scale };

	cairo_set_operator(cr, CAIRO_OPERATOR_OVER);
	cairo_set_fill_rule(cr, CAIRO_FILL_RULE_EVEN_ODD);
	cairo_new_path(cr);

	for (const Shape &shape : im.shapes)
	{
	    render_shape(shape, cr, im, trans);
	}
    }
}
