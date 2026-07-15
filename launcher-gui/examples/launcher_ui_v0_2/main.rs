mod app_data_object;
mod mock_data;
mod search_result_item;

use crate::app_data_object::AppDataObject;
use crate::mock_data::mock_app_list;
use crate::search_result_item::SearchResultItem;
use adw::gdk::Key;
use adw::prelude::{
    ActionMapExt, ApplicationCommandLineExt, ApplicationExt, ApplicationExtManual, DisplayExt,
    MonitorExt,
};
use adw::{Application, gdk};
use glib::object::{Cast, CastNone};
use glib::{ExitCode, Propagation};
use gtk::gdk::Display;
use gtk::gio::ListStore;
use gtk::prelude::{
    BoxExt, ButtonExt, EditableExt, EntryExt, GtkApplicationExt, GtkWindowExt, ListItemExt,
    ListModelExt, WidgetExt,
};
use gtk::{
    ApplicationWindow, CssProvider, Entry, EventControllerKey, ListItem, ListView, Orientation,
    SignalListItemFactory, SingleSelection, gio,
};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "debug.zhoujing.jz-launcher";

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();
    app.connect_startup(|app| {
        setup_actions(app);
        println!("启动成功～");
        println!("命令：{} --toggle", get_executable_path())
    });

    let window_ref: Rc<RefCell<Option<ApplicationWindow>>> = Rc::new(RefCell::new(None));

    // 初始创建窗口
    let window_clone = window_ref.clone();
    app.connect_activate(move |app| {
        if window_clone.borrow().is_none() {
            let window = build_ui(&app);
            window.set_visible(false);
            *window_clone.borrow_mut() = Some(window);
        }
    });

    // 命令处理信号
    let window_clone = window_ref.clone();
    app.connect_command_line(move |app, cmdline| {
        let args = cmdline.arguments();
        println!("参数信息：{:#?}", args);
        let has_toggle = args
            .iter()
            .any(|arg| arg.to_string_lossy().contains("--toggle"));
        // 初始化窗口，切换显隐状态
        app.activate();
        if has_toggle {
            if let Some(window) = window_clone.borrow().as_ref() {
                if window.get_visible() {
                    hide_window(&window);
                } else {
                    show_window(&window);
                }
            }
        }
        ExitCode::SUCCESS
    });

    app.run()
}

fn hide_window(window: &ApplicationWindow) {
    window.set_visible(false);
}

fn show_window(window: &ApplicationWindow) {
    window.set_visible(true);
    window.present();
}

/// `build_ui` 构建 UI 模板
fn build_ui(app: &Application) -> ApplicationWindow {
    let (screen_width, screen_height) = get_screen_size();
    let main_box_margin_top = (screen_height as f64 * 0.20) as i32;

    let main_box = gtk::Box::builder()
        .width_request(400)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .orientation(Orientation::Vertical)
        .margin_top(main_box_margin_top)
        .spacing(5)
        .css_classes(vec!["main-box"])
        .build();

    let search_entry = Entry::builder()
        .placeholder_text("请输入内容……")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .height_request(60)
        .css_classes(vec!["search-entry"])
        .build();
    main_box.append(&search_entry);

    let list_view = ListView::builder()
        .orientation(Orientation::Vertical)
        .css_classes(vec!["result-list"])
        .build();

    // 配置模型
    let app_store = ListStore::new::<AppDataObject>();
    let selection_model = SingleSelection::new(Some(app_store.clone()));

    // 配置工厂
    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_factory, obj| {
        let search_result_item = SearchResultItem::new();
        obj.downcast_ref::<ListItem>()
            .expect("获取 ListItem 失败")
            .set_child(Some(&search_result_item))
    });

    factory.connect_bind(|_factory, obj| {
        let app_data = obj
            .downcast_ref::<ListItem>()
            .expect("获取 ListItem 失败")
            .item()
            .and_downcast::<AppDataObject>()
            .expect("获取 AppDataObject 失败");

        let search_result_item = obj
            .downcast_ref::<ListItem>()
            .expect("获取 ListItem 失败")
            .child()
            .and_downcast::<SearchResultItem>()
            .expect("获取 SearchResultItem 失败");
        // 绑定关系
        search_result_item.bind(Some(app_data));
    });

    list_view.set_model(Some(&selection_model));
    list_view.set_factory(Some(&factory));

    let scrolled_window = gtk::ScrolledWindow::builder()
        .max_content_height(600)
        .propagate_natural_height(true)
        .visible(false)
        .child(&list_view)
        .build();

    main_box.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Debug Main UI V2")
        .default_width(screen_width)
        .default_height(screen_height)
        .decorated(false)
        .css_classes(vec!["max-window"])
        .build();

    // 遮罩布局
    let overlay = gtk::Overlay::builder().build();

    // 透明背景按钮（点击隐藏）
    let background_button = gtk::Button::builder()
        .css_classes(vec!["background-button"])
        .hexpand(true)
        .vexpand(true)
        .build();

    let window_clone = window.clone();
    background_button.connect_clicked(move |_| {
        hide_window(&window_clone);
    });

    overlay.set_child(Some(&background_button));
    overlay.add_overlay(&main_box);

    window.set_child(Some(&overlay));

    // 窗口显示时聚焦搜索框 并 清空之前的数据
    let search_entry_clone = search_entry.clone();
    let app_store_clone = app_store.clone();
    window.connect_show(move |_| {
        search_entry_clone.grab_focus();
        search_entry_clone.set_text("");
        app_store_clone.remove_all();
    });

    // 拦截关闭信号
    window.connect_close_request(|window| {
        hide_window(window);
        Propagation::Stop
    });

    // 配置搜索栏回调
    setup_entry_changed_callback(&search_entry, &app_store, &scrolled_window);
    setup_entry_activate_callback(&search_entry, &selection_model, &window);
    setup_entry_keyboard_navigation_callback(&search_entry, &selection_model, &list_view);
    // 配置列表项回调
    setup_list_view_row_activated_callback(&list_view, &selection_model, &window);

    // 加载css
    load_css();
    window
}

