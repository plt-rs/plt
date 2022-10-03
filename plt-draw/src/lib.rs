use std::{io, path};

/// The error type for this library.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum DrawError {
    #[error(transparent)]
    BackendError(#[from] anyhow::Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
    // TODO
    #[error("{0}")]
    UnsupportedFileFormat(String),
    #[error("{0}")]
    UnsupportedImageFormat(String),
    #[error("{0}")]
    UnsupportedShape(String),
}

/// 2D size in dot (pixel) numbers.
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Arbitrary point.
#[derive(Copy, Clone, Debug)]
pub struct Point {
    /// The x-position of the point.
    pub x: f64,
    /// The y-position of the point.
    pub y: f64,
}

/// A line from point-1 to point-2.
#[derive(Copy, Clone, Debug)]
pub struct Line {
    /// The first point, drawn from.
    pub p1: Point,
    /// The second point, drawn to.
    pub p2: Point,
}

/// Subarea of a 2D figure by dot (pixel) indices.
#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub xmin: u32,
    pub xmax: u32,
    pub ymin: u32,
    pub ymax: u32,
}
impl Area {
    /// Get the width of the area.
    pub fn xsize(&self) -> u32 {
        self.xmax - self.xmin
    }
    /// Get the height of the area.
    pub fn ysize(&self) -> u32 {
        self.ymax - self.ymin
    }
    /// Convert a fractional point, to a dot (pixel) point.
    pub fn fractional_to_point(&self, frac: Point) -> Point {
        Point {
            x: self.xmin as f64 + (frac.x * self.xsize() as f64),
            y: self.ymin as f64 + (frac.y * self.ysize() as f64),
        }
    }
}

/// An RGBA float representation of a color.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    /// Amount of red, from 0.0 to 1.0.
    pub r: f64,
    /// Amount of green, from 0.0 to 1.0.
    pub g: f64,
    /// Amount of blue, from 0.0 to 1.0.
    pub b: f64,
    /// Amount of alpha, from 0.0 to 1.0.
    pub a: f64,
}
impl Color {
    pub const TRANSPARENT: Color = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0, };
    pub const BLACK: Color = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0, };
    pub const WHITE: Color = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0, };
    pub const RED: Color = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0, };
    pub const ORANGE: Color = Self { r: 1.0, g: 0.64, b: 0.0, a: 1.0, };
    pub const YELLOW: Color = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0, };
    pub const GREEN: Color = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0, };
    pub const BLUE: Color = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0, };
    pub const PURPLE: Color = Self { r: 0.62, g: 0.12, b: 0.94, a: 1.0, };
}

/// A drawable shape.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Circle { r: u32 },
    Square { l: u32 },
    Rectangle { h: u32, w: u32 },
}
impl Shape {
    /// Scales the shape by some multiplicative factor.
    pub fn scale(&mut self, mult: u32) {
        *self = match self {
            Shape::Circle { r } => Shape::Circle { r: mult * *r },
            Shape::Square { l } => Shape::Square { l: mult * *l },
            Shape::Rectangle { h, w } => Shape::Rectangle { h: mult * *h, w: mult * *w },
        }
    }
}

/// Complete font settings.
#[derive(Copy, Clone, Debug)]
pub struct Font {
    /// Name of the font used.
    pub name: FontName,
    /// Size of the font.
    pub size: f32,
    /// Slant of the font.
    pub slant: FontSlant,
    /// Weight of the font.
    pub weight: FontWeight,
}
impl Default for Font {
    fn default() -> Self {
        Self {
            name: FontName::default(),
            size: 12.0,
            slant: FontSlant::default(),
            weight: FontWeight::default(),
        }
    }
}

/// The name of a text font.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum FontName {
    Arial,
    Georgia,
}
impl Default for FontName {
    fn default() -> Self {
        Self::Arial
    }
}

/// The slant of a font.
#[derive(Copy, Clone, Debug)]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique,
}
impl Default for FontSlant {
    fn default() -> Self {
        Self::Normal
    }
}

/// The weight of a font.
#[derive(Copy, Clone, Debug)]
pub enum FontWeight {
    Normal,
    Bold,
}
impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

/// How something should be aligned.
#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    /// Aligned to the center.
    Center,
    /// Aligned to the center of the left side.
    Left,
    /// Aligned to the center of the right side.
    Right,
    /// Aligned to the center of the top side.
    Top,
    /// Aligned to the center of the bottom side.
    Bottom,
    /// Aligned to the top left corner.
    TopLeft,
    /// Aligned to the top right corner.
    TopRight,
    /// Aligned to the bottom left corner.
    BottomLeft,
    /// Aligned to the bottom right corner.
    BottomRight,
}

/// A graphics image file format.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum FileFormat {
    /// A PNG file format.
    Png,
    /// An SVG file format.
    Svg,
}

