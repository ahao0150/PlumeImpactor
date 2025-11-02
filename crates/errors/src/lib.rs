use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Info.plist not found")]
    BundleInfoPlistMissing,
    #[error("Unknown bundle type")]
    BundleTypeUnknown,

    #[error("Entitlements not found")]
    ProvisioningEntitlementsUnknown,
    
    #[error("Developer session error {0}: {1}")]
    DeveloperSession(i64, String),
    #[error("Request to developer session failed")]
    DeveloperSessionRequestFailed,
    
    #[error("Authentication SRP error {0}: {1}")]
    AuthSrpWithMessage(i64, String),
    #[error("Authentication SRP error")]
    AuthSrp,
    #[error("Authentication extra step required: {0}")]
    ExtraStep(String),
    #[error("Bad 2FA code")]
    Bad2faCode,
    #[error("Failed to parse")]
    Parse,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Plist error: {0}")]
    Plist(#[from] plist::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Codesign error: {0}")]
    Codesign(#[from] apple_codesign::AppleCodesignError),
    #[error("Certificate PEM error: {0}")]
    Pem(#[from] pem::PemError),
    #[error("X509 certificate error: {0}")]
    X509(#[from] x509_certificate::X509CertificateError),
    #[error("Idevice error: {0}")]
    Idevice(#[from] idevice::IdeviceError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Anisette error: {0}")]
    Anisette(#[from] omnisette::AnisetteError),
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Missing certificate PEM data")]
    CertificatePemMissing,
    
    #[error("Device not found")]
    DeviceNotFound,
}