/// `get_screen_size` 获取屏幕大小（宽高）
fn get_screen_size() -> (i32, i32) {
    let display = Display::default().expect("无法连接到显示器");
    // 获取所有显示器列表中索引为 0 的显示器 (通常是主显示器)
    let monitors = display.monitors();
    let monitor = monitors
        .item(0)
        .and_downcast::<gdk::Monitor>()
        .expect("无法获取主显示器");
    let geo = monitor.geometry();
    (geo.width(), geo.height())
}

/// 获取当前可执行文件的完整路径
/// 场景：当前用户需要配置全局快捷键时，需要知道绝对路径
fn get_executable_path() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "jz-launcher".to_string())
}

/// 回调-搜索栏数据发生变更时
fn setup_entry_changed_callback(
    search_entry: &Entry,
    list_store: &ListStore,
    scrolled_window: &gtk::ScrolledWindow,
) {
    search_entry.connect_changed(glib::clone!(
        #[strong]
        list_store,
        #[weak]
        scrolled_window,
        move |entry| {
            // 清空旧数据
            list_store.remove_all();
            // 查询匹配
            let keyword = entry.text().to_string();
            println!("keyword: {}", keyword);
            if keyword.is_empty() {
                println!("输入的内容为空，取消查询操作");
                scrolled_window.set_visible(false);
                return;
            }

            mock_app_list().into_iter().for_each(|app| {
                if let Some(name) = app.name() {
                    if name.to_lowercase().contains(&keyword.to_lowercase()) {
                        list_store.append(&app);
                    }
                }
            });
            scrolled_window.set_visible(list_store.n_items() > 0);
        }
    ));
}

/// 回调-搜索栏被激活时
fn setup_entry_activate_callback(
    entry: &Entry,
    selection: &SingleSelection,
    window: &ApplicationWindow,
) {
    entry.connect_activate(glib::clone!(
        #[weak]
        selection,
        #[weak]
        window,
        move |_| exec_selected_item_app(&selection, &window)
    ));
}

/// 执行选中项的应用程序
fn exec_selected_item_app(selection: &SingleSelection, window: &ApplicationWindow) {
    let Some(data) = selection.selected_item() else {
        eprintln!("没有选中的项");
        return;
    };
    let app_data = data
        .downcast_ref::<AppDataObject>()
        .expect("转换为 AppDataObject 失败");
    println!("执行：{}", app_data.exec_cmd().unwrap());
    // TODO: 执行指令

    // 隐藏窗口
    let window_clone = window.clone();
    glib::idle_add_local_once(move || {
        hide_window(&window_clone);
    });
}

/// 回调-搜索栏键盘输入监听
/// 监听 Up / Down 方向键，控制列表选项上下切换
fn setup_entry_keyboard_navigation_callback(
    entry: &Entry,
    selection: &SingleSelection,
    list_view: &ListView,
) {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(glib::clone!(
        #[weak]
        selection,
        #[weak]
        list_view,
        #[upgrade_or]
        Propagation::Proceed,
        move |_controller, key, _code, _state| {
            return match key {
                Key::Up | Key::Down => {
                    handle_list_navigation(key, &selection, &list_view);
                    Propagation::Stop
                }
                _ => Propagation::Proceed,
            };
        }
    ));
    entry.add_controller(controller);
}

/// 处理 Up / Down 键切换列表选择项
fn handle_list_navigation(key: Key, selection: &SingleSelection, list_view: &ListView) {
    if selection.n_items() < 1 {
        // 结果项为空，不执行操作
        return;
    }
    let selected_index = selection.selected() as i32;
    let new_selected_index = if key == Key::Up {
        selected_index - 1
    } else {
        selected_index + 1
    };
    // 限制最小和最大边界
    let new_selected_index = new_selected_index.max(0) as u32;
    let new_selected_index = new_selected_index.min(selection.n_items() - 1);
    selection.set_selected(new_selected_index);
    list_view.scroll_to(new_selected_index, gtk::ListScrollFlags::FOCUS, None);
}

/// 回调-列表视图激活（双击）
fn setup_list_view_row_activated_callback(
    list_view: &ListView,
    selection: &SingleSelection,
    window: &ApplicationWindow,
) {
    list_view.connect_activate(glib::clone!(
        #[weak]
        selection,
        #[weak]
        window,
        move |_, _index| exec_selected_item_app(&selection, &window)
    ));
}

/// 配置 Action
fn setup_actions(app: &Application) {
    let quit_action = gio::SimpleAction::new("quit", None);
    let app_clone = app.clone();
    quit_action.connect_activate(move |_, _| {
        println!("退出程序");
        app_clone.quit();
    });
    app.add_action(&quit_action);
    app.set_accels_for_action("app.quit", &["<Ctrl>q"]);
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
