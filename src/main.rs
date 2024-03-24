use std::{
    fmt::Debug,
    ops::{self, Add, Div, Mul, Sub},
};

struct Image {
    width: usize,
    height: usize,
    data: Vec<i32>,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    fn set_color(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        assert!(
            x < self.width,
            "Tried setting a color in an invalid pixel."
        );
        assert!(
            y < self.height,
            "Tried setting a color in an invalid pixel."
        );

        self.data[y * self.width + x] = (r as i32) << 16 | (g as i32) << 8 | (b as i32);
    }

    fn save(&self, filename: &str) {
        let mut output = format!("P3\n#fractal output\n{} {}\n255\n", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.data[y * self.width + x];
                let r: u8 = (pixel >> 16) as u8;
                let g: u8 = (pixel >> 8) as u8;
                let b: u8 = pixel as u8;
                output.push_str(format!("{r} {g} {b}\n").as_str());
            }
        }

        std::fs::write(format!("{filename}.ppm"), output).expect("Unable to write.");
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    fn abs(&self) -> f64 {
        self.re.powi(2).add(self.im.powi(2)).sqrt()
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Self::Output::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Self::Output::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        let a = self.re;
        let b = self.im;
        let c = rhs.re;
        let d = rhs.im;
        Self::Output::new(a * c - b * d, a * d + b * c)
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.re).field(&self.im).finish()
    }
}

fn generate(
    from_x: f64,
    to_x: f64,
    from_y: f64,
    to_y: f64,
    width: usize,
    height: usize,
    filename: &str,
) {
    const MAX_ITERATIONS: usize = 255 + 255 + 255;

    let mut img: Image = Image::new(width, height);

    print!("0%");
    for x in 0..width {
        for y in 0..height {
            let mut iterations = 0;

            let a = map(x as f64, 0.0, width as f64, from_x, to_x);
            let b = map(y as f64, 0.0, height as f64, from_y, to_y);
            let c = Complex::new(-0.4f64, 0.5868f64);
            // let mut z = Complex::new(0f64, 0f64);
            let mut z = Complex::new(a, b);

            while z.abs() <= 2_f64 && iterations < MAX_ITERATIONS {
                z = z * z + c;
                iterations += 1;
            }

            let mut r = 0u8;
            let mut g = 0u8;
            let mut b = 0u8;
            if iterations <= 255 {
                r = iterations as u8;
            } else if iterations <= 255 + 255 {
                r = 255;
                g = (iterations - 255) as u8;
            } else {
                r = 255;
                g = 255;
                b = (iterations - 255 - 255) as u8;
            }
            // let r = map((iterations) as f64, 0f64, 255 as f64, 0f64, 255f64) as u8;
            // let g = map((iterations) as f64, 0f64, 255 as f64, 0f64, 255f64) as u8;
            img.set_color(x, y, r, g, b);
        }
        print!("\r{}%", (100.0 * x as f64) as usize / width);
    }
    print!("\rSaving to '{filename}'...");

    img.save(filename);
    println!("\rSaved to '{filename}'.   ");
}

// https://stackoverflow.com/a/5732390
fn map(v: f64, from_a: f64, from_b: f64, to_a: f64, to_b: f64) -> f64 {
    // output = output_start + ((output_end - output_start) / (input_end - input_start)) * (input - input_start)
    to_a.add(to_b.sub(to_a).div(from_b.sub(from_a)).mul(v.sub(from_a)))
}

fn main() {
    generate(-2f64, 2f64, -2f64, 2f64, 1 << 10, 1 << 10, "test1");
}

#[test]
fn test_image() {
    let width: usize = 100;
    let height: usize = 80;
    let mut img = Image::new(width, height);
    img.set_color(15, 50, 255, 0, 127);

    assert!(img.width == width, "THe width is different than expected.");
    assert!(img.height == height, "The height is different than expected.");
    assert!(
        img.data[50 * width + 15] == 16711807,
        "A wrong color has been set."
    );
}

#[test]
fn test_complex() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let c: f64 = -1.0;
    let d: f64 = 3.0;

    let z1: Complex = Complex { re: a, im: b };
    let z2: Complex = Complex::new(c, d);

    assert!(z1 + z2 == Complex::new(a + c, b + d));
    assert!(z1 - z2 == Complex::new(a - c, b - d));
    assert!(z1 * z2 == Complex::new(a * c - b * d, a * d + b * c));
}
