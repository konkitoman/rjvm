use std::{fs::File, io::Read};

pub struct JavaClassStream {
    file: File,
}

#[allow(dead_code)]
impl JavaClassStream {
    pub fn new(file_path: &str) -> Self {
        Self {
            file: File::options().read(true).open(file_path).unwrap(),
        }
    }

    pub fn read<const BYTES: usize>(&mut self) -> [u8; BYTES] {
        let mut buffer = [0; BYTES];
        self.file.read(&mut buffer).unwrap();
        buffer
    }

    pub fn read_u8(&mut self) -> u8 {
        u8::from_be_bytes(self.read::<1>())
    }

    pub fn read_u16(&mut self) -> u16 {
        u16::from_be_bytes(self.read::<2>())
    }

    pub fn read_u32(&mut self) -> u32 {
        u32::from_be_bytes(self.read::<4>())
    }

    pub fn read_u64(&mut self) -> u64 {
        u64::from_be_bytes(self.read::<8>())
    }

    pub fn read_i8(&mut self) -> i8 {
        i8::from_be_bytes(self.read::<1>())
    }
    pub fn read_i16(&mut self) -> i16 {
        i16::from_be_bytes(self.read::<2>())
    }
    pub fn read_i32(&mut self) -> i32 {
        i32::from_be_bytes(self.read::<4>())
    }

    pub fn read_i64(&mut self) -> i64 {
        i64::from_be_bytes(self.read::<8>())
    }

    pub fn read_f32(&mut self) -> f32 {
        f32::from_be_bytes(self.read::<4>())
    }

    pub fn read_f64(&mut self) -> f64 {
        f64::from_be_bytes(self.read::<8>())
    }
}
