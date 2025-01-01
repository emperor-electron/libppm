use crate::coordinate::Coordinate;
use crate::coordinate::LineCoordinates;
use crate::ppm::PPMImage;
use std::fmt::Display;

#[derive(Debug)]
pub enum ValidationError {
    OutOfBoundsError(Coordinate, PPMImage),
    NotEnoughPixelData(PPMImage),
    TooMuchPixelData(PPMImage),
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::OutOfBoundsError(coord, image) => {
                write!(
                    f,
                    "{} is out of bounds for image with dimensions {} rows by {} columns.",
                    coord, image.rows, image.cols
                )
            }
            ValidationError::NotEnoughPixelData(image) => {
                write!(
                    f,
                    "Not enough pixel data to write into {}. Expected {}, but found {}.",
                    image.filename,
                    image.cols * image.rows,
                    image.data.len()
                )
            }
            ValidationError::TooMuchPixelData(image) => {
                write!(
                    f,
                    "Too much pixel data to write into {}. Expected {}, but found {}.",
                    image.filename,
                    image.cols * image.rows,
                    image.data.len()
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates that given coordinates are within a given image
pub fn line_coordinates(image: &PPMImage, coords: &LineCoordinates) -> Result<(), ValidationError> {
    let LineCoordinates {
        first: coord_a,
        second: coord_b,
    } = coords;
    coordinate(image, coord_a)?;
    coordinate(image, coord_b)?;
    Ok(())
}

pub fn coordinate(image: &PPMImage, coord: &Coordinate) -> Result<(), ValidationError> {
    if coord.x >= (image.rows as i32)
        || coord.y >= (image.cols as i32)
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

pub fn pixel_data_length(image: &PPMImage) -> Result<(), ValidationError> {
    if image.data.len() > image.cols * image.rows {
        Err(ValidationError::TooMuchPixelData(image.clone()))
    } else if image.data.len() < image.cols * image.rows {
        Err(ValidationError::NotEnoughPixelData(image.clone()))
    } else {
        Ok(())
    }
}
