use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

use png::{Encoder, BitDepth, ColorType};

use super::{Orthogonal, LEFT_WALL, UP_WALL};
use crate::output::Draw;


impl Draw for Orthogonal {
    type Error = DrawError;
    fn draw<W: Write>(&self, writer: W) -> Result<(), Self::Error> {
        let image_width = (self.width * 2 + 1) as u32;
        let image_height = (self.height * 2 + 1) as u32;
        let image_width_byte_count = (image_width as usize / 8) + if image_width % 8 == 0 { 0 } else { 1 };
        let mut encoder = Encoder::new(writer, image_width, image_height);
        encoder.set_depth(BitDepth::One);
        encoder.set_color(ColorType::Indexed);
        encoder.set_palette(vec![0, 0, 0, 255, 255, 255]);
        let mut writer = encoder.write_header().map_err(|_| DrawError::WriteHeaderFailed)?;
        let mut writer = writer.stream_writer_with_size(128 * 1024).map_err(|_| DrawError::CreateStreamWriterFailed)?;
        if self.width == 0 || self.height == 0 {
            writer.write(&[0b0]).map_err(|_| DrawError::StreamWriteFailed)?;
            return Ok(());
        }
        for line in self.map.chunks(self.width) {
            for cell in line.chunks(4) {
                let mut byte = 0b0000_0000;
                for (i, v) in cell.iter().enumerate() {
                    if *v & UP_WALL != 0 { byte |= 0b1 << (6 - i * 2); }  // 如果上墙不存在，上墙位置设置为白色(1)
                }
                writer.write(&[byte]).map_err(|_| DrawError::StreamWriteFailed)?;
            }
            if line.len() % 4 == 0 { writer.write(&[0b0]).map_err(|_| DrawError::StreamWriteFailed)?; }
            for cell in line.chunks(4) {
                let mut byte = 0b0101_0101;
                for (i, v) in cell.iter().enumerate() {
                    if *v & LEFT_WALL != 0 { byte |= 0b1 << (7 - i * 2); }  // 如果左墙不存在，左墙位置设置为白色(1)
                }
                writer.write(&[byte]).map_err(|_| DrawError::StreamWriteFailed)?;
            }
            if line.len() % 4 == 0 { writer.write(&[0b0]).map_err(|_| DrawError::StreamWriteFailed)?; }
        }
        for _ in 0..image_width_byte_count { writer.write(&[0b0]).map_err(|_| DrawError::CreateStreamWriterFailed)?; }
        Ok(())
    }
}

#[derive(Debug)]
pub enum DrawError {
    WriteHeaderFailed,
    CreateStreamWriterFailed,
    StreamWriteFailed,

}

impl Display for DrawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawError::WriteHeaderFailed => write!(f, "Write header failed"),
            DrawError::CreateStreamWriterFailed => write!(f, "Create stream writer failed"),
            DrawError::StreamWriteFailed => write!(f, "Stream write failed"),
        }
    }
}

impl Error for DrawError {}