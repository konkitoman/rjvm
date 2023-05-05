#[derive(Debug)]
pub enum Error {
    StreamOutOfBounds,
    InvalidMagic,
    CannotOpenFile,
    InvalidConstantTag(u8),
    InvalidVerificationType(u8),
    InvalidStackMapFrame(u8),
    AttributeDontHaveName,
    InvalidClassOrMethod,
}
