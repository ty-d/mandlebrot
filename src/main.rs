use num::complex::Complex;
use raster::Color;
use raster::Image;

fn main() {
    let fi = FractalImage::new(-1.5, 0.0, -1.5, -0.5, 3000.0);

    raster::save(&draw_fractal(fi, 15), "testimage.png")
        .expect("couldn't save");
}

struct FractalImage {
    lower_left: Complex<f64>,
    upper_right: Complex<f64>,
    pixels_per_unit: f64,
}

impl FractalImage {
    pub fn new(x1: f64, x2: f64, y1: f64, y2: f64, ppu: f64) -> Self {
        FractalImage {
            lower_left: Complex::new(x1, y1),
            upper_right: Complex::new(x2, y2),
            pixels_per_unit: ppu,
        }
    }

    pub fn get_x_pixels(&self) -> i32 {
        ((&self.upper_right.re - &self.lower_left.re) * &self.pixels_per_unit) as i32
    }

    pub fn get_x_pos(&self, pixel: i32) -> f64 {
        (pixel as f64 / *&self.pixels_per_unit as f64) + *&self.lower_left.re as f64
    }

    pub fn get_y_pos(&self, pixel: i32) -> f64 {
        (pixel as f64 / *&self.pixels_per_unit as f64) + *&self.lower_left.im as f64
    }

    pub fn get_y_pixels(&self) -> i32 {
        ((&self.upper_right.im - &self.lower_left.im) * &self.pixels_per_unit) as i32
    }
}

fn draw_fractal(fi: FractalImage, max_iter: u64) -> Image {
    let x_pixs = fi.get_x_pixels();
    let y_pixs = fi.get_y_pixels();
    let mut canvas = Image::blank(x_pixs, y_pixs);

    for x in 0..x_pixs {
        for y in 0..y_pixs {
            let c = Complex::new(fi.get_x_pos(x), fi.get_y_pos(y));
            let n = mandlebrot(c, max_iter);
            let hue = (n * 255.0 / (max_iter as f64)) as u16;
            let value;
            if n < max_iter as f64 {
                value = 255;
            } else {
                value = 0;
            }
            let rgb = Color::to_rgb(hue, 255.0, value as f32);
            canvas
                .set_pixel(x, y, Color::rgb(rgb.0, rgb.1, rgb.2))
                .expect("prob pixels out of range");
        }
    }
    canvas
}

fn mandlebrot(c: Complex<f64>, max_iter: u64) -> f64 {
    let mut n = 0;
    let mut z = Complex::new(0.0, 0.0);
    while (z.norm_sqr() as f64).sqrt() < 2.0 && n < max_iter {
        z = z * z + c;
        n += 1;
    }
    if n == max_iter {
        max_iter as f64
    } else {
        (n as f64) + 1.0 - (z.norm_sqr() as f64).sqrt().log2().ln()
    }
}
