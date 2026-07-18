use glib::Properties;
use gtk::glib;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::ConfigDataObject)]
pub struct ConfigDataObject {
    // 通用
    #[property(name = "desktop-scan-path", get, set)]
    pub desktop_scan_path: RefCell<Vec<String>>,

    // 外观
    #[property(get, set)]
    pub theme: Cell<u32>,
    #[property(name = "font-size", get, set)]
    pub font_size: Cell<f64>,

    // 快捷键
    #[property(get, set)]
    pub quit: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for ConfigDataObject {
    const NAME: &'static str = "MyConfigDataObject";
    type Type = super::ConfigDataObject;
}

#[glib::derived_properties]
impl ObjectImpl for ConfigDataObject {}
