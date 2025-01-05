use crate::coordinate;
use crate::graphics::image::Image;
use crate::validate;

impl Image {
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
            Image::draw_diagonal_line(self, color, coords)
        } else if slope == 0.0 {
            Image::draw_horizontal_line(self, color, coords)
        } else if slope == f32::INFINITY {
            Image::draw_vertical_line(self, color, coords)
        } else if slope > 1.0 {
            Image::bresenham_slope_greater_than_1(self, color, coords)
        } else if slope > 0.0 && slope < 1.0 {
            Image::bresenham_general(self, color, coords)
        } else {
            panic!("Caught case in that doesn't fit Bresenham Line Algorithm Implementation. Input was {coords} and slope was {slope}");
        }
    }

    /// Function to calculate the pixels to be rendered in a cartesian plane where both
    /// coordinates are within the space enclosed by the image (origin is at the top left of the
    /// image) and the slope of the line represented by the LineCoordinates provided is:
    /// slope == 0
    pub fn draw_horizontal_line(
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
    pub fn draw_vertical_line(
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
        } = coords.ensure_y_lr();

        // Only y increments
        for y_coord in a.y..=b.y {
            self.set_pixel(coordinate::Coordinate::new(a.x, y_coord), color)?;
        }

        Ok(self)
    }

    /// Function to calculate the pixels to be rendered in a cartesian plane where both
    /// coordinates are within the space enclosed by the image (origin is at the top left of the
    /// image) and the slope of the line represented by the LineCoordinates provided is:
    /// slope == 1
    fn draw_diagonal_line(
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
    use crate::ppm::PPMImage;
    use crate::validate::ValidationError;
    use std::error::Error;

    #[test]
    fn test_big_image() -> Result<(), Box<dyn Error>> {
        let mut image = Image::builder().rows(1080).cols(1920).build()?;
        let checkerboard_image = image.fill(MAGENTA).checkerboard(16, BLACK);

        let _ = PPMImage::builder()
            .image(checkerboard_image)
            .filename("big_image.ppm")
            .build()?
            .write();

        Ok(())
    }

    #[test]
    fn test_fill() -> Result<(), Box<dyn Error>> {
        let mut image = Image::builder().rows(640).cols(640).build()?;
        let yellow_image = image.fill(YELLOW);

        let _ = PPMImage::builder()
            .image(yellow_image)
            .filename("yellow_image.ppm")
            .build()
            .unwrap()
            .write();

        Ok(())
    }

    #[test]
    fn test_line_dda() -> Result<(), Box<dyn Error>> {
        let rows = 512;
        let cols = 512;
        let mut image = Image::builder().rows(rows).cols(cols).build()?;
        let image = image.fill(MAGENTA);

        let image = image
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
            )?;

        let _ = PPMImage::builder()
            .image(image)
            .filename("test_line_dda.ppm")
            .build()
            .unwrap()
            .write();

        Ok(())
    }

    #[test]
    fn test_draw_line_with_oob_coordinate() -> Result<(), Box<dyn Error>> {
        let rows = 32;
        let cols = 32;
        let mut invalid_image = Image::builder().rows(rows).cols(cols).build()?;
        invalid_image.fill(MAGENTA);

        let invalid_image_without_error = invalid_image.clone();

        match invalid_image.draw_line_dda(
            BLACK,
            coordinate::LineCoordinates::new(0, 0, rows as i32, cols as i32),
        ) {
            Err(ValidationError::OutOfBoundsInImageError(coord, image)) => {
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

        Ok(())
    }

    #[test]
    fn test_draw_line_bresenham() -> Result<(), Box<dyn Error>> {
        let rows = 32;
        let cols = 32;
        let mut image = Image::builder().rows(rows).cols(cols).build()?;
        let magenta_image = image.fill(MAGENTA);

        // Line with slope == 1
        magenta_image
            .draw_line_bresenham(
                BLACK,
                coordinate::LineCoordinates::new(0, 0, (rows - 1) as i32, (cols - 1) as i32),
            )?
            // Line with slope == 1, but x1 < x0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new(
                    (rows - 2) as i32,
                    (cols - 2) as i32,
                    (rows / 2) as i32,
                    (cols / 2) as i32,
                ),
            )?
            // Line with slope = infinity (vertical line)
            .draw_line_bresenham(
                LIME,
                coordinate::LineCoordinates::new(1, 1, 1, (cols - 2) as i32),
            )?
            // Line with slope = infinity (vertical line), but y1 < y0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new(1, (cols / 2) as i32, 1, (cols - 2) as i32),
            )?
            // Line with slope = 0 (horizontal line)
            .draw_line_bresenham(
                GRAY,
                coordinate::LineCoordinates::new(1, 1, (rows - 2) as i32, 1),
            )?
            // Line with slope = 0 (horizontal line), but x1 < x0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new((rows / 2) as i32, 1, (rows - 2) as i32, 1),
            )?
            // Line with slope : 0 < slope < 1
            .draw_line_bresenham(
                SILVER,
                coordinate::LineCoordinates::new(1, 1, (rows - 2) as i32, (cols / 2) as i32),
            )?
            // Line with slope : 0 < slope < 1, but x1 < x0
            .draw_line_bresenham(
                OLIVE,
                coordinate::LineCoordinates::new(
                    (rows - 2) as i32,
                    (cols / 2) as i32,
                    (rows / 2) as i32,
                    (cols / 4) as i32,
                ),
            )?
            // Line with slope : 1 < slope
            .draw_line_bresenham(
                CYAN,
                coordinate::LineCoordinates::new(1, 1, (rows / 2) as i32, (cols - 2) as i32),
            )?
            // Line with slope : 1 < slope, but x1 < x0
            .draw_line_bresenham(
                WHITE,
                coordinate::LineCoordinates::new(
                    (rows / 2) as i32,
                    (cols - 2) as i32,
                    (rows / 4) as i32,
                    (cols / 4) as i32,
                ),
            )?;

        let _ = PPMImage::builder()
            .image(magenta_image)
            .filename("test_draw_line_bresenham.ppm")
            .build()
            .unwrap()
            .write();

        Ok(())
    }
}
