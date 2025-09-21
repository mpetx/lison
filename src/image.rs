
use std::fmt;
use serde::{Deserialize, Serialize};
use serde::de::{Deserializer, SeqAccess, Visitor};
use serde::ser::{Serializer, SerializeSeq};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Image {
    pub width: f64,
    pub height: f64,
    pub unit_per_inch: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<String>,
    pub pens: Vec<Pen>,
    pub brushes: Vec<Brush>,
    pub shapes: Vec<Shape>
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

struct PointVisitor;

impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("point")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Point, A::Error>
    where
        A: SeqAccess<'de>
    {
        let x = seq.next_element::<f64>()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let y = seq.next_element::<f64>()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

        match seq.next_element::<f64>()? {
            None => Ok(Point { x, y }),
            Some(_) => Err(serde::de::Error::invalid_length(2, &self))
        }
    }
}

impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(PointVisitor)
    }
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.end()
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("color")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Color, A::Error>
    where
        A: SeqAccess<'de>
    {
        let red = seq.next_element::<f64>()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let green = seq.next_element::<f64>()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        let blue = seq.next_element::<f64>()?
            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
        let alpha = seq.next_element::<f64>()?;

        match alpha {
            None => Ok(Color { red, green, blue, alpha: 1.0 }),
            Some(alpha) => match seq.next_element::<f64>()? {
                None => Ok(Color { red, green, blue, alpha }),
                Some(_) => Err(serde::de::Error::invalid_length(4, &self))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(ColorVisitor)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let ser_alpha = self.alpha >= 0.0 && self.alpha < 1.0;

        let mut seq = serializer.serialize_seq(Some(if ser_alpha { 4 } else { 3 }))?;
        seq.serialize_element(&self.red)?;
        seq.serialize_element(&self.green)?;
        seq.serialize_element(&self.blue)?;
        if ser_alpha { seq.serialize_element(&self.alpha)?; }
        seq.end()
    }
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct MonochromePattern {
    pub color: Color
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LinearGradientPattern {
    pub point_1: Point,
    pub color_1: Color,
    pub point_2: Point,
    pub color_2: Color
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RadialGradientPattern {
    pub center_1: Point,
    pub radius_1: f64,
    pub color_1: Color,
    pub center_2: Point,
    pub radius_2: f64,
    pub color_2: Color
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Pattern {
    Monochrome(MonochromePattern),
    LinearGradient(LinearGradientPattern),
    RadialGradient(RadialGradientPattern)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square
}

struct LineCapVisitor;

impl<'de> Visitor<'de> for LineCapVisitor {
    type Value = LineCap;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("line cap")
    }

    fn visit_str<E>(self, v: &str) -> Result<LineCap, E>
    where
        E: serde::de::Error
    {
        match v {
            "butt" => Ok(LineCap::Butt),
            "round" => Ok(LineCap::Round),
            "square" => Ok(LineCap::Square),
            other => Err(serde::de::Error::unknown_variant(other, &["butt", "round", "square"]))
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<LineCap, E>
    where
        E: serde::de::Error
    {
        match v {
            "butt" => Ok(LineCap::Butt),
            "round" => Ok(LineCap::Round),
            "square" => Ok(LineCap::Square),
            other => Err(serde::de::Error::unknown_variant(other, &["butt", "round", "square"]))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<LineCap, E>
    where
        E: serde::de::Error
    {
        match v.as_str() {
            "butt" => Ok(LineCap::Butt),
            "round" => Ok(LineCap::Round),
            "square" => Ok(LineCap::Square),
            other => Err(serde::de::Error::unknown_variant(other, &["butt", "round", "square"]))
        }
    }
}

impl<'de> Deserialize<'de> for LineCap {
    fn deserialize<D>(deserializer: D) -> Result<LineCap, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LineCapVisitor)
    }
}

impl Serialize for LineCap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            LineCap::Butt => serializer.serialize_str("butt"),
            LineCap::Round => serializer.serialize_str("round"),
            LineCap::Square => serializer.serialize_str("square"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel
}

struct LineJoinVisitor;

impl<'de> Visitor<'de> for LineJoinVisitor {
    type Value = LineJoin;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("line join")
    }

    fn visit_str<E>(self, v: &str) -> Result<LineJoin, E>
    where
        E: serde::de::Error
    {
        match v {
            "miter" => Ok(LineJoin::Miter),
            "round" => Ok(LineJoin::Round),
            "bevel" => Ok(LineJoin::Bevel),
            other => Err(serde::de::Error::unknown_variant(other, &["miter", "round", "bevel"]))
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<LineJoin, E>
    where
        E: serde::de::Error
    {
        match v {
            "miter" => Ok(LineJoin::Miter),
            "round" => Ok(LineJoin::Round),
            "bevel" => Ok(LineJoin::Bevel),
            other => Err(serde::de::Error::unknown_variant(other, &["miter", "round", "bevel"]))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<LineJoin, E>
    where
        E: serde::de::Error
    {
        match v.as_str() {
            "miter" => Ok(LineJoin::Miter),
            "round" => Ok(LineJoin::Round),
            "bevel" => Ok(LineJoin::Bevel),
            other => Err(serde::de::Error::unknown_variant(other, &["miter", "round", "bevel"]))
        }
    }
}

impl<'de> Deserialize<'de> for LineJoin {
    fn deserialize<D>(deserializer: D) -> Result<LineJoin, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LineJoinVisitor)
    }
}

impl Serialize for LineJoin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            LineJoin::Miter => serializer.serialize_str("miter"),
            LineJoin::Round => serializer.serialize_str("round"),
            LineJoin::Bevel => serializer.serialize_str("bevel"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Pen {
    pub pattern: Pattern,
    pub width: f64,
    pub cap: LineCap,
    pub join: LineJoin
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Brush {
    pub pattern: Pattern
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GroupShape {
    pub content: Vec<Shape>,
    #[serde(skip_serializing_if = "serde_json::Value::is_null", default)]
    pub edit_annot: serde_json::Value
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct CurveShape {
    pub pen: usize,
    pub data: CurveData
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RegionShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pen: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brush: Option<usize>,
    pub data: Vec<CurveData>
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Shape {
    Group(GroupShape),
    Curve(CurveShape),
    Region(RegionShape)
}

#[derive(Clone, Copy)]
pub struct LineSegment {
    pub point_2: Point
}

#[derive(Clone, Copy)]
pub struct QuadraticBezierSegment {
    pub point_2: Point,
    pub point_3: Point
}

#[derive(Clone, Copy)]
pub struct CubicBezierSegment {
    pub point_2: Point,
    pub point_3: Point,
    pub point_4: Point
}

#[derive(Clone, Copy)]
pub enum Segment {
    Line(LineSegment),
    QuadraticBezier(QuadraticBezierSegment),
    CubicBezier(CubicBezierSegment)
}

struct SegmentVisitor;

impl<'de> Visitor<'de> for SegmentVisitor {
    type Value = Segment;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("segment")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Segment, A::Error>
    where
        A: SeqAccess<'de>
    {
        let tag = seq.next_element::<String>()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

        match tag.as_str() {
            "L" => {
                let point_2 = seq.next_element::<Point>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                match seq.next_element::<Point>()? {
                    None => Ok(Segment::Line(LineSegment { point_2 })),
                    Some(_) => Err(serde::de::Error::invalid_length(2, &self))
                }
            },
            "Q" => {
                let point_2 = seq.next_element::<Point>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let point_3 = seq.next_element::<Point>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;

                match seq.next_element::<Point>()? {
                    None => Ok(Segment::QuadraticBezier(QuadraticBezierSegment { point_2, point_3 })),
                    Some(_) => Err(serde::de::Error::invalid_length(3, &self))
                }
            },
            "C" => {
                let point_2 = seq.next_element::<Point>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let point_3 = seq.next_element::<Point>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let point_4 = seq.next_element::<Point>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;

                match seq.next_element::<Point>()? {
                    None => Ok(Segment::CubicBezier(CubicBezierSegment { point_2, point_3, point_4 })),
                    Some(_) => Err(serde::de::Error::invalid_length(4, &self))
                }
            },
            other => Err(serde::de::Error::unknown_variant(other, &["L", "Q", "C"]))
        }
    }
}

impl<'de> Deserialize<'de> for Segment {
    fn deserialize<D>(deserializer: D) -> Result<Segment, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(SegmentVisitor)
    }
}

impl Serialize for Segment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut seq = serializer.serialize_seq(None)?;
        
        match self {
            Segment::Line(s) => {
                seq.serialize_element("L")?;
                seq.serialize_element(&s.point_2)?;
            },
            Segment::QuadraticBezier(s) => {
                seq.serialize_element("Q")?;
                seq.serialize_element(&s.point_2)?;
                seq.serialize_element(&s.point_3)?;
            },
            Segment::CubicBezier(s) => {
                seq.serialize_element("C")?;
                seq.serialize_element(&s.point_2)?;
                seq.serialize_element(&s.point_3)?;
                seq.serialize_element(&s.point_4)?;
            }
        }

        seq.end()
    }
}

#[derive(Clone)]
pub struct CurveData {
    pub start: Point,
    pub segments: Vec<Segment>
}

struct CurveDataVisitor;

impl<'de> Visitor<'de> for CurveDataVisitor {
    type Value = CurveData;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("curve data")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<CurveData, A::Error>
    where
        A: SeqAccess<'de>
    {
        let start = seq.next_element::<Point>()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

        let mut segments = vec![];

        while let Some(seg) = seq.next_element::<Segment>()? {
            segments.push(seg);
        }

        Ok(CurveData { start, segments })
    }
}

impl<'de> Deserialize<'de> for CurveData {
    fn deserialize<D>(deserializer: D) -> Result<CurveData, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(CurveDataVisitor)
    }
}

impl Serialize for CurveData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut seq = serializer.serialize_seq(None)?;
        seq.serialize_element(&self.start)?;

        for seg in self.segments.iter() {
            seq.serialize_element(&seg)?;
        }

        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait Relative {
        fn relative_error_from(&self, other: &Self) -> f64;
    }

    impl Relative for f64 {
        fn relative_error_from(&self, other: &f64) -> f64 {
            (self - other) / other
        }
    }

    impl Relative for Point {
        fn relative_error_from(&self, other: &Point) -> f64 {
            self.x.relative_error_from(&other.x)
                .max(self.y.relative_error_from(&other.y))
        }
    }

    impl Relative for Color {
        fn relative_error_from(&self, other: &Color) -> f64 {
            self.red.relative_error_from(&other.red)
                .max(self.green.relative_error_from(&other.green))
                .max(self.blue.relative_error_from(&other.blue))
                .max(self.alpha.relative_error_from(&other.alpha))
        }
    }

    impl Relative for Pattern {
        fn relative_error_from(&self, other: &Pattern) -> f64 {
            match self {
                Pattern::Monochrome(mono1) =>
                    match other {
                        Pattern::Monochrome(mono2) =>
                            mono1.color.relative_error_from(&mono2.color),
                        _ => f64::INFINITY
                    },
                Pattern::LinearGradient(grad1) =>
                    match other {
                        Pattern::LinearGradient(grad2) =>
                            grad1.point_1.relative_error_from(&grad2.point_1)
                            .max(grad1.color_1.relative_error_from(&grad2.color_1))
                            .max(grad1.point_2.relative_error_from(&grad2.point_2))
                            .max(grad1.color_2.relative_error_from(&grad2.color_2)) ,
                        _ => f64::INFINITY
                    },
                Pattern::RadialGradient(grad1) =>
                    match other {
                        Pattern::RadialGradient(grad2) =>
                            grad1.center_1.relative_error_from(&grad2.center_1)
                            .max(grad1.radius_1.relative_error_from(&grad2.radius_1))
                            .max(grad1.color_1.relative_error_from(&grad2.color_1))
                            .max(grad1.center_2.relative_error_from(&grad2.center_2))
                            .max(grad1.radius_2.relative_error_from(&grad2.radius_2))
                            .max(grad1.color_2.relative_error_from(&grad2.color_2)),
                        _ => f64::INFINITY
                    }
            }
        }
    }

    impl Relative for Segment {
        fn relative_error_from(&self, other: &Segment) -> f64 {
            match self {
                Segment::Line(line1) =>
                    match other {
                        Segment::Line(line2) =>
                            line1.point_2.relative_error_from(&line2.point_2),
                        _ => f64::INFINITY
                    },
                Segment::QuadraticBezier(bezier1) =>
                    match other {
                        Segment::QuadraticBezier(bezier2) =>
                            bezier1.point_2.relative_error_from(&bezier2.point_2)
                            .max(bezier1.point_3.relative_error_from(&bezier2.point_3)),
                        _ => f64::INFINITY
                    },
                Segment::CubicBezier(bezier1) =>
                    match other {
                        Segment::CubicBezier(bezier2) =>
                            bezier1.point_2.relative_error_from(&bezier2.point_2)
                            .max(bezier1.point_3.relative_error_from(&bezier2.point_3))
                            .max(bezier1.point_4.relative_error_from(&bezier2.point_4)),
                        _ => f64::INFINITY
                    }
            }
        }
    }

    macro_rules! assert_near {
        ($expect_expr:expr, $actual_expr:expr) => {
            assert_near!($expect_expr, $actual_expr, 0.0001);
        };
        ($expect_expr:expr, $actual_expr:expr, $max_error:expr) => {
            let actual = $actual_expr;
            let expect = $expect_expr;
            let error = actual.relative_error_from(&expect).abs();
            assert!(error <= $max_error);
        };
    }

    #[test]
    fn test_image_de() {
        let image_str = r#"{
  "width": 640,
  "height": 480,
  "unit-per-inch": 140,
  "pens": [],
  "brushes": [],
  "shapes": []
}"#;
        let image: Image = serde_json::from_str(image_str).unwrap();
        assert_near!(640.0, image.width);
        assert_near!(480.0, image.height);
        assert_near!(140.0, image.unit_per_inch);
        assert_eq!(None, image.editor);

        let image2_str = r#"{
  "width": 1920,
  "height": 1080,
  "unit-per-inch": 220,
  "editor": "T2SY95",
  "pens": [],
  "brushes": [],
  "shapes": []
}"#;
        let image2: Image = serde_json::from_str(image2_str).unwrap();
        assert_near!(1920.0, image2.width);
        assert_near!(1080.0, image2.height);
        assert_near!(220.0, image2.unit_per_inch);
        assert_eq!(Some(String::from("T2SY95")), image2.editor);
    }

    #[test]
    fn test_image_ser() {
        let image = Image {
            width: 200.0,
            height: 100.0,
            unit_per_inch: 72.0,
            editor: Some(String::from("A7E6W9UF")),
            pens: vec![],
            brushes: vec![],
            shapes: vec![]
        };
        let image_str = serde_json::to_string(&image).unwrap();
        assert_eq!(r#"{"width":200.0,"height":100.0,"unit-per-inch":72.0,"editor":"A7E6W9UF","pens":[],"brushes":[],"shapes":[]}"#, &image_str);

        let image2 = Image {
            width: 100.0,
            height: 200.0,
            unit_per_inch: 96.0,
            editor: None,
            pens: vec![],
            brushes: vec![],
            shapes: vec![]
        };
        let image2_str = serde_json::to_string(&image2).unwrap();
        assert_eq!(r#"{"width":100.0,"height":200.0,"unit-per-inch":96.0,"pens":[],"brushes":[],"shapes":[]}"#, &image2_str);
    }

    #[test]
    fn test_point_de() {
        let p_str = r#"[2.4, 5.6]"#;
        let p: Point = serde_json::from_str(p_str).unwrap();
        assert_near!(Point { x: 2.4, y: 5.6 }, p);

        let bad_p1_str = r#"[1]"#;
        let bad_p1 = serde_json::from_str::<Point>(bad_p1_str);
        assert!(bad_p1.is_err());

        let bad_p2_str = r#"[1, 2, 3]"#;
        let bad_p2 = serde_json::from_str::<Point>(bad_p2_str);
        assert!(bad_p2.is_err());
    }

    #[test]
    fn test_point_ser() {
        let p = Point { x: 10.0, y: -8.5 };
        let p_str = serde_json::to_string(&p).unwrap();
        assert_eq!(r#"[10.0,-8.5]"#, &p_str);
    }

    #[test]
    fn test_color_de() {
        let c1_str = r#"[0.5, 1.0, 0.0]"#;
        let c1: Color = serde_json::from_str(c1_str).unwrap();
        assert_near!(Color { red: 0.5, green: 1.0, blue: 0.0, alpha: 1.0 }, c1);

        let c2_str = r#"[0.541, 0.169, 0.886, 0.7]"#;
        let c2: Color = serde_json::from_str(c2_str).unwrap();
        assert_near!(Color { red: 0.541, green: 0.169, blue: 0.886, alpha: 0.7 }, c2);

        let bad_c1_str = r#"[0.1, 0.2]"#;
        let bad_c1 = serde_json::from_str::<Color>(bad_c1_str);
        assert!(bad_c1.is_err());

        let bad_c2_str = r#"[0.1, 0.2, 0.3, 0.4, 0.5]"#;
        let bad_c2 = serde_json::from_str::<Color>(bad_c2_str);
        assert!(bad_c2.is_err());
    }

    #[test]
    fn test_color_ser() {
        let c1 = Color { red: 1.0, green: 0.5, blue: 0.25, alpha: 1.0 };
        let c1_str = serde_json::to_string(&c1).unwrap();
        assert_eq!(r#"[1.0,0.5,0.25]"#, &c1_str);

        let c2 = Color { red: 0.25, green: 0.125, blue: 1.0, alpha: 0.5 };
        let c2_str = serde_json::to_string(&c2).unwrap();
        assert_eq!(r#"[0.25,0.125,1.0,0.5]"#, &c2_str);
    }

    #[test]
    fn test_pattern_de() {
        let p1_str = r#"{
  "type": "monochrome",
  "color": [1, 1, 0]
}"#;
        let p1: Pattern = serde_json::from_str(p1_str).unwrap();
        assert_near!(Pattern::Monochrome(MonochromePattern {
            color: Color { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 }
        }), p1);

        let p2_str = r#"{
  "type": "linear-gradient",
  "point-1": [0, 0],
  "color-1": [0, 1, 1],
  "point-2": [100, 100],
  "color-2": [1, 1, 1]
}"#;
        let p2: Pattern = serde_json::from_str(p2_str).unwrap();
        assert_near!(Pattern::LinearGradient(LinearGradientPattern {
            point_1: Point { x: 0.0, y: 0.0 },
            color_1: Color { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 },
            point_2: Point { x: 100.0, y: 100.0 },
            color_2: Color { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
        }), p2);

        let p3_str = r#"{
  "type": "radial-gradient",
  "center-1": [50, 50],
  "radius-1": 5,
  "color-1": [1, 0, 1],
  "center-2": [50, 50],
  "radius-2": 70.7,
  "color-2": [1, 0, 1, 0.1]
}"#;
        let p3: Pattern = serde_json::from_str(p3_str).unwrap();
        assert_near!(Pattern::RadialGradient(RadialGradientPattern {
            center_1: Point { x: 50.0, y: 50.0 },
            radius_1: 5.0,
            color_1: Color { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 },
            center_2: Point { x: 50.0, y: 50.0 },
            radius_2: 70.7,
            color_2: Color { red: 1.0, green: 0.0, blue: 1.0, alpha: 0.1 },
        }), p3);
    }

    #[test]
    fn test_pattern_ser() {
        let p1 = Pattern::Monochrome(MonochromePattern {
            color: Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }
        });
        let p1_str = serde_json::to_string(&p1).unwrap();
        assert_eq!(r#"{"type":"monochrome","color":[1.0,0.0,0.0]}"#, &p1_str);

        let p2 = Pattern::LinearGradient(LinearGradientPattern {
            point_1: Point { x: 0.0, y: 0.0 },
            color_1: Color { red: 0.5, green: 0.5, blue: 1.0, alpha: 1.0 },
            point_2: Point { x: 100.0, y: 0.0 },
            color_2: Color { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }
        });
        let p2_str = serde_json::to_string(&p2).unwrap();
        assert_eq!(r#"{"type":"linear-gradient","point-1":[0.0,0.0],"color-1":[0.5,0.5,1.0],"point-2":[100.0,0.0],"color-2":[0.0,0.0,1.0]}"#, &p2_str);

        let p3 = Pattern::RadialGradient(RadialGradientPattern {
            center_1: Point { x: 50.0, y: 50.0 },
            radius_1: 5.0,
            color_1: Color { red: 0.0, green: 0.5, blue: 0.0, alpha: 1.0 },
            center_2: Point { x: 50.0, y: 50.0 },
            radius_2: 50.0,
            color_2: Color { red: 0.0, green: 0.5, blue: 0.0, alpha: 0.25 },
            
        });
        let p3_str = serde_json::to_string(&p3).unwrap();
        assert_eq!(r#"{"type":"radial-gradient","center-1":[50.0,50.0],"radius-1":5.0,"color-1":[0.0,0.5,0.0],"center-2":[50.0,50.0],"radius-2":50.0,"color-2":[0.0,0.5,0.0,0.25]}"#, &p3_str);
    }

    #[test]
    fn test_line_cap_de() {
        let cap1_str = r#""butt""#;
        let cap1: LineCap = serde_json::from_str(&cap1_str).unwrap();
        assert!(LineCap::Butt == cap1);

        let cap2_str = r#""round""#;
        let cap2: LineCap = serde_json::from_str(&cap2_str).unwrap();
        assert!(LineCap::Round == cap2);

        let cap3_str = r#""square""#;
        let cap3: LineCap = serde_json::from_str(&cap3_str).unwrap();
        assert!(LineCap::Square == cap3);

        let cap4_str = r#""bad-cap""#;
        let cap4 = serde_json::from_str::<LineCap>(&cap4_str);
        assert!(cap4.is_err());
    }

    #[test]
    fn test_line_cap_ser() {
        let cap1 = LineCap::Butt;
        let cap1_str = serde_json::to_string(&cap1).unwrap();
        assert_eq!(r#""butt""#, &cap1_str);

        let cap2 = LineCap::Round;
        let cap2_str = serde_json::to_string(&cap2).unwrap();
        assert_eq!(r#""round""#, &cap2_str);

        let cap3 = LineCap::Square;
        let cap3_str = serde_json::to_string(&cap3).unwrap();
        assert_eq!(r#""square""#, &cap3_str);
    }

    #[test]
    fn test_line_join_de() {
        let join1_str = r#""miter""#;
        let join1: LineJoin = serde_json::from_str(&join1_str).unwrap();
        assert!(LineJoin::Miter == join1);

        let join2_str = r#""round""#;
        let join2: LineJoin = serde_json::from_str(&join2_str).unwrap();
        assert!(LineJoin::Round == join2);

        let join3_str = r#""bevel""#;
        let join3: LineJoin = serde_json::from_str(&join3_str).unwrap();
        assert!(LineJoin::Bevel == join3);

        let join4_str = r#""bad-join""#;
        let join4 = serde_json::from_str::<LineJoin>(&join4_str);
        assert!(join4.is_err());
    }

    #[test]
    fn test_line_join_ser() {
        let join1 = LineJoin::Miter;
        let join1_str = serde_json::to_string(&join1).unwrap();
        assert_eq!(r#""miter""#, &join1_str);

        let join2 = LineJoin::Round;
        let join2_str = serde_json::to_string(&join2).unwrap();
        assert_eq!(r#""round""#, &join2_str);

        let join3 = LineJoin::Bevel;
        let join3_str = serde_json::to_string(&join3).unwrap();
        assert_eq!(r#""bevel""#, &join3_str);
    }

    #[test]
    fn test_pen_de() {
        let pen_str = r#"{
  "pattern": {
    "type": "monochrome",
    "color": [0.3, 0.4, 0.5, 0.6]
  },
  "width": 5,
  "cap": "butt",
  "join": "bevel"
}"#;
        let pen: Pen = serde_json::from_str(pen_str).unwrap();
        assert_near!(Pattern::Monochrome(MonochromePattern {
            color: Color { red: 0.3, green: 0.4, blue: 0.5, alpha: 0.6 }
        }), pen.pattern);
        assert_near!(5.0, pen.width);
        assert!(LineCap::Butt == pen.cap);
        assert!(LineJoin::Bevel == pen.join);
    }

    #[test]
    fn test_pen_ser() {
        let pen = Pen {
            pattern: Pattern::Monochrome(MonochromePattern {
                color: Color { red: 0.9, green: 0.8, blue: 0.7, alpha: 0.6 }
            }),
            width: 2.5,
            cap: LineCap::Round,
            join: LineJoin::Round
        };
        let pen_str = serde_json::to_string(&pen).unwrap();
        assert_eq!(r#"{"pattern":{"type":"monochrome","color":[0.9,0.8,0.7,0.6]},"width":2.5,"cap":"round","join":"round"}"#, &pen_str);
    }

    #[test]
    fn test_brush_de() {
        let brush_str = r#"{
  "pattern": {
    "type": "monochrome",
    "color": [0.5, 0.6, 0.7]
  }
}"#;
        let brush: Brush = serde_json::from_str(brush_str).unwrap();
        assert_near!(Pattern::Monochrome(MonochromePattern {
            color: Color { red: 0.5, green: 0.6, blue: 0.7, alpha: 1.0 }
        }), brush.pattern);
    }

    #[test]
    fn test_brush_ser() {
        let brush = Brush {
            pattern: Pattern::Monochrome(MonochromePattern {
                color: Color { red: 0.5, green: 1.0, blue: 0.25, alpha: 1.0 }
            })
        };
        let brush_str = serde_json::to_string(&brush).unwrap();
        assert_eq!(r#"{"pattern":{"type":"monochrome","color":[0.5,1.0,0.25]}}"#, &brush_str);
    }

    #[test]
    fn test_segment_de() {
        let seg1_str = r#"["L", [10, 11]]"#;
        let seg1: Segment = serde_json::from_str(seg1_str).unwrap();
        assert_near!(Segment::Line(LineSegment {
            point_2: Point { x: 10.0, y: 11.0 }
        }), seg1);

        let seg2_str = r#"["Q", [12, 13], [14, 15]]"#;
        let seg2: Segment = serde_json::from_str(seg2_str).unwrap();
        assert_near!(Segment::QuadraticBezier(QuadraticBezierSegment {
            point_2: Point { x: 12.0, y: 13.0 },
            point_3: Point { x: 14.0, y: 15.0 },
        }), seg2);

        let seg3_str = r#"["C", [16, 17], [18, 19], [20, 21]]"#;
        let seg3: Segment = serde_json::from_str(seg3_str).unwrap();
        assert_near!(Segment::CubicBezier(CubicBezierSegment {
            point_2: Point { x: 16.0, y: 17.0 },
            point_3: Point { x: 18.0, y: 19.0 },
            point_4: Point { x: 20.0, y: 21.0 },
        }), seg3);
    }

    #[test]
    fn test_segment_ser() {
        let seg1 = Segment::Line(LineSegment {
            point_2: Point { x: 1.0, y: 2.0 }
        });
        let seg1_str = serde_json::to_string(&seg1).unwrap();
        assert_eq!(r#"["L",[1.0,2.0]]"#, &seg1_str);

        let seg2 = Segment::QuadraticBezier(QuadraticBezierSegment {
            point_2: Point { x: 1.0, y: 2.0 },
            point_3: Point { x: 3.0, y: -4.0 }
        });
        let seg2_str = serde_json::to_string(&seg2).unwrap();
        assert_eq!(r#"["Q",[1.0,2.0],[3.0,-4.0]]"#, &seg2_str);

        let seg3 = Segment::CubicBezier(CubicBezierSegment {
            point_2: Point { x: 1.0, y: 2.0 },
            point_3: Point { x: 3.0, y: 4.0 },
            point_4: Point { x: 5.0, y: 6.0 }
        });
        let seg3_str = serde_json::to_string(&seg3).unwrap();
        assert_eq!(r#"["C",[1.0,2.0],[3.0,4.0],[5.0,6.0]]"#, &seg3_str);
    }

    #[test]
    fn test_curve_data_de() {
        let dat_str = r#"[
  [10, 11],
  ["L", [12, 13]],
  ["Q", [14, 15], [16, 17]]
]"#;
        let dat: CurveData = serde_json::from_str(dat_str).unwrap();
        assert_near!(10.0, dat.start.x);
        assert_near!(11.0, dat.start.y);
        assert_eq!(2, dat.segments.len());
        assert_near!(Segment::Line(LineSegment {
            point_2: Point { x: 12.0, y: 13.0 }
        }), dat.segments[0]);
        assert_near!(Segment::QuadraticBezier(QuadraticBezierSegment {
            point_2: Point { x: 14.0, y: 15.0 },
            point_3: Point { x: 16.0, y: 17.0 }
        }), dat.segments[1]);
    }

    #[test]
    fn test_curve_data_ser() {
        let dat = CurveData {
            start: Point { x: 1.0, y: 2.0 },
            segments: vec![
                Segment::Line(LineSegment {
                    point_2: Point { x: 3.0, y: 4.0 }
                }),
                Segment::QuadraticBezier(QuadraticBezierSegment {
                    point_2: Point { x: 5.0, y: 6.0 },
                    point_3: Point { x: 7.0, y: 8.0 }
                })
            ]
        };
        let dat_str = serde_json::to_string(&dat).unwrap();
        assert_eq!(r#"[[1.0,2.0],["L",[3.0,4.0]],["Q",[5.0,6.0],[7.0,8.0]]]"#, &dat_str);
    }

    #[test]
    fn test_shape_de() {
        let sh1_str = r#"{
  "type": "group",
  "content": [{
    "type": "group",
    "content": [],
    "edit-annot": false
  }]
}"#;
        let sh: Shape = serde_json::from_str(sh1_str).unwrap();
        if let Shape::Group(s) = sh {
            assert!(s.edit_annot.is_null());
            assert_eq!(1, s.content.len());

            if let Shape::Group(s) = &s.content[0] {
                assert_eq!(false, s.edit_annot);
                assert_eq!(0, s.content.len())
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        let sh2_str = r#"{
  "type": "curve",
  "pen": 3,
  "data": [
    [10, 11],
    ["L", [12, 13]],
    ["Q", [14, 15], [16, 17]]
  ]
}"#;
        let sh2: Shape = serde_json::from_str(sh2_str).unwrap();
        if let Shape::Curve(s) = sh2 {
            assert_eq!(3, s.pen);
            assert_near!(10.0, s.data.start.x);
            assert_near!(11.0, s.data.start.y);
            assert_eq!(2, s.data.segments.len());
            assert_near!(Segment::Line(LineSegment {
                point_2: Point { x: 12.0, y: 13.0 }
            }), s.data.segments[0]);
            assert_near!(Segment::QuadraticBezier(QuadraticBezierSegment {
                point_2: Point { x: 14.0, y: 15.0 },
                point_3: Point { x: 16.0, y: 17.0 }
            }), s.data.segments[1]);
        } else {
            assert!(false);
        }

        let sh3_str = r#"{
  "type": "region",
  "pen": 0,
  "data": [[[7, 8]]]
}"#;
        let sh3: Shape = serde_json::from_str(sh3_str).unwrap();
        if let Shape::Region(s) = sh3 {
            assert_eq!(Some(0), s.pen);
            assert_eq!(None, s.brush);
            assert_eq!(1, s.data.len());
            assert_near!(7.0, s.data[0].start.x);
            assert_near!(8.0, s.data[0].start.y);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_shape_ser() {
        let sh1 = Shape::Group(GroupShape {
            content: vec![],
            edit_annot: serde_json::Value::Null
        });
        let sh1_str = serde_json::to_string(&sh1).unwrap();
        assert_eq!(r#"{"type":"group","content":[]}"#, &sh1_str);

        let sh2 = Shape::Group(GroupShape {
            content: vec![
                Shape::Group(GroupShape {
                    content: vec![],
                    edit_annot: serde_json::Value::Null
                })
            ],
            edit_annot: serde_json::Value::Bool(true)
        });
        let sh2_str = serde_json::to_string(&sh2).unwrap();
        assert_eq!(r#"{"type":"group","content":[{"type":"group","content":[]}],"edit-annot":true}"#, &sh2_str);

        let sh3 = Shape::Curve(CurveShape {
            pen: 1,
            data: CurveData {
                start: Point { x: 1.0, y: 2.0 },
                segments: vec![
                    Segment::Line(LineSegment {
                        point_2: Point { x: 3.0, y: 4.0 }
                    })
                ]
            }
        });
        let sh3_str = serde_json::to_string(&sh3).unwrap();
        assert_eq!(r#"{"type":"curve","pen":1,"data":[[1.0,2.0],["L",[3.0,4.0]]]}"#, &sh3_str);

        let sh4 = Shape::Region(RegionShape {
            pen: Some(0),
            brush: None,
            data: vec![
                CurveData {
                    start: Point { x: 5.0, y: 6.0 },
                    segments: vec![
                        Segment::Line(LineSegment {
                            point_2: Point { x: 7.0, y: 8.0 }
                        })
                    ]
                }
            ]
        });
        let sh4_str = serde_json::to_string(&sh4).unwrap();
        assert_eq!(r#"{"type":"region","pen":0,"data":[[[5.0,6.0],["L",[7.0,8.0]]]]}"#, &sh4_str);

        let sh5 = Shape::Region(RegionShape {
            pen: None,
            brush: Some(1),
            data: vec![
                CurveData {
                    start: Point { x: 9.0, y: 10.0 },
                    segments: vec![]
                }
            ]
        });
        let sh5_str = serde_json::to_string(&sh5).unwrap();
        assert_eq!(r#"{"type":"region","brush":1,"data":[[[9.0,10.0]]]}"#, &sh5_str);
    }
}
