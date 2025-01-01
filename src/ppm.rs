use crate::colors;
use crate::coordinate;
use crate::validate;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::Write;

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
    ///
    /// TODO : Don't return a new instance - modify existing instance
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
    ///
    /// TODO : Don't return a new instance - modify existing instance
    pub fn fill(&self, color: u32) -> Self {
        let fill_data = vec![color; self.rows * self.cols];
        PPMImage {
            data: fill_data,
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

        let coordinate::LineCoordinates {
            first: a,
            second: b,
        } = coords;

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

        let slope = coords.slope().abs();

        if slope == 1.0 {
            PPMImage::draw_diagonal_line(self, color, coords)
        } else if slope == 0.0 {
            PPMImage::draw_horizonal_line(self, color, coords)
        } else if slope == f32::INFINITY {
            PPMImage::draw_vertical_line(self, color, coords)
        } else if slope > 1.0 {
            PPMImage::bresenham_slope_greater_than_1(self, color, coords)
        } else if slope > 0.0 && slope < 1.0 {
            PPMImage::bresenham_general(self, color, coords)
        } else {
            panic!("Caught case in that doesn't fit Bresenham Line Algorithm Implementation. Input was {coords} and slope was {slope}");
        }
    }

    /// Private function to calculate the pixels to be rendered in a cartesian plane where both
    /// coordinates are within the space enclosed by the image (origin is at the top left of the
    /// image) and the slope of the line represented by the LineCoordinates provided is:
    /// slope == 0
    fn draw_horizonal_line(
        &mut self,
        color: u32,
        coords: coordinate::LineCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        // Assume that this function was called from draw_line_bresenham & coordinates have alredy
        // been validated.
        let coordinate::LineCoordinates {
            first: a,
            second: b,
        } = coords.ensure_x_lr();

        // Only x increments
        for x_coord in a.x..=b.x {
            self.set_pixel(coordinate::Coordinate::new(x_coord, a.y), color)?;
        }

        Ok(self)
    }

    /// Private function to calculate the pixels to be rendered in a cartesian plane where both
    /// coordinates are within the space enclosed by the image (origin is at the top left of the
    /// image) and the slope of the line represented by the LineCoordinates provided is:
    /// slope == INFINITY
    fn draw_vertical_line(
        &mut self,
        color: u32,
        coords: coordinate::LineCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        // Assume that this function was called from draw_line_bresenham & coordinates have alredy
        // been validated.
        let coordinate::LineCoordinates {
            first: a,
            second: b,
        } = coords.ensure_y_lr();

        // Only y increments
        for y_coord in a.y..=b.y {
            self.set_pixel(coordinate::Coordinate::new(a.x, y_coord), color)?;
        }

        Ok(self)
    }

    /// Private function to calculate the pixels to be rendered in a cartesian plane where both
    /// coordinates are within the space enclosed by the image (origin is at the top left of the
    /// image) and the slope of the line represented by the LineCoordinates provided is:
    /// slope == 1
    fn draw_diagonal_line(
        &mut self,
        color: u32,
        coords: coordinate::LineCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        // Assume that this function was called from draw_line_bresenham & coordinates have alredy
        // been validated.
        let coordinate::LineCoordinates {
            first: a,
            second: b,
        } = coords.ensure_x_lr();

        // Should be lesser x of the two
        let mut point = coordinate::Coordinate::new(a.x, a.y);

        // x & y increment together
        for _ in a.y..=b.y {
            self.set_pixel(point, color)?;
            point.x += 1;
            point.y += 1;
        }

        Ok(self)
    }

    /// Private function to calculate the pixels to be rendered in a cartesian plane where both
    /// coordinates are within the space enclosed by the image (origin is at the top left of the
    /// image) and the slope of the line represented by the LineCoordinates provided is:
    /// 1 < slope
    fn bresenham_slope_greater_than_1(
        &mut self,
        color: u32,
        coords: coordinate::LineCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        // Assume that this function was called from draw_line_bresenham & coordinates have alredy
        // been validated.
        let coordinate::LineCoordinates {
            first: a,
            second: b,
        } = coords.ensure_y_lr();

        let (dx, dy) = a.delta_wrt(&b);
        let mut d = 2 * dx - dy;
        let mut x = a.x;

        for y in a.y..=b.y {
            let coord = coordinate::Coordinate::new(x, y);
            self.set_pixel(coord, color)?;
            if d > 0 {
                d = d + (2 * dx - 2 * dy);
                x += 1;
            } else {
                d = d + 2 * dx;
            }
        }

        Ok(self)
    }

    /// Private function to calculate the pixels to be rendered in a cartesian plane where both
    /// coordinates are within the space enclosed by the image (origin is at the top left of the
    /// image) and the slope of the line represented by the LineCoordinates provided is:
    /// 0 < slope < 1
    fn bresenham_general(
        &mut self,
        color: u32,
        coords: coordinate::LineCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        // Assume that this function was called from draw_line_bresenham & coordinates have alredy
        // been validated.
        let coordinate::LineCoordinates {
            first: a,
            second: b,
        } = coords.ensure_x_lr();

        let (dx, dy) = a.delta_wrt(&b);
        let mut d = 2 * dy - dx;
        let mut y = a.y;

        for x in a.x..=b.x {
            self.set_pixel(coordinate::Coordinate::new(x, y), color)?;
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

    /// Draw Circle with the Midpoint Circle Algorithm
    pub fn draw_circle_mca(
        &mut self,
        color: u32,
        coords: coordinate::CircleCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        // TODO: Validation of Circle Coordinates

        let x = 0;
        let y = coords.radius;

        let mut p = (5 / 4) as f32 - coords.radius as f32;

        todo!()
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
    use crate::colors::CYAN;
    use crate::colors::GRAY;
    use crate::colors::LIME;
    use crate::colors::MAGENTA;
    use crate::colors::OLIVE;
    use crate::colors::SILVER;
    use crate::colors::WHITE;
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
                coordinate::LineCoordinates::new(0, 0, rows as i32 - 1, cols as i32 - 1),
            )?
            .draw_line_dda(
                BLACK,
                coordinate::LineCoordinates::new(rows as i32 - 1, cols as i32 - 1, 37, 128),
            )?
            .draw_line_dda(
                BLACK,
                coordinate::LineCoordinates::new(
                    0,
                    cols as i32 / 2,
                    rows as i32 / 2,
                    cols as i32 / 2,
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
        let rows = 32;
        let cols = 32;
        let mut image = PPMImage::new()
            .rows(rows)
            .cols(cols)
            .filename("test_line_bresenham.ppm")
            .build()
            .unwrap()
            .fill(MAGENTA);

        // Line with slope == 1
        image
            .draw_line_bresenham(
                BLACK,
                coordinate::LineCoordinates::new(0, 0, (rows - 1) as i32, (cols - 1) as i32),
            )
            .unwrap()
            // Line with slope == 1, but x1 < x0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new(
                    (rows - 2) as i32,
                    (cols - 2) as i32,
                    (rows / 2) as i32,
                    (cols / 2) as i32,
                ),
            )
            .unwrap()
            // Line with slope = infinity (vertical line)
            .draw_line_bresenham(
                LIME,
                coordinate::LineCoordinates::new(1, 1, 1, (cols - 2) as i32),
            )
            .unwrap()
            // Line with slope = infinity (vertical line), but y1 < y0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new(1, (cols / 2) as i32, 1, (cols - 2) as i32),
            )
            .unwrap()
            // Line with slope = 0 (horizontal line)
            .draw_line_bresenham(
                GRAY,
                coordinate::LineCoordinates::new(1, 1, (rows - 2) as i32, 1),
            )
            .unwrap()
            // Line with slope = 0 (horizontal line), but x1 < x0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new((rows / 2) as i32, 1, (rows - 2) as i32, 1),
            )
            .unwrap()
            // Line with slope : 0 < slope < 1
            .draw_line_bresenham(
                SILVER,
                coordinate::LineCoordinates::new(1, 1, (rows - 2) as i32, (cols / 2) as i32),
            )
            .unwrap()
            // Line with slope : 0 < slope < 1, but x1 < x0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new(
                    (rows - 2) as i32,
                    (cols / 2) as i32,
                    (rows / 2) as i32,
                    (cols / 4) as i32,
                ),
            )
            .unwrap()
            // Line with slope : 1 < slope
            .draw_line_bresenham(
                CYAN,
                coordinate::LineCoordinates::new(1, 1, (rows / 2) as i32, (cols - 2) as i32),
            )
            .unwrap()
            // Line with slope : 1 < slope, but x1 < x0
            .draw_line_bresenham(
                WHITE,
                coordinate::LineCoordinates::new(
                    (rows / 2) as i32,
                    (cols - 2) as i32,
                    (rows / 4) as i32,
                    (cols / 4) as i32,
                ),
            )
            .unwrap()
            .write()
            .unwrap();
    }
}
