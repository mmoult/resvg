// This file is autogenerated. Do not edit it!
// See ./codegen for details.

use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum EId {
    A,
    Circle,
    ClipPath,
    Defs,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    G,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Tref,
    Tspan,
    Use
}

static ELEMENTS: Map<EId> = Map {
    key: 3558916427560184125,
    disps: &[
        (1, 13),
        (5, 3),
        (0, 19),
        (6, 24),
        (0, 0),
        (0, 3),
        (0, 1),
        (11, 10),
        (0, 21),
        (0, 0),
        (5, 0),
    ],
    entries: &[
        ("feConvolveMatrix", EId::FeConvolveMatrix),
        ("tspan", EId::Tspan),
        ("style", EId::Style),
        ("feFuncB", EId::FeFuncB),
        ("rect", EId::Rect),
        ("marker", EId::Marker),
        ("feDiffuseLighting", EId::FeDiffuseLighting),
        ("g", EId::G),
        ("symbol", EId::Symbol),
        ("pattern", EId::Pattern),
        ("path", EId::Path),
        ("feFuncR", EId::FeFuncR),
        ("a", EId::A),
        ("textPath", EId::TextPath),
        ("use", EId::Use),
        ("feFuncA", EId::FeFuncA),
        ("tref", EId::Tref),
        ("circle", EId::Circle),
        ("fePointLight", EId::FePointLight),
        ("defs", EId::Defs),
        ("feTile", EId::FeTile),
        ("image", EId::Image),
        ("stop", EId::Stop),
        ("feGaussianBlur", EId::FeGaussianBlur),
        ("feFlood", EId::FeFlood),
        ("polyline", EId::Polyline),
        ("feComponentTransfer", EId::FeComponentTransfer),
        ("linearGradient", EId::LinearGradient),
        ("feFuncG", EId::FeFuncG),
        ("ellipse", EId::Ellipse),
        ("clipPath", EId::ClipPath),
        ("feMerge", EId::FeMerge),
        ("feSpotLight", EId::FeSpotLight),
        ("feBlend", EId::FeBlend),
        ("svg", EId::Svg),
        ("feColorMatrix", EId::FeColorMatrix),
        ("feTurbulence", EId::FeTurbulence),
        ("feSpecularLighting", EId::FeSpecularLighting),
        ("switch", EId::Switch),
        ("feMorphology", EId::FeMorphology),
        ("feImage", EId::FeImage),
        ("feMergeNode", EId::FeMergeNode),
        ("feOffset", EId::FeOffset),
        ("polygon", EId::Polygon),
        ("feComposite", EId::FeComposite),
        ("radialGradient", EId::RadialGradient),
        ("line", EId::Line),
        ("feDisplacementMap", EId::FeDisplacementMap),
        ("feDistantLight", EId::FeDistantLight),
        ("mask", EId::Mask),
        ("text", EId::Text),
        ("filter", EId::Filter),
    ],
};

impl EId {
    pub fn from_str(text: &str) -> Option<EId> {
        ELEMENTS.get(text).cloned()
    }

    pub fn to_str(&self) -> &'static str {
        ELEMENTS.key(self)
    }
}

