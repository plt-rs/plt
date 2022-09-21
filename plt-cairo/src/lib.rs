use std::{env, fs, f64, io, path};

/// The Cairo backend for `plt`.
#[derive(Debug)]
pub struct CairoCanvas {
    size: draw::Size,
    context: cairo::Context,
    graphics_type: draw::ImageFormat,
    temp_file: Option<path::PathBuf>,
}
impl CairoCanvas {
    /// Construct from existing context.
    pub fn from_context(context: &cairo::Context, size: draw::Size, graphics_type: draw::ImageFormat) -> Self {
        Self {
            size,
            context: context.clone(),
            graphics_type,
            temp_file: None,
        }
    }
}
impl draw::Canvas for CairoCanvas {
    fn new(desc: draw::CanvasDescriptor) -> Self {
        let (context, temp_file) = match desc.graphics_type {
            draw::ImageFormat::Bitmap => {
                let surface = cairo::ImageSurface::create(
                    cairo::Format::ARgb32,
                    desc.size.width as i32,
                    desc.size.height as i32,
                )
                .unwrap();

                (cairo::Context::new(&surface).unwrap(), None)
            },
            draw::ImageFormat::Svg => {
                #[cfg(feature = "svg")]
                {
                    let mut temp_filename = env::temp_dir();
                    temp_filename.push("plt_temp.svg");
                    let temp_file = Some(temp_filename);

                    let surface = cairo::SvgSurface::new(
                        desc.size.width.into(),
                        desc.size.height.into(),
                        temp_file.as_ref(),
                    )
                    .unwrap();

                    (cairo::Context::new(&surface).unwrap(), temp_file)
                }

                #[cfg(not(feature = "svg"))]
                panic!("svg feature is not enabled");
            },
        };

        context.set_source_rgba(
            desc.face_color.r,
            desc.face_color.g,
            desc.face_color.b,
            desc.face_color.a,
        );

        context.paint().unwrap();

        Self {
            size: desc.size,
            context,
            graphics_type: desc.graphics_type,
            temp_file,
        }
    }

