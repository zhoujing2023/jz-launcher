mod config;

use crate::config::Config;
use adw::gdk::Display;
use adw::gdk::pango::FontDescription;
use adw::gio::ActionEntry;
use adw::{Application, gdk};
use glib::Propagation;
use gtk::prelude::*;
use gtk::{
    Adjustment, ApplicationWindow, Box, Button, CssProvider, DropDown, Entry, EventControllerKey,
    Label, LinkButton, Orientation, Separator, Settings, SpinButton, Stack, StackSidebar,
    StringList,
};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "debug.zhoujing.jz_tools";
type ConfigRef = Rc<RefCell<Config>>;

const HOTKEY_ACTION: &str = "open";
const CLOSE_ACTION: &str = "close";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());

    app.connect_activate(|app| {
        let window = build_settings_window(app);
        window.present();
    });
    app.set_accels_for_action("win.close", &["<Control>q"]);
    app.set_accels_for_action("win.open", &["<Shift>space"]);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("No display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn setup_shortcut_key_action(_window: &ApplicationWindow) -> Vec<ActionEntry<ApplicationWindow>> {
    let mut action_entry_list: Vec<ActionEntry<ApplicationWindow>> = Vec::default();
    // 打开
    let action_open = ActionEntry::builder(HOTKEY_ACTION)
        .activate(|_window: &ApplicationWindow, _, _| {
            println!("打开应用程序……");
            // TODO: 打开应用程序
        })
        .build();

    // 关闭
    let action_close = ActionEntry::builder(CLOSE_ACTION)
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();

    action_entry_list.push(action_open);
    action_entry_list.push(action_close);
    action_entry_list
}

fn build_settings_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("设置")
        .width_request(600)
        .height_request(400)
        .build();

    // 加载并初始化配置
    let config = Config::load();
    let config = Rc::new(RefCell::new(config));
    init_config(&config, &window);

    // 配置快捷键
    let action_entry_list = setup_shortcut_key_action(&window);
    window.add_action_entries(action_entry_list);

    let main_box = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(0)
        .build();

    // 左侧：侧边栏
    let sidebar = StackSidebar::builder()
        .width_request(150)
        .vexpand(true)
        .build();

    // 分割线
    let separator = Separator::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // 右侧：内容堆栈
    let content_stack = Stack::builder()
        .vexpand(true) // 占用父容器剩余的空间
        .hexpand(true)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .transition_type(gtk::StackTransitionType::SlideLeftRight) // 过渡方式：左右滑动
        .transition_duration(200) // 过渡持续时间
        .build();

    // 创建页面
    let general_page = build_general_page(&config);
    let appearance_page = build_appearance_page(&config);
    let shortcuts_page = build_shortcuts_page(&window, &config);
    let about_page = build_about_page();

    // 添加页面并设置标题（修正为正确的 &str）
    content_stack.add_named(&general_page, Some("general_page"));
    let page = content_stack.page(&general_page);
    page.set_title("通用");

    content_stack.add_named(&appearance_page, Some("appearance_page"));
    let page = content_stack.page(&appearance_page);
    page.set_title("外观");

    content_stack.add_named(&shortcuts_page, Some("shortcuts_page"));
    let page = content_stack.page(&shortcuts_page);
    page.set_title("快捷键");

    content_stack.add_named(&about_page, Some("about_page"));
    let page = content_stack.page(&about_page);
    page.set_title("关于");

    // 关联侧边栏
    sidebar.set_stack(&content_stack);

    // 默认选中第一个页面
    content_stack.set_visible_child(&general_page);

    // 组装界面
    main_box.append(&sidebar);
    main_box.append(&separator);
    main_box.append(&content_stack);
    window.set_child(Some(&main_box));

    window
}

