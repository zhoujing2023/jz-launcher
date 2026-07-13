use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

// 根级项目名称
const ROOT_PROJECT_NAME: &str = "jz-launcher";

#[derive(Debug, Default)]
pub struct Env {
    pub home_dir: PathBuf,
    pub project_name: String,
    pub language: String,
}

impl Env {
    /// `load` 加载环境
    ///
    /// # Examples
    /// ```
    /// use launcher_core::Env;
    /// let config = Env::load().unwrap();
    /// ```
    pub fn load() -> Result<Self> {
        let home_dir = dirs::home_dir().context("无法获取 Home 目录，请检查系统环境变量")?;
        let language = env::var("LANG")
            .or_else(|_| env::var("LC_MESSAGES"))
            .unwrap_or_else(|_| "en_US.UTF-8".to_string()) // 默认使用英文
            .split(".")
            .next()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "en_US".to_string());
        Ok(Self {
            home_dir,
            project_name: env!("CARGO_PKG_NAME").to_string(),
            language,
        })
    }

    /// `usage_file_path` 获取 usage 路径
    ///
    /// # Examples
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use launcher_core::Env;
    /// let env = Env::load().unwrap();
    /// let file_path = env.usage_file_path();
    /// assert!(file_path.ends_with(".config/jz-launcher/usage.json"));
    /// ```
    pub fn usage_file_path(&self) -> PathBuf {
        self.usage_dir().join("usage.json")
    }

    /// `usage_dir` 获取存放 usage 的文件夹路径
    ///
    /// # Examples
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use launcher_core::Env;
    /// let env = Env::load().unwrap();
    /// let path = env.usage_dir();
    /// assert!(path.ends_with(".config/jz-launcher"));
    /// ```
    pub fn usage_dir(&self) -> PathBuf {
        self.home_dir.join(format!(".config/{}", ROOT_PROJECT_NAME))
    }
}
