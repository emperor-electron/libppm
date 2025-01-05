use crate::colors::BLACK;
use crate::coordinate::Coordinate;
use crate::validate;
use std::error::Error;
use std::fmt::Display;

/// General form of an image
///
/// TODO : Is there a generic that allows the elements within data to be any type?
#[derive(Debug, PartialEq, Clone)]
pub struct Image {
    rows: usize,
    cols: usize,
    data: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageBuilder {
    rows: Option<usize>,
    cols: Option<usize>,
    data: Option<Vec<u32>>,
}

impl Image {
    pub fn new() -> Self {
        todo!()
    }

    pub fn builder() -> ImageBuilder {
        ImageBuilder::new()
    }

    /// Produces a checkerboard pattern
    pub fn checkerboard(&mut self, tile_size: usize, tile_color: u32) -> &Self {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let pixel_index = row * self.cols + col;
                if (row / tile_size + col / tile_size) % 2 == 0 {
                    self.data[pixel_index] = tile_color;
                }
            }
        }

        self
    }

    /// Fills an image with a provided color
    pub fn fill(&mut self, color: u32) -> &mut Self {
        for index in 0..self.data.len() {
            self.data[index] = color;
        }
        self
    }

    /// Sets a single pixel to a provided color
    ///
    /// # Errors
    ///
    /// Will return ValidationError::OutOfBoundsError if provided pixel is outside of the range of
    /// the image.
    pub fn set_pixel(
        &mut self,
        coord: Coordinate,
        color: u32,
    ) -> Result<(), validate::ValidationError> {
        if let Err(e) = validate::coordinate(&self, &coord) {
            return Err(e);
        }

        let pixel_index = (coord.x as usize) * self.get_cols() + (coord.y as usize);

        self.data[pixel_index] = color;

        Ok(())
    }

    /// Gets a single pixel color
    ///
    /// # Errors
    ///
    /// Will return ValidationError::OutOfBoundsError if provided pixel is outside of the range of
    /// the image.
    pub fn get_pixel(&self, coord: Coordinate) -> Result<u32, validate::ValidationError> {
        if let Err(e) = validate::coordinate(&self, &coord) {
            return Err(e);
        }

        let pixel_index = (coord.x as usize) * self.get_cols() + (coord.y as usize);

        Ok(self.data[pixel_index])
    }

    pub fn get_rows(&self) -> &usize {
        &self.rows
    }

    pub fn get_cols(&self) -> &usize {
        &self.cols
    }

    pub fn get_data(&self) -> &Vec<u32> {
        &self.data
    }

    pub fn get_data_length(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug)]
pub enum ImageBuilderError {
    RowsNotProvided(String),
    ColumnsNotProvided(String),
    DataDoesntMatchDimensions(String),
    ZeroSizedImage(String),
}

impl Display for ImageBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageBuilderError::RowsNotProvided(msg) => {
                write!(f, "{}", msg)
            }
            ImageBuilderError::ColumnsNotProvided(msg) => {
                write!(f, "{}", msg)
            }
            ImageBuilderError::DataDoesntMatchDimensions(msg) => {
                write!(f, "{}", msg)
            }
            ImageBuilderError::ZeroSizedImage(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for ImageBuilderError {}

impl ImageBuilder {
    pub fn new() -> Self {
        Self {
            rows: None,
            cols: None,
            data: None,
        }
    }

    pub fn rows(&mut self, rows: usize) -> &mut Self {
        self.rows = Some(rows);
        self
    }

    pub fn cols(&mut self, cols: usize) -> &mut Self {
        self.cols = Some(cols);
        self
    }

    pub fn data(&mut self, data: Vec<u32>) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn build(&self) -> Result<Image, ImageBuilderError> {
        let rows = match self.rows {
            Some(rows) => match rows {
                0 => {
                    return Err(ImageBuilderError::ZeroSizedImage(String::from(
                        "Rows can't be zero.",
                    )));
                }
                any_other_value => any_other_value,
            },
            None => {
                return Err(ImageBuilderError::RowsNotProvided(String::from(
                    "Rows must be provided to build an image.",
                )));
            }
        };

        let cols = match self.cols {
            Some(cols) => match cols {
                0 => {
                    return Err(ImageBuilderError::ZeroSizedImage(String::from(
                        "Columns can't be zero.",
                    )));
                }
                any_other_value => any_other_value,
            },
            None => {
                return Err(ImageBuilderError::ColumnsNotProvided(String::from(
                    "Columns must be provided to build an image.",
                )));
            }
        };

        // TODO : Don't clone with '.to_vec()' here
        let data = match &self.data {
            Some(data) => {
                if data.len() != rows * cols {
                    return Err(ImageBuilderError::DataDoesntMatchDimensions(String::from(
                        "The number of elements in the provided data doesn't match the dimensions of the image being constructed.",
                    )));
                }
                data.to_vec()
            }
            None => {
                vec![BLACK; rows * cols]
            }
        };

        Ok(Image { rows, cols, data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_builder() {
        let image = Image::builder().rows(512).cols(512).build().unwrap();
        assert_eq!(
            image,
            Image {
                rows: 512,
                cols: 512,
                data: vec![BLACK; 512 * 512],
            }
        );
    }
}
