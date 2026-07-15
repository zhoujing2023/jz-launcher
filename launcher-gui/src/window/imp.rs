use crate::app_data_object::AppDataObject;
use crate::app_provider::to_data_object;
use crate::search_result_item::SearchResultItem;
use adw::gdk::Display;
use adw::prelude::{ActionMapExt, DisplayExt, ListModelExt, MonitorExt};
use adw::{gdk, gio};
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
    // 透明背景按钮控件
    #[template_child]
    pub background_button: TemplateChild<gtk::Button>,
    // 主容器控件
    #[template_child]
    pub main_box: TemplateChild<gtk::Box>,
    // 搜索栏控件
    #[template_child]
    pub search_entry: TemplateChild<gtk::Entry>,
    // 滚动窗口控件
    #[template_child]
    pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    // 列表视图
    #[template_child]
    pub result_list: TemplateChild<gtk::ListView>,
    // 列表视图存储的数据
    pub app_list: RefCell<Option<ListStore>>,

    // *********  core 属性 *********
    // 环境信息
    env: RefCell<Env>,
    // 搜索程序
    search_engine: RefCell<SearchEngine>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyWindow";
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

        // 配置 gui 数据： 1.配置 ListView 模式和工厂 2.信号回调 3.配置Actions 4.初始化控件样式
        self.setup_model();
        self.setup_factory();
        self.setup_actions();
        self.setup_callbacks();
        self.init_widget_style();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

// 自定义方法实现
impl Window {

    /// 初始化布局样式
    /// 由于 Wayland 安全限制，无法主动调整窗口位置，默认使用透明背景占满整个屏幕，使其主窗口（搜索框）定位
    fn init_widget_style(&self) {
        let (screen_width, screen_height) = Self::get_screen_size();

        // 搜索框离屏幕顶部 20% 距离
        let main_box_margin_top = (screen_height as f64 * 0.20) as i32;
        self.main_box.get().set_margin_top(main_box_margin_top);

        // 窗口默认占满整个屏幕
        let window = self.obj();
        window.set_default_width(screen_width);
        window.set_default_height(screen_height);
    }

    /// 获取屏幕大小（宽，高）
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

    /// 获取列表视图数据模型
    fn get_app_list(&self) -> ListStore {
        self.app_list
            .borrow()
            .clone()
            .expect("获取 app_list 失败，app_list 为空")
    }

    /// 配置列表视图模型
    /// 列表视图存储的数据类型为 AppDataObject
    fn setup_model(&self) {
        let store = ListStore::new::<AppDataObject>();
        self.app_list.replace(Some(store));

        let selection = SingleSelection::new(Some(self.get_app_list()));
        self.result_list.set_model(Some(&selection));
    }

    /// 配置列表视图工厂
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

    /// 集中配置回调信号
    fn setup_callbacks(&self) {
        self.setup_window_show_callback();
        self.setup_window_close_callback();
        self.setup_background_button_clicked_callback();
        self.setup_entry_changed_callback();
        self.setup_keyboard_navigation_callback();
        self.setup_search_entry_activate_callback();
        self.setup_list_view_connect_activate_callback();
    }

    /// 窗口显示-信号
    /// 1.聚焦搜索栏
    fn setup_window_show_callback(&self) {
        let search_entry = self.search_entry.get();
        let window = self.obj();
        window.connect_show(glib::clone!(
            #[weak]
            search_entry,
            move |_| {
                search_entry.grab_focus();
            }
        ));
    }

    /// 窗口关闭-信号
    /// 隐藏窗口
    fn setup_window_close_callback(&self) {
        let window = self.obj();
        window.connect_close_request(|window| {
            window.hide();
            Propagation::Stop
        });
    }

    /// 透明背景点击-信号
    /// 隐藏窗口
    fn setup_background_button_clicked_callback(&self) {
        let window = self.obj();
        self.background_button.connect_clicked(glib::clone!(
            #[weak]
            window,
            move |_| {
                window.hide();
            }
        ));
    }

    /// 搜索框内容变更-信号
    /// 1.查询。2.更新列表UI
    fn setup_entry_changed_callback(&self) {
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

    /// 搜索栏的按键监听-信号
    fn setup_keyboard_navigation_callback(&self) {
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
                        Propagation::Stop
                    }
                    _ => Propagation::Proceed,
                }
            }
        ));
        // 将键盘控制器信号添加至搜索栏上
        self.search_entry.get().add_controller(controller);
    }

    /// 搜索框激活（回车）-信号
    /// 运行选中项的执行指令
    fn setup_search_entry_activate_callback(&self) {
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

    /// 从列表视图控件中获取 SingleSelection 控件
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

    /// 列表视图激活（双击）-信号
    fn setup_list_view_connect_activate_callback(&self) {
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

    /// 通过 Up / Down 键切换选中的列表项
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

        // 列表视图滚动到选中的项
        self.result_list
            .get()
            .scroll_to(new_selected, ListScrollFlags::FOCUS, None);
    }

    /// 运行应用程序
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
        // 隐藏窗口
        let obj = self.obj();
        glib::idle_add_local_once(glib::clone!(
            #[weak]
            obj,
            move || {
                obj.hide();
            }
        ));
    }

    /// 更新应用分数
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

    /// 更新列表数据
    fn update_search_result(&self, apps: Vec<AppDataObject>) {
        let store = self.get_app_list();
        store.remove_all();
        
        // 限制最多显示 20 个结果
        let max_results = 20;
        for app in apps.iter().take(max_results) {
            store.append(app);
        }
        
        self.scrolled_window.set_visible(!apps.is_empty());
    }

    /// 隐藏窗口
    pub(crate) fn hide(&self) {
        self.get_app_list().remove_all();
        self.search_entry.set_text("");
        self.obj().set_visible(false);
    }

    /// 显示窗口
    pub(crate) fn show(&self) {
        let obj = self.obj();
        obj.set_visible(true);
        obj.present();
    }

    /// 配置 Actions
    fn setup_actions(&self) {
        let obj = self.obj();
        // 隐藏 Actions
        let hide_action = gio::SimpleAction::new("hide", None);
        hide_action.connect_activate(glib::clone! {
            #[weak]
            obj,
            move |_,_|{
                obj.hide();
            }
        });
        obj.add_action(&hide_action);
    }
}
