use std::{env, fs, path::PathBuf};

use tar::Builder;
use utils::colors;
use walkdir::WalkDir;

use super::compressor::Entry;
use crate::{compressors::Compressor, file::File, utils};

pub struct TarCompressor;

impl TarCompressor {
    // TODO: implement this
    fn make_archive_from_memory(_input: File) -> crate::Result<Vec<u8>> {
        println!(
            "{}[ERROR]{} .tar.tar and .zip.tar is currently unimplemented.",
            colors::red(),
            colors::reset()
        );
        Err(crate::Error::InvalidZipArchive(""))
    }

    fn make_archive_from_files(input_filenames: Vec<PathBuf>) -> crate::Result<Vec<u8>> {
        let buf = Vec::new();
        let mut b = Builder::new(buf);

        for filename in input_filenames {
            let previous_location = utils::change_dir_and_return_parent(&filename)?;
            let filename = filename.file_name().unwrap();
            for entry in WalkDir::new(&filename) {
                let entry = entry?;
                let path = entry.path();
                println!("Compressing {:?}", path);
                if path.is_dir() {
                    continue;
                }
                b.append_file(path, &mut fs::File::open(path)?)?;
            }
            env::set_current_dir(previous_location)?;
        }

        Ok(b.into_inner()?)
    }
}

impl Compressor for TarCompressor {
    fn compress(&self, from: Entry) -> crate::Result<Vec<u8>> {
        match from {
            Entry::Files(filenames) => Self::make_archive_from_files(filenames),
            Entry::InMemory(file) => Self::make_archive_from_memory(file),
        }
    }
}
