use crate::app_data_object::AppDataObject;
use crate::app_provider::to_data_object;
use crate::search_result_item::SearchResultItem;
use adw::gdk;
use adw::gdk::Display;
use adw::prelude::{DisplayExt, ListModelExt, MonitorExt};
use glib::Propagation;
use glib::prelude::{Cast, CastNone};
use glib::subclass::InitializingObject;
use gtk::gdk::Key;
use gtk::gio::ListStore;
use gtk::prelude::{ButtonExt, EditableExt, EntryExt, GtkWindowExt, ListItemExt, WidgetExt};
use gtk::subclass::prelude::*;
use gtk::{
    CompositeTemplate, EventControllerKey, ListItem, ListScrollFlags, SignalListItemFactory,
    SingleSelection, glib,
};
use launcher_core::{AppLoader, AppRunner, AppUsage, Env, SearchEngine};
use std::cell::RefCell;

// 窗口状态对象
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/zhoujing/jz-launcher/ui/window.ui")]
pub struct Window {
    #[template_child]
    pub background_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub main_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub search_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    #[template_child]
    pub result_list: TemplateChild<gtk::ListView>,
    pub app_list: RefCell<Option<ListStore>>,
    // core 属性
    env: RefCell<Env>,
    search_engine: RefCell<SearchEngine>,
}

// GObject 子类化
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // 绑定模板
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        // 初始化模板
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        // 先让父类完成构造
        self.parent_constructed();

        // 加载 core 数据： 1.加载环境信息 2.加载应用程序 3.初始化搜索
        *self.env.borrow_mut() = Env::load().expect("加载环境失败");
        let env = self.env.borrow();
        let apps = AppLoader::load(&env);
        *self.search_engine.borrow_mut() = SearchEngine::new(apps);

        // 配置 gui 数据： 1.配置 ListView 模式和工厂 2.事件绑定 3.初始化控件样式
        self.setup_model();
        self.setup_factory();
        self.setup_callbacks();
        self.init_widget_style();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

// 自定义方法实现
impl Window {
    fn init_widget_style(&self) {
        let (screen_width, screen_height) = Self::get_screen_size();
        let main_box_margin_top = (screen_height as f64 * 0.20) as i32;
        self.main_box.get().set_margin_top(main_box_margin_top);

        let window = self.obj();
        window.set_default_width(screen_width);
        window.set_default_height(screen_height);
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

    /// `get_app_list` 获取“搜索结果列表”数据模型
    fn get_app_list(&self) -> ListStore {
        self.app_list
            .borrow()
            .clone()
            .expect("获取 app_list 失败，app_list 为空")
    }

    /// `setup_model` 配置“搜索结果列表”模型
    fn setup_model(&self) {
        // 创建并配置 store 存储的数据类型为 AppDataObject
        let store = ListStore::new::<AppDataObject>();
        self.app_list.replace(Some(store));

        let selection = SingleSelection::new(Some(self.get_app_list()));
        self.result_list.set_model(Some(&selection));
    }

    /// `setup_factory` 配置“搜索结果列表”工厂
    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        // 创建模板
        factory.connect_setup(move |_, list_item| {
            let search_result_item = SearchResultItem::default();
            list_item
                .downcast_ref::<ListItem>()
                .expect("转换为 GtkListItem 失败")
                .set_child(Some(&search_result_item));
        });

        // 数据绑定
        factory.connect_bind(move |_, list_item| {
            let app_info = list_item
                .downcast_ref::<ListItem>()
                .expect("转换为 GtkListItem 失败")
                .item()
                .and_downcast::<AppDataObject>()
                .expect("转换为 AppDataObject 失败");

            let search_result_item = list_item
                .downcast_ref::<ListItem>()
                .expect("转换为 GtkListItem 失败")
                .child()
                .and_downcast::<SearchResultItem>()
                .expect("转换为 SearchResultItem 失败");

            search_result_item.bind(Some(app_info));
        });

        self.result_list.set_factory(Some(&factory));
    }

    /// `setup_callbacks` 集中配置回调事件
    fn setup_callbacks(&self) {
        self.setup_window_show_callbacks();
        self.setup_background_button_clicked_callbacks();
        self.setup_entry_changed_callbacks();
        self.setup_keyboard_navigation_callbacks();
        self.setup_search_entry_activate_callbacks();
        self.setup_list_view_connect_activate_callbacks();
    }

    /// 搜索栏获取焦点
    fn setup_window_show_callbacks(&self) {
        let search_entry_clone = self.search_entry.clone();
        let window = self.obj();
        window.connect_show(move |_| {
            search_entry_clone.grab_focus();
        });
    }

    /// 关闭
    fn setup_background_button_clicked_callbacks(&self) {
        let window = self.obj().clone();
        self.background_button.connect_clicked(move |_| {
            window.close();
        });
    }

