use libc::{c_char, c_int};
use std::ffi::CString;

use crate::get_error;
use crate::sdl;
use crate::sys::gfx::primitives;
use crate::video;

/// A surface which can have primitives drawn to it.
pub trait DrawRenderer {
    /// Pixel draw with blending enabled if a<255.
    ///
    /// # Arguments
    ///
    /// * `x` - The x (horizontal) coordinate of the pixel.
    /// * `y` - The y (vertical) coordinate of the pixel.
    /// * `color` - The color value of the pixel to draw.
    ///
    fn draw_pixel(&self, x: i16, y: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw horizontal line with blending.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point (i.e. left) of the line.
    /// * `x2` - X coordinate of the second point (i.e. right) of the line.
    /// * `y` - Y coordinate of the points of the line.
    /// * `color` - The color value of the line to draw.
    ///
    fn draw_hline(&self, x1: i16, x2: i16, y: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw vertical line with blending.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the points of the line.
    /// * `y1` - Y coordinate of the first point (i.e. top) of the line.
    /// * `y2` - Y coordinate of the second point (i.e. bottom) of the line.
    /// * `color` - The color value of the line to draw.
    ///
    fn draw_vline(&self, x: i16, y1: i16, y2: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw rectangle with blending.
    ///
    /// # Arguments
    /// * `x1` - X coordinate of the first point (i.e. top right) of the rectangle.
    /// * `y1` - Y coordinate of the first point (i.e. top right) of the rectangle.
    /// * `x2` - X coordinate of the second point (i.e. bottom left) of the rectangle.
    /// * `y2` - Y coordinate of the second point (i.e. bottom left) of the rectangle.
    /// * `color` - The color value of the rectangle to draw.
    ///
    fn draw_rectangle(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw rounded-corner rectangle with blending.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point (i.e. top right) of the rectangle.
    /// * `y1` - Y coordinate of the first point (i.e. top right) of the rectangle.
    /// * `x2` - X coordinate of the second point (i.e. bottom left) of the rectangle.
    /// * `y2` - Y coordinate of the second point (i.e. bottom left) of the rectangle.
    /// * `rad` - The radius of the corner arc.
    /// * `color` - The color value of the rectangle to draw.
    ///
    fn draw_rounded_rectangle(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        rad: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw box (filled rectangle) with blending.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point (i.e. top right) of the box.
    /// * `y1` - Y coordinate of the first point (i.e. top right) of the box.
    /// * `x2` - X coordinate of the second point (i.e. bottom left) of the box.
    /// * `y2` - Y coordinate of the second point (i.e. bottom left) of the box.
    /// * `color` - The color value of the box to draw.
    ///
    fn draw_box(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw rounded-corner box (filled rectangle) with blending.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point (i.e. top right) of the box.
    /// * `y1` - Y coordinate of the first point (i.e. top right) of the box.
    /// * `x2` - X coordinate of the second point (i.e. bottom left) of the box.
    /// * `y2` - Y coordinate of the second point (i.e. bottom left) of the box.
    /// * `rad` - The radius of the corner arcs of the box.
    /// * `color` - The color value of the box to draw.
    ///
    fn draw_rounded_box(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        rad: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw line with alpha blending.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point of the line.
    /// * `y1` - Y coordinate of the first point of the line.
    /// * `x2` - X coordinate of the second point of the line.
    /// * `y2` - Y coordinate of the second point of the line.
    /// * `color` - The color value of the line to draw.
    ///
    fn draw_line(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw anti-aliased line with alpha blending.
    ///
    /// # Arguments:
    ///
    /// * `x1` - X coordinate of the first point of the aa-line.
    /// * `y1` - Y coordinate of the first point of the aa-line.
    /// * `x2` - X coordinate of the second point of the aa-line.
    /// * `y2` - Y coordinate of the second point of the aa-line.
    /// * `color` - The color value of the aa-line to draw.
    ///
    fn draw_aa_line(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw a thick line with alpha blending.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point of the line.
    /// * `y1` - Y coordinate of the first point of the line.
    /// * `x2` - X coordinate of the second point of the line.
    /// * `y2` - Y coordinate of the second point of the line.
    /// * `width` - Width of the line in pixels. Must be >0.
    /// * `color` - The color value of the line to draw.
    ///
    fn draw_thick_line(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        width: u8,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw circle with blending.
    ///
    /// Note: Circle drawing routine is based on an algorithms from the sge
    /// library, but modified by A. Schiffler for multiple pixel-draw removal
    /// and other minor speedup changes.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the circle.
    /// * `y` - Y coordinate of the center of the circle.
    /// * `rad` - Radius in pixels of the circle.
    /// * `color` - The color value of the circle to draw.
    ///
    fn draw_circle(&self, x: i16, y: i16, rad: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw anti-aliased circle with blending.
    ///
    /// Note: The AA-circle routine is based on AA-ellipse with identical radii.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the aa-circle.
    /// * `y` - Y coordinate of the center of the aa-circle.
    /// * `rad` - Radius in pixels of the aa-circle.
    /// * `color` - The color value of the aa-circle to draw.
    ///
    fn draw_aa_circle(&self, x: i16, y: i16, rad: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw filled circle with blending.
    ///
    /// Note: Based on algorithms from sge library with modifications by A.
    /// Schiffler for multiple-hline draw removal and other minor speedup
    /// changes.
    ///
    /// # Arguments
    ///
    /// `x` - X coordinate of the center of the filled circle. `y` - Y
    /// coordinate of the center of the filled circle. `rad` - Radius in pixels
    /// of the filled circle. `color` - The color value of the filled circle to
    /// draw.
    ///
    fn draw_filled_circle(&self, x: i16, y: i16, rad: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Arc with blending.
    ///
    /// Note Arc drawing is based on circle algorithm by A. Schiffler and
    /// written by D. Raber. Calculates which octants arc goes through and
    /// renders pixels accordingly.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the arc.
    /// * `y` - Y coordinate of the center of the arc.
    /// * `rad` - Radius in pixels of the arc.
    /// * `start` - Starting radius in degrees of the arc. 0 degrees is down,
    ///   increasing counterclockwise.
    /// * `end` - Ending radius in degrees of the arc. 0 degrees is down,
    ///   increasing counterclockwise.
    /// * `color` - The color value of the arc to draw.
    ///
    fn draw_arc(
        &self,
        x: i16,
        y: i16,
        rad: i16,
        start: i16,
        end: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw ellipse with blending.
    ///
    /// Note: Based on algorithms from sge library with modifications by A.
    /// Schiffler for multiple-pixel draw removal and other minor speedup
    /// changes.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the ellipse.
    /// * `y` - Y coordinate of the center of the ellipse.
    /// * `rx` - Horizontal radius in pixels of the ellipse.
    /// * `ry` - Vertical radius in pixels of the ellipse.
    /// * `color` - The color value of the ellipse to draw.
    ///
    fn draw_ellipse(&self, x: i16, y: i16, rx: i16, ry: i16, color: sdl::Color) -> sdl::Result<()>;

    /// Draw anti-aliased ellipse with blending.
    ///
    /// Note: Based on code from Anders Lindstroem, which is based on code from
    /// sge library, which is based on code from TwinLib.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the aa-ellipse.
    /// * `y` - Y coordinate of the center of the aa-ellipse.
    /// * `rx` - Horizontal radius in pixels of the aa-ellipse.
    /// * `ry` - Vertical radius in pixels of the aa-ellipse.
    /// * `color` - The color value of the aa-ellipse to draw.
    ///
    fn draw_aa_ellipse(
        &self,
        x: i16,
        y: i16,
        rx: i16,
        ry: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw filled ellipse with blending.
    ///
    /// Note: Based on algorithm from sge library with multiple-hline draw removal
    /// and other speedup changes.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the filled ellipse.
    /// * `y` - Y coordinate of the center of the filled ellipse.
    /// * `rx` - Horizontal radius in pixels of the filled ellipse.
    /// * `ry` - Vertical radius in pixels of the filled ellipse.
    /// * `color` - The color value of the filled ellipse to draw.
    fn draw_filled_ellipse(
        &self,
        x: i16,
        y: i16,
        rx: i16,
        ry: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw pie (outline) with alpha blending.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the pie.
    /// * `y` - Y coordinate of the center of the pie.
    /// * `rad` - Radius in pixels of the pie.
    /// * `start` - Starting radius in degrees of the pie.
    /// * `end` - Ending radius in degrees of the pie.
    /// * `color` - The color value of the pie to draw.
    ///
    fn draw_pie(
        &self,
        x: i16,
        y: i16,
        rad: i16,
        start: i16,
        end: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw filled pie with alpha blending.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the center of the filled pie.
    /// * `y` - Y coordinate of the center of the filled pie.
    /// * `rad` - Radius in pixels of the filled pie.
    /// * `start` - Starting radius in degrees of the filled pie.
    /// * `end` - Ending radius in degrees of the filled pie.
    /// * `color` - The color value of the filled pie to draw
    ///
    fn draw_filled_pie(
        &self,
        x: i16,
        y: i16,
        rad: i16,
        start: i16,
        end: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw trigon (triangle outline) with alpha blending.
    ///
    /// Note: Creates vertex array and uses polygon routine to render.
    ///
    /// # Arguments
    /// * `x1` - X coordinate of the first point of the trigon.
    /// * `y1` - Y coordinate of the first point of the trigon.
    /// * `x2` - X coordinate of the second point of the trigon.
    /// * `y2` - Y coordinate of the second point of the trigon.
    /// * `x3` - X coordinate of the third point of the trigon.
    /// * `y3` - Y coordinate of the third point of the trigon.
    /// * `color` - The color value of the trigon to draw.
    ///
    fn draw_trigon(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        x3: i16,
        y3: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw anti-aliased trigon (triangle outline) with alpha blending.
    ///
    /// Note: Creates vertex array and uses aapolygon routine to render.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point of the aa-trigon.
    /// * `y1` - Y coordinate of the first point of the aa-trigon.
    /// * `x2` - X coordinate of the second point of the aa-trigon.
    /// * `y2` - Y coordinate of the second point of the aa-trigon.
    /// * `x3` - X coordinate of the third point of the aa-trigon.
    /// * `y3` - Y coordinate of the third point of the aa-trigon.
    /// * `color` - The color value of the aa-trigon to draw.
    ///
    fn draw_aa_trigon(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        x3: i16,
        y3: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw filled trigon (triangle) with alpha blending.
    ///
    /// Note: Creates vertex array and uses aapolygon routine to render.
    ///
    /// # Arguments
    ///
    /// * `x1` - X coordinate of the first point of the filled trigon.
    /// * `y1` - Y coordinate of the first point of the filled trigon.
    /// * `x2` - X coordinate of the second point of the filled trigon.
    /// * `y2` - Y coordinate of the second point of the filled trigon.
    /// * `x3` - X coordinate of the third point of the filled trigon.
    /// * `y3` - Y coordinate of the third point of the filled trigon.
    /// * `color` - The color value of the filled trigon to draw.
    ///
    fn draw_filled_trigon(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        x3: i16,
        y3: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw polygon with alpha blending.
    ///
    /// # Arguments
    ///
    /// * `vx` - Vertex array containing X coordinates of the points of the polygon.
    /// * `vy` - Vertex array containing Y coordinates of the points of the polygon.
    /// * `n` - Number of points in the vertex array. Minimum number is 3.
    /// * `color` - The color value of the polygon to draw.
    ///
    fn draw_polygon(&self, vx: &[i16], vy: &[i16], color: sdl::Color) -> sdl::Result<()>;

    /// Draw anti-aliased polygon with alpha blending.
    ///
    /// # Arguments
    ///
    /// * `dst` - The surface to draw on.
    /// * `vx` - Vertex array containing X coordinates of the points of the aa-polygon.
    /// * `vy` - Vertex array containing Y coordinates of the points of the aa-polygon.
    /// * `n` - Number of points in the vertex array. Minimum number is 3.
    /// * `color` - The color value of the aa-polygon to draw.
    fn draw_aa_polygon(&self, vx: &[i16], vy: &[i16], color: sdl::Color) -> sdl::Result<()>;

    /// Draw filled polygon with alpha blending.
    ///
    /// Note: Standard filledPolygon function is calling multithreaded version with
    /// NULL parameters to use the global vertex cache.
    ///
    /// # Arguments
    ///
    /// * `vx`  Vertex array containing X coordinates of the points of the filled
    ///   polygon.
    /// * `vy`  Vertex array containing Y coordinates of the points of the filled
    ///   polygon.
    /// * `n`   Number of points in the vertex array. Minimum number is 3.
    /// * `color`   The color value of the filled polygon to draw.
    ///
    fn draw_filled_polygon(&self, vx: &[i16], vy: &[i16], color: sdl::Color) -> sdl::Result<()>;

    /// Draws a polygon filled with the given texture.
    ///
    /// This standard version is calling multithreaded versions with NULL cache parameters.
    ///
    /// # Arguments
    ///
    /// * `vx` - array of x vector components
    /// * `vy` - array of x vector components
    /// * `n` - the amount of vectors in the vx and vy array
    /// * `texture` - the sdl surface to use to fill the polygon
    /// * `texture_dx` - the offset of the texture relative to the screeen. if you move the polygon 10 pixels to the left and want the texture to apear the same you need to increase the texture_dx value
    /// * `texture_dy` - see texture_dx
    ///
    fn draw_textured_polygon(
        &self,
        vx: &[i16],
        vy: &[i16],
        texture: &video::Surface,
        texture_dx: i16,
        texture_dy: i16,
        color: sdl::Color,
    ) -> sdl::Result<()>;

    /// Draw a bezier curve with alpha blending.
    ///
    /// # Arguments
    ///
    /// * `vx` - Vertex array containing X coordinates of the points of the bezier curve.
    /// * `vy` - Vertex array containing Y coordinates of the points of the bezier curve.
    /// * `n` - Number of points in the vertex array. Minimum number is 3.
    /// * `s` - Number of steps for the interpolation. Minimum number is 2.
    /// * `color` - The color value of the bezier curve to draw.
    ///
    fn draw_bezier(&self, vx: &[i16], vy: &[i16], s: i32, color: sdl::Color) -> sdl::Result<()>;

    /// Draw a character of the currently set font.
    ///
    /// On first call for a particular character and color combination, the
    /// function needs to generate the character surface (slower. Subsequent
    /// calls blit a cached surface (fast). Uses alpha blending if A<255 in
    /// color.
    ///
    /// # Arguments
    ///
    /// * `x` - X (horizontal) coordinate of the upper left corner of the
    ///   character.
    /// * `y` - Y (vertical) coordinate of the upper left corner of the
    ///   character.
    /// * `c` - The character to draw.
    /// * `color` - The color value of the character to draw.
    ///
    fn draw_character(&self, x: i16, y: i16, c: char, color: sdl::Color) -> sdl::Result<()>;

    /// Draw a string in the currently set font.
    ///
    /// The spacing between consequtive characters in the string is the fixed number of pixels of the character width of the current global font.
    ///
    /// # Arguments
    ///
    /// * `x` - X (horizontal) coordinate of the upper left corner of the string.
    /// * `y` - Y (vertical) coordinate of the upper left corner of the string.
    /// * `s` - The string to draw.
    /// color	The color value of the string to draw.
    ///
    fn draw_string(&self, x: i16, y: i16, s: &str, color: sdl::Color) -> sdl::Result<()>;
}

impl DrawRenderer for video::Surface {
    fn draw_pixel(&self, x: i16, y: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::pixelColor(self.raw(), x, y, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_hline(&self, x1: i16, x2: i16, y: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::hlineColor(self.raw(), x1, x2, y, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_vline(&self, x: i16, y1: i16, y2: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::vlineColor(self.raw(), x, y1, y2, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_rectangle(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe { primitives::rectangleColor(self.raw(), x1, y1, x2, y2, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_rounded_rectangle(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        rad: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe {
            primitives::roundedRectangleColor(self.raw(), x1, y1, x2, y2, rad, color.into())
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_box(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::boxColor(self.raw(), x1, y1, x2, y2, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_rounded_box(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        rad: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret =
            unsafe { primitives::roundedBoxColor(self.raw(), x1, y1, x2, y2, rad, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_line(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::lineColor(self.raw(), x1, y1, x2, y2, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_aa_line(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe { primitives::aalineColor(self.raw(), x1, y1, x2, y2, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_thick_line(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        width: u8,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret =
            unsafe { primitives::thickLineColor(self.raw(), x1, y1, x2, y2, width, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_circle(&self, x: i16, y: i16, rad: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::circleColor(self.raw(), x, y, rad, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_aa_circle(&self, x: i16, y: i16, rad: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::aacircleColor(self.raw(), x, y, rad, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_filled_circle(&self, x: i16, y: i16, rad: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::filledCircleColor(self.raw(), x, y, rad, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_arc(
        &self,
        x: i16,
        y: i16,
        rad: i16,
        start: i16,
        end: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe { primitives::arcColor(self.raw(), x, y, rad, start, end, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_ellipse(&self, x: i16, y: i16, rx: i16, ry: i16, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe { primitives::ellipseColor(self.raw(), x, y, rx, ry, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_aa_ellipse(
        &self,
        x: i16,
        y: i16,
        rx: i16,
        ry: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe { primitives::aaellipseColor(self.raw(), x, y, rx, ry, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_filled_ellipse(
        &self,
        x: i16,
        y: i16,
        rx: i16,
        ry: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe { primitives::filledEllipseColor(self.raw(), x, y, rx, ry, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_pie(
        &self,
        x: i16,
        y: i16,
        rad: i16,
        start: i16,
        end: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe { primitives::pieColor(self.raw(), x, y, rad, start, end, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_filled_pie(
        &self,
        x: i16,
        y: i16,
        rad: i16,
        start: i16,
        end: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret =
            unsafe { primitives::filledPieColor(self.raw(), x, y, rad, start, end, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_trigon(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        x3: i16,
        y3: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret =
            unsafe { primitives::trigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_aa_trigon(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        x3: i16,
        y3: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret =
            unsafe { primitives::aatrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    fn draw_filled_trigon(
        &self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        x3: i16,
        y3: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        let ret = unsafe {
            primitives::filledTrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.into())
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    // FIXME: may we use pointer tuple?
    fn draw_polygon(&self, vx: &[i16], vy: &[i16], color: sdl::Color) -> sdl::Result<()> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            primitives::polygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.into())
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    fn draw_aa_polygon(&self, vx: &[i16], vy: &[i16], color: sdl::Color) -> sdl::Result<()> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            primitives::aapolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.into())
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    fn draw_filled_polygon(&self, vx: &[i16], vy: &[i16], color: sdl::Color) -> sdl::Result<()> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            primitives::filledPolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.into())
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
    #[allow(unused_variables)]
    fn draw_textured_polygon(
        &self,
        vx: &[i16],
        vy: &[i16],
        texture: &video::Surface,
        texture_dx: i16,
        texture_dy: i16,
        color: sdl::Color,
    ) -> sdl::Result<()> {
        unimplemented!()
    }

    fn draw_bezier(&self, vx: &[i16], vy: &[i16], s: i32, color: sdl::Color) -> sdl::Result<()> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            primitives::bezierColor(
                self.raw(),
                vx.as_ptr(),
                vy.as_ptr(),
                n,
                s as c_int,
                color.into(),
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    fn draw_character(&self, x: i16, y: i16, c: char, color: sdl::Color) -> sdl::Result<()> {
        let ret =
            unsafe { primitives::characterColor(self.raw(), x, y, c as c_char, color.into()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    fn draw_string(&self, x: i16, y: i16, s: &str, color: sdl::Color) -> sdl::Result<()> {
        let ret = unsafe {
            let cstring = CString::new(s).unwrap();
            let buf = cstring.as_bytes().as_ptr();
            primitives::stringColor(self.raw(), x, y, buf as *mut c_char, color.into())
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }
}
