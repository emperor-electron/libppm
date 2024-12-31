use crate::colors;
use crate::coordinate;
use crate::validate;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::process;

#[derive(Debug, PartialEq, Clone)]
pub struct PPMImage {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<u32>,
    header: Vec<u8>,
    pub filename: String,
}

impl Default for PPMImage {
    fn default() -> Self {
        let header_data = "P6\n640 640\n255\n".as_bytes();
        let mut magenta_image = Vec::new();

        for _ in 0..(640 * 640) {
            magenta_image.push(colors::MAGENTA);
        }

        PPMImage {
            rows: 640,
            cols: 640,
            data: magenta_image,
            header: Vec::from(header_data),
            filename: String::from("output.ppm"),
        }
    }
}

impl Display for PPMImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PPMImage(rows={}, cols={}, data.len()={}, filename={})",
            self.rows,
            self.cols,
            self.data.len(),
            self.filename
        )
    }
}

impl PPMImage {
    pub fn new() -> PPMImageBuilder {
        PPMImageBuilder::new()
    }

    pub fn from_dims(rows: usize, cols: usize) -> Self {
        let header = format!("P6\n{cols} {rows}\n255\n");
        let header_data = header.as_bytes();

        let mut magenta_image = Vec::new();
        for _ in 0..(rows * cols) {
            magenta_image.push(colors::MAGENTA);
        }

        let image = PPMImage {
            rows,
            cols,
            data: magenta_image,
            header: Vec::from(header_data),
            filename: String::from("output.ppm"),
        };

        image
    }

    pub fn from_dims_and_pixel_data(rows: usize, cols: usize, pixel_data: &Vec<u32>) -> Self {
        let mut image = PPMImage::from_dims(rows, cols);
        pixel_data.iter().for_each(|pixel| image.data.push(*pixel));
        image
    }

    /// Writes image to file - will panic if there is not enough data. Calculations are based on the
    /// cols & rows PPMImage struct member values.
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        if let Err(e) = validate::pixel_data_length(self) {
            return Err(Box::new(e));
        }

        let mut fh = fs::File::create(&self.filename)?;
        let mut buffer: Vec<u8> = Vec::new();

        // Push header data into write buffer
        self.header.iter().for_each(|byte| buffer.push(*byte));

