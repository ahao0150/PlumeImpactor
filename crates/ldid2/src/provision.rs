use std::fs;
use std::path::PathBuf;

use plist::{Dictionary, Value};

use errors::Error;

pub struct MobileProvision {
    provision_file: PathBuf,
    provision_entitlements_dictionary: Value,
}

impl MobileProvision {
    pub fn new<P: Into<PathBuf>>(provision_path: P) -> Result<Self, Error> {
        let path = provision_path.into();
        
        if !path.exists() {
            return Err(Error::ProvisioningEntitlementsUnknown);
        }

        let provision_entitlements_dictionary = Self::extract_entitlements_from_provision_file(&path)?;

        Ok(Self {
            provision_file: path.clone(),
            provision_entitlements_dictionary,
        })
    }
    
    pub fn get_file_path(&self) -> &PathBuf {
        &self.provision_file
    }

    fn extract_entitlements_from_provision_file(provision_file: &PathBuf) -> Result<Value, Error> {
        let data = fs::read(provision_file)?;
        let start = data.windows(6).position(|w| w == b"<plist").ok_or(Error::ProvisioningEntitlementsUnknown)?;
        let end = data.windows(8).rposition(|w| w == b"</plist>").ok_or(Error::ProvisioningEntitlementsUnknown)? + 8;
        let plist_data = &data[start..end];
        
        let plist = plist::Value::from_reader_xml(plist_data)?;
        let dict = plist
            .as_dictionary()
            .and_then(|d| d.get("Entitlements"))
            .and_then(|v| v.as_dictionary())
            .cloned()
            .ok_or(Error::ProvisioningEntitlementsUnknown)?;
        
        Ok(Value::Dictionary(dict))
    }
    
    pub fn get_entitlements_as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        self.provision_entitlements_dictionary.to_writer_xml(&mut buf)?;
        Ok(buf)
    }

    pub fn get_entitlements_dictionary(&self) -> Result<&Dictionary, Error> {
        self.provision_entitlements_dictionary.as_dictionary().ok_or(Error::ProvisioningEntitlementsUnknown)
    }
    
}
