mod imp;

use crate::app_data_object::AppDataObject;
use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct SearchResultItem(ObjectSubclass<imp::SearchResultItem>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SearchResultItem {
    pub fn bind(&self, app: Option<AppDataObject>) {
        self.imp().bind(app);
    }
}

impl Default for SearchResultItem {
    fn default() -> Self {
        Object::builder().build()
    }
}