// 页面构建函数
fn build_general_page(config: &ConfigRef) -> Box {
    let page = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .build();

    // 开机自动运行
    let switch = gtk::Switch::builder()
        .active(config.borrow().general.auto_start_at_boot)
        .build();
    let row_1 = build_setting_row("开机时自动运行", &switch);

    let config_clone = config.clone();
    switch.connect_state_set(move |_, state| {
        // TODO: 修改启动项
        if state {
            println!("设置开机时自动启动……");
        } else {
            println!("取消开机时自动启动……");
        }
        config_clone.borrow_mut().general.auto_start_at_boot = state;
        let _ = config_clone.borrow().save();
        Propagation::Proceed
    });
    page.append(&row_1);

    // Desktop 路径
    let desktop_paths_label = Label::builder()
        .label("Desktop 路径：")
        .halign(gtk::Align::Start)
        .build();
    page.append(&desktop_paths_label);

    let desktop_paths_view = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::None)
        .margin_top(3)
        .margin_bottom(3)
        .tooltip_text("Desktop 路径，通过换行区分目录")
        .editable(false)
        .css_classes(vec!["desktop-paths-list-view-disabled"])
        .build();
    let desktop_paths = &config.borrow().general.desktop_scan_path;
    desktop_paths_view
        .buffer()
        .set_text(desktop_paths.join("\n").as_str());

    let scrolled_window = gtk::ScrolledWindow::builder()
        .height_request(200)
        .hexpand(true)
        .halign(gtk::Align::Fill)
        .child(&desktop_paths_view)
        .build();
    page.append(&scrolled_window);

    let button_box = Box::builder().halign(gtk::Align::End).spacing(12).build();
    let edit_button = Button::builder().label("编辑").build();
    button_box.append(&edit_button);
    let save_button = Button::builder()
        .label("保存")
        .css_classes(vec!["suggested-action"])
        .visible(false)
        .build();
    button_box.append(&save_button);
    page.append(&button_box);
    let cancel_button = Button::builder().label("取消").visible(false).build();
    button_box.append(&cancel_button);

    // 更新 TextView UI
    update_desktop_paths_view_ui(
        &cancel_button,
        &save_button,
        &edit_button,
        &desktop_paths_view,
        &config,
    );
    page
}

fn update_desktop_paths_view_ui(
    cancel_button: &Button,
    save_button: &Button,
    edit_button: &Button,
    desktop_paths_view: &gtk::TextView,
    config: &ConfigRef,
) {
    // 修改前的数据
    let origin_text_data = Rc::new(RefCell::new(String::new()));

    // 编辑按钮
    let cancel_button_clone = cancel_button.clone();
    let save_button_clone = save_button.clone();
    let desktop_paths_view_clone = desktop_paths_view.clone();
    let origin_text_data_clone = origin_text_data.clone();
    edit_button.connect_clicked(move |edit_button| {
        edit_button.set_visible(false);
        save_button_clone.set_visible(true);
        cancel_button_clone.set_visible(true);
        desktop_paths_view_clone.set_editable(true);
        desktop_paths_view_clone.add_css_class("desktop-paths-list-view-enabled");
        desktop_paths_view_clone.remove_css_class("desktop-paths-list-view-disabled");
        // 临时保存原数据，当点击“取消”时进行还原
        let buffer = desktop_paths_view_clone.buffer();
        *origin_text_data_clone.borrow_mut() = buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .to_string();
    });

    // 取消按钮
    let edit_button_clone = edit_button.clone();
    let save_button_clone = save_button.clone();
    let desktop_paths_view_clone = desktop_paths_view.clone();
    let origin_text_data_clone = origin_text_data.clone();
    cancel_button.connect_clicked(move |cancel_button| {
        edit_button_clone.set_visible(true);
        save_button_clone.set_visible(false);
        cancel_button.set_visible(false);
        desktop_paths_view_clone.set_editable(false);
        desktop_paths_view_clone.remove_css_class("desktop-paths-list-view-enabled");
        desktop_paths_view_clone.add_css_class("desktop-paths-list-view-disabled");
        // 撤销修改的数据
        desktop_paths_view_clone
            .buffer()
            .set_text(origin_text_data_clone.borrow().as_str());
    });

    // 保存按钮
    let edit_button_clone = edit_button.clone();
    let cancel_button_clone = cancel_button.clone();
    let desktop_paths_view_clone = desktop_paths_view.clone();
    let config_clone = config.clone();
    save_button.connect_clicked(move |save_button| {
        edit_button_clone.set_visible(true);
        cancel_button_clone.set_visible(false);
        save_button.set_visible(false);
        desktop_paths_view_clone.set_editable(false);
        desktop_paths_view_clone.remove_css_class("desktop-paths-list-view-enabled");
        desktop_paths_view_clone.add_css_class("desktop-paths-list-view-disabled");

        // 更新配置文件
        let buffer = desktop_paths_view_clone.buffer();
        let new_desktop_paths = buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .to_string();
        println!("更新 desktop 解析路径，新内容为：\n{}", new_desktop_paths);
        let new_desktop_paths = new_desktop_paths
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        config_clone.borrow_mut().general.desktop_scan_path = new_desktop_paths;
        let _ = config_clone.borrow().save();
        // TODO: 1.解析 desktop 路径
    });
}

