pub mod ppm {
    use std::error::Error;
    use std::fmt::Display;
    use std::fs;
    use std::io::Write;
    use std::process;

    // 0x00_RR_GG_BB
    pub const GREEN: u32 = 0x00_00_00_FF;
    pub const BLUE: u32 = 0x00_00_FF_00;
    pub const RED: u32 = 0x00_FF_00_00;

    pub const MAGENTA: u32 = 0x00_FF_00_FF;
    pub const CYAN: u32 = 0x00_00_FF_FF;
    pub const YELLOW: u32 = 0x00_FF_FF_00;

    pub const BLACK: u32 = 0x00_00_00_00;
    pub const WHITE: u32 = 0x00_FF_FF_FF;

    #[derive(Debug, PartialEq)]
    pub struct PPMImage {
        pub rows: usize,
        pub cols: usize,
        pub data: Vec<u32>,
        pub header: Vec<u8>,
        pub filename: String,
    }

    #[derive(Debug, PartialEq)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }

    pub struct LineCoordinates(pub Coordinate, pub Coordinate);
    pub struct TriangleCoordinates(pub Coordinate, pub Coordinate, pub Coordinate);

    impl Default for PPMImage {
        fn default() -> Self {
            let header_data = "P6\n640 640\n255\n".as_bytes();
            let mut magenta_image = Vec::new();

            for _ in 0..(640 * 640) {
                magenta_image.push(MAGENTA);
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

    impl PPMImage {
        pub fn new() -> Self {
            PPMImage::default()
        }

        pub fn from_dims(rows: usize, cols: usize) -> Self {
            let header = format!("P6\n{rows} {cols}\n255\n");
            let header_data = header.as_bytes();

            let mut magenta_image = Vec::new();
            for _ in 0..(rows * cols) {
                magenta_image.push(MAGENTA);
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
            assert!(
                self.data.len() == self.cols * self.rows,
                "Not enough data to write into file"
            );

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

        pub fn read() {
            todo!()
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

        pub fn triangle(&self, _color: u32, coords: TriangleCoordinates) -> Self {
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

        /// Renders a line using naive algorithm - only the equation for a line (y = mx + b) is
        /// used.
        pub fn line(&self, color: u32, coords: LineCoordinates) -> Self {
            if let Err(e) = validate::line_coordinates(&self, &coords) {
                eprintln!("ERROR: {e}");
                process::exit(1);
            }

            let line_data = Vec::from(self.data.clone());

            let mut image = PPMImage {
                cols: self.cols,
                rows: self.rows,
                data: line_data,
                filename: self.filename.clone(),
                header: self.header.clone(),
            };

            let LineCoordinates(a, b) = coords;

            // handle case where there is a vertical line
            if a.x == b.x {
                todo!()
            }

            // y = mx+b
            // m = dy/dx
            // b = y-mx
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let m = (dy as f32) / (dx as f32);
            let y_intercept = (a.y as f32) - m * (a.x as f32);

            let lesser_x;
            let greater_x;
            let mut y;

            // In order to iterate in raster-scan fashion
            if a.x > b.x {
                greater_x = a.x;
                lesser_x = b.x;
            } else {
                greater_x = b.x;
                lesser_x = a.x;
            }

            for x_coordinate in lesser_x..greater_x {
                y = m * x_coordinate as f32 + y_intercept;
                let coord = Coordinate {
                    x: x_coordinate,
                    y: y as i32,
                };
                println!(
                    "Coordinate given to set_pixel in line(): {} ; Y-intercept: {} ; m: {}",
                    coord, y_intercept, m
                );
                image.set_pixel(coord, color);
            }

            image
        }

        /// Renders a line using the Digital Differential Analyzer algorithm.
        pub fn line_dda(&self, color: u32, coords: LineCoordinates) -> Self {
            if let Err(e) = validate::line_coordinates(&self, &coords) {
                eprintln!("ERROR: {e}");
                process::exit(1);
            }

            let line_data = Vec::from(self.data.clone());

            let LineCoordinates(a, b) = coords;

            let dx = (b.x - a.x).abs();
            let dy = (b.y - a.y).abs();
            let mut x: f32 = a.x as f32;
            let mut y: f32 = a.y as f32;

            let steps;

            if dx.abs() > dy.abs() {
                steps = dx.abs();
            } else {
                steps = dy.abs();
            }

            let x_increment: f32 = (dx as f32) / (steps as f32);
            let y_increment: f32 = (dy as f32) / (steps as f32);

            let mut image = PPMImage {
                cols: self.cols,
                rows: self.rows,
                data: line_data,
                filename: self.filename.clone(),
                header: self.header.clone(),
            };

            for _ in 0..steps {
                let coord = Coordinate {
                    x: x as i32,
                    y: y as i32,
                };
                image.set_pixel(coord, color);
                x += x_increment;
                y += y_increment;
            }

            image
        }

        fn set_pixel(&mut self, coord: Coordinate, color: u32) {
            if let Err(e) = validate::coordinate(&self, &coord) {
                eprintln!("ERROR: {e}");
                process::exit(1);
            }

            self.data[(coord.x as usize) * self.rows + (coord.y as usize)] = color;
        }

        fn get_pixel(&self, coord: Coordinate) -> u32 {
            self.data[(coord.x as usize) * self.rows + (coord.y as usize)]
        }
    } /* PPMImage */

    impl Display for Coordinate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Coordinate(x:{}, y:{})", self.x, self.y)
        }
    }

    mod validate {
        use super::*;

        /// Validates that given coordinates are within a given image
        pub fn triangle_coordinates<'a>(
            image: &'a PPMImage,
            coords: &'a TriangleCoordinates,
        ) -> Result<(), String> {
            let TriangleCoordinates(coord_a, coord_b, coord_c) = coords;
            coordinate(image, coord_a)?;
            coordinate(image, coord_b)?;
            coordinate(image, coord_c)?;
            Ok(())
        }

        pub fn line_coordinates(image: &PPMImage, coords: &LineCoordinates) -> Result<(), String> {
            let LineCoordinates(coord_a, coord_b) = coords;
            coordinate(image, coord_a)?;
            coordinate(image, coord_b)?;
            Ok(())
        }

        pub fn coordinate(image: &PPMImage, coord: &Coordinate) -> Result<(), String> {
            if coord.x > (image.cols as i32)
                || coord.y > (image.rows as i32)
                || coord.x < 0
                || coord.y < 0
            {
                let err_msg = format!("{} is outside the bounds of the image.", coord);
                return Err(err_msg);
            }
            Ok(())
        }
    } /* validate */
} /* ppm */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dimensions_func() {
        let image_a = ppm::PPMImage::from_dims(640, 640);
        let image_b = ppm::PPMImage::new();
        dbg!(&image_a);
        assert_eq!(image_a, image_b);
    }

    #[test]
    fn test_write_func() {
        let mut image = ppm::PPMImage::new();
        image.filename = String::from("test_write_func.ppm");
        let _ = image.write();
    }

    #[test]
    fn test_checkboard() {
        let mut image = ppm::PPMImage::from_dims(512, 256).checkerboard(32, ppm::BLACK);
        image.filename = String::from("test_checkboard.ppm");
        let _ = image.write();
    }

    #[test]
    fn test_fill() {
        let mut image = ppm::PPMImage::new().fill(ppm::YELLOW);
        image.filename = String::from("test_fill.ppm");
        let _ = image.write();
    }

    #[test]
    fn test_line_dda() {
        let mut image = ppm::PPMImage::from_dims(32, 32)
            .fill(ppm::MAGENTA)
            .line_dda(
                ppm::BLACK,
                ppm::LineCoordinates(
                    ppm::Coordinate { x: 0, y: 0 },
                    ppm::Coordinate { x: 32, y: 32 },
                ),
            );
        image.filename = String::from("test_line_dda.ppm");
        let _ = image.write();
        dbg!(image);
    }

    #[test]
    fn test_line() {
        let mut image = ppm::PPMImage::from_dims(32, 32)
            .fill(ppm::MAGENTA)
            .line(
                ppm::BLACK,
                ppm::LineCoordinates(
                    ppm::Coordinate { x: 0, y: 0 },
                    ppm::Coordinate { x: 32, y: 32 },
                ),
            )
            .line(
                ppm::BLACK,
                ppm::LineCoordinates(
                    ppm::Coordinate { x: 0, y: 32 },
                    ppm::Coordinate { x: 32, y: 0 },
                ),
            );
        image.filename = String::from("test_line.ppm");
        let _ = image.write();
        dbg!(image);
    }
}
