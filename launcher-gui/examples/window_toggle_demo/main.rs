use adw::prelude::*;
use glib::ExitCode;
use gtk::{gio, glib, Application, ApplicationWindow, Label, Box, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.zhoujing.window_toggle_demo";

fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    // 存储窗口引用，用于托盘图标操作
    let window_ref: Rc<RefCell<Option<ApplicationWindow>>> = Rc::new(RefCell::new(None));

    let window_ref_clone = window_ref.clone();
    app.connect_activate(move |app| {
        let window = build_ui(app);
        *window_ref_clone.borrow_mut() = Some(window);
    });

    // 添加菜单动作
    setup_actions(&app, window_ref.clone());

    app.run()
}

fn build_ui(app: &Application) -> ApplicationWindow {
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let label = Label::builder()
        .label("这是一个窗口隐藏/显示的 Demo")
        .build();

    let hint_label = Label::builder()
        .label("点击关闭按钮不会退出程序，只是隐藏窗口\n\n使用以下方式重新显示：\n• 快捷键：Ctrl+Shift+L\n• 命令行：点击应用图标")
        .wrap(true)
        .justify(gtk::Justification::Center)
        .build();

    let quit_button = gtk::Button::builder()
        .label("真正退出程序")
        .margin_top(12)
        .build();

    let app_clone = app.clone();
    quit_button.connect_clicked(move |_| {
        app_clone.quit();
    });

    content_box.append(&label);
    content_box.append(&hint_label);
    content_box.append(&quit_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Window Toggle Demo")
        .child(&content_box)
        .default_width(400)
        .default_height(300)
        .build();

    // 关键：拦截关闭信号，改为隐藏窗口
    window.connect_close_request(|window| {
        println!("窗口关闭请求被拦截，隐藏窗口而不是退出");
        window.set_visible(false);
        glib::Propagation::Stop // 阻止默认的关闭行为
    });

    window.present();
    window
}

fn setup_actions(app: &Application, window_ref: Rc<RefCell<Option<ApplicationWindow>>>) {
    // 创建"显示/隐藏窗口"动作
    let toggle_action = gio::SimpleAction::new("toggle-launcher_window", None);
    let window_ref_clone = window_ref.clone();
    toggle_action.connect_activate(move |_, _| {
        if let Some(window) = window_ref_clone.borrow().as_ref() {
            if window.is_visible() {
                println!("隐藏窗口");
                window.set_visible(false);
            } else {
                println!("显示窗口");
                window.set_visible(true);
                window.present();
            }
        }
    });
    app.add_action(&toggle_action);

    // 注册快捷键：Ctrl+Shift+L
    app.set_accels_for_action("app.toggle-launcher_window", &["<Ctrl><Shift>L"]);

    // 创建"退出"动作
    let quit_action = gio::SimpleAction::new("quit", None);
    let app_clone = app.clone();
    quit_action.connect_activate(move |_, _| {
        println!("真正退出程序");
        app_clone.quit();
    });
    app.add_action(&quit_action);

    // 注册退出快捷键：Ctrl+Q
    app.set_accels_for_action("app.quit", &["<Ctrl>Q"]);
}
