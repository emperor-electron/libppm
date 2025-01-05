use crate::{
    coordinate::{CircleCoordinates, Coordinate, LineCoordinates},
    graphics::image::Image,
    validate,
};

impl Image {
    /// Draws a circle using the Midpoint-Circle Algorithm.
    pub fn draw_circle(
        &mut self,
        color: u32,
        coords: CircleCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        validate::circle_coordinates(self, &coords)?;

        let mut x = 0;
        let mut y = -(coords.radius as i32);

        while x < -y {
            let y_midpoint = (y as f32) + 0.5;

            if (x * x) as f32 + y_midpoint * y_midpoint > (coords.radius * coords.radius) as f32 {
                y += 1;
            }

            self.set_pixel(
                Coordinate::new(coords.center.x + x, coords.center.y + y),
                color,
            )?;
            self.set_pixel(
                Coordinate::new(coords.center.x - x, coords.center.y + y),
                color,
            )?;
            self.set_pixel(
                Coordinate::new(coords.center.x + x, coords.center.y - y),
                color,
            )?;
            self.set_pixel(
                Coordinate::new(coords.center.x - x, coords.center.y - y),
                color,
            )?;
            self.set_pixel(
                Coordinate::new(coords.center.x + y, coords.center.y + x),
                color,
            )?;
            self.set_pixel(
                Coordinate::new(coords.center.x + y, coords.center.y - x),
                color,
            )?;
            self.set_pixel(
                Coordinate::new(coords.center.x - y, coords.center.y + x),
                color,
            )?;
            self.set_pixel(
                Coordinate::new(coords.center.x - y, coords.center.y - x),
                color,
            )?;

            x += 1;
        }

        Ok(self)
    }

    /// Draws a circle using the Midpoint-Circle Algorithm & then fills in the resulting circle
    /// with the provided color.
    pub fn draw_filled_circle(
        &mut self,
        color: u32,
        coords: CircleCoordinates,
    ) -> Result<&mut Self, validate::ValidationError> {
        // Circle will be validated inside of draw_circle function
        self.draw_circle(color, coords)?;

        for row in 0..(*self.get_rows() as i32) {
            let mut horizontal_line_coords = LineCoordinates::new(0, 0, 0, 0);

            for col in 0..(*self.get_cols() as i32) {
                let maybe_circle_border = Coordinate { x: row, y: col };

                if self.get_pixel(maybe_circle_border)? == color
                    && horizontal_line_coords.first.y == horizontal_line_coords.second.y
                {
                    horizontal_line_coords.first = Coordinate { x: row, y: col };
                } else if self.get_pixel(maybe_circle_border)? == color {
                    horizontal_line_coords.second = Coordinate { x: row, y: col };
                }
            }

            if horizontal_line_coords != LineCoordinates::new(0, 0, 0, 0) {
                self.draw_vertical_line(color, horizontal_line_coords)?;
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::{BLACK, MAGENTA, RED, WHITE};
    use crate::coordinate::CircleCoordinates;
    use crate::ppm::PPMImage;
    use std::error::Error;

    #[test]
    fn test_draw_circle() -> Result<(), Box<dyn Error>> {
        let mut image = Image::builder().rows(512).cols(512).build()?;
        image.fill(MAGENTA).draw_circle(
            BLACK,
            CircleCoordinates {
                center: Coordinate::new(256, 256),
                radius: 100,
            },
        )?;

        let _ = PPMImage::builder()
            .image(&image)
            .filename("test_draw_circle.ppm")
            .build()?
            .write();

        Ok(())
    }

    #[test]
    fn test_draw_japanese_flag() -> Result<(), Box<dyn Error>> {
        // Dimensions from https://www.japan.go.jp/japan/flagandanthem/index.html
        // flag height : flag width = 2:3
        // circle diameter : 3/5 of flags' height
        let unit_length = 512;
        let radius: f32 = ((3.0 / 5.0) * (unit_length * 2) as f32) / 2.0;

        let flag_height = unit_length * 2;
        let flag_width = unit_length * 3;

        let mut image = Image::builder()
            .rows(flag_height)
            .cols(flag_width)
            .build()?;
        image.fill(WHITE).draw_filled_circle(
            RED,
            CircleCoordinates {
                center: Coordinate::new(flag_height as i32 / 2, flag_width as i32 / 2),
                radius: radius as u32,
            },
        )?;

        let _ = PPMImage::builder()
            .image(&image)
            .filename("japanese_flag.ppm")
            .build()?
            .write();

        Ok(())
    }
}
