use tgaimage_sys as tga_sys;
use std::ffi::CString;

pub struct Image {
    image: tga_sys::TGAImage
}

pub struct Color {
    color: tga_sys::TGAColor
}

pub enum Format {
    Grayscale = tga_sys::TGAImage_Format::GRAYSCALE as isize,
    RGB = tga_sys::TGAImage_Format::RGB as isize,
    RBGA = tga_sys::TGAImage_Format::RGBA as isize,
}

impl Image {
 /*   pub fn new() -> Self {
        Image{
            image: unsafe { tga_sys::TGAImage::new() }
        }
    } */

    pub fn with_size(width: i32, height: i32, format: Format) -> Self {
        Image{
            image: unsafe { tga_sys::TGAImage::new1(width, height, format as i32) }
        }
    }

    pub fn flip_vertically(&mut self) -> bool {
        unsafe { self.image.flip_vertically() }
    }

    pub fn write_to_path(&self, path: &str, rle: bool) -> bool {
        unsafe { self.image.write_tga_file(CString::new(path).unwrap().as_ptr(), rle) }
    }

    pub fn set(&mut self, x: i32, y: i32, color: &Color) -> bool {
        unsafe { self.image.set1(x, y, &color.color) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { self.image.destruct() }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color{
            color: unsafe { tga_sys::TGAColor::new1(r, g, b, a) }
        }
    }
}
