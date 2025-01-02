use crate::coordinate::Coordinate;
use crate::coordinate::LineCoordinates;
use crate::graphics::image::Image;
use std::fmt::Display;

#[derive(Debug)]
pub enum ValidationError {
    OutOfBoundsError(Coordinate, Image),
    NotEnoughPixelData(Image),
    TooMuchPixelData(Image),
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::OutOfBoundsError(coord, image) => {
                write!(
                    f,
                    "{} is out of bounds for image with dimensions {} rows by {} columns.",
                    coord,
                    image.get_rows(),
                    image.get_cols()
                )
            }
            ValidationError::NotEnoughPixelData(image) => {
                write!(
                    f,
                    "Not enough pixel. Expected {}, but found {}.",
                    image.get_cols() * image.get_rows(),
                    image.get_data_length()
                )
            }
            ValidationError::TooMuchPixelData(image) => {
                write!(
                    f,
                    "Too much pixel data. Expected {}, but found {}.",
                    image.get_cols() * image.get_rows(),
                    image.get_data_length()
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates that given coordinates are within a given image
pub fn line_coordinates(image: &Image, coords: &LineCoordinates) -> Result<(), ValidationError> {
    let LineCoordinates {
        first: coord_a,
        second: coord_b,
    } = coords;
    coordinate(image, coord_a)?;
    coordinate(image, coord_b)?;
    Ok(())
}

pub fn coordinate(image: &Image, coord: &Coordinate) -> Result<(), ValidationError> {
    if coord.x >= (*image.get_rows() as i32)
        || coord.y >= (*image.get_cols() as i32)
        || coord.x < 0
        || coord.y < 0
    {
        return Err(ValidationError::OutOfBoundsError(
            coord.clone(),
            image.clone(),
        ));
    }
    Ok(())
}

pub fn pixel_data_length(image: &Image) -> Result<(), ValidationError> {
    if image.get_data_length() > image.get_cols() * image.get_rows() {
        Err(ValidationError::TooMuchPixelData(image.clone()))
    } else if image.get_data_length() < image.get_cols() * image.get_rows() {
        Err(ValidationError::NotEnoughPixelData(image.clone()))
    } else {
        Ok(())
    }
}
