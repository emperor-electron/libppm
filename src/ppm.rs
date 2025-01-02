use crate::graphics::image::Image;
use crate::validate;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct PPMImage {
    image: Image,
    header: Vec<u8>,
    pub filename: String,
}

impl PPMImage {
    pub fn builder() -> PPMImageBuilder {
        PPMImageBuilder::new()
    }

    /// Writes an Image to a .ppm file
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        if let Err(e) = validate::pixel_data_length(&self.image) {
            return Err(Box::new(e));
        }

        let mut fh = fs::File::create(&self.filename)?;
        let mut buffer: Vec<u8> = Vec::new();

        // Push header data into write buffer
        self.header.iter().for_each(|byte| buffer.push(*byte));

        // Push pixel data into write buffer
        for pixel in self.image.get_data().iter() {
            // RGB - 0x00_RR_GG_BB
            buffer.push(((pixel >> 8 * 2) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 1) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 0) & 0xFF) as u8);
        }

        let _ = fh.write(&buffer);
        Ok(())
    }
} /* PPMImage */

#[derive(Default, Clone)]
pub struct PPMImageBuilder {
    image: Option<Image>,
    filename: Option<String>,
}

#[derive(Debug)]
pub enum PPMImageBuilderError {
    ImageNotProvided(String),
    FilenameNotProvided(String),
}

impl Display for PPMImageBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PPMImageBuilderError::ImageNotProvided(msg) => {
                write!(f, "{}", msg)
            }
            PPMImageBuilderError::FilenameNotProvided(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for PPMImageBuilderError {}

impl PPMImageBuilder {
    pub fn new() -> Self {
        PPMImageBuilder {
            image: None,
            filename: None,
        }
    }

    pub fn image(&mut self, image: &Image) -> &mut Self {
        self.image = Some(image.clone());
        self
    }

    pub fn filename(&mut self, filename: &str) -> &mut Self {
        self.filename = Some(filename.to_string());
        self
    }

    pub fn build(&mut self) -> Result<PPMImage, PPMImageBuilderError> {
        let image = match &self.image {
            None => {
                return Err(PPMImageBuilderError::ImageNotProvided(String::from(
                    "Image must be provided to build a PPMImage.",
                )));
            }
            Some(image) => image,
        };

        let filename = match &self.filename {
            None => {
                return Err(PPMImageBuilderError::FilenameNotProvided(String::from(
                    "Filename must be provided to build a PPMImage.",
                )));
            }
            Some(filename) => filename,
        };

        let header = format!("P6\n{} {}\n255\n", *image.get_cols(), *image.get_rows());

        // TODO : Do not clone here
        Ok(PPMImage {
            image: image.clone(),
            header: header.into(),
            filename: filename.clone(),
        })
    }
}
