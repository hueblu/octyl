use std::io::{Read, Write};

use crate::Result;

#[derive(Debug, Clone)]
pub struct CharBuffer {
    buffer_size: (usize, usize),
    data: Vec<char>,
}

impl CharBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![' '; (width * height) as usize];
        Self {
            buffer_size: (width, height),
            data,
        }
    }

    pub fn clear(&mut self) {
        self.data = vec![' '; (self.buffer_size.0 * self.buffer_size.1) as usize];
    }

    // reads the data from the reader into the buffer
    // and truncates the data to fit the buffer size
    pub fn read(&mut self, reader: &mut impl Read) -> Result<()> {
        Ok(())
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<()> {
        let data = self.data.iter().map(|c| *c as u8).collect::<Vec<u8>>();
        writer.write_all(&data)?;
        Ok(())
    }

    pub fn set_char_idx(&mut self, c: char, idx: usize) -> Result<()> {
        self.data[idx as usize] = c;
        Ok(())
    }

    pub fn set_char(&mut self, c: char, x: usize, y: usize) -> Result<()> {
        self.data[(y * self.buffer_size.0 + x) as usize] = c;
        Ok(())
    }
}

impl Read for CharBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut data = self.data.iter().map(|c| *c as u8).collect::<Vec<u8>>();
        let mut buf = buf.as_mut();
        buf = data.as_mut_slice();

        Ok(std::cmp::min(buf.len(), self.data.len()))
    }
}

impl Write for CharBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data = buf.iter().map(|b| *b as char).collect();

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
