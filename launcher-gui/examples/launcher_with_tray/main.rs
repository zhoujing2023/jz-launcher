use adw::prelude::*;
use glib::{ExitCode, Propagation};
use gtk::{
    Application, Box, Entry, Label, ListBox, ListBoxRow, Orientation, Overlay, Window, gdk, gio,
    glib,
};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.zhoujing.jz_launcher";

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        // 允许自定义参数 如：以下指令在启动项目时指定参数为 ”toggle“
        // cargo run --example launcher_with_tray -- --toggle
        // 后续使用 connect_command_line 信号对参数进行解析
        // 注意：启用了此配置后 Application 的 activate 不会自动执行，需自行手动调用
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    // 存储窗口引用
    let window_ref: Rc<RefCell<Option<Window>>> = Rc::new(RefCell::new(None));

    let window_ref_clone = window_ref.clone();
    app.connect_startup(move |app| {
        // 添加动作
        setup_actions(app, window_ref_clone.clone());

        println!("🚀 JZ Launcher 已启动");
        println!();
        println!("📌 在 Ubuntu 设置 → 键盘 → 自定义快捷键 中添加：");
        println!("   名称: JZ Launcher");
        println!("   命令: {} --toggle", get_executable_path());
        println!("   快捷键: 按你喜欢的组合键 (如 Super+Space)");
        println!();
        println!("📌 或者在终端运行:");
        println!("   cargo run --example launcher_with_tray -- --toggle");
        println!();
        println!("📌 Ctrl+Q 退出程序");
    });

    // 由于上面启用了 ApplicationFlags::HANDLES_COMMAND_LINE 所以此信号在项目启动时不会自动触发，需在其它方法中手动触发
    // 由于 app.connect_command_line 方法每次触发时都会调用此方法，所以使用 is_none 进行校验，避免重复创建窗口
    // 初次创建窗口时将状态设置为隐藏，因为 connect_command_line 函数后面会进行 toggle 操作
    let window_ref_clone = window_ref.clone();
    app.connect_activate(move |app| {
        println!("hello");
        // 如果窗口还不存在，创建它
        if window_ref_clone.borrow().is_none() {
            let window = build_ui(app);
            window.set_visible(false);
            *window_ref_clone.borrow_mut() = Some(window);
        }
    });

    // 处理命令行参数
    // 由于上面启用了 ApplicationFlags::HANDLES_COMMAND_LINE 标准，所以在项目启动时会自动触发此信号
    let window_ref_clone = window_ref.clone();
    app.connect_command_line(move |app, cmdline| {
        let args = cmdline.arguments();
        println!("参数信息：{:#?}", args);
        let has_toggle = args
            .iter()
            .any(|arg| arg.to_string_lossy().contains("--toggle"));

        // 为什么此处需要手动调用 Application 的 activate 信号？
        // 当 GTK 应用同时满足以下两个条件时，Application 的 activate 不会自动执行：
        // 1.应用设置了 ApplicationFlags::HANDLES_COMMAND_LINE
        // 2.通过命名行启动（即使没有传递任何参数）
        // 创建窗口
        app.activate();

        // 如果有 --toggle 参数，切换窗口显示
        if has_toggle {
            if let Some(window) = window_ref_clone.borrow().as_ref() {
                if window.is_visible() {
                    hide_window(window);
                } else {
                    show_window(window);
                }
            }
        }

        ExitCode::new(0) // 返回 0 表示成功
    });

    app.run()
}

// 获取当前可执行文件的完整路径
// 场景：当前用户需要配置全局快捷键时，需要知道绝对路径
fn get_executable_path() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "jz-launcher".to_string())
}

