use std::io::{Read, Write};

use crate::{geometry::Position, Result};

#[derive(Debug, Clone)]
pub struct CharBuffer {
    buffer_size: Position,
    data: Vec<char>,
}

impl CharBuffer {
    pub fn new(size: Position) -> Self {
        let data = vec![' '; size.area()];
        Self {
            buffer_size: size.into(),
            data,
        }
    }

    pub fn with_data(self, data: Vec<char>) -> Self {
        Self {
            buffer_size: self.buffer_size,
            data: data.into(),
        }
    }

    pub fn size(&self) -> usize {
        self.buffer_size.area()
    }

    pub fn clear(&mut self) {
        self.data = vec![' '; self.size()];
    }

    // reads the data from the reader into the buffer
    // and truncates the data to fit the buffer size
    pub fn read(&mut self, reader: &mut impl Read) -> Result<()> {
        todo!()
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<()> {
        let data = self.data.iter().map(|c| *c as u8).collect::<Vec<u8>>();
        writer.write_all(&data)?;
        Ok(())
    }

    pub fn set_char_idx(&mut self, c: char, idx: usize) -> Result<()> {
        self.data[idx] = c;
        Ok(())
    }

    pub fn set_char(&mut self, c: char, x: usize, y: usize) -> Result<()> {
        self.data[y * self.buffer_size.x + x] = c;
        Ok(())
    }

    pub fn rows(&self) -> Vec<String> {
        self.data
            .split(|char| char == &'\n')
            .map(|row| row.iter().collect())
            .collect()
    }
}

impl Read for CharBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let data = self.data.iter().map(|c| *c as u8).collect::<Vec<u8>>();
        buf.as_mut().write(&*data)?;

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
