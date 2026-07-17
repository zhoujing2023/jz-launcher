use crate::app_data_object::AppDataObject;
use launcher_core::AppEntry;

pub fn to_data_object(app: &AppEntry) -> AppDataObject {
    AppDataObject::new(
        &app.name,
        &app.desktop_file,
        app.icon_path.as_deref().unwrap_or(""),
        &app.exec_cmd,
        &app.comment,
    )
}