        // Push pixel data into write buffer
        for pixel in self.data.iter() {
            // RGB - 0x00_RR_GG_BB
            buffer.push(((pixel >> 8 * 2) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 1) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 0) & 0xFF) as u8);
        }

        let _ = fh.write(&buffer);
        Ok(())
    }

    /// Populates PPM Image with checkboard pattern
    pub fn checkerboard(&self, tile_size: usize, tile_color: u32) -> Self {
        assert!(self.rows != 0 && self.cols != 0);

        let mut checkboard_data: Vec<u32> = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let pixel_index = row * self.cols + col;
                if (row / tile_size + col / tile_size) % 2 == 0 {
                    let pixel = self.data[pixel_index];
                    checkboard_data.push(pixel);
                } else {
                    checkboard_data.push(tile_color);
                }
            }
        }

        PPMImage {
            data: checkboard_data,
            header: self.header.clone(),
            filename: self.filename.clone(),
            ..*self
        }
    }

    /// Fill an image with a given input color - expects the rows and cols members to be set to
    /// valid values.
    pub fn fill(&self, color: u32) -> Self {
        let fill_data = vec![color; self.rows * self.cols];
        PPMImage {
            data: fill_data,
            filename: self.filename.clone(),
            header: self.header.clone(),
            ..*self
        }
    }

    pub fn triangle(&self, _color: u32, coords: coordinate::TriangleCoordinates) -> Self {
        if let Err(e) = validate::triangle_coordinates(&self, &coords) {
            eprintln!("ERROR: {e}");
            process::exit(1);
        }

        let triangle_data = Vec::new();

        PPMImage {
            data: triangle_data,
            filename: self.filename.clone(),
            header: self.header.clone(),
            ..*self
        }
    }

    /// Renders a line using the Digital Differential Analyzer algorithm.
    pub fn draw_line_dda(
        &mut self,
        color: u32,
        coords: coordinate::LineCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        if let Err(e) = validate::line_coordinates(&self, &coords) {
            return Err(e);
        }

        let coordinate::LineCoordinates(a, b) = coords;

        let (dx, dy) = a.delta_wrt(&b);
        let mut x: f32 = a.x as f32;
        let mut y: f32 = a.y as f32;

        let steps = dx.abs().max(dy.abs());

        let x_increment: f32 = (dx as f32) / (steps as f32);
        let y_increment: f32 = (dy as f32) / (steps as f32);

        for _ in 0..steps {
            let coord = coordinate::Coordinate {
                x: x as i32,
                y: y as i32,
            };
            self.set_pixel(coord, color)?;
            x += x_increment;
            y += y_increment;
        }

        Ok(self)
    }

    /// Renders a line using Bresenham's Line Algorithm.
    pub fn draw_line_bresenham(
        &mut self,
        color: u32,
        coords: coordinate::LineCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        if let Err(e) = validate::line_coordinates(&self, &coords) {
            return Err(e);
        }

        let slope = coords.slope();

        // TODO : Create 3 private functions:
        //   bresenham_normal - normal calculation, slope is 0..=1
        //   bresenham_lt0    - slope less than 0
        //   bresenham_gt1    - slope greater than 1
        if slope < 0.0 {
            // TODO : If negative slope, get a line with positive slope by reflecting line about
            // x-axis. Perform algorithm per usual & then reflect line back around the x-axis.
            todo!("Calculation for line drawing for lines with negative slope not implemented")
        } else if slope > 1.0 {
            // TODO : If slope greater than 1, swap the x,y coordinates to be y,x which will give a
            // line with slope less than 1. Perform the algorithm per usual & then swap the output
            // pixel locations back from y,x -> x,y to get the line.
            todo!(
                "Calculation for line drawing for lines with slope greater than 1 not implemented"
            )
        }

        let coordinate::LineCoordinates(a, b) = coords;

        let (dx, dy) = a.delta_wrt(&b);
        let mut d = 2 * dy - dx;
        let mut y = a.y;

        println!("dy={dy}, dx={dx}");

        for x in a.x..=b.x {
            self.set_pixel(coordinate::Coordinate::new(x, y), color)?;
            println!("d={d}, x={x}, y={y}");
            if d > 0 {
                d = d + (2 * dy - 2 * dx);
                y += 1;
            } else {
                d = d + 2 * dy;
            }
        }

        Ok(self)
    }

    pub fn set_pixel(
        &mut self,
        coord: coordinate::Coordinate,
        color: u32,
    ) -> Result<(), validate::ValidationError> {
        if let Err(e) = validate::coordinate(&self, &coord) {
            return Err(e);
        }

        let pixel_index = (coord.x as usize) * self.rows + (coord.y as usize);

        self.data[pixel_index] = color;

        Ok(())
    }

    pub fn get_pixel(
        &self,
        coord: coordinate::Coordinate,
    ) -> Result<u32, validate::ValidationError> {
        if let Err(e) = validate::coordinate(&self, &coord) {
            return Err(e);
        }

        let pixel_index = (coord.x as usize) * self.rows + (coord.y as usize);

        Ok(self.data[pixel_index])
    }
} /* PPMImage */

#[derive(Default)]
pub struct PPMImageBuilder {
    rows: Option<usize>,
    cols: Option<usize>,
    filename: Option<String>,
}

impl PPMImageBuilder {
    pub fn new() -> Self {
        PPMImageBuilder {
            rows: None,
            cols: None,
            filename: None,
        }
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn cols(mut self, cols: usize) -> Self {
        self.cols = Some(cols);
        self
    }

    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }

