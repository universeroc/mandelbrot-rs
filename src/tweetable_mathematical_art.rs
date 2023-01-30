use std::fs::File;
use std::io::Write;

trait TweetableMathematicalArt {
    fn rd(&self, i: u32, j: u32) -> u8;
    fn gr(&self, i: u32, j: u32) -> u8;
    fn bl(&self, i: u32, j: u32) -> u8;
}

#[derive(Debug)]
pub struct Mandelbrot {
    width: usize,
    height: usize,
}

impl Mandelbrot {
    pub fn new(width: usize, height: usize) -> Self {
        Mandelbrot { width, height }
    }

    fn write_pixel(&self, color: &mut [u8; 3], file: &mut std::fs::File, i: u32, j: u32) {
        color[0] = self.rd(i, j) & 255;
        color[1] = self.gr(i, j) & 255;
        color[2] = self.bl(i, j) & 255;
        file.write_all(color).expect("failed to write file");
    }
}

impl Drop for Mandelbrot {
    fn drop(&mut self) {
        // write file
        let name = "mandelbrot";
        let mut file = File::create(format!("{}.ppm", name)).expect("failed to create file!");
        let header = format!("P6\n{} {}\n255\n", self.width, self.height);
        let mut color: [u8; 3] = [0, 0, 0];
        file.write_all(header.as_bytes()).expect(
            "failed to write file",
        );
        for j in 0..self.height {
            for i in 0..self.width {
                self.write_pixel(&mut color, &mut file, i as u32, j as u32);
            }
        }
    }
}

impl TweetableMathematicalArt for Mandelbrot {
    fn rd(&self, i: u32, j: u32) -> u8 {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut n: f64 = 0.0;
        loop {
            let c = a * a;
            let d = b * b;
            if c + d < 4.0 && n < 880.0 {
                n += 1.0;
                b = 2.0 * a * b + j as f64 * 8e-9 - 0.645411;
                a = c - d + i as f64 * 8e-9 + 0.356888;
            } else {
                n += 1.0;
                return (255.0 * ((n - 80.0) / 800.0).powf(3.0)) as u8;
            }
        }
    }


    fn gr(&self, i: u32, j: u32) -> u8 {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut n: f64 = 0.0;
        loop {
            let c = a * a;
            let d = b * b;
            if c + d < 4.0 && n < 880.0 {
                n += 1.0;
                b = 2.0 * a * b + j as f64 * 8e-9 - 0.645411;
                a = c - d + i as f64 * 8e-9 + 0.356888;
            } else {
                n += 1.0;

                return (255.0 * ((n - 80.0) / 800.0).powf(0.7)) as u8;
            }
        }
    }

    fn bl(&self, i: u32, j: u32) -> u8 {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut n: f64 = 0.0;
        loop {
            let c = a * a;
            let d = b * b;
            if c + d < 4.0 && n < 880.0 {
                n += 1.0;
                b = 2.0 * a * b + j as f64 * 8e-9 - 0.645411;
                a = c - d + i as f64 * 8e-9 + 0.356888;
            } else {
                n += 1.0;

                return (255.0 * ((n - 80.0) / 800.0).powf(0.5)) as u8;
            }
        }
    }
}
