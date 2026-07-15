use glib::Properties;
use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::DerivedObjectProperties;
use std::cell::RefCell;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::AppDataObject)]
pub struct AppDataObject {
    // 名称
    #[property(get, set)]
    pub name: RefCell<Option<String>>,
    // desktop 路径
    #[property(get, set)]
    pub desktop_file: RefCell<Option<String>>,
    // 图标
    #[property(get, set)]
    pub icon: RefCell<Option<String>>,
    // 描述
    #[property(get, set)]
    pub comment: RefCell<Option<String>>,
    // 启动命令
    #[property(get, set)]
    pub exec_cmd: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppDataObject {
    const NAME: &'static str = "MyAppDataObject";
    type Type = super::AppDataObject;
}

#[glib::derived_properties]
impl ObjectImpl for AppDataObject {}
