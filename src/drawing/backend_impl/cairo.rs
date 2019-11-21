use cairo::{Context as CairoContext, FontSlant, FontWeight, Status as CairoStatus};

#[allow(unused_imports)]
use crate::drawing::backend::{BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind};
use crate::style::text_anchor::{HPos, VPos};
#[allow(unused_imports)]
use crate::style::{Color, FontStyle, FontTransform, RGBAColor, TextStyle};

/// The drawing backend that is backed with a Cairo context
pub struct CairoBackend<'a> {
    context: &'a CairoContext,
    width: u32,
    height: u32,
    init_flag: bool,
}

#[derive(Debug)]
pub struct CairoError(CairoStatus);

impl std::fmt::Display for CairoError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for CairoError {}

impl<'a> CairoBackend<'a> {
    fn call_cairo<F: Fn(&CairoContext)>(&self, f: F) -> Result<(), DrawingErrorKind<CairoError>> {
        f(self.context);
        if self.context.status() == CairoStatus::Success {
            return Ok(());
        }
        Err(DrawingErrorKind::DrawingError(CairoError(
            self.context.status(),
        )))
    }

    fn set_color(&self, color: &RGBAColor) -> Result<(), DrawingErrorKind<CairoError>> {
        self.call_cairo(|c| {
            c.set_source_rgba(
                f64::from(color.rgb().0) / 255.0,
                f64::from(color.rgb().1) / 255.0,
                f64::from(color.rgb().2) / 255.0,
                f64::from(color.alpha()),
            )
        })?;
        Ok(())
    }

    fn set_stroke_width(&self, width: u32) -> Result<(), DrawingErrorKind<CairoError>> {
        self.call_cairo(|c| c.set_line_width(f64::from(width)))?;
        Ok(())
    }

    pub fn new(context: &'a CairoContext, (w, h): (u32, u32)) -> Result<Self, CairoError> {
        let ret = Self {
            context,
            width: w,
            height: h,
            init_flag: false,
        };
        Ok(ret)
    }
}

impl<'a> DrawingBackend for CairoBackend<'a> {
    type ErrorType = CairoError;

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if !self.init_flag {
            let (x0, y0, x1, y1) = self.context.clip_extents();
            self.call_cairo(|c| {
                c.scale(
                    (x1 - x0) / f64::from(self.width),
                    (y1 - y0) / f64::from(self.height),
                )
            })?;
            self.init_flag = true;
        }
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: &RGBAColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.call_cairo(|c| c.rectangle(f64::from(point.0), f64::from(point.1), 1.0, 1.0))?;
        self.call_cairo(|c| {
            c.set_source_rgba(
                f64::from(color.rgb().0) / 255.0,
                f64::from(color.rgb().1) / 255.0,
                f64::from(color.rgb().2) / 255.0,
                f64::from(color.alpha()),
            )
        })?;
        self.call_cairo(|c| c.fill())?;
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.call_cairo(|c| c.move_to(f64::from(from.0), f64::from(from.1)))?;

        self.set_color(&style.as_color())?;
        self.set_stroke_width(style.stroke_width())?;

        self.call_cairo(|c| c.line_to(f64::from(to.0), f64::from(to.1)))?;
        self.call_cairo(|c| c.stroke())?;
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.set_color(&style.as_color())?;
        self.set_stroke_width(style.stroke_width())?;

        self.call_cairo(|c| {
            c.rectangle(
                f64::from(upper_left.0),
                f64::from(upper_left.1),
                f64::from(bottom_right.0 - upper_left.0),
                f64::from(bottom_right.1 - upper_left.1),
            )
        })?;

        if fill {
            self.call_cairo(|c| c.fill())?;
        } else {
            self.call_cairo(|c| c.stroke())?;
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.set_color(&style.as_color())?;
        self.set_stroke_width(style.stroke_width())?;

        let mut path = path.into_iter();

        if let Some((x, y)) = path.next() {
            self.call_cairo(|c| c.move_to(f64::from(x), f64::from(y)))?;
        }

        for (x, y) in path {
            self.call_cairo(|c| c.line_to(f64::from(x), f64::from(y)))?;
        }

        self.call_cairo(|c| c.stroke())?;

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.set_color(&style.as_color())?;
        self.set_stroke_width(style.stroke_width())?;

        let mut path = path.into_iter();

        if let Some((x, y)) = path.next() {
            self.call_cairo(|c| c.move_to(f64::from(x), f64::from(y)))?;
        } else {
            return Ok(());
        }

        for (x, y) in path {
            self.call_cairo(|c| c.line_to(f64::from(x), f64::from(y)))?;
        }

        self.call_cairo(|c| c.close_path())?;
        self.call_cairo(|c| c.fill())?;

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.set_color(&style.as_color())?;
        self.set_stroke_width(style.stroke_width())?;

        self.call_cairo(|c| c.new_sub_path())?;
        self.call_cairo(|c| {
            c.arc(
                f64::from(center.0),
                f64::from(center.1),
                f64::from(radius),
                0.0,
                std::f64::consts::PI * 2.0,
            )
        })?;

        if fill {
            self.call_cairo(|c| c.fill())?;
        } else {
            self.call_cairo(|c| c.stroke())?;
        }
        Ok(())
    }

