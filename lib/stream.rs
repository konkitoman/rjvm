#![allow(dead_code)]
use std::{fs::File, io::Read, path::Path};

use crate::error::Error;

pub trait ReadStream {
    fn read<const BYTES: usize>(&mut self) -> Result<[u8; BYTES], Error>;

    fn read_u8(&mut self) -> Result<u8, Error> {
        Ok(u8::from_be_bytes(self.read::<1>()?))
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes(self.read::<2>()?))
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_be_bytes(self.read::<4>()?))
    }

    fn read_u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_be_bytes(self.read::<8>()?))
    }

    fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(i8::from_be_bytes(self.read::<1>()?))
    }

    fn read_i16(&mut self) -> Result<i16, Error> {
        Ok(i16::from_be_bytes(self.read::<2>()?))
    }

    fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(i32::from_be_bytes(self.read::<4>()?))
    }

    fn read_i64(&mut self) -> Result<i64, Error> {
        Ok(i64::from_be_bytes(self.read::<8>()?))
    }

    fn read_f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_be_bytes(self.read::<4>()?))
    }

    fn read_f64(&mut self) -> Result<f64, Error> {
        Ok(f64::from_be_bytes(self.read::<8>()?))
    }
}

#[derive(Debug)]
pub struct FileStream {
    file: File,
}

impl FileStream {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl TryFrom<&str> for FileStream {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(file) = File::options().read(true).open(value) {
            Ok(Self { file })
        } else {
            Err(Error::CannotOpenFile)
        }
    }
}

impl TryFrom<&Path> for FileStream {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        if let Ok(file) = File::options().read(true).open(value) {
            Ok(Self { file })
        } else {
            Err(Error::CannotOpenFile)
        }
    }
}

impl ReadStream for FileStream {
    fn read<const BYTES: usize>(&mut self) -> Result<[u8; BYTES], Error> {
        let mut buff = [0; BYTES];
        if let Ok(_) = self.file.read(&mut buff) {
            Ok(buff)
        } else {
            Err(Error::StreamOutOfBounds)
        }
    }
}

#[derive(Debug)]
pub struct BytesStream {
    bytes: Vec<u8>,
}

impl BytesStream {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl From<Vec<u8>> for BytesStream {
    fn from(value: Vec<u8>) -> Self {
        let mut bytes = value;
        bytes.reverse();
        Self { bytes }
    }
}

impl From<&[u8]> for BytesStream {
    fn from(value: &[u8]) -> Self {
        let mut bytes = value.to_vec();
        bytes.reverse();
        Self { bytes }
    }
}

impl ReadStream for BytesStream {
    fn read<const BYTES: usize>(&mut self) -> Result<[u8; BYTES], Error> {
        let mut buff = [0; BYTES];
        for i in 0..BYTES {
            if let Some(b) = self.bytes.pop() {
                buff[i] = b;
            } else {
                return Err(Error::StreamOutOfBounds);
            }
        }
        Ok(buff)
    }
}

#[derive(Debug)]
pub enum Stream {
    FileStream(FileStream),
    BytesStream(BytesStream),
}

impl ReadStream for Stream {
    fn read<const BYTES: usize>(&mut self) -> Result<[u8; BYTES], Error> {
        match self {
            Stream::FileStream(s) => s.read(),
            Stream::BytesStream(s) => s.read(),
        }
    }
}
