use crate::app_usage::AppUsage;
use crate::env::Env;
use crate::model::{AppEntry, Apps};
use std::fs::{DirEntry, read_dir, read_to_string};
use std::path::PathBuf;

/// 应用程序加载器
pub struct AppLoader;

impl AppLoader {
    /// `load` 加载应用程序
    pub fn load(env: &Env) -> Apps {
        let desktop_paths: Vec<PathBuf> = vec![
            PathBuf::from("/usr/share/applications"),
            PathBuf::from("/var/lib/snapd/desktop/applications"),
            env.home_dir.join(".local/share/applications"),
        ];
        // 目录条目集合
        let dir_entry_list = Self::parse_dir_entry(desktop_paths);
        // 应用集合
        let mut apps = Vec::with_capacity(30);

        // 获取当前环境变量语言
        let (env_name_prefix, env_comment_prefix) = (
            format!("Name[{}]=", &env.language),
            format!("Comment[{}]=", &env.language),
        );

        for dir_entry in dir_entry_list {
            let path = dir_entry.path();
            // 只处理 desktop 结尾的文件
            if let Some(ext) = path.extension() {
                if ext != "desktop" {
                    continue;
                }
            }

            // 读取 desktop 中的内容
            let filename = path.display().to_string();
            let content = match read_to_string(&filename) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!(
                        "读取desktop文件失败，文件信息：{},错误信息：{}",
                        filename, err
                    );
                    continue;
                }
            };

            // 解析 desktop 为 app_entry
            let app_entry = match Self::parse_desktop_generate_app_entry(
                &filename,
                &content,
                &env_name_prefix,
                &env_comment_prefix,
            ) {
                Some(app_entry) => app_entry,
                None => continue,
            };
            apps.push(app_entry);
        }

        // 读取分数
        let usage = AppUsage::get_usage(&env);
        if !usage.scores.is_empty() {
            for app in &mut apps {
                *app.score.borrow_mut() = usage.scores.get(&app.desktop_file).copied().unwrap_or(0);
            }
        }
        apps
    }

    /// `parse_dir_entry` 解析目录条目
    fn parse_dir_entry(desktop_paths: Vec<PathBuf>) -> Vec<DirEntry> {
        let mut dir_entry_list: Vec<DirEntry> = Vec::with_capacity(30);
        for desktop_path in desktop_paths {
            let entries = match read_dir(&desktop_path) {
                Ok(entries) => entries,
                Err(err) => {
                    eprintln!("读取目录:{},出现异常:{}", desktop_path.display(), err);
                    continue;
                }
            };
            for entry in entries {
                match entry {
                    Ok(entry) => dir_entry_list.push(entry),
                    Err(err) => {
                        eprintln!("读取目录条目失败，异常信息：{}", err);
                        continue;
                    }
                }
            }
        }
        dir_entry_list
    }

    /// `check_desktop` 检查 desktop 是否符合标准
    fn check_desktop(content: &str) -> Option<String> {
        // 只解析 [Desktop Entry] 这部分的数据
        let sections = content.split("[Desktop");
        let content: String = sections.filter(|s| s.starts_with(" Entry]")).collect();
        if content.is_empty() {
            return None;
        }

        // 过滤隐藏的desktop
        if let Some(no_display) = content
            .lines()
            .find(|line| line.contains("NoDisplay=true"))
            .map(|line| !line.is_empty())
        {
            if no_display {
                return None;
            }
        }
        Some(content)
    }

    /// `parse_desktop_generate_app_entry` 解析 desktop 为 app_entry
    fn parse_desktop_generate_app_entry(
        filename: &str,
        content: &str,
        env_name_prefix: &str,
        env_comment_prefix: &str,
    ) -> Option<AppEntry> {
        let mut app_entry = AppEntry::default();

        // 检查 desktop 是否符合显示标准
        let content = match Self::check_desktop(content) {
            Some(content) => content,
            None => return None,
        };

        // 获取名称 和 搜索key
        let mut search_key: String = String::with_capacity(32);
        let mut is_find_env_name = false;
        for line in content.lines() {
            if line.starts_with("Name") {
                // 获取默认名称
                if let Some(default_name) = line.strip_prefix("Name=") {
                    search_key.push_str(default_name);
                    search_key.push(',');
                    app_entry.name = default_name.to_string();
                }
                if !is_find_env_name {
                    // 获取当前环境语言的名称
                    if let Some(cur_env_name) = line.strip_prefix(env_name_prefix) {
                        search_key.push_str(cur_env_name);
                        search_key.push(',');
                        app_entry.name = cur_env_name.to_string();
                        is_find_env_name = true;
                    }
                }
            }
        }
        // 移除结尾的逗号
        if search_key.ends_with(",") {
            search_key.pop();
        }
        app_entry.search_key = search_key.to_lowercase();

        // 获取 desktop 中的 exec,icon,comment 属性
        if let Some(exec) = content
            .lines()
            .find(|line| line.starts_with("Exec"))
            .map(|line| line.to_string())
        {
            if let Some(exec_value) = exec.strip_prefix("Exec=") {
                app_entry.exec = exec_value.to_string();
            }
        } else {
            return None;
        }

        if let Some(icon) = content
            .lines()
            .find(|line| line.starts_with("Icon"))
            .map(|line| line.to_string())
        {
            if let Some(icon_value) = icon.strip_prefix("Icon=") {
                app_entry.icon = Some(icon_value.to_string());
            }
        }

        for line in content.lines() {
            if line.starts_with("Comment") {
                if line.starts_with(env_comment_prefix) {
                    if let Some(comment_value) = line.strip_prefix(env_comment_prefix) {
                        app_entry.comment = comment_value.to_string();
                        break;
                    }
                } else {
                    if line.starts_with("Comment=") {
                        if let Some(comment_value) = line.strip_prefix("Comment=") {
                            app_entry.comment = comment_value.to_string();
                        }
                    }
                }
            }
        }
        app_entry.desktop_file = String::from(filename);
        Some(app_entry)
    }
}
