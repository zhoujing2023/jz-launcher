use crate::env::Env;
use crate::model::{AppEntry, Apps};
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 每次使用增加的分数
const SCORE_INCREMENT: u32 = 1;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppUsage {
    // key : desktop_file 路径作为唯一键，避免重名冲突
    // value : 打开次数
    pub scores: HashMap<String, u32>,
}

impl AppUsage {
    /// `load` 加载 usage 数据
    pub fn load(env: &Env) -> Self {
        Self::read_usage_file(&env).unwrap_or_else(|err| {
            eprintln!("读取 usage.json 数据出现异常：{}", err);
            AppUsage::default()
        })
    }

    /// `read_usage_file` 读取 usage.json 数据
    fn read_usage_file(env: &Env) -> Result<Self> {
        let usage_path = env.usage_file_path();
        let content = std::fs::read_to_string(&usage_path)
            .with_context(|| format!("读取 {} 出现异常", usage_path.display()))?;
        let data: Self =
            serde_json::from_str(&content).context("解析 usage 字符串为 AppUsage 对象失败")?;
        Ok(data)
    }

    /// `save` 保存 usage.json 文件
    /// 如果目录不存在则创建目录并再次写入
    fn save(&self, env: &Env) -> Result<()> {
        let usage_path = env.usage_file_path();
        let json_str =
            serde_json::to_string_pretty(self).context("将 AppUsage 对象转换为 json 字符串失败")?;
        if let Err(err) = std::fs::write(&usage_path, &json_str) {
            if err.kind() == std::io::ErrorKind::NotFound {
                let usage_dir = env.usage_dir();
                if let Err(err) = std::fs::create_dir_all(&usage_dir) {
                    return Err(anyhow!("创建 {} 目录，失败：{}", usage_dir.display(), err));
                }
                let _ = std::fs::write(&usage_path, &json_str).with_context(|| {
                    format!("将 AppUsage json 写入 {} 再次失败", usage_path.display())
                })?;
                return Ok(());
            }
            return Err(anyhow!(
                "将 AppUsage json 写入 {} 失败：{}",
                usage_path.display(),
                err
            ));
        }
        Ok(())
    }

    /// `update_scores_from_apps` 从 Apps 更新 usage 数据
    fn update_scores_from_apps(&mut self, apps: &Apps) {
        self.scores.clear();
        for app in apps {
            self.scores
                .insert(app.desktop_file.clone(), *app.score.borrow());
        }
    }

    /// `record_launch` 分数递增并保存
    pub fn record_launch(&mut self, env: &Env, application: &AppEntry, apps: &Apps) -> Result<()> {
        // 应用分数递增
        let current_score = *application.score.borrow();
        *application.score.borrow_mut() = current_score.saturating_add(SCORE_INCREMENT);
        // 更新分数并保存
        self.update_scores_from_apps(apps);
        self.save(&env)
    }
}
