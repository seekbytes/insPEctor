use crate::pe::address::Address;
use crate::pe::file_read::FileRead;
use anyhow::{anyhow, bail, Result};
use goblin::Object;
use std::fs;
use std::path::Path;

/// Helper that reads, parses, and understands a binary file based on PE file format
pub struct BinaryParser {}

impl BinaryParser {
    /// Read a file
    pub fn read(path_str: &String) -> Result<FileRead> {
        let path = Path::new(&path_str);

        let buffer = fs::read(path)?;
        let pe = match Object::parse(&buffer)? {
            Object::PE(pe) => pe,
            _ => bail!("We do not support any files beside PE"),
        };

        let entrypoint = pe.entry;
        let text_section_offset = pe
            .sections
            .iter()
            .find(|s| {
                entrypoint > s.virtual_address as usize
                    && entrypoint < (s.virtual_address + s.virtual_size) as usize
                    && s.characteristics & (IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_EXECUTE)
                        == (IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_EXECUTE)
            })
            .ok_or_else(|| anyhow!("No executable and readable section found"))?;

        let begin_text_section_addr = text_section_offset.pointer_to_raw_data as Address;
        let size_text_section = text_section_offset.size_of_raw_data as usize;

        let result = FileRead::new(
            path_str.clone(),
            buffer,
            begin_text_section_addr,
            size_text_section,
            entrypoint as u64,
        );

        Ok(result)
    }
}

/// The section has readable permissions
pub const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
/// The section has executable permissions
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
