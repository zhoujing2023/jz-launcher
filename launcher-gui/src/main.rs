mod app_data_object;
mod search_result_item;
mod window;
mod app_provider;

use glib::ExitCode;
use gtk::gdk::Display;
use gtk::gio;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt};
use window::Window;

const APP_ID: &str = "org.zhoujing.jz-launcher";

fn main() -> ExitCode {
    // 注册 GResource 资源
    gio::resources_register_include!("org.zhoujing.storage").expect("Failed to register resources");

    // 创建 GTK 应用
    let app = adw::Application::builder().application_id(APP_ID).build();

    // 连接激活信号
    app.connect_startup(|_| {
        load_css();
    });
    app.set_accels_for_action("window.close", &["<Ctrl>W", "<Ctrl>Q", "Escape"]);
    app.connect_activate(build_ui);

    // 运行应用
    app.run()
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/org/zhoujing/jz_tools/css/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Failed to get default display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}

fn build_ui(app: &adw::Application) {
    // 创建并显示主窗口
    let window = Window::new(app);
    window.present();
}
