use crate::config_data_object::ConfigDataObject;
use crate::system_env::SystemEnv;
use adw::gdk;
use glib::Propagation;
use glib::object::ObjectExt;
use glib::subclass::InitializingObject;
use gtk::prelude::{
    ButtonExt, EditableExt, EventControllerExt, TextBufferExt, TextViewExt, WidgetExt,
};
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, Entry, EventControllerKey, TextBuffer, glib};
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/zhoujing/jz-launcher/ui/settings-window.ui")]
pub struct SettingsWindow {
    #[template_child]
    pub main_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub content_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub auto_start_switch: TemplateChild<gtk::Switch>,
    #[template_child]
    pub desktop_paths_view: TemplateChild<gtk::TextView>,
    #[template_child]
    pub desktop_paths_edit_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub desktop_paths_cancel_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub desktop_paths_save_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub theme_drop_down: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub font_size_spin_button: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub font_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub show_shortcut_entry: TemplateChild<Entry>,
    #[template_child]
    pub quit_shortcut_entry: TemplateChild<Entry>,
    #[template_child]
    pub hint_label_entry: TemplateChild<Entry>,
    // Desktop paths 修改前的数据（临时数据）
    desktop_paths_temp_data: Rc<RefCell<String>>,
    // 配置
    pub config: OnceCell<ConfigDataObject>,
}

#[glib::object_subclass]
impl ObjectSubclass for SettingsWindow {
    const NAME: &'static str = "MySettingWindow";
    type Type = super::SettingsWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SettingsWindow {
    fn constructed(&self) {
        self.parent_constructed();
        // 配置信号回调
        self.setup_callbacks();
    }
}

impl WidgetImpl for SettingsWindow {}
impl WindowImpl for SettingsWindow {}
impl ApplicationWindowImpl for SettingsWindow {}

impl SettingsWindow {
    /// 集中配置回调信号
    fn setup_callbacks(&self) {
        self.auto_start_switch_state_callback();
        self.desktop_paths_edit_button_clicked_callback();
        self.desktop_paths_cancel_button_clicked_callback();
        self.desktop_paths_save_button_clicked_callback();
        self.theme_drop_down_selected_notify_callback();
        self.font_size_spin_button_value_changed_callback();
    }

    /// 加载配置
    pub(super) fn apply_config(&self, config_data: &ConfigDataObject) {
        // 配置字段绑定关系
        self.setup_bind(&config_data);

        // 配置快捷键按键监听关系
        let show_entry = self.show_shortcut_entry.get();
        self.setup_bind_shortcut_keys(&show_entry);
        let quit_entry = self.quit_shortcut_entry.get();
        self.setup_bind_shortcut_keys(&quit_entry);

        // 获取自定义快捷键命令
        let exec_path = SystemEnv::get_executable_path();
        let hint_label = format!("{} --toggle", exec_path);
        self.hint_label_entry.set_text(hint_label.as_str());
    }


