use crate::coordinate::Coordinate;
use crate::coordinate::LineCoordinates;
use crate::coordinate::TriangleCoordinates;
use crate::ppm::PPMImage;

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
    if coord.x > (image.cols as i32) || coord.y > (image.rows as i32) || coord.x < 0 || coord.y < 0
    {
        let err_msg = format!("{} is outside the bounds of the image.", coord);
        return Err(err_msg);
    }
    Ok(())
}
