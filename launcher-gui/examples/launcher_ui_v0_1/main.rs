mod mock_data;

use crate::mock_data::mock_app_list;
use adw::gdk::Key;
use adw::prelude::{ApplicationExt, ApplicationExtManual};
use adw::Application;
use glib::object::CastNone;
use glib::{ExitCode, Propagation};
use gtk::gdk::Display;
use gtk::pango::EllipsizeMode;
use gtk::prelude::{BoxExt, EditableExt, EntryExt, GtkApplicationExt, GtkWindowExt, ListBoxRowExt, WidgetExt};
use gtk::{Align, ApplicationWindow, Box, CssProvider, Entry, EventControllerKey, Image, Label, ListBox, ListBoxRow, Orientation};

const APP_ID: &str = "debug.zhoujing.jz-launcher";

pub struct AppDataObject {
    pub name: String,
    pub icon: String,
    pub exec_cmd: String,
    pub comment: String,
}

impl AppDataObject {
    pub fn new(name: &str, icon: &str, exec_cmd: &str, comment: &str) -> Self {
        Self {
            name: name.to_string(),
            icon: icon.to_string(),
            exec_cmd: exec_cmd.to_string(),
            comment: comment.to_string(),
        }
    }
}

/// 调试 UI
fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| {
        load_css();
    });
    app.connect_activate(debug_build_ui);
    app.set_accels_for_action("window.close", &["Escape"]);
    app.run()
}

/// `debug_build_ui` 构建调试时的 UI 模板
fn debug_build_ui(app: &Application) {
    let main_box = gtk::Box::builder()
        .css_classes(vec!["main_box"])
        .orientation(Orientation::Vertical)
        .build();

    let list_box = ListBox::builder()
        .margin_bottom(3)
        .css_name("search-list")
        .build();

    let search_entry = Entry::builder()
        .placeholder_text("输入内容……")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(12)
        .margin_end(12)
        .height_request(50)
        .css_classes(vec!["search-entry"])
        .build();

    main_box.append(&search_entry);
    main_box.append(&list_box);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Debug Main UI V1")
        .child(&main_box)
        .width_request(400)
        .decorated(false)
        .build();

    // 配置搜索栏回调
    setup_entry_changed_callback(&search_entry, &list_box, &window);
    setup_entry_activate_callback(&search_entry, &list_box);
    setup_entry_keyboard_navigation_callback(&search_entry, &list_box);
    // 配置列表项回调
    setup_list_box_row_activated_callback(&list_box);
    window.present();
}

/// 回调-搜索栏数据发生变更时
fn setup_entry_changed_callback(
    search_entry: &Entry,
    list_box: &ListBox,
    window: &ApplicationWindow,
) {
    search_entry.connect_changed(glib::clone!(
        #[weak]
        list_box,
        #[weak]
        window,
        move |entry| {
            // 清空旧数据
            list_box.remove_all();
            // 查询匹配
            let keyword = entry.text().to_string();
            println!("keyword: {}", keyword);
            if keyword.is_empty() {
                println!("输入的内容为空，取消查询操作");
                // 强制刷新高度
                // 注意：window 和 其它控件不同，它是只增不减的类型（只管放大不管缩小），所以此处强制使 window 重新计算高度
                // 这里的 -1 是个固定值，将 window 高度设置成最小，再通过 queue_resize 根据子控件的高度计算 Window 的高度
                window.set_default_size(400, -1);
                window.queue_resize();
                return;
            }
            // 简单搜索
            let apps: Vec<AppDataObject> = mock_app_list()
                .into_iter()
                .filter(|app| app.name.to_lowercase().contains(&keyword.to_lowercase()))
                .collect();
            for (index, app) in apps.iter().enumerate() {
                let icon = Image::builder().pixel_size(50).build();
                if app.icon.contains('/') {
                    icon.set_from_file(Some(&app.icon));
                } else {
                    icon.set_icon_name(Some(&app.icon));
                }
                let name_label = Label::builder()
                    .label(&app.name)
                    .halign(Align::Start)
                    .build();
                let comment_label = Label::builder()
                    .label(&app.comment)
                    .halign(Align::Start)
                    .max_width_chars(40)
                    .ellipsize(EllipsizeMode::End)
                    .build();
                let exe_cmd = Label::builder().label(&app.exec_cmd).visible(false).build();
                let text_box = Box::builder()
                    .spacing(5)
                    .orientation(Orientation::Vertical)
                    .build();
                text_box.append(&name_label);
                text_box.append(&comment_label);
                text_box.append(&exe_cmd);

                let result_item = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .spacing(10)
                    .margin_top(5)
                    .margin_bottom(5)
                    .margin_start(8)
                    .margin_end(8)
                    .build();
                result_item.append(&icon);
                result_item.append(&text_box);

                let box_row = ListBoxRow::builder().child(&result_item).build();
                list_box.append(&box_row);
                // 默认选中第一个
                if index == 0 {
                    list_box.select_row(Some(&box_row));
                }
            }
            // 强制刷新高度
            window.set_default_size(400, -1);
            window.queue_resize();
        }
    ));
}

