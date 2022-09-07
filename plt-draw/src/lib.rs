use std::{fs, path};

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
#[derive(Copy, Clone, Debug)]
pub enum FileFormat {
    /// A PNG file format.
    Png,
    /// A JPEG file format.
    Jpeg,
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
    pub graphics_type: ImageFormat,
}
impl Default for CanvasDescriptor {
    fn default() -> Self {
        Self {
            size: Size { height: 100, width: 100 },
            face_color: Color::WHITE,
            graphics_type: ImageFormat::Bitmap,
        }
    }
}

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
    fn new(desc: CanvasDescriptor) -> Self;
    /// Draws a shape described by a [`ShapeDescriptor`].
    fn draw_shape(&mut self, desc: ShapeDescriptor);
    /// Draws a line described by a [`LineDescriptor`].
    fn draw_line(&mut self, desc: LineDescriptor);
    /// Draws a curve described by a [`CurveDescriptor`].
    fn draw_curve(&mut self, desc: CurveDescriptor);
    /// Draws text described by a [`TextDescriptor`].
    fn draw_text(&mut self, desc: TextDescriptor);
    /// Returns a [`Size`] representing the extent of the text described by a [`TextDescriptor`].
    fn text_size(&mut self, desc: TextDescriptor) -> Size;
    /// Save the image to a file.
    fn save_file<P: AsRef<path::Path>>(&mut self, desc: SaveFileDescriptor<P>);
    /// Get canvas size.
    fn size(&self) -> Size;
}

/// The Cairo backend.
#[derive(Debug)]
pub struct CairoCanvas {
    size: Size,
    context: cairo::Context,
    graphics_type: ImageFormat,
    temp_file: Option<path::PathBuf>,
}
impl CairoCanvas {
    /// Construct from existing context.
    pub fn from_context(
        context: &cairo::Context,
        size: Size,
        graphics_type: ImageFormat,
    ) -> Self {
        Self {
            size,
            context: context.clone(),
            graphics_type,
            temp_file: None,
        }
    }
}
impl Canvas for CairoCanvas {
    fn new(desc: CanvasDescriptor) -> Self {
        let mut temp_file = None;

        let context = match desc.graphics_type {
            ImageFormat::Bitmap => {
                let surface = cairo::ImageSurface::create(
                    cairo::Format::ARgb32,
                    desc.size.width as i32,
                    desc.size.height as i32,
                ).unwrap();

                cairo::Context::new(&surface).unwrap()
            },
            ImageFormat::Svg => {
                let mut temp_filename = std::env::temp_dir();
                temp_filename.push("plt_temp.svg");
                temp_file = Some(temp_filename);

                let surface = cairo::SvgSurface::new(
                    desc.size.width.into(),
                    desc.size.height.into(),
                    temp_file.as_ref(),
                ).unwrap();

                cairo::Context::new(&surface).unwrap()
            },
        };

        context.set_source_rgba(desc.face_color.r, desc.face_color.g, desc.face_color.b, desc.face_color.a);

        context.paint().unwrap();

        Self {
            size: desc.size,
            context,
            graphics_type: desc.graphics_type,
            temp_file,
        }
    }