/// 构建 外观页面
fn build_appearance_page(config: &ConfigRef) -> Box {
    let page = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .build();

    // 主题模式
    let options = StringList::new(&["跟随系统", "亮色", "暗色"]);
    let drop_down = DropDown::builder()
        .model(&options)
        .enable_search(false)
        .build();

    // 默认选项
    drop_down.set_selected(config.borrow().appearance.theme);

    // 获取 libadwaita 全局的 StyleManager
    let style_manager = adw::StyleManager::default();
    let config_clone = config.clone();
    drop_down.connect_selected_notify(move |dd| {
        let selected_index = dd.selected();
        apply_color_scheme(selected_index, &style_manager);
        config_clone.borrow_mut().appearance.theme = selected_index;
        let _ = config_clone.borrow().save();
    });
    let row_1 = build_setting_row("主题模式", &drop_down);
    page.append(&row_1);

    // 字体
    let cur_sys_font_info = get_system_current_font_info();
    let config_font_size = config.borrow().appearance.font_size;
    let adjustment = Adjustment::builder()
        .lower(10.0)
        .upper(24.0)
        .step_increment(1.0)
        .page_increment(2.0)
        .value(config_font_size)
        .build();
    let spin_button = SpinButton::builder().adjustment(&adjustment).build();

    let settings = Settings::default().expect("无法获取全局 Settings");
    let config_clone = config.clone();
    spin_button.connect_value_changed(move |btn| {
        apply_font_size(&cur_sys_font_info.0, btn.value(), &settings);
        // 更新配置文件
        config_clone.borrow_mut().appearance.font_size = btn.value();
        let _ = config_clone.borrow().save();
    });

    let row_2 = build_setting_row("字体大小", &spin_button);
    page.append(&row_2);
    page
}

/// 应用主题
fn apply_color_scheme(selected_index: u32, style_manager: &adw::StyleManager) {
    let scheme = match selected_index {
        0 => adw::ColorScheme::Default,
        1 => adw::ColorScheme::ForceLight,
        2 => adw::ColorScheme::ForceDark,
        _ => adw::ColorScheme::Default,
    };
    style_manager.set_color_scheme(scheme);
}

/// 将字体和大小应用到整个应用
fn apply_font_size(name: &str, size: f64, settings: &Settings) {
    let new_font = format!("{} {}", name, size);
    settings.set_gtk_font_name(Some(new_font.as_str()));
}