    fn draw_text(
        &mut self,
        text: &str,
        style: &TextStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let font = &style.font;
        let color = &style.color;
        let (mut x, mut y) = (pos.0, pos.1);

        let degree = match font.get_transform() {
            FontTransform::None => 0.0,
            FontTransform::Rotate90 => 90.0,
            FontTransform::Rotate180 => 180.0,
            FontTransform::Rotate270 => 270.0,
        } / 180.0
            * std::f64::consts::PI;

        if degree != 0.0 {
            self.call_cairo(|c| c.save())?;
            self.call_cairo(|c| c.translate(f64::from(x), f64::from(y)))?;
            self.call_cairo(|c| c.rotate(degree))?;
            x = 0;
            y = 0;
        }

        self.call_cairo(|c| match font.get_style() {
            FontStyle::Normal => {
                c.select_font_face(font.get_name(), FontSlant::Normal, FontWeight::Normal)
            }
            FontStyle::Bold => {
                c.select_font_face(font.get_name(), FontSlant::Normal, FontWeight::Bold)
            }
            FontStyle::Oblique => {
                c.select_font_face(font.get_name(), FontSlant::Oblique, FontWeight::Normal)
            }
            FontStyle::Italic => {
                c.select_font_face(font.get_name(), FontSlant::Italic, FontWeight::Normal)
            }
        })?;
        let actual_size = font.get_size();
        self.call_cairo(|c| c.set_font_size(actual_size))?;
        self.set_color(&color)?;

        self.call_cairo(|c| {
            let extents = c.text_extents(text);
            let dx = match style.pos.h_pos {
                HPos::Left => 0.0,
                HPos::Right => -(extents.width + extents.x_bearing),
                HPos::Center => -(extents.width / 2.0 + extents.x_bearing),
            };
            let dy = match style.pos.v_pos {
                VPos::Top => extents.height,
                VPos::Center => extents.height / 2.0,
                VPos::Bottom => 0.0,
            };
            c.move_to(f64::from(x) + dx, f64::from(y) + dy);
        })?;
        self.call_cairo(|c| c.show_text(text))?;

        if degree != 0.0 {
            self.call_cairo(|c| c.restore())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    use crate::style::text_anchor::{HPos, Pos, VPos};
    use std::fs;
    use std::path::Path;

    static DST_DIR: &str = "target/test/cairo";

    fn checked_save_file(name: &str, content: &str) {
        /*
          Please use the PS file to manually verify the results.

          You may want to use Ghostscript to view the file.
        */
        assert!(!content.is_empty());
        fs::create_dir_all(DST_DIR).unwrap();
        let file_name = format!("{}.ps", name);
        let file_path = Path::new(DST_DIR).join(file_name);
        println!("{:?} created", file_path);
        fs::write(file_path, &content).unwrap();
    }

    fn draw_mesh_with_custom_ticks(tick_size: i32, test_name: &str) {
        let buffer: Vec<u8> = vec![];
        let surface = cairo::PsSurface::for_stream(500.0, 500.0, buffer);
        let cr = CairoContext::new(&surface);
        let root = CairoBackend::new(&cr, (500, 500))
            .unwrap()
            .into_drawing_area();

        // Text could be rendered to different elements if has whitespaces
        let mut chart = ChartBuilder::on(&root)
            .caption("this-is-a-test", ("sans-serif", 20))
            .set_all_label_area_size(40)
            .build_ranged(0..10, 0..10)
            .unwrap();

        chart
            .configure_mesh()
            .set_all_tick_mark_size(tick_size)
            .draw()
            .unwrap();

        let buffer = *surface.finish_output_stream().unwrap().downcast().unwrap();
        let content = String::from_utf8(buffer).unwrap();
        checked_save_file(test_name, &content);

        assert!(content.contains("this-is-a-test"));
    }

    #[test]
    fn test_draw_mesh_no_ticks() {
        draw_mesh_with_custom_ticks(0, "test_draw_mesh_no_ticks");
    }

    #[test]
    fn test_draw_mesh_negative_ticks() {
        draw_mesh_with_custom_ticks(-10, "test_draw_mesh_negative_ticks");
    }

    #[test]
    fn test_text_draw() {
        let buffer: Vec<u8> = vec![];
        let (width, height) = (1000, 500);
        let surface = cairo::PsSurface::for_stream(width.into(), height.into(), buffer);
        let cr = CairoContext::new(&surface);
        let root = CairoBackend::new(&cr, (width, height))
            .unwrap()
            .into_drawing_area();

        let mut chart = ChartBuilder::on(&root)
            .caption("All anchor point positions", ("sans-serif", 20))
            .set_all_label_area_size(40)
            .build_ranged(0..100, 0..50)
            .unwrap();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("X Axis")
            .y_desc("Y Axis")
            .draw()
            .unwrap();

        for (dy, trans) in [
            FontTransform::None,
            FontTransform::Rotate90,
            FontTransform::Rotate180,
            FontTransform::Rotate270,
        ]
        .iter()
        .enumerate()
        {
            for (dx1, h_pos) in [HPos::Left, HPos::Right, HPos::Center].iter().enumerate() {
                for (dx2, v_pos) in [VPos::Top, VPos::Center, VPos::Bottom].iter().enumerate() {
                    let x = 100_i32 + (dx1 as i32 * 3 + dx2 as i32) * 100;
                    let y = 100 + dy as i32 * 100;
                    root.draw(&Circle::new((x, y), 3, &BLACK.mix(0.5))).unwrap();
                    let style = TextStyle::from(("sans-serif", 20).into_font())
                        .pos(Pos::new(*h_pos, *v_pos))
                        .transform(trans.clone());
                    root.draw_text("test", &style, (x, y)).unwrap();
                }
            }
        }

        let buffer = *surface.finish_output_stream().unwrap().downcast().unwrap();
        let content = String::from_utf8(buffer).unwrap();
        checked_save_file("test_text_draw", &content);

        assert_eq!(content.matches("test").count(), 36);
    }
}
