use std::sync::{Arc, RwLock};

// Language enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "chinese" | "中文" | "zh" | "zh_cn" | "zh-cn" => Language::Chinese,
            _ => Language::English,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
        }
    }

    pub fn as_code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
        }
    }
}

// Translation keys - organized by category
pub mod keys {
    // Main Application
    pub const APP_NAME: &str = "app_name";

    // Navigation
    pub const BACK: &str = "back";
    pub const NEXT: &str = "next";
    pub const CANCEL: &str = "cancel";
    pub const VERIFY: &str = "verify";
    pub const SETTINGS: &str = "settings";

    // General Screen
    pub const DEVICE_UTILITIES: &str = "device_utilities";
    pub const IMPORT_IPA: &str = "import_ipa";
    pub const GIVE_STAR: &str = "give_star";
    pub const NO_DEVICE: &str = "no_device";
    pub const IDLE: &str = "idle";

    // Package Screen
    pub const NAME: &str = "name";
    pub const IDENTIFIER: &str = "identifier";
    pub const VERSION: &str = "version";
    pub const SIGNING_OPTIONS: &str = "signing_options";
    pub const INSTALL_OPTIONS: &str = "install_options";
    pub const APP_NAME_PLACEHOLDER: &str = "app_name";
    pub const BUNDLE_IDENTIFIER: &str = "bundle_identifier";
    pub const GENERAL: &str = "general";
    pub const ADVANCED: &str = "advanced";

    // Checkbox labels
    pub const SUPPORT_OLDER_VERSIONS: &str = "support_older_versions";
    pub const FORCE_FILE_SHARING: &str = "force_file_sharing";
    pub const FORCE_IPAD_FULLSCREEN: &str = "force_ipad_fullscreen";
    pub const FORCE_GAME_MODE: &str = "force_game_mode";
    pub const FORCE_PRO_MOTION: &str = "force_pro_motion";
    pub const ONLY_REGISTER_MAIN_BUNDLE: &str = "only_register_main_bundle";
    pub const FORCE_LIQUID_GLASS: &str = "force_liquid_glass";

    // Install mode
    pub const INSTALL: &str = "install";
    pub const EXPORT: &str = "export";
    pub const APPLE_ID: &str = "apple_id";
    pub const ADHOC: &str = "adhoc";
    pub const NO_MODIFY: &str = "no_modify";

    // Tweaks
    pub const TWEAKS: &str = "tweaks";
    pub const ADD_TWEAK: &str = "add_tweak";
    pub const REMOVE_TWEAK: &str = "remove_tweak";
    pub const ADD_BUNDLE: &str = "add_bundle";
    pub const NO_TWEAKS_ADDED: &str = "no_tweaks_added";
    pub const REMOVE: &str = "remove";

    // Package Screen
    pub const NO_PACKAGE_SELECTED: &str = "no_package_selected";
    pub const GO_BACK_SELECT_FILE: &str = "go_back_select_file";
    pub const MODE: &str = "mode";
    pub const SELECT_MODE: &str = "select_mode";
    pub const SIGNING_METHOD: &str = "signing_method";
    pub const SELECT_SIGNING_METHOD: &str = "select_signing_method";
    pub const SIGN_ONLY_DESCRIPTION: &str = "sign_only_description";
    pub const INSTALL_DESCRIPTION: &str = "install_description";

    // Tweak File Dialog
    pub const TWEAK_FILES: &str = "tweak_files";
    pub const SELECT_TWEAK_FILE: &str = "select_tweak_file";
    pub const SELECT_BUNDLE_FOLDER: &str = "select_bundle_folder";

    // Settings Screen
    pub const ACCOUNTS: &str = "accounts";
    pub const ADD_ACCOUNT: &str = "add_account";
    pub const REMOVE_SELECTED: &str = "remove_selected";
    pub const EXPORT_P12: &str = "export_p12";
    pub const LANGUAGE: &str = "language";
    pub const SELECT_LANGUAGE: &str = "select_language";
    pub const LOADING_ACCOUNTS: &str = "loading_accounts";
    pub const NO_ACCOUNTS_ADDED: &str = "no_accounts_added";

    // Login Window
    pub const SIGN_IN_WITH_APPLE_ID: &str = "sign_in_with_apple_id";
    pub const EMAIL: &str = "email";
    pub const PASSWORD: &str = "password";
    pub const TWO_FACTOR_CODE: &str = "two_factor_code";
    pub const SUBMIT: &str = "submit";
    pub const EMAIL_REQUIRED: &str = "email_required";
    pub const PASSWORD_REQUIRED: &str = "password_required";
    pub const CODE_REQUIRED: &str = "code_required";