/// `get_system_current_font_info` 获取系统当前字体族名称和大小
fn get_system_current_font_info() -> (String, f64) {
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

/// 构建 快捷键页面
fn build_shortcuts_page(window: &ApplicationWindow, config: &ConfigRef) -> Box {
    let page = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .build();

    let hotkeys_entry = Entry::builder()
        .editable(false)
        .text(&config.borrow().shortcut.hotkey)
        .width_request(240)
        .build();
    let row_1 = build_setting_row("热键", &hotkeys_entry);
    setup_bind_shortcut_keys(&window, HOTKEY_ACTION.to_string(), &hotkeys_entry, &config);
    page.append(&row_1);

    let close_entry = Entry::builder()
        .editable(false)
        .text(&config.borrow().shortcut.close)
        .width_request(240)
        .build();
    let row_2 = build_setting_row("关闭", &close_entry);
    setup_bind_shortcut_keys(&window, CLOSE_ACTION.to_string(), &close_entry, &config);
    page.append(&row_2);

    page
}

/// 配置 entry控件绑定快捷键
fn setup_bind_shortcut_keys(
    window: &ApplicationWindow,
    action_name: String,
    entry: &Entry,
    config: &ConfigRef,
) {
    // 监听按键，用于获取新的快捷键
    let controller = EventControllerKey::new();
    let entry_clone = entry.clone();
    let window_clone = window.clone();
    let config_clone = config.clone();
    controller.connect_key_pressed(move |_controller, key, _code, state| {
        let key_name = key.name().unwrap_or_default().to_lowercase();
        let is_modifier = matches!(
            key,
            gdk::Key::Shift_L
                | gdk::Key::Shift_R
                | gdk::Key::Control_L
                | gdk::Key::Control_R
                | gdk::Key::Alt_L
                | gdk::Key::Alt_R
                | gdk::Key::Super_L
                | gdk::Key::Super_R
                | gdk::Key::Meta_L
                | gdk::Key::Meta_R
        );
        // 如果按下的是修饰键则拦截本次操作，等待用户按下普通键
        if is_modifier {
            return Propagation::Stop;
        }
        // 处理普通键
        let modifiers = state.bits();
        let mut shortcut_str = String::default();
        if modifiers & gdk::ModifierType::SHIFT_MASK.bits() != 0 {
            shortcut_str.push_str("<Shift>");
        }
        if modifiers & gdk::ModifierType::CONTROL_MASK.bits() != 0 {
            shortcut_str.push_str("<Control>");
        }
        if modifiers & gdk::ModifierType::ALT_MASK.bits() != 0 {
            shortcut_str.push_str("<Alt>");
        }
        if modifiers & gdk::ModifierType::SUPER_MASK.bits() != 0 {
            shortcut_str.push_str("<Super>");
        }

        // 如果只按下了一个普通键没有配合修饰键则拦截本次操作
        if shortcut_str.is_empty() {
            return Propagation::Stop;
        }
        shortcut_str.push_str(&key_name);

        println!("捕获快捷键：{}", shortcut_str);
        entry_clone.set_text(&shortcut_str);
        update_window_shortcut(&window_clone, &action_name, &shortcut_str, &config_clone);

        Propagation::Stop
    });
    // 设置为捕获阶段，用于全局快捷键
    controller.set_propagation_phase(gtk::PropagationPhase::Capture);
    entry.add_controller(controller);
}

/// 更新窗口快捷键绑定
fn update_window_shortcut(
    window: &ApplicationWindow,
    action_name: &str,
    shortcut_str: &str,
    config: &ConfigRef,
) {
    // 覆盖原有的 Action
    if let Some(application) = window.application() {
        application.set_accels_for_action(&format!("win.{}", action_name), &[shortcut_str]);
        match action_name {
            HOTKEY_ACTION => config.borrow_mut().shortcut.hotkey = shortcut_str.to_string(),
            CLOSE_ACTION => config.borrow_mut().shortcut.close = shortcut_str.to_string(),
            _ => unreachable!(),
        }
        let _ = config.borrow().save();
    }
}

/// 构建 关于页面
fn build_about_page() -> Box {
    let page = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    let app_name = Label::builder()
        .label("JzLauncher")
        .halign(gtk::Align::Center)
        .css_classes(vec!["app_name"])
        .build();

    let version_label = Label::builder()
        .label("v0.1.0")
        .halign(gtk::Align::Center)
        .build();

    let code_repository_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .spacing(16)
        .build();

    let github_button = LinkButton::builder()
        .uri("https://github.com/zhoujing2023/jz-launcher.git")
        .label("")
        .child(
            &gtk::Image::builder()
                .file("launcher-gui/examples/settings_ui/github.svg")
                .pixel_size(48)
                .build(),
        )
        .build();

    let gitee_button = LinkButton::builder()
        .uri("https://gitee.com/FanGccU/jz-launcher.git")
        .label("")
        .child(
            &gtk::Image::builder()
                .file("launcher-gui/examples/settings_ui/gitee.svg")
                .pixel_size(48)
                .build(),
        )
        .build();
    code_repository_box.append(&github_button);
    code_repository_box.append(&gitee_button);

    let comment = Label::builder()
        .label("Linux 桌面程序启动器")
        .build();

    page.append(&app_name);
    page.append(&version_label);
    page.append(&comment);
    page.append(&code_repository_box);
    page
}

/// `build_setting_row` 构建 row
fn build_setting_row(label_text: &str, control: &impl IsA<gtk::Widget>) -> Box {
    let row = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(gtk::Align::Start)
        .build();

    let label = Label::builder()
        .label(label_text)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();

    row.append(&label);
    row.append(control);
    row
}

/// 初始化配置信息
fn init_config(config: &ConfigRef, window: &ApplicationWindow) {
    // 外观
    let style_manager = adw::StyleManager::default();
    apply_color_scheme(config.borrow().appearance.theme, &style_manager);

    let cur_sys_font_info = get_system_current_font_info();
    let settings = Settings::default().expect("无法获取全局 Settings");
    apply_font_size(
        &cur_sys_font_info.0,
        config.borrow().appearance.font_size,
        &settings,
    );

    // 快捷键
    if let Some(application) = window.application() {
        let hotkey_shortcut = &config.borrow().shortcut.hotkey;
        application.set_accels_for_action(&format!("win.{}", HOTKEY_ACTION), &[hotkey_shortcut]);
        let close_shortcut = &config.borrow().shortcut.close;
        application.set_accels_for_action(&format!("win.{}", CLOSE_ACTION), &[close_shortcut]);
    }
}
