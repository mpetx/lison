
use crate::image::*;

use cairo::{Context, Result};

struct Scaler {
    factor: f64
}

impl Scaler {
    fn new(image: &Image, ppi: f64, scale: f64) -> Scaler {
        Scaler {
            factor: ppi / image.unit_per_inch * scale
        }
    }

    fn scale(&self, value: f64) -> f64 {
        value * self.factor
    }
}

pub fn render(context: &Context, image: &Image, ppi: f64, scale: f64) -> Result<()> {
    let scaler = Scaler::new(image, ppi, scale);

    context.set_operator(cairo::Operator::Over);
    context.set_fill_rule(cairo::FillRule::EvenOdd);
    context.new_path();

    for shape in image.shapes.iter() {
        render_shape(context, shape, image, &scaler)?;
    }

    Ok(())
}

fn render_shape(context: &Context, shape: &Shape, image: &Image, scaler: &Scaler) -> Result<()> {
    match shape {
        Shape::Group(group) => render_group(context, group, image, scaler),
        Shape::Curve(curve) => render_curve(context, curve, image, scaler),
        Shape::Region(region) => render_region(context, region, image, scaler)
    }
}

fn render_group(context: &Context, group: &GroupShape, image: &Image, scaler: &Scaler) -> Result<()> {
    for child in group.content.iter() {
        render_shape(context, child, image, scaler)?;
    }

    Ok(())
}

fn set_pattern(context: &Context, pattern: &Pattern, scaler: &Scaler) -> Result<()> {
    match pattern {
        Pattern::Monochrome(pat) => {
            context.set_source_rgba(pat.color.red, pat.color.green, pat.color.blue, pat.color.alpha);
        },
        Pattern::LinearGradient(pat) => {
            let grad = cairo::LinearGradient::new(
                scaler.scale(pat.point_1.x),
                scaler.scale(pat.point_1.y),
                scaler.scale(pat.point_2.x),
                scaler.scale(pat.point_2.y)
            );
            grad.add_color_stop_rgba(
                0.0,
                pat.color_1.red,
                pat.color_1.green,
                pat.color_1.blue,
                pat.color_1.alpha
            );
            grad.add_color_stop_rgba(
                1.0,
                pat.color_2.red,
                pat.color_2.green,
                pat.color_2.blue,
                pat.color_2.alpha
            );
            context.set_source(grad)?;
        },
        Pattern::RadialGradient(pat) => {
            let grad = cairo::RadialGradient::new(
                scaler.scale(pat.center_1.x),
                scaler.scale(pat.center_1.y),
                scaler.scale(pat.radius_1),
                scaler.scale(pat.center_2.x),
                scaler.scale(pat.center_2.y),
                scaler.scale(pat.radius_2),
            );
            grad.add_color_stop_rgba(
                0.0,
                pat.color_1.red,
                pat.color_1.green,
                pat.color_1.blue,
                pat.color_1.alpha
            );
            grad.add_color_stop_rgba(
                1.0,
                pat.color_2.red,
                pat.color_2.green,
                pat.color_2.blue,
                pat.color_2.alpha
            );
            context.set_source(grad)?;
        }
    }

    Ok(())
}

fn translate_line_cap(cap: LineCap) -> cairo::LineCap {
    match cap {
        LineCap::Butt => cairo::LineCap::Butt,
        LineCap::Round => cairo::LineCap::Round,
        LineCap::Square => cairo::LineCap::Square
    }
}

fn translate_line_join(join: LineJoin) -> cairo::LineJoin {
    match join {
        LineJoin::Miter => cairo::LineJoin::Miter,
        LineJoin::Round => cairo::LineJoin::Round,
        LineJoin::Bevel => cairo::LineJoin::Bevel
    }
}

fn set_pen(context: &Context, pen: &Pen, scaler: &Scaler) -> Result<()> {
    set_pattern(context, &pen.pattern, scaler)?;
    context.set_line_width(scaler.scale(pen.width));
    context.set_line_cap(translate_line_cap(pen.cap));
    context.set_line_join(translate_line_join(pen.join));

    Ok(())
}

fn set_brush(context: &Context, brush: &Brush, scaler: &Scaler) -> Result<()> {
    set_pattern(context, &brush.pattern, scaler)
}

fn plot_curve_data(context: &Context, data: &CurveData, scaler: &Scaler, closed: bool) -> Result<()> {
    context.move_to(scaler.scale(data.start.x), scaler.scale(data.start.y));

    for seg in data.segments.iter() {
        match seg {
            Segment::Line(line) => {
                context.line_to(scaler.scale(line.point_2.x), scaler.scale(line.point_2.y));
            },
            Segment::QuadraticBezier(bezier) => {
                let (x1, y1) = context.current_point()?;
                let x2 = scaler.scale(bezier.point_2.x);
                let y2 = scaler.scale(bezier.point_2.y);
                let x3 = scaler.scale(bezier.point_3.x);
                let y3 = scaler.scale(bezier.point_3.y);
                context.curve_to(
                    1.0 / 3.0 * x1 + 2.0 / 3.0 * x2,
                    1.0 / 3.0 * y1 + 2.0 / 3.0 * y2,
                    1.0 / 3.0 * x3 + 2.0 / 3.0 * x2,
                    1.0 / 3.0 * y3 + 2.0 / 3.0 * y2,
                    x3,
                    y3
                );
            },
            Segment::CubicBezier(bezier) => {
                context.curve_to(
                    scaler.scale(bezier.point_2.x),
                    scaler.scale(bezier.point_2.y),
                    scaler.scale(bezier.point_3.x),
                    scaler.scale(bezier.point_3.y),
                    scaler.scale(bezier.point_4.x),
                    scaler.scale(bezier.point_4.y)
                );
            }
        }
    }

    if closed {
        context.close_path();
    }

    Ok(())
}

fn render_curve(context: &Context, curve: &CurveShape, image: &Image, scaler: &Scaler) -> Result<()> {
    plot_curve_data(context, &curve.data, scaler, false)?;

    if curve.pen >= image.pens.len() {
        panic!("invalid pen index {}, must be less than {}.", curve.pen, image.pens.len());
    }

    set_pen(context, &image.pens[curve.pen], scaler)?;
    context.stroke()
}

fn render_region(context: &Context, region: &RegionShape, image: &Image, scaler: &Scaler) -> Result<()> {
    if region.data.len() != 0 {
        plot_curve_data(context, &region.data[0], scaler, true)?;
    }

    for i in 1..region.data.len() {
        context.new_sub_path();
        plot_curve_data(context, &region.data[i], scaler, true)?;
    }

    if let Some(brush) = region.brush {
        if brush >= image.brushes.len() {
            panic!("invalid brush index {}, must be less than {}.", brush, image.brushes.len());
        }

        set_brush(context, &image.brushes[brush], scaler)?;
        context.fill_preserve()?;
    }

    if let Some(pen) = region.pen {
        if pen >= image.pens.len() {
            panic!("invalid pen index {}, must be less than {}.", pen, image.pens.len());
        }

        set_pen(context, &image.pens[pen], scaler)?;
        context.stroke()?;
    } else {
        context.new_path();
    }

    Ok(())
}