    fn draw_shape(&mut self, desc: ShapeDescriptor) {
        let origin = CairoPoint::from_point(desc.point, self.size);

        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        match desc.shape {
            Shape::Rectangle { h, w } => {
                self.context.rectangle(
                    origin.x - (w as f64) / 2.0, origin.y - (h as f64) / 2.0,
                    w as f64, h as f64,
                );
            },
            Shape::Square { l } => {
                self.context.rectangle(
                    origin.x - (l as f64) / 2.0, origin.y - (l as f64) / 2.0,
                    l as f64, l as f64,
                );
                self.context.close_path();
            },
            Shape::Circle { r } => {
                self.context.arc(
                    origin.x, origin.y,
                    r as f64, 0.0, 2.0*std::f64::consts::PI,
                );
            },
        };

        // fill shape
        self.context.set_source_rgba(desc.fill_color.r, desc.fill_color.g, desc.fill_color.b, desc.fill_color.a);
        self.context.fill_preserve().unwrap();

        // outline shape
        self.context.set_dash(desc.line_dashes, 0.0);
        self.context.set_line_width(desc.line_width as f64);
        self.context.set_source_rgba(desc.line_color.r, desc.line_color.g, desc.line_color.b, desc.line_color.a);
        self.context.stroke().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn draw_line(&mut self, desc: LineDescriptor) {
        let p1 = CairoPoint::from_point(desc.line.p1, self.size);
        let p2 = CairoPoint::from_point(desc.line.p2, self.size);

        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        self.context.set_source_rgba(desc.line_color.r, desc.line_color.g, desc.line_color.b, desc.line_color.a);
        self.context.set_line_width(desc.line_width as f64);

        self.context.set_dash(desc.dashes, 0.0);

        let offset = if desc.line_width % 2 == 0 { 0.0 } else { 0.5 };

        self.context.line_to(p1.x+offset, p1.y-offset);
        self.context.line_to(p2.x+offset, p2.y-offset);

        self.context.stroke().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn draw_curve(&mut self, desc: CurveDescriptor) {
        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        self.context.set_source_rgba(desc.line_color.r, desc.line_color.g, desc.line_color.b, desc.line_color.a);
        self.context.set_line_width(desc.line_width as f64);
        self.context.set_line_join(cairo::LineJoin::Round);

        self.context.set_dash(desc.dashes, 0.0);

        let offset = if desc.line_width % 2 == 0 { 0.0 } else { 0.5 };

        for point in desc.points {
            let point = CairoPoint::from_point(point, self.size);

            self.context.line_to(point.x+offset, point.y-offset);
        }

        self.context.stroke().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn draw_text(&mut self, desc: TextDescriptor) {
        let position = CairoPoint::from_point(desc.position, self.size);

        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        self.context.set_source_rgba(desc.color.r, desc.color.g, desc.color.b, desc.color.a);

        self.context.select_font_face(
            font_to_cairo(desc.font.name),
            font_slant_to_cairo(desc.font.slant),
            font_weight_to_cairo(desc.font.weight),
        );
        self.context.set_font_size(desc.font.size as f64);

        let extents = self.context.text_extents(&desc.text).unwrap();

        let position = align_text(position, desc.rotation, extents, desc.alignment);
        self.context.move_to(position.x, position.y);

        self.context.save().unwrap();
        self.context.rotate(desc.rotation);
        self.context.show_text(&desc.text).unwrap();
        self.context.restore().unwrap();

        self.context.stroke().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn text_size(&mut self, desc: TextDescriptor) -> Size {
        self.context.save().unwrap();

        self.context.set_source_rgba(desc.color.r, desc.color.g, desc.color.b, desc.color.a);

        self.context.select_font_face(
            font_to_cairo(desc.font.name),
            font_slant_to_cairo(desc.font.slant),
            font_weight_to_cairo(desc.font.weight),
        );
        self.context.set_font_size(desc.font.size as f64);

        let extents = self.context.text_extents(&desc.text).unwrap();

        self.context.stroke().unwrap();

        self.context.restore().unwrap();

        Size {
            width: extents.width.ceil() as u32,
            height: extents.height.ceil() as u32,
        }
    }

    fn save_file<P: AsRef<path::Path>>(&mut self, desc: SaveFileDescriptor<P>) {
        match self.graphics_type {
            ImageFormat::Bitmap => {
                match desc.format {
                    FileFormat::Png => {
                        // temporarily remove surface from context
                        let mut surface = cairo::ImageSurface::try_from(self.context.target()).unwrap();
                        let blank_surface = cairo::ImageSurface::create(
                            cairo::Format::ARgb32,
                            0,
                            0,
                        ).unwrap();
                        self.context = cairo::Context::new(&blank_surface).unwrap();

                        let file = fs::File::create(desc.filename).unwrap();
                        let w = &mut std::io::BufWriter::new(file);

                        // configure encoder
                        let mut encoder = png::Encoder::new(
                            w,
                            self.size.width as u32,
                            self.size.height as u32,
                        );
                        encoder.set_color(png::ColorType::Rgba);
                        encoder.set_depth(png::BitDepth::Eight);
                        let mut writer = encoder.write_header().unwrap();

                        // extract buffer from cairo
                        let buffer_raw = surface.data().unwrap();
                        // fix color byte ordering
                        let buffer = buffer_raw.chunks(4)
                            .flat_map(|rgba| {
                                [rgba[2], rgba[1], rgba[0], rgba[3]]
                            })
                            .collect::<Vec<_>>();

                        // set dpi
                        let ppu = (desc.dpi as f64 * (1000.0/25.4)) as u32;
                        let xppu = ppu.to_be_bytes();
                        let yppu = ppu.to_be_bytes();
                        let unit = png::Unit::Meter;
                        writer.write_chunk(
                            png::chunk::pHYs,
                            &[
                                xppu[0], xppu[1], xppu[2], xppu[3],
                                yppu[0], yppu[1], yppu[2], yppu[3],
                                unit as u8,
                            ],
                        ).unwrap();

                        writer.write_image_data(&buffer[..]).unwrap();

                        drop(buffer_raw);
                        drop(buffer);

                        // return surface to self
                        self.context = cairo::Context::new(&surface).unwrap();
                    },
                    _ => {
                        panic!("unsupported filetype for bitmap canvas");
                    },
                }
            },
            ImageFormat::Svg => {
                match desc.format {
                    FileFormat::Svg => {
                        // finish writing file
                        let old_surface = cairo::SvgSurface::try_from(
                            self.context.target()
                        ).unwrap();
                        old_surface.finish();

                        if let Some(temp_file) = &self.temp_file {
                            // copy temp file to new specified location
                            std::fs::copy(temp_file, desc.filename.as_ref()).unwrap();

                            // remove temp file
                            std::fs::remove_file(temp_file).unwrap();
                        }
                    },
                    _ => {
                        panic!("unsupported filetype for vector canvas");
                    },
                }
            },
        };
    }
    fn size(&self) -> Size {
        self.size
    }
}
impl CairoCanvas {
    fn reset_clip(&mut self) {
        self.context.reset_clip();
    }
    fn clip_area(&mut self, area: Area) {
        self.context.reset_clip();
        self.context.new_path();

        let points = [
            Point { x: area.xmin as f64, y: area.ymin as f64 },
            Point { x: area.xmin as f64, y: area.ymax as f64 },
            Point { x: area.xmax as f64, y: area.ymax as f64 },
            Point { x: area.xmax as f64, y: area.ymin as f64 },
        ];

        for point in points {
            let point = CairoPoint::from_point(point, self.size);
            self.context.line_to(point.x, point.y);
        }

        self.context.clip();
    }
}

// private

#[derive(Copy, Clone, Debug)]
struct CairoPoint {
    pub x: f64,
    pub y: f64,
}
impl CairoPoint {
    fn from_point(point: Point, size: Size) -> Self {
        Self { x: point.x, y: (size.height as f64 - point.y) }
    }
}

fn font_to_cairo(name: FontName) -> &'static str {
    match name {
        FontName::Georgia => "Georgia",
        FontName::Arial => "Arial",
    }
}
fn font_slant_to_cairo(slant: FontSlant) -> cairo::FontSlant {
    match slant {
        FontSlant::Normal => cairo::FontSlant::Normal,
        FontSlant::Italic => cairo::FontSlant::Italic,
        FontSlant::Oblique => cairo::FontSlant::Oblique,
    }
}
fn font_weight_to_cairo(weight: FontWeight) -> cairo::FontWeight {
    match weight {
        FontWeight::Normal => cairo::FontWeight::Normal,
        FontWeight::Bold => cairo::FontWeight::Bold,
    }
}

fn align_text(
    position: CairoPoint,
    rotation: f64,
    extents: cairo::TextExtents,
    alignment: Alignment,
) -> CairoPoint {
        let (x, y) = match alignment {
            Alignment::Center => {
                (
                    position.x
                    - (extents.x_bearing + extents.width / 2.0)*rotation.cos()
                    + (extents.y_bearing + extents.height / 2.0)*rotation.sin(),
                    position.y
                    - (extents.y_bearing + extents.height / 2.0)*rotation.cos()
                    - (extents.x_bearing + extents.width / 2.0)*rotation.sin(),
                )
            },
            Alignment::Right => {
                (
                    position.x
                    - extents.x_bearing*rotation.cos()
                    - extents.width*rotation.cos().clamp(0.0, 1.0)
                    + extents.y_bearing*rotation.sin().clamp(0.0, 1.0),
                    position.y
                    - (extents.y_bearing + (extents.height / 2.0))*rotation.cos()
                    - (extents.x_bearing + extents.width / 2.0)*rotation.sin(),
                )
            },
            Alignment::Left => {
                (
                    position.x
                    - extents.x_bearing*rotation.cos()
                    - extents.width*rotation.cos().clamp(-1.0, 0.0)
                    + extents.y_bearing*rotation.sin()
                    + extents.height*rotation.sin().clamp(0.0, 1.0),
                    position.y
                    - (extents.y_bearing + extents.height / 2.0)*rotation.cos()
                    - (extents.x_bearing + extents.width / 2.0)*rotation.sin(),
                )
            },
            Alignment::Top => {
                (
                    position.x
                    - (extents.x_bearing + extents.width / 2.0)*rotation.cos()
                    + (extents.y_bearing + extents.height / 2.0)*rotation.sin(),
                    position.y
                    - extents.y_bearing*rotation.cos()
                    - extents.x_bearing*rotation.sin()
                    - extents.width*rotation.sin().clamp(-1.0, 0.0)
                    - extents.height*rotation.cos().clamp(-1.0, 0.0),
                )
            },
            Alignment::Bottom => {
                (
                    position.x
                    - (extents.x_bearing + extents.width / 2.0)*rotation.cos()
                    + (extents.y_bearing + extents.height / 2.0)*rotation.sin(),
                    position.y
                    - extents.y_bearing*rotation.cos()
                    - extents.height*rotation.cos().clamp(0.0, 1.0)
                    - extents.x_bearing*rotation.sin()
                    - extents.width*rotation.sin().clamp(0.0, 1.0),
                )
            },
            Alignment::TopRight => {
                (
                    position.x
                    - extents.x_bearing*rotation.cos()
                    - extents.width*rotation.cos().clamp(0.0, 1.0)
                    + extents.y_bearing*rotation.sin()
                    + extents.height*rotation.sin().clamp(-1.0, 0.0),
                    position.y
                    - extents.y_bearing*rotation.cos()
                    - extents.height*rotation.cos().clamp(-1.0, 0.0)
                    - extents.x_bearing*rotation.sin()
                    - extents.width*rotation.sin().clamp(-1.0, 0.0),
                )
            },
            Alignment::TopLeft => {
                (
                    position.x
                    - extents.x_bearing*rotation.cos()
                    - extents.width*rotation.cos().clamp(-1.0, 0.0)
                    + extents.y_bearing*rotation.sin()
                    + extents.height*rotation.sin().clamp(0.0, 1.0),
                    position.y
                    - extents.y_bearing*rotation.cos()
                    - extents.height*rotation.cos().clamp(-1.0, 0.0)
                    + extents.x_bearing*rotation.sin()
                    - extents.width*rotation.sin().clamp(-1.0, 0.0),
                )
            },
            Alignment::BottomRight => {
                (
                    position.x
                    - extents.x_bearing*rotation.cos()
                    - extents.width*rotation.cos().clamp(0.0, 1.0)
                    + extents.y_bearing*rotation.sin()
                    + extents.height*rotation.sin().clamp(-1.0, 0.0),
                    position.y
                    - extents.y_bearing*rotation.cos()
                    - extents.height*rotation.cos().clamp(0.0, 1.0)
                    + extents.x_bearing*rotation.sin()
                    - extents.width*rotation.sin().clamp(0.0, 1.0),
                )
            },
            Alignment::BottomLeft => {
                (
                    position.x
                    - extents.x_bearing*rotation.cos()
                    - extents.width*rotation.cos().clamp(-1.0, 0.0)
                    + extents.y_bearing*rotation.sin()
                    + extents.height*rotation.sin().clamp(0.0, 1.0),
                    position.y
                    - extents.y_bearing*rotation.cos()
                    - extents.height*rotation.cos().clamp(0.0, 1.0)
                    + extents.x_bearing*rotation.sin()
                    - extents.width*rotation.sin().clamp(0.0, 1.0),
                )
            },
        };

        CairoPoint { x, y }
}