/// 回调-搜索栏被激活时
fn setup_entry_activate_callback(entry: &Entry, list_box: &ListBox) {
    entry.connect_activate(glib::clone!(
        #[weak]
        list_box,
        move |_| {
            let box_row = list_box.selected_row();
            let Some(box_row) = box_row else {
                eprintln!("没有选中的项");
                return;
            };
            let exe_cmd_label = box_row
                .child()
                .and_downcast_ref::<gtk::Box>()
                .cloned()
                .unwrap()
                .last_child()
                .and_downcast_ref::<gtk::Box>()
                .cloned()
                .unwrap()
                .last_child()
                .and_downcast_ref::<Label>()
                .cloned()
                .unwrap();
            println!("执行指令： {}", exe_cmd_label.text().to_string());
        }
    ));
}

/// 回调-搜索栏键盘输入监听
/// 监听 Up / Down 方向键，控制列表选项上下切换
fn setup_entry_keyboard_navigation_callback(entry: &Entry, list_box: &ListBox) {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(glib::clone!(
        #[weak]
        list_box,
        #[upgrade_or]
        Propagation::Proceed,
        move |_controller, key, _code, _state| {
            return match key {
                Key::Up | Key::Down => {
                    handle_list_navigation(key, &list_box);
                    Propagation::Stop
                }
                _ => Propagation::Proceed,
            };
        }
    ));
    entry.add_controller(controller);
}

/// 处理 Up / Down 键切换列表选择项
fn handle_list_navigation(key: Key, list_box: &ListBox) {
    let selected_index = list_box.selected_row().map_or(0, |row| row.index());
    let new_index = if key == Key::Up {
        selected_index - 1
    } else {
        selected_index + 1
    };
    let Some(box_row) = list_box.row_at_index(new_index) else {
        eprintln!("无选项，无法进行切换操作");
        return;
    };
    list_box.select_row(Some(&box_row));
}

/// 回调-列表项被激活时
fn setup_list_box_row_activated_callback(list_box: &ListBox) {
    list_box.connect_row_activated(|_, box_row| {
        let app_box = box_row
            .child()
            .and_downcast_ref::<gtk::Box>()
            .cloned()
            .unwrap();
        let exe_cmd_label = app_box
            .last_child()
            .and_downcast_ref::<gtk::Box>()
            .cloned()
            .expect("获取内容 box 失败")
            .last_child()
            .and_downcast_ref::<Label>()
            .cloned()
            .expect("获取 cmd Label失败");
        println!("执行指令： {}", exe_cmd_label.text().to_string());
        // TODO: 执行 cmd 指令
    });
}

/// 加载 CSS
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
