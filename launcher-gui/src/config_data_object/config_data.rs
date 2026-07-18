use launcher_core::Env;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const FILE_NAME: &str = "config.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub desktop_scan_path: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: u32,
    pub font_size: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub quit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub general: GeneralConfig,
    pub appearance: AppearanceConfig,
    pub shortcut: ShortcutConfig,
}

impl Default for ConfigData {
    fn default() -> Self {
        // 构建初始目录
        let env = Env::load().expect("加载环境失败");
        let mut desktop_paths: Vec<String> = vec![
            "/usr/share/applications".to_string(),
            "/var/lib/snapd/desktop/applications".to_string(),
        ];
        if let Some(home_share) = env.home_dir.join(".local/share/applications").to_str() {
            desktop_paths.push(home_share.to_string());
        }
        if let Some(home_desktop) = env.home_dir.join("桌面").to_str() {
            desktop_paths.push(home_desktop.to_string());
        }

        Self {
            general: GeneralConfig {
                desktop_scan_path: desktop_paths,
            },
            appearance: AppearanceConfig {
                theme: 0,
                font_size: 11f64,
            },
            shortcut: ShortcutConfig {
                quit: "<Control>q".to_string(),
            },
        }
    }
}

impl ConfigData {
    /// 获取配置文件目录
    fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("jz-launcher")
    }

    /// 加载配置文件
    pub(super) fn load() -> Result<Self, std::io::Error> {
        let config_file_path = Self::get_config_dir().join(FILE_NAME);
        if let Ok(content) = std::fs::read_to_string(&config_file_path) {
            if let Ok(config) = serde_json::from_str::<Self>(&content) {
                return Ok(config);
            }
        }
        // 配置文件不存在或已被损坏，使用默认值生成文件
        let config = Self::default();
        config.save()?;
        Ok(config)
    }

    /// 保存配置文件
    pub(super) fn save(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(Self::get_config_dir())?;
        let config_file_path = Self::get_config_dir().join(FILE_NAME);
        let json_str = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(config_file_path, json_str)
    }
}
