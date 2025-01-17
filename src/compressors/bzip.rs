use std::{fs, io::Write, path::PathBuf};

use utils::colors;

use super::{Compressor, Entry};
use crate::{extension::CompressionFormat, file::File, utils};

pub struct BzipCompressor;

impl BzipCompressor {
    fn compress_files(files: Vec<PathBuf>, format: CompressionFormat) -> crate::Result<Vec<u8>> {
        utils::check_for_multiple_files(&files, &format)?;
        let path = &files[0];
        utils::ensure_exists(path)?;
        let contents = {
            let bytes = fs::read(path)?;
            Self::compress_bytes(&*bytes)?
        };

        println!(
            "{}[INFO]{} compressed {:?} into memory ({})",
            colors::yellow(),
            colors::reset(),
            &path,
            utils::Bytes::new(contents.len() as u64)
        );

        Ok(contents)
    }

    fn compress_file_in_memory(file: File) -> crate::Result<Vec<u8>> {
        // Ensure that our file has in-memory content
        let bytes = match file.contents_in_memory {
            Some(bytes) => bytes,
            None => {
                return Err(crate::Error::InternalError);
            },
        };

        Self::compress_bytes(&*bytes)
    }

    fn compress_bytes(bytes: &[u8]) -> crate::Result<Vec<u8>> {
        let buffer = vec![];
        let mut encoder = bzip2::write::BzEncoder::new(buffer, bzip2::Compression::new(6));
        encoder.write_all(bytes)?;
        Ok(encoder.finish()?)
    }
}

// TODO: customizable compression level
impl Compressor for BzipCompressor {
    fn compress(&self, from: Entry) -> crate::Result<Vec<u8>> {
        match from {
            Entry::Files(files) => Ok(Self::compress_files(files, CompressionFormat::Bzip)?),
            Entry::InMemory(file) => Ok(Self::compress_file_in_memory(file)?),
        }
    }
}