    #[deprecated(
        since = "1.0.0",
        note = "由用户自定义快捷键启动，不提供自动启动"
    )]
    fn auto_start_switch_state_callback(&self) {
        self.auto_start_switch.connect_state_set(move |_, state| {
            if state {
                println!("设置开机时自动启动……");
            } else {
                println!("取消开机时自动启动……");
            }
            Propagation::Proceed
        });
    }

    /// 编辑按钮点击时-信号
    /// 1.启动 desktop path内容输入框
    /// 2.隐藏“编辑按钮”
    /// 3.将当前数据临时存储，点击“取消按钮”时回退
    fn desktop_paths_edit_button_clicked_callback(&self) {
        let origin_text_data_clone = self.desktop_paths_temp_data.clone();

        let save_button = self.desktop_paths_save_button.get();
        let cancel_button = self.desktop_paths_cancel_button.get();
        let paths_view = self.desktop_paths_view.get();
        self.desktop_paths_edit_button.connect_clicked(glib::clone!(
            #[weak]
            save_button,
            #[weak]
            cancel_button,
            #[weak]
            paths_view,
            move |edit_button| {
                edit_button.set_visible(false);
                save_button.set_visible(true);
                cancel_button.set_visible(true);
                paths_view.set_editable(true);
                paths_view.add_css_class("desktop-paths-list-view-enabled");
                paths_view.remove_css_class("desktop-paths-list-view-disabled");
                // 临时保存原数据，当点击 “取消按钮” 时将数据还原
                let buffer = paths_view.buffer();
                *origin_text_data_clone.borrow_mut() = buffer
                    .text(&buffer.start_iter(), &buffer.end_iter(), false)
                    .to_string();
            }
        ));
    }

    /// 保存按钮点击时-信号
    /// 1.禁用 desktop path内容输入框
    /// 2.隐藏“保存按钮”和“取消按钮”
    /// 3.将临时数据清空
    /// 4.手动更新 config 数据
    fn desktop_paths_save_button_clicked_callback(&self) {
        let settings_window = self.obj();
        // 将临时数据清空
        self.desktop_paths_temp_data.borrow_mut().clear();

        let edit_button = self.desktop_paths_edit_button.get();
        let cancel_button = self.desktop_paths_cancel_button.get();
        let paths_view = self.desktop_paths_view.get();
        self.desktop_paths_save_button.connect_clicked(glib::clone!(
            #[weak]
            edit_button,
            #[weak]
            cancel_button,
            #[weak]
            paths_view,
            #[weak]
            settings_window,
            move |save_button| {
                save_button.set_visible(false);
                edit_button.set_visible(true);
                cancel_button.set_visible(false);
                paths_view.set_editable(false);
                paths_view.remove_css_class("desktop-paths-list-view-enabled");
                paths_view.add_css_class("desktop-paths-list-view-disabled");
                // 保存
                let buffer = paths_view.buffer();
                let new_desktop_paths_value = buffer
                    .text(&buffer.start_iter(), &buffer.end_iter(), false)
                    .to_string();
                let new_desktop_paths_value: Vec<String> = new_desktop_paths_value
                    .lines()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().to_string())
                    .collect();
                let config = settings_window
                    .imp()
                    .config
                    .get()
                    .expect("获取配置数据失败");
                config.set_desktop_scan_path(new_desktop_paths_value);
                // 通过 config 属性变更信号-更新 desktop 列表
                // 实现处：[`launcher_window::mod::setup_config_callbacks`]
            }
        ));
    }

    /// 取消按钮点击时-信号
    /// 1.禁用 desktop path内容输入框
    /// 2.隐藏“保存按钮”和“取消按钮”
    /// 3.将临时数据覆盖编辑数据，最后清空临时数据
    fn desktop_paths_cancel_button_clicked_callback(&self) {
        // 编辑前的数据
        let desktop_paths_temp_data = self.desktop_paths_temp_data.clone();

        let edit_button = self.desktop_paths_edit_button.get();
        let save_button = self.desktop_paths_save_button.get();
        let paths_view = self.desktop_paths_view.get();
        self.desktop_paths_cancel_button
            .connect_clicked(glib::clone!(
                #[weak]
                edit_button,
                #[weak]
                save_button,
                #[weak]
                paths_view,
                move |cancel_button| {
                    cancel_button.set_visible(false);
                    save_button.set_visible(false);
                    edit_button.set_visible(true);
                    paths_view.set_editable(false);
                    paths_view.remove_css_class("desktop-paths-list-view-enabled");
                    paths_view.add_css_class("desktop-paths-list-view-disabled");
                    // 撤销修改的数据 并 清理临时数据
                    paths_view
                        .buffer()
                        .set_text(desktop_paths_temp_data.borrow().as_str());
                    desktop_paths_temp_data.borrow_mut().clear();
                }
            ));
    }

    /// 主题选项发生变更时-信号
    fn theme_drop_down_selected_notify_callback(&self) {
        self.theme_drop_down.connect_selected_notify(|dd| {
            SystemEnv::apply_color_scheme(dd.selected());
        });
    }

    /// 字体大小滑块发生变更时-信号
    fn font_size_spin_button_value_changed_callback(&self) {
        let (font_name, _font_size) = SystemEnv::get_system_current_font_info();
        self.font_size_spin_button
            .connect_value_changed(move |btn| {
                SystemEnv::apply_font_size(&font_name, btn.value());
            });
    }

    /// 配置快捷键 Entry 按键监听
    fn setup_bind_shortcut_keys(&self, entry: &Entry) {
        // 监听按键，用于获取新的快捷键
        let controller = EventControllerKey::new();
        controller.connect_key_pressed(glib::clone!(
            #[weak]
            entry,
            #[upgrade_or]
            Propagation::Proceed,
            move |_controller, key, _code, state| {
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
                entry.set_text(&shortcut_str);
                Propagation::Stop
                // 通过 config 属性变更信号-更新 desktop 列表
                // 实现处：显示按钮 [`launcher_window::mod::setup_config_callbacks`]
                //        隐藏按钮 [`main::setup_actions`]
            }
        ));
        // 设置为捕获阶段，用于全局快捷键
        controller.set_propagation_phase(gtk::PropagationPhase::Capture);
        entry.add_controller(controller);
    }

    /// 配置绑定关系
    fn setup_bind(&self, config_object: &ConfigDataObject) {
        let auto_start_switch = self.auto_start_switch.get();
        config_object
            .bind_property("auto-start-at-boot", &auto_start_switch, "active")
            .sync_create()
            .bidirectional()
            .build();
        let desktop_paths_view = self.desktop_paths_view.get();
        config_object
            .bind_property("desktop-scan-path", &desktop_paths_view, "buffer")
            .sync_create()
            .transform_to(|_, value: Vec<String>| {
                let text_buffer = TextBuffer::builder().text(value.join("\n")).build();
                Some(text_buffer)
            })
            .build();
        let theme_drop_down = self.theme_drop_down.get();
        config_object
            .bind_property("theme", &theme_drop_down, "selected")
            .sync_create()
            .bidirectional()
            .build();
        let font_adjustment = self.font_adjustment.get();
        config_object
            .bind_property("font-size", &font_adjustment, "value")
            .sync_create()
            .bidirectional()
            .build();
        let show_shortcut_entry = self.show_shortcut_entry.get();
        config_object
            .bind_property("show", &show_shortcut_entry, "text")
            .sync_create()
            .bidirectional()
            .build();
        let quit_shortcut_entry = self.quit_shortcut_entry.get();
        config_object
            .bind_property("quit", &quit_shortcut_entry, "text")
            .sync_create()
            .bidirectional()
            .build();
    }
}
