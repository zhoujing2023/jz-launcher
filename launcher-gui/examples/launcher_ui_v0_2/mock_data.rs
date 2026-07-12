use crate::app_data_object::AppDataObject;


/// 模拟数据
pub fn mock_app_list() -> Vec<AppDataObject> {
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
            "ChatGpt",
            "/home/zhoujing/Applications/icons/chatgpt.png",
            "xdg-open https://chatgpt.com/",
            "随时随地快速获取答案——ChatGPT可以帮助您寻找书籍推荐、诗歌创作或旅行计划。亲自体验一下吧！",
        ),
    ]
}
