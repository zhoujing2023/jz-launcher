use glib::Object;
use glib::prelude::ObjectExt;

mod config_data;
mod imp;

use config_data::{AppearanceConfig, ConfigData, GeneralConfig, ShortcutConfig};

glib::wrapper! {
    pub struct ConfigDataObject(ObjectSubclass<imp::ConfigDataObject>);
}

impl ConfigDataObject {
    pub fn new(
        desktop_scan_path: Vec<String>,
        theme: u32,
        font_size: f64,
        show: &str,
        quit: &str,
    ) -> Self {
        Object::builder()
            .property("desktop-scan-path", desktop_scan_path)
            .property("theme", theme)
            .property("font-size", font_size)
            .property("show", show)
            .property("quit", quit)
            .build()
    }

    pub fn load() -> Result<Self, std::io::Error> {
        let data = ConfigData::load()?;
        Ok(Self::new(
            data.general.desktop_scan_path,
            data.appearance.theme,
            data.appearance.font_size,
            &data.shortcut.show,
            &data.shortcut.quit,
        ))
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        ConfigData {
            general: GeneralConfig {
                desktop_scan_path: self.property("desktop-scan-path"),
            },
            appearance: AppearanceConfig {
                theme: self.property("theme"),
                font_size: self.property("font-size"),
            },
            shortcut: ShortcutConfig {
                show: self.property("show"),
                quit: self.property("quit"),
            },
        }
        .save()
    }
}

impl Default for ConfigDataObject {
    fn default() -> Self {
        let data = ConfigData::default();
        Self::new(
            data.general.desktop_scan_path,
            data.appearance.theme,
            data.appearance.font_size,
            &data.shortcut.show,
            &data.shortcut.quit,
        )
    }
}
