


// for app extensions, there should be three options
// - default (sign and add mobileprovisions to everything)
// - default-remove-plugins (just like default, but some extensions are removed)
// - zsign (sign extensions with the main apps mobileprovision)
pub enum SignerMode {
    Default,
    Zsign,
}

pub struct SignerSettings {
    pub sign_shallow: bool,
    pub sign_mode: SignerMode,
    pub custom_name: Option<String>,
    pub custom_identifier: Option<String>,
    pub custom_version: Option<String>,
    pub custom_build_version: Option<String>,
    pub support_file_sharing: Option<bool>,
    pub support_older_versions: Option<bool>,
    pub support_more_devices: Option<bool>,
}

impl Default for SignerSettings {
    fn default() -> Self {
        Self {
            sign_shallow: false,
            sign_mode: SignerMode::Default,
            custom_name: None,
            custom_identifier: None,
            custom_version: None,
            custom_build_version: None,
            support_file_sharing: None,
            support_older_versions: None,
            support_more_devices: None,
        }
    }
}
