use crate::app_data_object::AppDataObject;

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
            "Vim",
            "gvim",
            "vim",
            "编辑文本文件",
        ),
        AppDataObject::new(
            "Gimp",
            "/home/zhoujing/Applications/icons/gimp.png",
            "/home/zhoujing/Applications/GIMP-3.2.4-x86_64.AppImage",
            "图像处理程序",
        ),
        AppDataObject::new(
            "Blender",
            "/opt/blender/blender-5.1.1-linux-x64/blender.svg",
            "/opt/blender/blender-5.1.1-linux-x64/blender",
            "3D模型处理程序",
        ),
        AppDataObject::new(
            "Visual Studio Code",
            "vscode",
            "/usr/share/code/code",
            "代码编辑器",
        ),
        AppDataObject::new(
            "Google Chrome",
            "google-chrome",
            "/usr/bin/google-chrome-stable",
            "浏览器",
        ),
        AppDataObject::new(
            "Htop",
            "htop",
            "htop",
            "显示系统进程",
        ),
    ]
}
