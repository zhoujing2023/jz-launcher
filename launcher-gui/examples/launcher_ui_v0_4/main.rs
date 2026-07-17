use adw::prelude::*;
use glib::{ExitCode, Propagation};
use gtk::{
    gdk, gio, glib, Application, Box, Entry, Label, ListBox, ListBoxRow, Orientation,
    Window, Overlay,
};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.zhoujing.fullscreen_launcher";

fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    // 创建共享状态
    let app_state: Rc<RefCell<Option<Window>>> = Rc::new(RefCell::new(None));

    let state_clone = app_state.clone();
    app.connect_startup(move |app| {
        // 注册全局快捷键（Shift+Space）
        app.set_accels_for_action("app.toggle", &["<Shift>space"]);

        // 添加 toggle action
        let action = gio::SimpleAction::new("toggle", None);
        let state_clone2 = state_clone.clone();
        action.connect_activate(move |_, _| {
            if let Some(window) = state_clone2.borrow().as_ref() {
                toggle_window(window);
            }
        });
        app.add_action(&action);
    });

    let state_clone = app_state.clone();
    app.connect_activate(move |app| {
        let window = build_ui(app);
        // 立即显示用于测试（实际使用时应该隐藏）
        window.present();
        *state_clone.borrow_mut() = Some(window);
    });

    app.run()
}

fn build_ui(app: &Application) -> Window {
    // 创建全屏无装饰窗口
    let window = Window::builder()
        .application(app)
        .decorated(false)
        .default_width(1920)
        .default_height(1080)
        .css_classes(vec!["fullscreen-launcher"])
        .build();

    // 创建 Overlay 用于层叠布局
    let overlay = Overlay::new();

    // 半透明背景（点击关闭）
    let background = gtk::Button::builder()
        .css_classes(vec!["transparent-background"])
        .hexpand(true)
        .vexpand(true)
        .build();

    let window_clone = window.clone();
    background.connect_clicked(move |_| {
        window_clone.close();
    });

    // 主内容容器（居中显示）
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .margin_top(200) // 距离顶部 200px
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

    // 创建结果列表容器
    let list_scroll = gtk::ScrolledWindow::builder()
        .max_content_height(500)
        .propagate_natural_height(true)
        .css_classes(vec!["results-scroll"])
        .build();

    let list_box = ListBox::builder()
        .css_classes(vec!["results-list"])
        .build();

    list_scroll.set_child(Some(&list_box));

    // 模拟数据
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

    // 初始化显示所有应用
    for (app_name, icon_name) in &apps {
        add_app_to_list(&list_box, app_name, icon_name);
    }

    // 默认选中第一个
    if let Some(row) = list_box.row_at_index(0) {
        list_box.select_row(Some(&row));
    }

    // 搜索过滤
    let list_clone = list_box.clone();
    let apps_clone = apps.clone();
    search_entry.connect_changed(move |entry| {
        let keyword = entry.text().to_string().to_lowercase();

        // 清空列表
        while let Some(child) = list_clone.first_child() {
            list_clone.remove(&child);
        }

        // 添加匹配的应用
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

    // 回车确认选中项
    let window_clone = window.clone();
    let list_clone = list_box.clone();
    search_entry.connect_activate(move |_| {
        if let Some(row) = list_clone.selected_row() {
            if let Some(hbox) = row.child().and_downcast::<Box>() {
                if let Some(vbox) = hbox.last_child().and_downcast::<Box>() {
                    if let Some(label) = vbox.first_child().and_downcast::<Label>() {
                        let app_name = label.text().to_string();
                        println!("启动应用: {}", app_name);
                        
                        // 延迟关闭窗口
                        let window = window_clone.clone();
                        glib::idle_add_local_once(move || {
                            window.close();
                        });
                    }
                }
            }
        }
    });

    // 键盘导航：方向键控制列表
    let list_clone = list_box.clone();
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
                // ESC 关闭窗口
                if let Some(window) = list_clone.root().and_downcast::<Window>() {
                    window.close();
                }
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
                    println!("启动应用: {}", app_name);
                    
                    // 延迟关闭窗口
                    let window = window_clone.clone();
                    glib::idle_add_local_once(move || {
                        window.close();
                    });
                }
            }
        }
    });

    content_box.append(&search_entry);
    content_box.append(&list_scroll);

    // 组装 Overlay
    overlay.set_child(Some(&background));
    overlay.add_overlay(&content_box);

    window.set_child(Some(&overlay));

    // 窗口显示时聚焦搜索框
    let search_entry_clone = search_entry.clone();
    window.connect_show(move |_| {
        search_entry_clone.grab_focus();
    });

    // 加载 CSS 样式
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

    // 图标
    let icon = gtk::Image::builder()
        .icon_name(icon_name)
        .pixel_size(48)
        .build();

    // 文本容器
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

fn toggle_window(window: &Window) {
    if window.is_visible() {
        window.close();
    } else {
        window.present();
    }
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(
        "
        /* 全屏半透明背景 */
        .fullscreen-launcher {
            background: transparent;
        }
        
        .transparent-background {
            background: rgba(0, 0, 0, 0.0);
            border: none;
            box-shadow: none;
        }
        
        .transparent-background:hover {
            background: rgba(0, 0, 0, 0.0);
        }
        
        /* 主内容容器 */
        .launcher-content {
            background: rgba(30, 30, 30, 0.95);
            border-radius: 16px;
            padding: 16px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
        }
        
        /* 搜索框 */
        .search-entry {
            background: rgba(50, 50, 50, 0.95);
            border: 2px solid rgba(100, 100, 100, 0.5);
            border-radius: 12px;
            color: white;
            font-size: 18px;
            padding: 12px 16px;
        }
        
        .search-entry:focus {
            border-color: rgba(52, 120, 246, 0.8);
            box-shadow: 0 0 0 3px rgba(52, 120, 246, 0.3);
        }
        
        /* 结果列表滚动区域 */
        .results-scroll {
            background: transparent;
            border: none;
        }
        
        /* 结果列表 */
        .results-list {
            background: rgba(40, 40, 40, 0.95);
            border-radius: 12px;
        }
        
        .results-list row {
            background: transparent;
            border: none;
        }
        
        /* 应用行 */
        .app-row {
            border-radius: 8px;
            margin: 4px;
            transition: all 200ms;
        }
        
        .app-row:hover {
            background: rgba(70, 70, 70, 0.8);
        }
        
        .app-row:selected {
            background: rgba(52, 120, 246, 0.6);
        }
        
        .app-row:selected:hover {
            background: rgba(52, 120, 246, 0.7);
        }
        
        /* 应用名称 */
        .app-name {
            color: white;
            font-size: 16px;
            font-weight: 500;
        }
        ",
    );

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("无法连接到显示器"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
