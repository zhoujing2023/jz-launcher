/// 测试数据模块
/// 
/// 此模块包含用于开发和测试的模拟数据
/// 在正式版本中，应该替换为真实的应用数据源

use crate::app_data_object::AppDataObject;

/// 模拟搜索应用程序
pub fn mock_search_apps(keyword: &str) -> Vec<AppDataObject> {
    let apps = mock_app_list();
    
    let keyword_lower = keyword.to_lowercase();
    apps.into_iter()
        .filter(|info| {
            info.name()
                .map(|name| name.to_lowercase().contains(&keyword_lower))
                .unwrap_or(false)
        })
        .collect()
}

/// 获取模拟的应用列表
fn mock_app_list() -> Vec<AppDataObject> {
    vec![
        AppDataObject::new(
            "GIMP",
            "/home/zhoujing/Applications/icons/gimp.png",
            "/home/zhoujing/Applications/GIMP-3.2.4-x86_64.AppImage",
            "开源的图形处理程序",
        ),
        AppDataObject::new(
            "Krita",
            "/home/zhoujing/Applications/icons/krita.svg",
            "env QT_QPA_PLATFORM=wayland krita",
            "自由开源的数字绘画程序",
        ),
        AppDataObject::new(
            "Deepseek",
            "/home/zhoujing/Applications/icons/deepseek.svg",
            "xdg-open https://chat.deepseek.com/",
            "探索未知。立即开始",
        ),
        AppDataObject::new(
            "ChatGpt",
            "/home/zhoujing/Applications/icons/chatgpt.png",
            "xdg-open https://chatgpt.com/",
            "随时随地快速获取答案——ChatGPT可以帮助您寻找书籍推荐、诗歌创作或旅行计划。亲自体验一下吧！",
        ),
        AppDataObject::new(
            "AnotherRedis",
            "/home/zhoujing/Applications/icons/AnotherRedis.png",
            "/home/zhoujing/Applications/Another-Redis-Desktop-Manager-linux-1.7.1-x86_64.AppImage --no-sandbox",
            "Redis可视化工具",
        ),
        AppDataObject::new(
            "QqMusic",
            "/home/zhoujing/Applications/icons/QqMusic.svg",
            "/home/zhoujing/Applications/qqmusic-1.1.8.AppImage --no-sandbox",
            "QQ音乐",
        ),
        AppDataObject::new(
            "BambuStudio",
            "/home/zhoujing/Applications/icons/BambuStudio.png",
            "/home/zhoujing/Applications/BambuStudio_ubuntu-24.04-v02.07.01.57-20260601192128.AppImage",
            "拓竹3D打印控制程序",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_search_apps() {
        let results = mock_search_apps("git");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name().unwrap(), "ChatGpt");
    }

    #[test]
    fn test_mock_search_case_insensitive() {
        let results = mock_search_apps("KRITA");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name().unwrap(), "Krita");
    }

    #[test]
    fn test_mock_search_empty() {
        let results = mock_search_apps("");
        assert_eq!(results.len(), 7);
    }
}