    fn draw_shape(&mut self, desc: draw::ShapeDescriptor) {
        let origin = CairoPoint::from_point(desc.point, self.size);

        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        match desc.shape {
            draw::Shape::Rectangle { h, w } => {
                self.context.rectangle(
                    origin.x - (w as f64) / 2.0,
                    origin.y - (h as f64) / 2.0,
                    w as f64,
                    h as f64,
                );
                self.context.close_path();
            },
            draw::Shape::Square { l } => {
                self.context.rectangle(
                    origin.x - (l as f64) / 2.0,
                    origin.y - (l as f64) / 2.0,
                    l as f64,
                    l as f64,
                );
                self.context.close_path();
            },
            draw::Shape::Circle { r } => {
                self.context.arc(
                    origin.x,
                    origin.y,
                    r as f64,
                    0.0,
                    2.0 * f64::consts::PI,
                );
                self.context.close_path();
            },
        };

        // fill shape
        self.context.set_source_rgba(
            desc.fill_color.r,
            desc.fill_color.g,
            desc.fill_color.b,
            desc.fill_color.a,
        );
        self.context.fill_preserve().unwrap();

        // outline shape
        self.context.set_dash(desc.line_dashes, 0.0);
        self.context.set_line_width(desc.line_width as f64);
        self.context.set_source_rgba(
            desc.line_color.r,
            desc.line_color.g,
            desc.line_color.b,
            desc.line_color.a,
        );
        self.context.stroke().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn draw_line(&mut self, desc: draw::LineDescriptor) {
        let p1 = CairoPoint::from_point(desc.line.p1, self.size);
        let p2 = CairoPoint::from_point(desc.line.p2, self.size);

        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        self.context.set_source_rgba(
            desc.line_color.r,
            desc.line_color.g,
            desc.line_color.b,
            desc.line_color.a,
        );
        self.context.set_line_width(desc.line_width as f64);

        self.context.set_dash(desc.dashes, 0.0);

        let offset = if desc.line_width % 2 == 0 { 0.0 } else { 0.5 };

        self.context.line_to(p1.x + offset, p1.y - offset);
        self.context.line_to(p2.x + offset, p2.y - offset);

        self.context.stroke().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn draw_curve(&mut self, desc: draw::CurveDescriptor) {
        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        self.context.set_source_rgba(
            desc.line_color.r,
            desc.line_color.g,
            desc.line_color.b,
            desc.line_color.a,
        );
        self.context.set_line_width(desc.line_width as f64);
        self.context.set_line_join(cairo::LineJoin::Round);

        self.context.set_dash(desc.dashes, 0.0);

        let offset = if desc.line_width % 2 == 0 { 0.0 } else { 0.5 };

        for point in desc.points {
            let point = CairoPoint::from_point(point, self.size);

            self.context.line_to(point.x + offset, point.y - offset);
        }

        self.context.stroke().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn fill_region(&mut self, desc: draw::FillDescriptor) {
        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        self.context.set_source_rgba(
            desc.fill_color.r,
            desc.fill_color.g,
            desc.fill_color.b,
            desc.fill_color.a,
        );

        for point in desc.points {
            let point = CairoPoint::from_point(point, self.size);

            self.context.line_to(point.x, point.y);
        }

        self.context.close_path();

        self.context.fill().unwrap();

        self.reset_clip();

        self.context.restore().unwrap();
    }

    fn draw_text(&mut self, desc: draw::TextDescriptor) {
        let position = CairoPoint::from_point(desc.position, self.size);

        self.context.save().unwrap();

        if let Some(area) = desc.clip_area {
            self.clip_area(area);
        }

        self.context.set_source_rgba(
            desc.color.r,
            desc.color.g,
            desc.color.b,
            desc.color.a,
        );

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

    fn text_size(&mut self, desc: draw::TextDescriptor) -> draw::Size {
        self.context.save().unwrap();

        self.context.set_source_rgba(
            desc.color.r,
            desc.color.g,
            desc.color.b,
            desc.color.a,
        );

        self.context.select_font_face(
            font_to_cairo(desc.font.name),
            font_slant_to_cairo(desc.font.slant),
            font_weight_to_cairo(desc.font.weight),
        );
        self.context.set_font_size(desc.font.size as f64);

        let extents = self.context.text_extents(&desc.text).unwrap();

        self.context.stroke().unwrap();

        self.context.restore().unwrap();

        draw::Size {
            width: extents.width.ceil() as u32,
            height: extents.height.ceil() as u32,
        }
    }

    fn save_file<P: AsRef<path::Path>>(&mut self, desc: draw::SaveFileDescriptor<P>) {
        match self.graphics_type {
            draw::ImageFormat::Bitmap => {
                match desc.format {
                    #[cfg(feature = "png")]
                    draw::FileFormat::Png => {
                        // temporarily remove surface from context
                        let mut surface = cairo::ImageSurface::try_from(
                            self.context.target()
                        )
                        .unwrap();
                        let blank_surface = cairo::ImageSurface::create(
                            cairo::Format::ARgb32,
                            0,
                            0,
                        )
                        .unwrap();
                        self.context = cairo::Context::new(&blank_surface).unwrap();

                        let file = fs::File::create(desc.filename).unwrap();
                        let w = &mut io::BufWriter::new(file);

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
                            .flat_map(|rgba| [rgba[2], rgba[1], rgba[0], rgba[3]])
                            .collect::<Vec<_>>();

                        // set dpi
                        let ppu = (desc.dpi as f64 * (1000.0 / 25.4)) as u32;
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
                        )
                        .unwrap();

                        writer.write_image_data(&buffer[..]).unwrap();

                        drop(buffer_raw);
                        drop(buffer);

                        // return surface to self
                        self.context = cairo::Context::new(&surface).unwrap();
                    },
                    #[cfg(not(feature = "png"))]
                    draw::FileFormat::Png => {
                        panic!("png feature not enabled");
                    },
                    _ => {
                        panic!("unsupported filetype for bitmap canvas");
                    },
                }
            },
            draw::ImageFormat::Svg => {
                #[cfg(feature = "svg")]
                match desc.format {
                    draw::FileFormat::Svg => {
                        // finish writing file
                        let old_surface = cairo::SvgSurface::try_from(
                            self.context.target()
                        )
                        .unwrap();
                        old_surface.finish();

                        if let Some(temp_file) = &self.temp_file {
                            // copy temp file to new specified location
                            fs::copy(temp_file, desc.filename.as_ref()).unwrap();

                            // remove temp file
                            fs::remove_file(temp_file).unwrap();
                        }
                    },
                    _ => {
                        panic!("unsupported filetype for svg canvas");
                    },
                }

                #[cfg(not(feature = "svg"))]
                panic!("svg feature is not enabled");
            },
        };
    }
    fn size(&self) -> draw::Size {
        self.size
    }
}
impl CairoCanvas {
    fn reset_clip(&mut self) {
        self.context.reset_clip();
    }
    fn clip_area(&mut self, area: draw::Area) {
        self.context.reset_clip();
        self.context.new_path();

        let points = [
            draw::Point { x: area.xmin as f64, y: area.ymin as f64 },
            draw::Point { x: area.xmin as f64, y: area.ymax as f64 },
            draw::Point { x: area.xmax as f64, y: area.ymax as f64 },
            draw::Point { x: area.xmax as f64, y: area.ymin as f64 },
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
    fn from_point(point: draw::Point, size: draw::Size) -> Self {
        Self { x: point.x, y: (size.height as f64 - point.y) }
    }
}

fn font_to_cairo(name: draw::FontName) -> &'static str {
    match name {
        draw::FontName::Georgia => "Georgia",
        draw::FontName::Arial => "Arial",
    }
}
fn font_slant_to_cairo(slant: draw::FontSlant) -> cairo::FontSlant {
    match slant {
        draw::FontSlant::Normal => cairo::FontSlant::Normal,
        draw::FontSlant::Italic => cairo::FontSlant::Italic,
        draw::FontSlant::Oblique => cairo::FontSlant::Oblique,
    }
}
fn font_weight_to_cairo(weight: draw::FontWeight) -> cairo::FontWeight {
    match weight {
        draw::FontWeight::Normal => cairo::FontWeight::Normal,
        draw::FontWeight::Bold => cairo::FontWeight::Bold,
    }
}

fn align_text(
    position: CairoPoint,
    rotation: f64,
    extents: cairo::TextExtents,
    alignment: draw::Alignment,
) -> CairoPoint {
    let (x, y) = match alignment {
        draw::Alignment::Center => (
            position.x - (extents.x_bearing + extents.width / 2.0)*rotation.cos()
                + (extents.y_bearing + extents.height / 2.0)*rotation.sin(),
            position.y - (extents.y_bearing + extents.height / 2.0)*rotation.cos()
                - (extents.x_bearing + extents.width / 2.0)*rotation.sin(),
        ),
        draw::Alignment::Right => (
            position.x - extents.x_bearing*rotation.cos()
                - extents.width*rotation.cos().clamp(0.0, 1.0)
                + extents.y_bearing*rotation.sin().clamp(0.0, 1.0),
            position.y - (extents.y_bearing + (extents.height / 2.0))*rotation.cos()
                - (extents.x_bearing + extents.width / 2.0)*rotation.sin(),
        ),
        draw::Alignment::Left => (
            position.x - extents.x_bearing*rotation.cos()
                - extents.width*rotation.cos().clamp(-1.0, 0.0)
                + extents.y_bearing*rotation.sin()
                + extents.height*rotation.sin().clamp(0.0, 1.0),
            position.y - (extents.y_bearing + extents.height / 2.0)*rotation.cos()
                - (extents.x_bearing + extents.width / 2.0)*rotation.sin(),
        ),
        draw::Alignment::Top => (
            position.x - (extents.x_bearing + extents.width / 2.0)*rotation.cos()
                + (extents.y_bearing + extents.height / 2.0)*rotation.sin(),
            position.y - extents.y_bearing*rotation.cos()
                - extents.x_bearing*rotation.sin()
                - extents.width*rotation.sin().clamp(-1.0, 0.0)
                - extents.height*rotation.cos().clamp(-1.0, 0.0),
        ),
        draw::Alignment::Bottom => (
            position.x - (extents.x_bearing + extents.width / 2.0)*rotation.cos()
                + (extents.y_bearing + extents.height / 2.0)*rotation.sin(),
            position.y - extents.y_bearing*rotation.cos()
                - extents.height*rotation.cos().clamp(0.0, 1.0)
                - extents.x_bearing*rotation.sin()
                - extents.width*rotation.sin().clamp(0.0, 1.0),
        ),
        draw::Alignment::TopRight => (
            position.x - extents.x_bearing*rotation.cos()
                - extents.width*rotation.cos().clamp(0.0, 1.0)
                + extents.y_bearing*rotation.sin()
                + extents.height*rotation.sin().clamp(-1.0, 0.0),
            position.y - extents.y_bearing*rotation.cos()
                - extents.height*rotation.cos().clamp(-1.0, 0.0)
                - extents.x_bearing*rotation.sin()
                - extents.width*rotation.sin().clamp(-1.0, 0.0),
        ),
        draw::Alignment::TopLeft => (
            position.x - extents.x_bearing*rotation.cos()
                - extents.width*rotation.cos().clamp(-1.0, 0.0)
                + extents.y_bearing*rotation.sin()
                + extents.height*rotation.sin().clamp(0.0, 1.0),
            position.y - extents.y_bearing*rotation.cos()
                - extents.height*rotation.cos().clamp(-1.0, 0.0)
                + extents.x_bearing*rotation.sin()
                - extents.width*rotation.sin().clamp(-1.0, 0.0),
        ),
        draw::Alignment::BottomRight => (
            position.x - extents.x_bearing*rotation.cos()
                - extents.width*rotation.cos().clamp(0.0, 1.0)
                + extents.y_bearing*rotation.sin()
                + extents.height*rotation.sin().clamp(-1.0, 0.0),
            position.y - extents.y_bearing*rotation.cos()
                - extents.height*rotation.cos().clamp(0.0, 1.0)
                + extents.x_bearing*rotation.sin()
                - extents.width*rotation.sin().clamp(0.0, 1.0),
        ),
        draw::Alignment::BottomLeft => (
            position.x - extents.x_bearing*rotation.cos()
                - extents.width*rotation.cos().clamp(-1.0, 0.0)
                + extents.y_bearing*rotation.sin()
                + extents.height*rotation.sin().clamp(0.0, 1.0),
            position.y - extents.y_bearing*rotation.cos()
                - extents.height*rotation.cos().clamp(0.0, 1.0)
                + extents.x_bearing*rotation.sin()
                - extents.width*rotation.sin().clamp(0.0, 1.0),
        ),
    };

    CairoPoint { x, y }
}