impl fmt::Debug for EId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for EId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AId {
    Amplitude,
    BaselineShift,
    Class,
    ClipPath,
    ClipRule,
    ClipPathUnits,
    Color,
    ColorInterpolationFilters,
    Cx,
    Cy,
    D,
    Direction,
    Display,
    Dx,
    Dy,
    Exponent,
    Fill,
    FillOpacity,
    FillRule,
    Filter,
    FilterUnits,
    FloodColor,
    FloodOpacity,
    FontFamily,
    FontSize,
    FontStretch,
    FontStyle,
    FontVariant,
    FontWeight,
    Fx,
    Fy,
    GradientTransform,
    GradientUnits,
    Height,
    Href,
    Id,
    ImageRendering,
    In,
    In2,
    Intercept,
    K1,
    K2,
    K3,
    K4,
    LetterSpacing,
    MarkerEnd,
    MarkerMid,
    MarkerStart,
    MarkerHeight,
    MarkerUnits,
    MarkerWidth,
    Mask,
    MaskContentUnits,
    MaskUnits,
    Mode,
    Offset,
    Opacity,
    Operator,
    Orient,
    Overflow,
    PatternContentUnits,
    PatternTransform,
    PatternUnits,
    Points,
    PreserveAspectRatio,
    PrimitiveUnits,
    R,
    RefX,
    RefY,
    RequiredExtensions,
    RequiredFeatures,
    Result,
    Rotate,
    Rx,
    Ry,
    ShapeRendering,
    Slope,
    Space,
    SpreadMethod,
    StartOffset,
    StdDeviation,
    StopColor,
    StopOpacity,
    Stroke,
    StrokeDasharray,
    StrokeDashoffset,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeOpacity,
    StrokeWidth,
    Style,
    SystemLanguage,
    TableValues,
    TextAnchor,
    TextDecoration,
    TextRendering,
    Transform,
    Type,
    ViewBox,
    Visibility,
    Width,
    WordSpacing,
    WritingMode,
    X,
    X1,
    X2,
    Y,
    Y1,
    Y2
}

static ATTRIBUTES: Map<AId> = Map {
    key: 3213172566270843353,
    disps: &[
        (0, 55),
        (0, 75),
        (0, 56),
        (5, 15),
        (6, 11),
        (0, 0),
        (0, 91),
        (0, 55),
        (0, 48),
        (1, 9),
        (0, 38),
        (0, 34),
        (1, 0),
        (10, 79),
        (0, 2),
        (2, 6),
        (3, 69),
        (2, 73),
        (0, 9),
        (0, 1),
        (2, 0),
        (27, 13),
    ],
    entries: &[
        ("exponent", AId::Exponent),
        ("gradientTransform", AId::GradientTransform),
        ("result", AId::Result),
        ("y2", AId::Y2),
        ("shape-rendering", AId::ShapeRendering),
        ("transform", AId::Transform),
        ("text-rendering", AId::TextRendering),
        ("spreadMethod", AId::SpreadMethod),
        ("id", AId::Id),
        ("ry", AId::Ry),
        ("dy", AId::Dy),
        ("gradientUnits", AId::GradientUnits),
        ("font-stretch", AId::FontStretch),
        ("style", AId::Style),
        ("markerUnits", AId::MarkerUnits),
        ("r", AId::R),
        ("stroke-opacity", AId::StrokeOpacity),
        ("stroke-dashoffset", AId::StrokeDashoffset),
        ("points", AId::Points),
        ("maskContentUnits", AId::MaskContentUnits),
        ("in2", AId::In2),
        ("k2", AId::K2),
        ("patternUnits", AId::PatternUnits),
        ("refX", AId::RefX),
        ("type", AId::Type),
        ("filterUnits", AId::FilterUnits),
        ("cx", AId::Cx),
        ("dx", AId::Dx),
        ("letter-spacing", AId::LetterSpacing),
        ("visibility", AId::Visibility),
        ("text-anchor", AId::TextAnchor),
        ("fill", AId::Fill),
        ("stop-color", AId::StopColor),
        ("k3", AId::K3),
        ("color", AId::Color),
        ("d", AId::D),
        ("y", AId::Y),
        ("in", AId::In),
        ("viewBox", AId::ViewBox),
        ("x2", AId::X2),
        ("stroke-dasharray", AId::StrokeDasharray),
        ("amplitude", AId::Amplitude),
        ("stroke-linecap", AId::StrokeLinecap),
        ("space", AId::Space),
        ("rx", AId::Rx),
        ("filter", AId::Filter),
        ("direction", AId::Direction),
        ("refY", AId::RefY),
        ("image-rendering", AId::ImageRendering),
        ("stroke", AId::Stroke),
        ("width", AId::Width),
        ("stdDeviation", AId::StdDeviation),
        ("markerHeight", AId::MarkerHeight),
        ("patternContentUnits", AId::PatternContentUnits),
        ("display", AId::Display),
        ("overflow", AId::Overflow),
        ("clip-path", AId::ClipPath),
        ("font-variant", AId::FontVariant),
        ("patternTransform", AId::PatternTransform),
        ("rotate", AId::Rotate),
        ("fx", AId::Fx),
        ("height", AId::Height),
        ("text-decoration", AId::TextDecoration),
        ("word-spacing", AId::WordSpacing),
        ("mask", AId::Mask),
        ("marker-end", AId::MarkerEnd),
        ("font-size", AId::FontSize),
        ("x1", AId::X1),
        ("href", AId::Href),
        ("cy", AId::Cy),
        ("offset", AId::Offset),
        ("requiredExtensions", AId::RequiredExtensions),
        ("fill-rule", AId::FillRule),
        ("stroke-linejoin", AId::StrokeLinejoin),
        ("fill-opacity", AId::FillOpacity),
        ("primitiveUnits", AId::PrimitiveUnits),
        ("writing-mode", AId::WritingMode),
        ("k1", AId::K1),
        ("stroke-miterlimit", AId::StrokeMiterlimit),
        ("x", AId::X),
        ("y1", AId::Y1),
        ("markerWidth", AId::MarkerWidth),
        ("color-interpolation-filters", AId::ColorInterpolationFilters),
        ("preserveAspectRatio", AId::PreserveAspectRatio),
        ("orient", AId::Orient),
        ("k4", AId::K4),
        ("operator", AId::Operator),
        ("stroke-width", AId::StrokeWidth),
        ("requiredFeatures", AId::RequiredFeatures),
        ("stop-opacity", AId::StopOpacity),
        ("class", AId::Class),
        ("tableValues", AId::TableValues),
        ("font-weight", AId::FontWeight),
        ("clip-rule", AId::ClipRule),
        ("mode", AId::Mode),
        ("flood-color", AId::FloodColor),
        ("marker-start", AId::MarkerStart),
        ("font-style", AId::FontStyle),
        ("font-family", AId::FontFamily),
        ("intercept", AId::Intercept),
        ("clipPathUnits", AId::ClipPathUnits),
        ("opacity", AId::Opacity),
        ("baseline-shift", AId::BaselineShift),
        ("flood-opacity", AId::FloodOpacity),
        ("marker-mid", AId::MarkerMid),
        ("slope", AId::Slope),
        ("startOffset", AId::StartOffset),
        ("fy", AId::Fy),
        ("systemLanguage", AId::SystemLanguage),
        ("maskUnits", AId::MaskUnits),
    ],
};

