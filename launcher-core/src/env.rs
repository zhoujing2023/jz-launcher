use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Env {
    pub home_dir: PathBuf,
    pub project_name: String,
    pub language: String,
}

impl Env {
    /// `get_env` 获取环境
    ///
    /// # Examples
    /// ```
    /// use launcher_core::Env;
    /// let config = Env::get_env().unwrap();
    /// ```
    pub fn get_env() -> Result<Self> {
        let home_dir = dirs::home_dir().context("无法获取 Home 目录，请检查系统环境变量")?;
        let language = env::var("LANG")
            .or_else(|_| env::var("LC_MESSAGES"))
            .unwrap_or_else(|_| "en_US.UTF-8".to_string()) // 默认使用英文
            .split(".")
            .next()
            .map(|s| s.to_string())
            .unwrap_or_else(||"en_US".to_string());
        Ok(Self {
            home_dir,
            project_name: env!("CARGO_PKG_NAME").to_string(),
            language,
        })
    }

    /// `get_usage_file_path` 获取 usage 路径
    ///
    /// # Examples
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use launcher_core::Env;
    /// let env = Env::get_env().unwrap();
    /// let file_path = env.get_usage_file_path();
    /// assert!(file_path.ends_with(".config/launcher-core/usage.json"));
    /// ```
    pub fn get_usage_file_path(&self) -> PathBuf {
        self.home_dir
            .join(format!(".config/{}/usage.json", self.project_name))
    }

    /// `get_usage_dir` 获取存放 usage 的文件夹路径
    ///
    /// # Examples
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use launcher_core::Env;
    /// let env = Env::get_env().unwrap();
    /// let path = env.get_usage_dir();
    /// assert!(path.ends_with(".config/launcher-core"));
    /// ```
    pub fn get_usage_dir(&self) -> PathBuf {
        self.home_dir.join(format!(".config/{}", self.project_name))
    }
}
