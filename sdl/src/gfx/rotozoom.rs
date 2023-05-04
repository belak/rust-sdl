use libc::c_int;
pub use std::f64::consts::PI;

use sdl_sys::gfx::rotozoom;

use crate::get_error;
use crate::video::Surface;

pub trait RotozoomSurface {
    /// Rotates and zooms a surface and optional anti-aliasing.
    ///
    /// Rotates and zoomes a 32bit or 8bit 'src' surface to newly created 'dst'
    /// surface. 'angle' is the rotation in degrees and 'zoom' a scaling factor.
    /// If 'smooth' is set then the destination 32bit surface is anti-aliased.
    /// If the surface is not 8bit or 32bit RGBA/ABGR it will be converted into
    /// a 32bit RGBA format on the fly.
    ///
    /// # Arguments
    ///
    /// * `angle` - The angle to rotate in degrees.
    /// * `zoom` - The scaling factor.
    /// * `smooth` - Antialiasing flag; set to SMOOTHING_ON to enable.
    ///
    fn rotozoom(&self, angle: f64, zoom: f64, smooth: bool) -> crate::Result<Surface>;

    /// Rotates and zooms a surface with different horizontal and vertival
    /// scaling factors and optional anti-aliasing.
    ///
    /// Rotates and zooms a 32bit or 8bit 'src' surface to newly created 'dst'
    /// surface. 'angle' is the rotation in degrees, 'zoomx and 'zoomy' scaling
    /// factors. If 'smooth' is set then the destination 32bit surface is
    /// anti-aliased. If the surface is not 8bit or 32bit RGBA/ABGR it will be
    /// converted into a 32bit RGBA format on the fly.
    ///
    /// # Arguments
    ///
    /// * `angle` - The angle to rotate in degrees.
    /// * `zoomx` - The horizontal scaling factor.
    /// * `zoomy` - The vertical scaling factor.
    /// * `smooth` - Antialiasing flag; set to SMOOTHING_ON to enable.
    ///
    fn rotozoom_xy(
        &self,
        angle: f64,
        zoomx: f64,
        zoomy: f64,
        smooth: bool,
    ) -> crate::Result<Surface>;

    /// Zoom a surface by independent horizontal and vertical factors with
    /// optional smoothing.
    ///
    /// Zooms a 32bit or 8bit 'src' surface to newly created 'dst' surface.
    /// 'zoomx' and 'zoomy' are scaling factors for width and height. If
    /// 'smooth' is on then the destination 32bit surface is anti-aliased. If
    /// the surface is not 8bit or 32bit RGBA/ABGR it will be converted into a
    /// 32bit RGBA format on the fly. If zoom factors are negative, the image is
    /// flipped on the axes.
    ///
    /// # Arguments
    ///
    /// * `src` - The surface to zoom.
    /// * `zoomx` - The horizontal zoom factor.
    /// * `zoomy` - The vertical zoom factor.
    /// * `smooth` - Antialiasing flag; set to SMOOTHING_ON to enable.
    ///
    fn zoom(&self, zoomx: f64, zoomy: f64, smooth: bool) -> crate::Result<Surface>;

    /// Shrink a surface by an integer ratio using averaging.
    ///
    /// Shrinks a 32bit or 8bit 'src' surface to a newly created 'dst' surface.
    /// 'factorx' and 'factory' are the shrinking ratios (i.e. 2=1/2 the size,
    /// 3=1/3 the size, etc.) The destination surface is antialiased by
    /// averaging the source box RGBA or Y information. If the surface is not
    /// 8bit or 32bit RGBA/ABGR it will be converted into a 32bit RGBA format on
    /// the fly. The input surface is not modified. The output surface is newly
    /// allocated.
    ///
    /// # Arguments
    ///
    /// * `factorx` - The horizontal shrinking ratio.
    /// * `factory` - The vertical shrinking ratio.
    ///
    fn shrink(&self, factorx: i32, factory: i32) -> crate::Result<Surface>;

    /// Rotates a 32 bit surface in increments of 90 degrees.
    ///
    /// Specialized 90 degree rotator which rotates a 'src' surface in 90 degree
    /// increments clockwise returning a new surface. Faster than rotozoomer
    /// since not scanning or interpolation takes place. Input surface must be
    /// 32 bit. (code contributed by J. Schiller, improved by C. Allport and A.
    /// Schiffler)
    ///
    /// # Arguments
    ///
    /// - `numClockwiseTurns` - Number of clockwise 90 degree turns to apply to
    ///   the source.
    ///
    fn rotate_90deg(&self, turns: i32) -> crate::Result<Surface>;
}

impl RotozoomSurface for Surface {
    fn rotozoom(&self, angle: f64, zoom: f64, smooth: bool) -> crate::Result<Surface> {
        let raw = unsafe { rotozoom::rotozoomSurface(self.raw(), angle, zoom, smooth as c_int) };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Surface::new(raw))
        }
    }

    fn rotozoom_xy(
        &self,
        angle: f64,
        zoomx: f64,
        zoomy: f64,
        smooth: bool,
    ) -> crate::Result<Surface> {
        let raw = unsafe {
            rotozoom::rotozoomSurfaceXY(self.raw(), angle, zoomx, zoomy, smooth as c_int)
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Surface::new(raw))
        }
    }

    fn zoom(&self, zoomx: f64, zoomy: f64, smooth: bool) -> crate::Result<Surface> {
        let raw = unsafe { rotozoom::zoomSurface(self.raw(), zoomx, zoomy, smooth as c_int) };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Surface::new(raw))
        }
    }

    fn shrink(&self, factorx: i32, factory: i32) -> crate::Result<Surface> {
        let raw =
            unsafe { rotozoom::shrinkSurface(self.raw(), factorx as c_int, factory as c_int) };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Surface::new(raw))
        }
    }

    fn rotate_90deg(&self, turns: i32) -> crate::Result<Surface> {
        let raw = unsafe { rotozoom::rotateSurface90Degrees(self.raw(), turns as c_int) };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Surface::new(raw))
        }
    }
}

pub fn get_zoom_size(width: i32, height: i32, zoomx: f64, zoomy: f64) -> (i32, i32) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe {
        rotozoom::zoomSurfaceSize(
            width as c_int,
            height as c_int,
            zoomx,
            zoomy,
            &mut w,
            &mut h,
        )
    }
    (w as i32, h as i32)
}

pub fn get_rotozoom_size(width: i32, height: i32, angle: f64, zoom: f64) -> (i32, i32) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe {
        rotozoom::rotozoomSurfaceSize(width as c_int, height as c_int, angle, zoom, &mut w, &mut h)
    }
    (w as i32, h as i32)
}

pub fn get_rotozoom_xy_size(
    width: i32,
    height: i32,
    angle: f64,
    zoomx: f64,
    zoomy: f64,
) -> (i32, i32) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe {
        rotozoom::rotozoomSurfaceSizeXY(
            width as c_int,
            height as c_int,
            angle,
            zoomx,
            zoomy,
            &mut w,
            &mut h,
        )
    }
    (w as i32, h as i32)
}
