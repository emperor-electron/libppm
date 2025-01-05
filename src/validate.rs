use crate::coordinate::CircleCoordinates;
use crate::coordinate::Coordinate;
use crate::coordinate::LineCoordinates;
use crate::graphics::image::Image;
use std::fmt::Display;

#[derive(Debug)]
pub enum ValidationError {
    OutOfBoundsInImageError(Coordinate, Image),
    OutOfBoundsInMemoryError(Coordinate, Image),
    NotEnoughPixelData(Image),
    TooMuchPixelData(Image),
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::OutOfBoundsInImageError(coord, image) => {
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
                    "Not enough pixel data. Expected {}, but found {}.",
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
            ValidationError::OutOfBoundsInMemoryError(coord, image) => {
                write!(
                    f,
                    "The index {}, calculated from {:?}, is out of bounds in memory for image with dimensions ({},{}). Image has valid indexes from 0..{}.",
                    coord.x * *image.get_rows() as i32 + coord.y,
                    coord,
                    image.get_rows(),
                    image.get_cols(),
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
        return Err(ValidationError::OutOfBoundsInImageError(
            coord.clone(),
            image.clone(),
        ));
    }

    if ((coord.x as usize * image.get_cols()) + coord.y as usize) > image.get_data_length() {
        return Err(ValidationError::OutOfBoundsInMemoryError(
            coord.clone(),
            image.clone(),
        ));
    }

    Ok(())
}

pub fn circle_coordinates(image: &Image, coord: &CircleCoordinates) -> Result<(), ValidationError> {
    // TODO : figure out validation needed for the radius
    coordinate(image, &coord.center)?;

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
