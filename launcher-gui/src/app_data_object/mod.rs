use glib::Object;

mod imp;

glib::wrapper! {
    pub struct AppDataObject(ObjectSubclass<imp::AppDataObject>);
}

impl AppDataObject {
    pub fn new(name: &str, icon_path: &str, exec_cmd: &str, comment: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("icon_path", icon_path)
            .property("exec_cmd", exec_cmd)
            .property("comment", comment)
            .build()
    }
}
