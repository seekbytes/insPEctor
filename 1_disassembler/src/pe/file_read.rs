use crate::pe::address::Address;

/// Contains basic information about the file read, such as the buffer, path, and address of the
/// potential text section
pub struct FileRead {
    /// path of the file read
    pub path: String,
    /// all the bytes contained in the file
    pub bytes: Vec<u8>,
    /// first address of the executable section
    pub instr_address: Address,
    /// size of the executable section
    pub size: usize,
    /// entrypoint
    pub entrypoint: Address,
}

impl FileRead {
    /// Create a new instance of file read
    pub fn new(
        path: String,
        bytes: Vec<u8>,
        instr_address: Address,
        size: usize,
        entrypoint: Address,
    ) -> Self {
        Self {
            path,
            bytes,
            instr_address,
            size,
            entrypoint,
        }
    }
}
