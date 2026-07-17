use adw::Application;
use adw::prelude::{ApplicationExt, ApplicationExtManual};
use glib::ExitCode;
use gtk::ApplicationWindow;
use gtk::prelude::GtkWindowExt;

const APP_ID: &str = "org.zhoujing.Demo";

fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        println!("hello");
        build_ui(&app);
    });
    app.run()
}

fn build_ui(app: &Application) {
    let label = gtk::Label::new(Some("hello"));
    let window = ApplicationWindow::builder()
        .child(&label)
        .title("Launcher UI")
        .application(app)
        .build();
    window.present();
}
