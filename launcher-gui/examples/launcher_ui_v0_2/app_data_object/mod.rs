use glib::Object;

mod imp;

glib::wrapper! {
    pub struct AppDataObject(ObjectSubclass<imp::AppDataObject>);
}

impl AppDataObject {
    pub fn new(name: &str, icon: &str, exec_cmd: &str, comment: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("icon", icon)
            .property("exec_cmd", exec_cmd)
            .property("comment", comment)
            .build()
    }
}

impl Default for AppDataObject {
    fn default() -> Self {
        Object::builder().build()
    }
}
