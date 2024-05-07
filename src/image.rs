use std::fs::File;
use std::io::{self, Write};

#[derive(Clone)]
pub struct PPMPixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl PPMPixel {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    
    fn to_u8(&self, value: f64) -> u8 {
        // Clamping the value to ensure it falls within the range [0, 255]
        (255.999 * value).clamp(0.0, 255.0) as u8
    }

    fn to_ir(&self) -> u8 {
        self.to_u8(self.r)
    }

    fn to_ig(&self) -> u8 {
        self.to_u8(self.g)
    }

    fn to_ib(&self) -> u8 {
        self.to_u8(self.b)
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.to_ir(), self.to_ig(), self.to_ib())
    }
}

pub struct PPMImage {
    width: usize,
    height: usize,
    pub pixels: Vec<Vec<PPMPixel>>,
}

impl PPMImage {
    pub fn new(width:usize, height:usize) -> Self {
        let pixels = vec![vec![PPMPixel::new(0.0, 0.0, 0.0); width]; height];
        Self { width, height, pixels }
    }

    pub fn write_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        for row in &self.pixels {
            for elem in row {
                writeln!(file, "{}", elem.to_string())?;
            }
        }

        Ok(())
    }
}
