use std::cell::RefCell;

pub type Apps = Vec<AppEntry>;

#[derive(Debug, Default)]
pub struct AppEntry {
    // 应用名称
    pub name: String,
    // 执行应用程序路径（如：/usr/bin/wechat %U）
    pub exec: String,
    // desktop 路径
    pub desktop_file: String,
    // 搜索词
    pub search_key: String,
    // 说明
    pub comment: String,
    // 图表
    pub icon: Option<String>,
    // 分数，用于排序
    pub score: RefCell<u32>,
}