use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub appearance: AppearanceConfig,
    pub shortcut: ShortcutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub auto_start_at_boot: bool,
    pub desktop_scan_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: u32,
    pub font_size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub hotkey: String,
    pub close: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                auto_start_at_boot: false,
                desktop_scan_path: vec![
                    "${HOME}/.local/share/applications/".to_string(),
                    "/usr/share/applications".to_string(),
                    "/var/lib/snapd/desktop/applications".to_string(),
                ],
            },
            appearance: AppearanceConfig {
                theme: 0,
                font_size: 11f64,
            },
            shortcut: ShortcutConfig {
                hotkey: "<Shift>space".to_string(),
                close: "<Control>q".to_string(),
            },
        }
    }
}

impl Config {
    /// 加载配置文件
    pub fn load() -> Self {
        // let config_path = dirs::config_dir()
        //     .unwrap_or_else(|| PathBuf::from("."))
        //     .join("jz_tools/config.json");
        let config_path = PathBuf::from("examples/settings_ui/mock_config.json");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str::<Config>(&content) {
                return config;
            }
        }
        // 配置文件不存在或已被损坏，使用默认值生成文件
        let config = Config::default();
        let _ = config.save();
        config
    }

    /// 保存配置文件
    pub fn save(&self) -> Result<(), std::io::Error> {
        // let config_path = dirs::config_dir()
        //     .unwrap_or_else(|| PathBuf::from("."))
        //     .join("jz_tools");
        // std::fs::create_dir_all(&config_path)?;
        // let config_path = config_path.join("config.json");
        let config_path = PathBuf::from("examples/settings_ui/mock_config.json");
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, json)?;
        Ok(())
    }
}
