use adw::gdk::pango::FontDescription;
use gtk::Settings;

/// 系统环境
pub struct SystemEnv;

impl SystemEnv {
    /// `get_system_current_font_info` 获取系统当前字体族名称和大小
    pub fn get_system_current_font_info() -> (String, f64) {
        let settings = Settings::default().expect("无法获取全局 Settings");
        let current_font = settings
            .gtk_font_name()
            .unwrap_or_else(|| "Sans 14".into())
            .to_string();

        let font_desc = FontDescription::from_string(&current_font);
        let family_name = font_desc.family().expect("无法获取字体族名称").to_string();
        let size = font_desc.size() as f64 / 1024.0;
        (family_name, size)
    }

    /// 将字体和大小应用到整个应用
    pub fn apply_font_size(name: &str, size: f64) {
        let new_font = format!("{} {}", name, size);
        let settings = Settings::default().expect("无法获取全局 Settings");
        settings.set_gtk_font_name(Some(new_font.as_str()));
    }

    /// 应用主题
    pub fn apply_color_scheme(selected_index: u32) {
        let scheme = match selected_index {
            0 => adw::ColorScheme::Default,
            1 => adw::ColorScheme::ForceLight,
            2 => adw::ColorScheme::ForceDark,
            _ => adw::ColorScheme::Default,
        };
        let style_manager = adw::StyleManager::default();
        style_manager.set_color_scheme(scheme);
    }

    /// 获取当前可执行文件的完整路径
    pub fn get_executable_path() -> String {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "jz-launcher".to_string())
    }
}