/// Describes a [`Canvas`] to be constructed.
#[derive(Clone, Debug)]
pub struct CanvasDescriptor {
    /// The size in dots (pixels) of the canvas.
    pub size: Size,
    /// The background color of the canvas.
    pub face_color: Color,
    /// What type of image format will be drawn.
    pub image_format: ImageFormat,
}
impl Default for CanvasDescriptor {
    fn default() -> Self {
        Self {
            size: Size { height: 100, width: 100 },
            face_color: Color::WHITE,
            image_format: ImageFormat::Bitmap,
        }
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ImageFormat {
    /// An image represented as a bitmap or pixel map.
    Bitmap,
    /// An image represented as an SVG image.
    Svg,
}

/// Describes a shape to be drawn.
#[derive(Clone, Debug)]
pub struct ShapeDescriptor<'a> {
    /// The point at which the shape is drawn.
    pub point: Point,
    /// The shape to be drawn.
    pub shape: Shape,
    /// The fill color of the shape.
    pub fill_color: Color,
    /// The width of the outline line.
    pub line_width: u32,
    /// The color of the outline.
    pub line_color: Color,
    /// How the outline will be dashed.
    pub line_dashes: &'a [f64],
    /// Optionally clip drawing to some area.
    pub clip_area: Option<Area>,
}
impl Default for ShapeDescriptor<'_> {
    fn default() -> Self {
        Self {
            point: Point { x: 0.0, y: 0.0 },
            shape: Shape::Circle { r: 1 },
            fill_color: Color::WHITE,
            line_width: 2,
            line_color: Color::BLACK,
            line_dashes: &[],
            clip_area: None,
        }
    }
}

/// Describes a line to be drawn.
#[derive(Clone, Debug)]
pub struct LineDescriptor<'a> {
    /// Where to draw the line.
    pub line: Line,
    /// The width of the line.
    pub line_width: u32,
    /// The color of the line.
    pub line_color: Color,
    /// How the line will be dashed.
    pub dashes: &'a [f64],
    /// Optionally clip drawing to some area.
    pub clip_area: Option<Area>,
}
impl Default for LineDescriptor<'_> {
    fn default() -> Self {
        Self {
            line: Line {
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 0.0, y: 0.0 },
            },
            line_width: 2,
            line_color: Color::BLACK,
            dashes: &[],
            clip_area: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CurveDescriptor<'a> {
    /// Where to draw the curve
    pub points: Vec<Point>,
    /// The width of the line.
    pub line_width: u32,
    /// The color of the line.
    pub line_color: Color,
    /// How the line will be dashed.
    pub dashes: &'a [f64],
    /// Optionally clip drawing to some area.
    pub clip_area: Option<Area>,
}
impl Default for CurveDescriptor<'_> {
    fn default() -> Self {
        Self {
            points: vec![],
            line_width: 2,
            line_color: Color::BLACK,
            dashes: &[],
            clip_area: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextDescriptor {
    /// The text to be drawn.
    pub text: String,
    /// The font to draw the text in.
    pub font: Font,
    /// Where to draw the text.
    pub position: Point,
    /// The color of the text.
    pub color: Color,
    /// How the text should be rotated.
    pub rotation: f64,
    /// What side of the text to align to the position.
    pub alignment: Alignment,
    /// Optionally clip drawing to some area.
    pub clip_area: Option<Area>,
}
impl Default for TextDescriptor {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            font: Font::default(),
            position: Point { x: 0.0, y: 0.0 },
            color: Color::BLACK,
            rotation: 0.0,
            alignment: Alignment::Center,
            clip_area: None,
        }
    }
}

/// Describes a region to be filled with a specified color.
#[derive(Clone, Debug)]
pub struct FillDescriptor {
    /// Points the define the region of interest.
    pub points: Vec<Point>,
    /// The color of the region.
    pub fill_color: Color,
    /// Optionally clip drawing to some area.
    pub clip_area: Option<Area>,
}

/// Describes how to save the image to a file.
#[derive(Clone, Debug)]
pub struct SaveFileDescriptor<P: AsRef<path::Path>> {
    /// The name of the output file.
    pub filename: P,
    /// The image format of the file.
    pub format: FileFormat,
    /// The dots (pixels) per inch.
    pub dpi: u16,
}

/// Represents a structure used for drawing.
pub trait Canvas {
    /// The main constructor.
    fn new(desc: CanvasDescriptor) -> Result<Self, DrawError> where Self: Sized;
    /// Draws a shape described by a [`ShapeDescriptor`].
    fn draw_shape(&mut self, desc: ShapeDescriptor) -> Result<(), DrawError>;
    /// Draws a line described by a [`LineDescriptor`].
    fn draw_line(&mut self, desc: LineDescriptor) -> Result<(), DrawError>;
    /// Draws a curve described by a [`CurveDescriptor`].
    fn draw_curve(&mut self, desc: CurveDescriptor) -> Result<(), DrawError>;
    /// Draws color in a closed, arbitrary region described by a [`FillDescriptor`].
    fn fill_region(&mut self, desc: FillDescriptor) -> Result<(), DrawError>;
    /// Draws text described by a [`TextDescriptor`].
    fn draw_text(&mut self, desc: TextDescriptor) -> Result<(), DrawError>;
    /// Returns a [`Size`] representing the extent of the text described by a [`TextDescriptor`].
    fn text_size(&mut self, desc: TextDescriptor) -> Result<Size, DrawError>;
    /// Save the image to a file.
    fn save_file<P: AsRef<path::Path>>(
        &mut self,
        desc: SaveFileDescriptor<P>,
    ) -> Result<(), DrawError>;
    /// Get canvas size.
    fn size(&self) -> Result<Size, DrawError>;
}
