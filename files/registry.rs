//! Windows registry read helpers for Openscad.

pub const APP_KEY: &str = "Software\\Openscad\\Openscad";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Openscad";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Openscad") },
        RegValue { name: "Version",     value: String::from("v3.6.3") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Openscad") },
        RegValue { name: "Publisher",       value: String::from("Openscad") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Openscad\\uninstall.exe\"") },
    ]
}
