mod imp;

use glib::Object;
use gtk::{gio, glib};
use glib::subclass::types::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    /// 隐藏窗口
    pub fn hide(&self){
        self.imp().hide();
    }

    /// 显示窗口
    pub fn show(&self){
        self.imp().show();
    }
}