    /// `setup_entry_changed_callbacks` 配置“搜索框内容更新”事件
    /// 1.查询。2.更新列表UI
    fn setup_entry_changed_callbacks(&self) {
        let obj = self.obj();
        self.search_entry.connect_changed(glib::clone!(
            #[weak]
            obj,
            move |entry| {
                let search_key = entry.text().to_string();
                let results = if !search_key.is_empty() {
                    // 查询
                    let search_engine = obj.imp().search_engine.borrow();
                    let apps = search_engine.search(&search_key);
                    apps.iter().map(|app| to_data_object(app)).collect()
                } else {
                    Vec::new()
                };
                obj.imp().update_search_result(results);
            }
        ));
    }

    /// `setup_keyboard_navigation_callbacks` 配置“搜索栏-键盘”回调
    fn setup_keyboard_navigation_callbacks(&self) {
        let controller = EventControllerKey::new();
        let obj = self.obj();
        controller.connect_key_pressed(glib::clone!(
            #[weak]
            obj,
            #[upgrade_or]
            Propagation::Proceed,
            move |_controller, key, _code, _state| {
                match key {
                    Key::Up | Key::Down => {
                        obj.imp().handle_list_navigation(key);
                        // Propagation::Stop 表示：我已经处理了，搜索栏不要再处理
                        Propagation::Stop
                    }
                    _ => Propagation::Proceed,
                }
            }
        ));
        // 将键盘控制器事件添加至搜索栏上
        self.search_entry.get().add_controller(controller);
    }

    /// `setup_search_entry_activate_callbacks` 配置“搜索栏”激活事件
    /// 运行选中项的执行指令
    fn setup_search_entry_activate_callbacks(&self) {
        let obj = self.obj();
        self.search_entry.connect_activate(glib::clone!(
            #[weak]
            obj,
            move |_entry| {
                // 打开应用程序
                obj.imp().handle_run_app_cmd();
            }
        ));
    }

    /// `get_selection` 获取 SingleSelection 控件
    fn get_selection(&self) -> SingleSelection {
        let list = self.result_list.get();
        let select_model = list
            .model()
            .expect("获取 SelectionModel 控件失败，数据为空");
        select_model
            .downcast_ref::<SingleSelection>()
            .expect("将 SelectionModel 转换为 SingleSelection 失败，数据为空")
            .clone()
    }

    /// `setup_list_view_connect_activate_callbacks` 配置 “ListView 激活”事件
    /// 鼠标选中项双击执行
    fn setup_list_view_connect_activate_callbacks(&self) {
        let result_list = self.result_list.get();
        let obj = self.obj();
        result_list.connect_activate(glib::clone!(
            #[weak]
            obj,
            move |_list_view, _| {
                obj.imp().handle_run_app_cmd();
            }
        ));
    }

    /// `handle_list_navigation` 通过 Up / Down 键切换选中的列表项
    fn handle_list_navigation(&self, key: Key) {
        let selection = self.get_selection();
        if selection.n_items() < 1 {
            // 结果项为空，不执行操作
            return;
        }
        let selected_index = selection.selected() as i32;
        let new_selected = if key == Key::Up {
            selected_index - 1
        } else {
            selected_index + 1
        };
        // 限制最大/最小范围
        let new_selected = new_selected.max(0) as u32;
        let new_selected = new_selected.min(selection.n_items() - 1);

        selection.set_selected(new_selected);
        self.result_list
            .get()
            .scroll_to(new_selected, ListScrollFlags::FOCUS, None);
    }

    /// `handle_run_app_cmd` 运行应用程序
    fn handle_run_app_cmd(&self) {
        let selection = self.get_selection();
        let Some(selected_object) = selection.selected_item() else {
            eprintln!("没有选中的结果相");
            return;
        };
        let app_info = selected_object
            .downcast_ref::<AppDataObject>()
            .expect("获取 App_Info 失败，数据为空");
        let exec_cmd = app_info
            .exec_cmd()
            .expect("获取 App_Info 中的 exec_cmd 属性失败，数据为空");
        AppRunner::run(&exec_cmd);
        // 更新应用分数
        self.record_launch(app_info);
    }

    /// `record_launch` 更新应用分数
    fn record_launch(&self, app_info: &AppDataObject) {
        let search_engine = self.search_engine.borrow();
        let apps = search_engine.get_apps();
        if let Some(desktop_file) = app_info.desktop_file() {
            if let Some(application) = apps.iter().find(|app| app.desktop_file == desktop_file) {
                let mut usage = AppUsage::default();
                if let Err(err) = usage.record_launch(&self.env.borrow(), application, apps) {
                    eprintln!("更新分数失败：{}", err);
                }
            }
        }
    }

    /// `update_search_result` 更新搜索结果列表
    fn update_search_result(&self, apps: Vec<AppDataObject>) {
        // 清空旧结果
        self.get_app_list().remove_all();
        for app in &apps {
            self.get_app_list().append(app);
        }
        self.scrolled_window.set_visible(apps.len() > 0);
    }
}
