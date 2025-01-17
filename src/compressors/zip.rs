use std::{
    io::{Cursor, Write},
    path::PathBuf,
};

use walkdir::WalkDir;

use super::compressor::Entry;
use crate::{compressors::Compressor, file::File, utils};

pub struct ZipCompressor;

impl ZipCompressor {
    // TODO: this function does not seem to be working correctly ;/
    fn make_archive_from_memory(input: File) -> crate::Result<Vec<u8>> {
        let buffer = vec![];
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(buffer));

        let inner_file_path: Box<str> = input
            .path
            .file_stem()
            .ok_or(
                // TODO: Is this reachable?
                crate::Error::InvalidInput,
            )?
            .to_string_lossy()
            .into();

        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        writer.start_file(inner_file_path, options)?;

        let input_bytes = match input.contents_in_memory {
            Some(bytes) => bytes,
            None => {
                // TODO: error description, although this block should not be
                // reachable
                return Err(crate::Error::InvalidInput);
            },
        };

        writer.write_all(&*input_bytes)?;

        let bytes = writer.finish().unwrap();

        Ok(bytes.into_inner())
    }

    fn make_archive_from_files(input_filenames: Vec<PathBuf>) -> crate::Result<Vec<u8>> {
        let buffer = vec![];
        let mut writer = zip::ZipWriter::new(Cursor::new(buffer));

        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        for filename in input_filenames {
            let previous_location = utils::change_dir_and_return_parent(&filename)?;
            let filename = filename
                .file_name()
                // Safe unwrap since the function call above would fail in scenarios
                // where this unwrap would panic
                .unwrap();

            for entry in WalkDir::new(filename) {
                let entry = entry?;
                let entry_path = &entry.path();
                if entry_path.is_dir() {
                    continue;
                }

                writer.start_file(entry_path.to_string_lossy(), options)?;
                println!("Compressing {:?}", entry_path);
                let file_bytes = std::fs::read(entry.path())?;
                writer.write_all(&*file_bytes)?;
            }

            std::env::set_current_dir(previous_location)?;
        }

        let bytes = writer.finish()?;

        Ok(bytes.into_inner())
    }
}

impl Compressor for ZipCompressor {
    fn compress(&self, from: Entry) -> crate::Result<Vec<u8>> {
        match from {
            Entry::Files(filenames) => Ok(Self::make_archive_from_files(filenames)?),
            Entry::InMemory(file) => Ok(Self::make_archive_from_memory(file)?),
        }
    }
}