    // Progress Screen
    pub const PREPARING: &str = "preparing";
    pub const INSTALLING: &str = "installing";
    pub const SIGNING: &str = "signing";
    pub const COMPLETE: &str = "complete";
    pub const FAILED: &str = "failed";
    pub const INSTALLATION_COMPLETE: &str = "installation_complete";
    pub const INSTALLATION_FAILED: &str = "installation_failed";

    // Error Messages
    pub const ERROR_EMAIL_PASSWORD_REQUIRED: &str = "error_email_password_required";
    pub const ERROR_NO_DEVICE_SELECTED: &str = "error_no_device_selected";
    pub const ERROR_NO_PACKAGE_SELECTED: &str = "error_no_package_selected";
    pub const ERROR_NO_ACCOUNT_SELECTED: &str = "error_no_account_selected";
    pub const ERROR_LOGIN_FAILED: &str = "error_login_failed";

    // File dialog
    pub const IOS_APP_PACKAGE: &str = "ios_app_package";
    pub const SELECT_IPA_TIPA_FILE: &str = "select_ipa_tipa_file";
}

// Translation struct
pub struct I18n {
    language: Arc<RwLock<Language>>,
}

impl I18n {
    pub fn new() -> Self {
        let language = Self::detect_system_language();
        I18n {
            language: Arc::new(RwLock::new(language)),
        }
    }

