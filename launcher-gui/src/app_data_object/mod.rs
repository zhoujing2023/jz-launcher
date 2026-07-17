use glib::Object;

mod imp;

glib::wrapper! {
    pub struct AppDataObject(ObjectSubclass<imp::AppDataObject>);
}

impl AppDataObject {
    pub fn new(name: &str, desktop_file: &str, icon: &str, exec_cmd: &str, comment: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("desktop_file", desktop_file)
            .property("icon", icon)
            .property("exec_cmd", exec_cmd)
            .property("comment", comment)
            .build()
    }
}