fn build_ui(app: &Application) -> Window {
    // 创建全屏透明窗口
    let window = Window::builder()
        .application(app)
        .decorated(false)
        .default_width(1920)
        .default_height(1080)
        .css_classes(vec!["fullscreen-launcher"])
        .build();

    // 创建 Overlay
    let overlay = Overlay::new();

    // 半透明背景
    let background = gtk::Button::builder()
        .css_classes(vec!["transparent-background"])
        .hexpand(true)
        .vexpand(true)
        .build();

    let window_clone = window.clone();
    background.connect_clicked(move |_| {
        hide_window(&window_clone);
    });

    // 主内容容器
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .margin_top(200)
        .spacing(12)
        .width_request(600)
        .css_classes(vec!["launcher-content"])
        .build();

    // 搜索框
    let search_entry = Entry::builder()
        .placeholder_text("搜索应用...")
        .height_request(60)
        .css_classes(vec!["search-entry"])
        .build();

    // 结果列表
    let list_scroll = gtk::ScrolledWindow::builder()
        .max_content_height(500)
        .propagate_natural_height(true)
        .css_classes(vec!["results-scroll"])
        .build();

    let list_box = ListBox::builder().css_classes(vec!["results-list"]).build();

    list_scroll.set_child(Some(&list_box));

    // 模拟应用数据
    let apps = vec![
        ("终端", "utilities-terminal"),
        ("文件管理器", "system-file-manager"),
        ("浏览器", "web-browser"),
        ("VSCode", "code"),
        ("Typora", "typora"),
        ("GIMP", "gimp"),
        ("Krita", "krita"),
        ("Blender", "blender"),
    ];

    // 初始化列表
    for (app_name, icon_name) in &apps {
        add_app_to_list(&list_box, app_name, icon_name);
    }

    if let Some(row) = list_box.row_at_index(0) {
        list_box.select_row(Some(&row));
    }

    // 搜索过滤
    let list_clone = list_box.clone();
    let apps_clone = apps.clone();
    search_entry.connect_changed(move |entry| {
        let keyword = entry.text().to_string().to_lowercase();

        while let Some(child) = list_clone.first_child() {
            list_clone.remove(&child);
        }

        let mut first = true;
        for (app_name, icon_name) in &apps_clone {
            if app_name.to_lowercase().contains(&keyword) {
                add_app_to_list(&list_clone, app_name, icon_name);
                if first {
                    if let Some(row) = list_clone.row_at_index(0) {
                        list_clone.select_row(Some(&row));
                    }
                    first = false;
                }
            }
        }
    });

    // 回车确认
    let window_clone = window.clone();
    let list_clone = list_box.clone();
    search_entry.connect_activate(move |_| {
        if let Some(row) = list_clone.selected_row() {
            if let Some(hbox) = row.child().and_downcast::<Box>() {
                if let Some(vbox) = hbox.last_child().and_downcast::<Box>() {
                    if let Some(label) = vbox.first_child().and_downcast::<Label>() {
                        let app_name = label.text().to_string();
                        println!("🚀 启动应用: {}", app_name);

                        let window = window_clone.clone();
                        glib::idle_add_local_once(move || {
                            hide_window(&window);
                        });
                    }
                }
            }
        }
    });

    // 键盘导航
    let list_clone = list_box.clone();
    let window_clone = window.clone();
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Down => {
                if let Some(current_row) = list_clone.selected_row() {
                    let current_index = current_row.index();
                    if let Some(next_row) = list_clone.row_at_index(current_index + 1) {
                        list_clone.select_row(Some(&next_row));
                    }
                }
                return Propagation::Stop;
            }
            gdk::Key::Up => {
                if let Some(current_row) = list_clone.selected_row() {
                    let current_index = current_row.index();
                    if current_index > 0 {
                        if let Some(prev_row) = list_clone.row_at_index(current_index - 1) {
                            list_clone.select_row(Some(&prev_row));
                        }
                    }
                }
                return Propagation::Stop;
            }
            gdk::Key::Escape => {
                hide_window(&window_clone);
                return Propagation::Stop;
            }
            _ => {}
        }
        Propagation::Proceed
    });
    search_entry.add_controller(key_controller);

    // 列表项激活
    let window_clone = window.clone();
    list_box.connect_row_activated(move |_, row| {
        if let Some(hbox) = row.child().and_downcast::<Box>() {
            if let Some(vbox) = hbox.last_child().and_downcast::<Box>() {
                if let Some(label) = vbox.first_child().and_downcast::<Label>() {
                    let app_name = label.text().to_string();
                    println!("🚀 启动应用: {}", app_name);

                    let window = window_clone.clone();
                    glib::idle_add_local_once(move || {
                        hide_window(&window);
                    });
                }
            }
        }
    });

    content_box.append(&search_entry);
    content_box.append(&list_scroll);

    overlay.set_child(Some(&background));
    overlay.add_overlay(&content_box);

    window.set_child(Some(&overlay));

    // 拦截关闭信号，改为隐藏
    window.connect_close_request(|window| {
        hide_window(window);
        Propagation::Stop
    });

    // 显示时聚焦搜索框并重置状态
    let search_entry_clone = search_entry.clone();
    let list_box_clone = list_box.clone();
    let apps_clone = apps.clone();
    window.connect_show(move |_| {
        // 清空搜索框
        search_entry_clone.set_text("");

        // 重置列表
        while let Some(child) = list_box_clone.first_child() {
            list_box_clone.remove(&child);
        }
        for (app_name, icon_name) in &apps_clone {
            add_app_to_list(&list_box_clone, app_name, icon_name);
        }
        if let Some(row) = list_box_clone.row_at_index(0) {
            list_box_clone.select_row(Some(&row));
        }

        // 聚焦搜索框
        search_entry_clone.grab_focus();
    });

    load_css();

    window
}