    fn detect_system_language() -> Language {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            if let Ok(output) = Command::new("powershell")
                .args(["-Command", "Get-UICulture | Select-Object -ExpandProperty Name"])
                .output()
            {
                let locale = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
                if locale.contains("zh") || locale.contains("chinese") {
                    return Language::Chinese;
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            if let Ok(output) = Command::new("defaults")
                .args(["read", "NSGlobalDomain", "AppleLanguages"])
                .output()
            {
                let output = String::from_utf8_lossy(&output.stdout);
                if output.to_lowercase().contains("zh") || output.contains("中文") {
                    return Language::Chinese;
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(lang) = std::env::var("LANG") {
                if lang.to_lowercase().contains("zh") {
                    return Language::Chinese;
                }
            }
        }

        Language::English
    }

    pub fn set_language(&self, language: Language) {
        if let Ok(mut lang) = self.language.write() {
            *lang = language;
        }
    }

    pub fn get_language(&self) -> Language {
        if let Ok(lang) = self.language.read() {
            *lang
        } else {
            Language::English
        }
    }

    pub fn t(&self, key: &str) -> String {
        let lang = self.get_language();
        match lang {
            Language::English => self.get_english(key),
            Language::Chinese => self.get_chinese(key),
        }
    }

    fn get_english(&self, key: &str) -> String {
        match key {
            // Main Application
            keys::APP_NAME => "Impactor",

            // Navigation
            keys::BACK => "Back",
            keys::NEXT => "Next",
            keys::CANCEL => "Cancel",
            keys::VERIFY => "Verify",
            keys::SETTINGS => "Settings",

            // General Screen
            keys::DEVICE_UTILITIES => "Device Utilities",
            keys::IMPORT_IPA => "Import .ipa / .tipa",
            keys::GIVE_STAR => "Give me a ⭐ star :3",
            keys::NO_DEVICE => "No Device",
            keys::IDLE => "Idle.",

            // Package Screen
            keys::NAME => "Name:",
            keys::IDENTIFIER => "Identifier:",
            keys::VERSION => "Version:",
            keys::SIGNING_OPTIONS => "Signing Options",
            keys::INSTALL_OPTIONS => "Install Options",
            keys::APP_NAME_PLACEHOLDER => "App name",
            keys::BUNDLE_IDENTIFIER => "Bundle identifier",
            keys::GENERAL => "General:",
            keys::ADVANCED => "Advanced:",

            // Checkbox labels
            keys::SUPPORT_OLDER_VERSIONS => "Support older versions (7+)",
            keys::FORCE_FILE_SHARING => "Force File Sharing",
            keys::FORCE_IPAD_FULLSCREEN => "Force iPad Fullscreen",
            keys::FORCE_GAME_MODE => "Force Game Mode",
            keys::FORCE_PRO_MOTION => "Force Pro Motion",
            keys::ONLY_REGISTER_MAIN_BUNDLE => "Only register main bundle",
            keys::FORCE_LIQUID_GLASS => "Force Liquid Glass (26+)",

            // Install mode
            keys::INSTALL => "Install",
            keys::EXPORT => "Export",
            keys::APPLE_ID => "Apple ID",
            keys::ADHOC => "Adhoc",
            keys::NO_MODIFY => "No Modify",

            // Tweaks
            keys::TWEAKS => "Tweaks",
            keys::ADD_TWEAK => "Add Tweak",
            keys::REMOVE_TWEAK => "Remove Tweak",
            keys::ADD_BUNDLE => "Add Bundle",
            keys::NO_TWEAKS_ADDED => "No tweaks added",
            keys::REMOVE => "Remove",

            // Package Screen
            keys::NO_PACKAGE_SELECTED => "No package selected",
            keys::GO_BACK_SELECT_FILE => "Go back and select a file",
            keys::MODE => "Mode:",
            keys::SELECT_MODE => "Select mode",
            keys::SIGNING_METHOD => "Signing:",
            keys::SELECT_SIGNING_METHOD => "Select signing method",
            keys::SIGN_ONLY_DESCRIPTION => "Sign and export (no installation)",
            keys::INSTALL_DESCRIPTION => "Sign and install to device",

            // Tweak File Dialog
            keys::TWEAK_FILES => "Tweak files",
            keys::SELECT_TWEAK_FILE => "Select Tweak File",
            keys::SELECT_BUNDLE_FOLDER => "Select Bundle Folder",

            // Settings Screen
            keys::ACCOUNTS => "Accounts",
            keys::ADD_ACCOUNT => "Add Account",
            keys::REMOVE_SELECTED => "Remove Selected",
            keys::EXPORT_P12 => "Export P12",
            keys::LANGUAGE => "Language",
            keys::SELECT_LANGUAGE => "Select Language",
            keys::LOADING_ACCOUNTS => "Loading accounts...",
            keys::NO_ACCOUNTS_ADDED => "No accounts added yet",

            // Login Window
            keys::SIGN_IN_WITH_APPLE_ID => "Sign in with Apple ID",
            keys::EMAIL => "Email",
            keys::PASSWORD => "Password",
            keys::TWO_FACTOR_CODE => "2FA Code",
            keys::SUBMIT => "Submit",
            keys::EMAIL_REQUIRED => "Email required",
            keys::PASSWORD_REQUIRED => "Password required",
            keys::CODE_REQUIRED => "Code required",

            // Progress Screen
            keys::PREPARING => "Preparing...",
            keys::INSTALLING => "Installing...",
            keys::SIGNING => "Signing...",
            keys::COMPLETE => "Complete!",
            keys::FAILED => "Failed",
            keys::INSTALLATION_COMPLETE => "Installation complete!",
            keys::INSTALLATION_FAILED => "Installation failed",

            // Error Messages
            keys::ERROR_EMAIL_PASSWORD_REQUIRED => "Email and password required",
            keys::ERROR_NO_DEVICE_SELECTED => "No device selected",
            keys::ERROR_NO_PACKAGE_SELECTED => "No package selected",
            keys::ERROR_NO_ACCOUNT_SELECTED => "No account selected",
            keys::ERROR_LOGIN_FAILED => "Login failed",

            // File dialog
            keys::IOS_APP_PACKAGE => "iOS App Package",
            keys::SELECT_IPA_TIPA_FILE => "Select IPA/TIPA file",

            _ => key,
        }
        .to_string()
    }

    fn get_chinese(&self, key: &str) -> String {
        match key {
            // Main Application
            keys::APP_NAME => "Impactor",

            // Navigation
            keys::BACK => "返回",
            keys::NEXT => "下一步",
            keys::CANCEL => "取消",
            keys::VERIFY => "验证",
            keys::SETTINGS => "设置",

            // General Screen
            keys::DEVICE_UTILITIES => "设备工具",
            keys::IMPORT_IPA => "导入 .ipa / .tipa",
            keys::GIVE_STAR => "给我个⭐星吧 :3",
            keys::NO_DEVICE => "无设备",
            keys::IDLE => "空闲。",

            // Package Screen
            keys::NAME => "名称：",
            keys::IDENTIFIER => "标识符：",
            keys::VERSION => "版本：",
            keys::SIGNING_OPTIONS => "签名选项",
            keys::INSTALL_OPTIONS => "安装选项",
            keys::APP_NAME_PLACEHOLDER => "应用名称",
            keys::BUNDLE_IDENTIFIER => "Bundle 标识符",
            keys::GENERAL => "常规：",
            keys::ADVANCED => "高级：",

            // Checkbox labels
            keys::SUPPORT_OLDER_VERSIONS => "支持旧版本 (7+)",
            keys::FORCE_FILE_SHARING => "强制文件共享",
            keys::FORCE_IPAD_FULLSCREEN => "强制 iPad 全屏",
            keys::FORCE_GAME_MODE => "强制游戏模式",
            keys::FORCE_PRO_MOTION => "强制 Pro Motion",
            keys::ONLY_REGISTER_MAIN_BUNDLE => "仅注册主 bundle",
            keys::FORCE_LIQUID_GLASS => "强制 Liquid Glass (26+)",

            // Install mode
            keys::INSTALL => "安装",
            keys::EXPORT => "导出",
            keys::APPLE_ID => "Apple ID",
            keys::ADHOC => "Adhoc",
            keys::NO_MODIFY => "不修改",

            // Tweaks
            keys::TWEAKS => "Tweaks",
            keys::ADD_TWEAK => "添加 Tweak",
            keys::REMOVE_TWEAK => "移除 Tweak",
            keys::ADD_BUNDLE => "添加 Bundle",
            keys::NO_TWEAKS_ADDED => "未添加 Tweak",
            keys::REMOVE => "移除",

            // Package Screen
            keys::NO_PACKAGE_SELECTED => "未选择包",
            keys::GO_BACK_SELECT_FILE => "返回并选择文件",
            keys::MODE => "模式：",
            keys::SELECT_MODE => "选择模式",
            keys::SIGNING_METHOD => "签名：",
            keys::SELECT_SIGNING_METHOD => "选择签名方式",
            keys::SIGN_ONLY_DESCRIPTION => "签名并导出（不安装）",
            keys::INSTALL_DESCRIPTION => "签名并安装到设备",

            // Tweak File Dialog
            keys::TWEAK_FILES => "Tweak 文件",
            keys::SELECT_TWEAK_FILE => "选择 Tweak 文件",
            keys::SELECT_BUNDLE_FOLDER => "选择 Bundle 文件夹",

            // Settings Screen
            keys::ACCOUNTS => "账户",
            keys::ADD_ACCOUNT => "添加账户",
            keys::REMOVE_SELECTED => "移除选中",
            keys::EXPORT_P12 => "导出 P12",
            keys::LANGUAGE => "语言",
            keys::SELECT_LANGUAGE => "选择语言",
            keys::LOADING_ACCOUNTS => "正在加载账户...",
            keys::NO_ACCOUNTS_ADDED => "尚未添加账户",

            // Login Window
            keys::SIGN_IN_WITH_APPLE_ID => "使用 Apple ID 登录",
            keys::EMAIL => "电子邮件",
            keys::PASSWORD => "密码",
            keys::TWO_FACTOR_CODE => "双重验证码",
            keys::SUBMIT => "提交",
            keys::EMAIL_REQUIRED => "需要电子邮件",
            keys::PASSWORD_REQUIRED => "需要密码",
            keys::CODE_REQUIRED => "需要验证码",

            // Progress Screen
            keys::PREPARING => "准备中...",
            keys::INSTALLING => "安装中...",
            keys::SIGNING => "签名中...",
            keys::COMPLETE => "完成！",
            keys::FAILED => "失败",
            keys::INSTALLATION_COMPLETE => "安装完成！",
            keys::INSTALLATION_FAILED => "安装失败",

            // Error Messages
            keys::ERROR_EMAIL_PASSWORD_REQUIRED => "需要电子邮件和密码",
            keys::ERROR_NO_DEVICE_SELECTED => "未选择设备",
            keys::ERROR_NO_PACKAGE_SELECTED => "未选择包",
            keys::ERROR_NO_ACCOUNT_SELECTED => "未选择账户",
            keys::ERROR_LOGIN_FAILED => "登录失败",

            // File dialog
            keys::IOS_APP_PACKAGE => "iOS 应用包",
            keys::SELECT_IPA_TIPA_FILE => "选择 IPA/TIPA 文件",

            _ => key,
        }
        .to_string()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

// Global instance
use once_cell::sync::Lazy;

static I18N: Lazy<I18n> = Lazy::new(I18n::new);

// Public API
pub fn set_language(language: Language) {
    I18N.set_language(language);
}

pub fn get_language() -> Language {
    I18N.get_language()
}

pub fn t(key: &str) -> String {
    I18N.t(key)
}

// Format with arguments
pub fn tf(key: &str, args: &[&str]) -> String {
    let mut text = t(key);
    for (i, arg) in args.iter().enumerate() {
        text = text.replace(&format!("{{{}}}", i), arg);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_str() {
        assert_eq!(Language::from_str("English"), Language::English);
        assert_eq!(Language::from_str("chinese"), Language::Chinese);
        assert_eq!(Language::from_str("中文"), Language::Chinese);
        assert_eq!(Language::from_str("zh"), Language::Chinese);
        assert_eq!(Language::from_str("zh_cn"), Language::Chinese);
    }

    #[test]
    fn test_translation() {
        let i18n = I18n::new();
        i18n.set_language(Language::English);
        assert_eq!(i18n.t(keys::BACK), "Back");

        i18n.set_language(Language::Chinese);
        assert_eq!(i18n.t(keys::BACK), "返回");
    }
}
