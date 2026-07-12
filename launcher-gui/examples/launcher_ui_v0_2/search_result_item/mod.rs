use crate::app_data_object::AppDataObject;
use glib::Object;
use glib::subclass::types::ObjectSubclassIsExt;

mod imp;

glib::wrapper! {
    pub struct SearchResultItem(ObjectSubclass<imp::SearchResultItem>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SearchResultItem {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, app_data_obj: Option<AppDataObject>) {
        self.imp().bind(app_data_obj);
    }
}
