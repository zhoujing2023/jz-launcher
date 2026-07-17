mod imp;

use crate::config_data_object::ConfigDataObject;
use glib::Object;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct LauncherWindow(ObjectSubclass<imp::LauncherWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl LauncherWindow {
    pub fn new(app: &adw::Application, config: &ConfigDataObject) -> Self {
        let window: LauncherWindow = Object::builder().property("application", app).build();
        // 1.初始化配置 2.加载应用列表 3.应用配置
        window
            .imp()
            .config
            .set(config.clone())
            .expect("config 应当仅初始化一次");
        window.imp().load_apps(config);
        window.imp().apply_appearance_config(config);
        window.imp().apply_shortcut_config(config);
        window.setup_config_callbacks(config);
        window
    }

    /// 响应共享配置变化。SettingsWindow 只修改配置，不直接依赖 LauncherWindow。
    fn setup_config_callbacks(&self, config: &ConfigDataObject) {
        config.connect_desktop_scan_path_notify(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |config| window.imp().load_apps(config)
        ));

        config.connect_theme_notify(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |config| window.imp().apply_appearance_config(config)
        ));

        config.connect_font_size_notify(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |config| window.imp().apply_appearance_config(config)
        ));

        config.connect_show_notify(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |config| window.imp().apply_shortcut_config(config)
        ));
    }

    /// 隐藏窗口
    pub fn hide(&self) {
        self.imp().hide();
    }

    /// 显示窗口
    pub fn show(&self) {
        self.imp().show();
    }
}
