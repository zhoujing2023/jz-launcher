use glib::Properties;
use glib::object::ObjectExt;
use glib::subclass::object::DerivedObjectProperties;
use glib::subclass::prelude::{ObjectImpl, ObjectSubclass};
use std::cell::RefCell;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::AppDataObject)]
pub struct AppDataObject {
    #[property(get, set)]
    pub name: RefCell<Option<String>>,
    #[property(get, set)]
    pub icon: RefCell<Option<String>>,
    #[property(get, set)]
    pub exec_cmd: RefCell<Option<String>>,
    #[property(get, set)]
    pub comment: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppDataObject {
    const NAME: &'static str = "MyAppDataObject";
    type Type = super::AppDataObject;
}

#[glib::derived_properties] // 只有当 GObject 存在自定义属性才需要加上此宏
impl ObjectImpl for AppDataObject {}

impl AppDataObject {}
