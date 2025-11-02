use std::fs;
use std::path::PathBuf;

use apple_codesign::{SigningSettings, UnifiedSigner};

use errors::Error;
use crate::certificate::Certificate;
use super::signer_settings::SignerSettings;
use types::Bundle;

pub struct Signer {
    certificate: Option<Certificate>,
    settings: SignerSettings,
}

impl Signer {
    pub fn new(certificate: Option<Certificate>, settings: SignerSettings) -> Self {
        Signer { certificate, settings }
    }

    pub fn sign(&self, paths: Vec<PathBuf>) -> Result<(), Error> {
        let mut settings = SigningSettings::default();
        
        if let Some(certificate) = &self.certificate {
            certificate.load_into_signing_settings(&mut settings)?;
        }
        
        settings.set_team_id_from_signing_certificate();
        settings.set_shallow(self.settings.sign_shallow);
        settings.set_for_notarization(false);
        
        let signer = UnifiedSigner::new(settings);
        
        

        // signer.sign_path_in_place(&paths)?;

        if let Some(certificate) = &self.certificate {
            if let Some(key) = &certificate.key {
                key.finish()?;
            }
        }

        Ok(())
    }
}
