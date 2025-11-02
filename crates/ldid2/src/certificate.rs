use std::path::PathBuf;

use apple_codesign::{cryptography::{InMemoryPrivateKey, PrivateKey}, SigningSettings};
use x509_certificate::CapturedX509Certificate;

use errors::Error;

pub struct Certificate {
    pub cert: Option<CapturedX509Certificate>,
    pub key: Option<Box<dyn PrivateKey>>,
}

impl Certificate {
    pub fn new(paths: Option<Vec<PathBuf>>) -> Result<Self, Error> {
        let mut cert = Self { 
            cert: None, 
            key: None 
        };

        if let Some(paths) = paths {
            for path in &paths {
                cert.resolve_certificate_from_path(path)?;
            }
        }

        Ok(cert)
    }

    pub fn resolve_certificate_from_path(&mut self, path: &PathBuf) -> Result<(), Error> {
        println!("reading PEM data from {}", path.display());
        let pem_data = std::fs::read(path)?;

        for pem in pem::parse_many(pem_data).map_err(Error::Pem)? {
            match pem.tag() {
                "CERTIFICATE" => {
                    println!("adding certificate from {}", path.display());
                    self.cert = Some(CapturedX509Certificate::from_der(pem.contents())?);
                }
                "PRIVATE KEY" => {
                    println!("adding private key from {}", path.display());
                    self.key = Some(Box::new(InMemoryPrivateKey::from_pkcs8_der(pem.contents())?));
                }
                "RSA PRIVATE KEY" => {
                    println!("adding RSA private key from {}", path.display());
                    self.key = Some(Box::new(InMemoryPrivateKey::from_pkcs1_der(pem.contents())?));
                }
                tag => println!("(unhandled PEM tag {}; ignoring)", tag),
            }
        }

        Ok(())
    }
    
    pub fn load_into_signing_settings<'settings, 'slf: 'settings>(
        &'slf self,
        settings: &'settings mut SigningSettings<'slf>,
    ) -> Result<(), Error> {
        let signing_cert = self.cert.clone().ok_or(Error::CertificatePemMissing)?;
        let signing_key = self.key.as_ref().ok_or(Error::CertificatePemMissing)?;

        if !signing_cert.time_constraints_valid(None) {
            println!(
                "Warning: signing certificate expired as of {}; signatures may not be valid",
                signing_cert.validity_not_after().to_rfc3339()
            );
        }

        settings.set_signing_key(signing_key.as_key_info_signer(), signing_cert);

        if let Some(certs) = settings.chain_apple_certificates() {
            for cert in certs {
                println!(
                    "Automatically registered Apple CA certificate: {}",
                    cert.subject_common_name().unwrap_or_else(|| "default".into())
                );
            }
        }

        Ok(())
    }
}