    pub fn build(self) -> Result<PPMImage, String> {
        if let None = self.rows {
            return Err("Rows member must be specified when building a PPMImage.".to_string());
        } else if let None = self.cols {
            return Err("Cols member must be specified when building a PPMImage.".to_string());
        } else if let None = self.filename {
            return Err("Filename member must be specified when building a PPMImage.".to_string());
        } else {
            Ok(PPMImage {
                rows: self.rows.unwrap(),
                cols: self.cols.unwrap(),
                filename: self.filename.unwrap().clone(),
                header: format!("P6\n{} {}\n255\n", self.cols.unwrap(), self.rows.unwrap())
                    .as_bytes()
                    .to_vec(),
                data: vec![],
            }
            .fill(colors::MAGENTA))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::colors::BLACK;
    use crate::colors::MAGENTA;
    use crate::colors::SILVER;
    use crate::colors::YELLOW;

    use crate::validate::ValidationError;

    // TODO : Write tests for the write() method errors

    #[test]
    fn test_from_dimensions() {
        let image_a = PPMImage::from_dims(640, 640);
        let image_b = PPMImage::new()
            .rows(640)
            .cols(640)
            .filename("output.ppm")
            .build()
            .unwrap();
        assert_eq!(image_a, image_b);
    }

    #[test]
    fn test_big_image() {
        let image = PPMImage::new()
            .rows(1080)
            .cols(1920)
            .filename("test_big.ppm")
            .build()
            .unwrap()
            .fill(MAGENTA)
            .checkerboard(16, BLACK);
        let _ = image.write();
    }

    #[test]
    fn test_write() {
        let image = PPMImage::new()
            .rows(640)
            .cols(640)
            .filename("test_write.ppm")
            .build()
            .unwrap();
        let _ = image.write();
    }

    #[test]
    fn test_checkboard() {
        let image = PPMImage::new()
            .rows(640)
            .cols(640)
            .filename("test_checkboard.ppm")
            .build()
            .unwrap()
            .checkerboard(32, BLACK);
        let _ = image.write();
    }

    #[test]
    fn test_fill() {
        let image = PPMImage::new()
            .rows(640)
            .cols(640)
            .filename("test_fill.ppm")
            .build()
            .unwrap()
            .fill(YELLOW);
        let _ = image.write();
    }

    #[test]
    fn test_line_dda() -> Result<(), ValidationError> {
        let rows = 512;
        let cols = 512;
        let mut image = PPMImage::new()
            .rows(rows)
            .cols(cols)
            .filename("test_line_dda.ppm")
            .build()
            .unwrap()
            .fill(MAGENTA);

        let _ = match image
            .draw_line_dda(
                BLACK,
                coordinate::LineCoordinates(
                    coordinate::Coordinate { x: 0, y: 0 },
                    coordinate::Coordinate {
                        x: rows as i32 - 1,
                        y: cols as i32 - 1,
                    },
                ),
            )?
            .draw_line_dda(
                BLACK,
                coordinate::LineCoordinates(
                    coordinate::Coordinate {
                        x: rows as i32 - 1,
                        y: cols as i32 - 1,
                    },
                    coordinate::Coordinate { x: 37, y: 128 },
                ),
            )?
            .draw_line_dda(
                BLACK,
                coordinate::LineCoordinates(
                    coordinate::Coordinate {
                        x: 0,
                        y: cols as i32 / 2,
                    },
                    coordinate::Coordinate {
                        x: rows as i32 / 2,
                        y: cols as i32 / 2,
                    },
                ),
            ) {
            Err(e) => panic!("{e}"),
            Ok(image) => image.write(),
        };

        Ok(())
    }

    #[test]
    fn test_draw_line_with_oob_coordinate() {
        let rows = 32;
        let cols = 32;
        let mut invalid_image = PPMImage::new()
            .rows(rows)
            .cols(cols)
            .filename("test_line_naive.ppm")
            .build()
            .unwrap()
            .fill(MAGENTA);

        let invalid_image_without_error = invalid_image.clone();

        match invalid_image.draw_line_dda(
            BLACK,
            coordinate::LineCoordinates::new(0, 0, rows as i32, cols as i32),
        ) {
            Err(ValidationError::OutOfBoundsError(coord, image)) => {
                assert_eq!(
                    coord,
                    coordinate::Coordinate {
                        x: rows as i32,
                        y: cols as i32,
                    },
                );

                assert_eq!(image, invalid_image_without_error);
            }
            _ => panic!("Expected to get an error."),
        }
    }

    #[test]
    fn test_get_pixel() {
        let image = PPMImage::new()
            .rows(64)
            .cols(64)
            .filename("output.ppm")
            .build()
            .unwrap()
            .fill(YELLOW);

        match image.get_pixel(coordinate::Coordinate { x: 0, y: 0 }) {
            Ok(pixel_value) => assert_eq!(pixel_value, colors::YELLOW),
            _ => panic!("Value should be ok."),
        };
    }

    #[test]
    fn test_draw_line_bresenham() {
        let rows = 480;
        let cols = 640;
        let mut image = PPMImage::new()
            .rows(rows)
            .cols(cols)
            .filename("test_line_bresenham.ppm")
            .build()
            .unwrap()
            .fill(SILVER);

        let _ = match image.draw_line_bresenham(
            YELLOW,
            coordinate::LineCoordinates::new(0, 0, (rows - 1) as i32, (cols - 1) as i32),
        ) {
            Err(e) => panic!("{e}"),
            Ok(image) => image.write(),
        };
    }
}
