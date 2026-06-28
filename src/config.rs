use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bluetooth: BluetoothConfig,
    pub audio: AudioConfig,
    pub notifications: NotificationsConfig,
    pub contacts: ContactsConfig,
}

#[derive(Debug, Deserialize)]
pub struct BluetoothConfig {
    pub device_name: String,
    pub device_class: String,
}

#[derive(Debug, Deserialize)]
pub struct AudioConfig {
    pub backend: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationsConfig {
    pub enabled: bool,
    pub show_content: bool,
}

#[derive(Debug, Deserialize)]
pub struct ContactsConfig {
    pub sync: bool,
}

impl Config {
    pub fn load() -> Result<Config> {
        let content: String = std::fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
