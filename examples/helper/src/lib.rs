use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar};

use std::fs::File;
use std::io::Write;
use std::mem::size_of;
use std::slice::from_raw_parts;

#[repr(C)]
pub struct BufferDimensions {
    width: usize,
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

#[no_mangle]
pub extern "C" fn buffer_dimensions_new(width: usize, height: usize) -> BufferDimensions {
    return BufferDimensions::new(width, height);
}

impl BufferDimensions {
    fn new(width: usize, height: usize) -> Self {
        let bytes_per_pixel = size_of::<u32>();
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = 256 as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

#[no_mangle]
pub extern "C" fn save_png(
    path: *const c_char,
    data: *const c_uchar,
    buffer_dimensions: &BufferDimensions,
) {
    let png_output_path: &str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let padded_buffer = unsafe {
        from_raw_parts(
            data,
            buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height,
        )
    };

    let mut png_encoder = png::Encoder::new(
        File::create(png_output_path).unwrap(),
        buffer_dimensions.width as u32,
        buffer_dimensions.height as u32,
    );
    png_encoder.set_depth(png::BitDepth::Eight);
    png_encoder.set_color(png::ColorType::RGBA);

    let mut png_writer = png_encoder
        .write_header()
        .unwrap()
        .into_stream_writer_with_size(buffer_dimensions.unpadded_bytes_per_row);

    for chunk in padded_buffer.chunks(buffer_dimensions.padded_bytes_per_row) {
        png_writer
            .write(&chunk[..buffer_dimensions.unpadded_bytes_per_row])
            .unwrap();
    }
    png_writer.finish().unwrap();
}
