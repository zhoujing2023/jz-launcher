use crate::AppDataObject;

/// 模拟数据
pub fn mock_app_list() -> Vec<AppDataObject> {
    vec![
        AppDataObject::new(
            "VLC media player",
            "vlc",
            "/usr/bin/vlc --started-from-file",
            "读取、捕获、广播您的多媒体流",
        ),
        AppDataObject::new(
            "Krita",
            "krita",
            "env QT_QPA_PLATFORM=wayland krita",
            "数字绘画程序",
        ),
        AppDataObject::new(
            "ChatGpt",
            "/home/zhoujing/Applications/icons/chatgpt.png",
            "xdg-open https://chatgpt.com/",
            "随时随地快速获取答案——ChatGPT可以帮助您寻找书籍推荐、诗歌创作或旅行计划。亲自体验一下吧！",
        ),
    ]
}
