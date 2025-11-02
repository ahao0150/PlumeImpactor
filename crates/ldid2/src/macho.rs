use std::fs;
use std::path::PathBuf;

use apple_codesign::MachFile;

use errors::Error;
    
pub struct MachO<'a> {
    macho_file: MachFile<'a>,
}

impl<'a> MachO<'a> {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path = path.into();
        let macho_data = fs::read(&path)?;
        let macho_data = Box::leak(macho_data.into_boxed_slice());
        let macho_file = MachFile::parse(macho_data)?;

        Ok(MachO {
            macho_file,
        })
    }
    
    pub fn get_entitlements(&self) -> Result<Option<String>, Error> {
        let macho = self.macho_file.nth_macho(0)?;
        if let Some(embedded_sig) = macho.code_signature()? {
            if let Ok(Some(slot)) = embedded_sig.entitlements() {
                return Ok(Some(slot.to_string()));
            }
        }
        Ok(None)
    }
}
