use gtk::Settings;
use gtk::pango::FontDescription;

/// `test_get_desktop_env_font_info` 获取当前桌面环境字体信息
#[test]
fn test_get_desktop_env_font_info() {
    let _ = gtk::init();

    let settings = Settings::default().expect("无法获取全局 Settings");
    let current_font = settings
        .gtk_font_name()
        .unwrap_or_else(|| "Sans 14".into())
        .to_string();
    println!("当前的字体信息：{current_font}");

    // 解析出字体族名称（去掉尾部的空格和数字）
    let font_family: (&str, u32) =
        current_font
            .rfind(' ')
            .map_or((current_font.as_str(), 11), |pos| {
                (
                    &current_font[..pos],
                    (&current_font[pos..]).trim().parse().expect("转换失败"),
                )
            });
    println!("字体名称：{}，字体大小：{}", font_family.0, font_family.1);
}

/// `test_get_desktop_env_font_info_v2` 获取当前桌面环境字体信息 V2
#[test]
fn test_get_desktop_env_font_info_v2() {
    let _ = gtk::init();

    let settings = Settings::default().expect("无法获取全局 Settings");
    let current_font = settings
        .gtk_font_name()
        .unwrap_or_else(|| "Sans 14".into())
        .to_string();
    println!("当前的字体信息：{current_font}");

    let font_desc = FontDescription::from_string(&current_font);
    let family_name = font_desc.family().expect("无法获取字体族名称").to_string();
    let size = font_desc.size() as f64 / 1024.0;
    println!("当前字体族名称：{family_name}，字体大小：{size}");
}