fn add_app_to_list(list_box: &ListBox, app_name: &str, icon_name: &str) {
    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_start(12)
        .margin_end(12)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let icon = gtk::Image::builder()
        .icon_name(icon_name)
        .pixel_size(48)
        .build();

    let text_box = Box::builder()
        .orientation(Orientation::Vertical)
        .valign(gtk::Align::Center)
        .build();

    let name_label = Label::builder()
        .label(app_name)
        .halign(gtk::Align::Start)
        .css_classes(vec!["app-name"])
        .build();

    text_box.append(&name_label);
    row_box.append(&icon);
    row_box.append(&text_box);

    let row = ListBoxRow::builder()
        .child(&row_box)
        .css_classes(vec!["app-row"])
        .build();

    list_box.append(&row);
}

fn hide_window(window: &Window) {
    println!("👻 隐藏窗口（程序仍在后台运行）");
    window.set_visible(false);
}

fn show_window(window: &Window) {
    println!("👁️  显示窗口");
    window.set_visible(true);
    window.present();
}

// 配置动作
fn setup_actions(app: &Application, window_ref: Rc<RefCell<Option<Window>>>) {
    // Toggle 动作
    // let toggle_action = gio::SimpleAction::new("toggle", None);
    // let window_ref_clone = window_ref.clone();
    // toggle_action.connect_activate(move |_, _| {
    //     if let Some(launcher_window) = window_ref_clone.borrow().as_ref() {
    //         if launcher_window.is_visible() {
    //             hide_window(launcher_window);
    //         } else {
    //             show_window(launcher_window);
    //         }
    //     }
    // });
    // app.add_action(&toggle_action);

    // Quit 动作
    let quit_action = gio::SimpleAction::new("quit", None);
    let app_clone = app.clone();
    quit_action.connect_activate(move |_, _| {
        println!("👋 退出程序");
        app_clone.quit();
    });
    app.add_action(&quit_action);

    // 注册 Ctrl+Q 快捷键用于退出
    app.set_accels_for_action("app.quit", &["<Ctrl>Q"]);
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(
        "
        .fullscreen-launcher {
            background: transparent;
        }
        
        .transparent-background {
            background: rgba(0, 0, 0, 0.75);
            border: none;
            box-shadow: none;
        }
        
        .transparent-background:hover {
            background: rgba(0, 0, 0, 0.75);
        }
        
        .launcher-content {
            background: rgba(30, 30, 30, 0.96);
            border-radius: 20px;
            padding: 20px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.6);
        }
        
        .search-entry {
            background: rgba(50, 50, 50, 0.98);
            border: 2px solid rgba(100, 100, 100, 0.5);
            border-radius: 14px;
            color: white;
            font-size: 20px;
            padding: 14px 18px;
        }
        
        .search-entry:focus {
            border-color: rgba(52, 120, 246, 0.9);
            box-shadow: 0 0 0 4px rgba(52, 120, 246, 0.25);
        }
        
        .results-scroll {
            background: transparent;
            border: none;
        }
        
        .results-list {
            background: rgba(40, 40, 40, 0.96);
            border-radius: 14px;
        }
        
        .results-list row {
            background: transparent;
            border: none;
        }
        
        .app-row {
            border-radius: 10px;
            margin: 6px;
            transition: all 150ms cubic-bezier(0.4, 0.0, 0.2, 1);
        }
        
        .app-row:hover {
            background: rgba(70, 70, 70, 0.9);
            transform: scale(1.02);
        }
        
        .app-row:selected {
            background: rgba(52, 120, 246, 0.7);
        }
        
        .app-row:selected:hover {
            background: rgba(52, 120, 246, 0.85);
        }
        
        .app-name {
            color: white;
            font-size: 17px;
            font-weight: 600;
        }
        ",
    );

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("无法连接到显示器"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
