use crate::payload::Payload;
use anyhow::Result;
use std::fs::File;
use std::path::Path;
use zip::read::ZipFile;
use zip::ZipArchive;

pub struct Archive {
    zip: ZipArchive<File>,
}

impl Archive {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let archive = ZipArchive::new(file)?;
        Ok(Self { zip: archive })
    }

    pub fn payload(&mut self) -> Result<Payload<ZipFile>> {
        let payload_zip = self.zip.by_name("payload.bin")?;
        Payload::new(payload_zip)
    }
}