impl AId {
    pub fn from_str(text: &str) -> Option<AId> {
        ATTRIBUTES.get(text).cloned()
    }

    pub fn to_str(&self) -> &'static str {
        ATTRIBUTES.key(self)
    }
}

impl fmt::Debug for AId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for AId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

// A stripped down `phf` crate fork.
//
// https://github.com/sfackler/rust-phf

struct Map<V: 'static> {
    pub key: u64,
    pub disps: &'static [(u32, u32)],
    pub entries: &'static[(&'static str, V)],
}

impl<V: PartialEq> Map<V> {
    fn get(&self, key: &str) -> Option<&V> {
        use std::borrow::Borrow;

        let hash = hash(key, self.key);
        let index = get_index(hash, &*self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        let b = entry.0.borrow();
        if b == key {
            Some(&entry.1)
        } else {
            None
        }
    }

    fn key(&self, value: &V) -> &'static str {
        self.entries.iter().find(|kv| kv.1 == *value).unwrap().0
    }
}

#[inline]
fn hash(x: &str, key: u64) -> u64 {
    use std::hash::Hasher;

    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, key);
    hasher.write(x.as_bytes());
    hasher.finish()
}

#[inline]
fn get_index(hash: u64, disps: &[(u32, u32)], len: usize) -> u32 {
    let (g, f1, f2) = split(hash);
    let (d1, d2) = disps[(g % (disps.len() as u32)) as usize];
    displace(f1, f2, d1, d2) % (len as u32)
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1;

    ((hash & MASK) as u32,
     ((hash >> BITS) & MASK) as u32,
     ((hash >> (2 * BITS)) & MASK) as u32)
}

#[inline]
fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}
