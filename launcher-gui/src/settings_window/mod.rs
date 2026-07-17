mod imp;
use crate::config_data_object::ConfigDataObject;
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct SettingsWindow(ObjectSubclass<imp::SettingsWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl SettingsWindow {
    pub fn new(app: &adw::Application, config: &ConfigDataObject) -> Self {
        let window: SettingsWindow = Object::builder().property("application", app).build();
        window
            .imp()
            .config
            .set(config.clone())
            .expect("config 应当仅初始化一次");
        window.imp().apply_config(config);
        window
    }
}
