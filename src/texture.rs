use std::sync::Arc;

use image::RgbImage;

use crate::vec3::{Color, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexCoord {
    pub u: f64,
    pub v: f64,
}

impl TexCoord {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}

pub trait Texture: Send + Sync {
    fn sample(&self, uv: TexCoord, point: Vec3) -> Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn sample(&self, _uv: TexCoord, _point: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even_texture: Arc<dyn Texture>,
    odd_texture: Arc<dyn Texture>,
}

impl CheckerTexture {
    #[allow(unused)]
    pub fn new(scale: f64, even_texture: Arc<dyn Texture>, odd_texture: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even_texture,
            odd_texture,
        }
    }

    pub fn from_colors(scale: f64, even_color: Color, odd_color: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even_texture: Arc::new(SolidColor { color: even_color }),
            odd_texture: Arc::new(SolidColor { color: odd_color }),
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, uv: TexCoord, point: Vec3) -> Color {
        let x_int = (self.inv_scale * point.x()).floor() as i64;
        let y_int = (self.inv_scale * point.y()).floor() as i64;
        let z_int = (self.inv_scale * point.z()).floor() as i64;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even_texture.sample(uv, point)
        } else {
            self.odd_texture.sample(uv, point)
        }
    }
}

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> Self {
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn sample(&self, uv: TexCoord, _point: Vec3) -> Color {
        let u = uv.u.clamp(0.0, 1.0);
        let v = 1.0 - uv.v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let rgb = self.image.get_pixel(i, j).0;

        let color_scale = 1.0 / 255.0;
        Color::new(rgb[0] as f64, rgb[1] as f64, rgb[2] as f64) * color_scale
    }
}
