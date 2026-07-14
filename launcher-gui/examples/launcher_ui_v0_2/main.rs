mod app_data_object;
mod mock_data;
mod search_result_item;

use crate::app_data_object::AppDataObject;
use crate::mock_data::mock_app_list;
use crate::search_result_item::SearchResultItem;
use adw::Application;
use adw::gdk::Key;
use adw::prelude::{ApplicationExt, ApplicationExtManual};
use glib::object::{Cast, CastNone};
use glib::{ExitCode, Propagation};
use gtk::gdk::Display;
use gtk::gio::ListStore;
use gtk::prelude::{
    BoxExt, EditableExt, EntryExt, GtkApplicationExt, GtkWindowExt, ListItemExt, ListModelExt,
    WidgetExt,
};
use gtk::{
    ApplicationWindow, CssProvider, Entry, EventControllerKey, ListItem, ListView, Orientation,
    SignalListItemFactory, SingleSelection,
};

const APP_ID: &str = "debug.zhoujing.jz_tools";

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
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();

    let search_entry = Entry::builder()
        .placeholder_text("请输入内容……")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .height_request(50)
        .css_classes(vec!["search-entry"])
        .build();
    main_box.append(&search_entry);

    let list_view = ListView::builder()
        .orientation(Orientation::Vertical)
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
        .max_content_height(400)
        .propagate_natural_height(true)
        .child(&list_view)
        .build();

    main_box.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Debug Main UI V2")
        .child(&main_box)
        .width_request(400)
        .decorated(false)
        .build();

    // 初始化控件高度
    reload_search_list_height(0, &scrolled_window, &window);
    // 配置搜索栏回调
    setup_entry_changed_callback(&search_entry, &app_store, &scrolled_window, &window);
    setup_entry_activate_callback(&search_entry, &selection_model);
    setup_entry_keyboard_navigation_callback(&search_entry, &selection_model, &list_view);
    // 配置列表项回调
    setup_list_view_row_activated_callback(&list_view, &selection_model);

    window.present();
}

/// 回调-搜索栏数据发生变更时
fn setup_entry_changed_callback(
    search_entry: &Entry,
    list_store: &ListStore,
    scrolled_window: &gtk::ScrolledWindow,
    window: &ApplicationWindow,
) {
    search_entry.connect_changed(glib::clone!(
        #[strong]
        list_store,
        #[weak]
        scrolled_window,
        #[weak]
        window,
        move |entry| {
            // 清空旧数据
            list_store.remove_all();
            // 查询匹配
            let keyword = entry.text().to_string();
            println!("keyword: {}", keyword);
            if keyword.is_empty() {
                println!("输入的内容为空，取消查询操作");
                scrolled_window.set_height_request(0);
                // 刷新控件高度
                reload_search_list_height(0, &scrolled_window, &window);
                return;
            }

            mock_app_list().into_iter().for_each(|app| {
                if let Some(name) = app.name() {
                    if name.to_lowercase().contains(&keyword.to_lowercase()) {
                        list_store.append(&app);
                    }
                }
            });
            // 刷新控件高度
            reload_search_list_height(list_store.n_items(), &scrolled_window, &window);
        }
    ));
}

/// 强制刷新高度
/// 注意：window 和 其它控件不同，它是只增不减的类型（只管放大不管缩小），所以此处强制使 window 重新计算高度
/// 这里的 -1 是个固定值，将 window 高度设置成最小，再通过 queue_resize 根据子控件的高度计算 Window 的高度
fn reload_search_list_height(
    item_count: u32,
    scrolled_window: &gtk::ScrolledWindow,
    window: &ApplicationWindow,
) {
    scrolled_window.set_visible(item_count > 0);
    window.set_default_size(400, -1);
    window.queue_resize();
}

/// 回调-搜索栏被激活时
fn setup_entry_activate_callback(entry: &Entry, selection: &SingleSelection) {
    entry.connect_activate(glib::clone!(
        #[weak]
        selection,
        move |_| exec_selected_item_app(&selection)
    ));
}

/// 执行选中项的应用程序
fn exec_selected_item_app(selection: &SingleSelection) {
    let Some(data) = selection.selected_item() else {
        eprintln!("没有选中的项");
        return;
    };
    let app_data = data
        .downcast_ref::<AppDataObject>()
        .expect("转换为 AppDataObject 失败");
    println!("执行：{}", app_data.exec_cmd().unwrap());
    // TODO: 执行指令
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
fn setup_list_view_row_activated_callback(list_view: &ListView, selection: &SingleSelection) {
    list_view.connect_activate(glib::clone!(
        #[weak]
        selection,
        move |_, _index| exec_selected_item_app(&selection)
    ));
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
