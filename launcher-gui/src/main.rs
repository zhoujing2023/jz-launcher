mod app_data_object;
mod config_data_object;
mod global_constant;
mod launcher_window;
mod search_result_item;
mod settings_window;
mod system_env;

use crate::config_data_object::ConfigDataObject;
use crate::global_constant::{HIDE_ACTION, QUIT_ACTION};
use adw::Application;
use adw::prelude::{ActionMapExt, ApplicationCommandLineExt};
use glib::ExitCode;
use gtk::gdk::Display;
use gtk::gio;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt, ObjectExt, WidgetExt};
use launcher_window::LauncherWindow;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.zhoujing.jz-launcher";

type WindowRef = Rc<RefCell<Option<LauncherWindow>>>;

fn main() -> ExitCode {
    // 注册 GResource 资源
    gio::resources_register_include!("org.zhoujing.storage").expect("Failed to register resources");

    // 创建 GTK 应用
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    // 加载配置
    let config = ConfigDataObject::load().unwrap_or_else(|err| {
        eprintln!("加载配置失败，使用默认配置：{err}");
        ConfigDataObject::default()
    });
    setup_config_auto_save(&config);

    let window_ref: WindowRef = Rc::new(RefCell::new(None));

    // 配置信号回调
    setup_startup(&app, &config);
    setup_app_command_line_callback(&app, &window_ref);
    setup_app_activate_callback(&app, &window_ref, &config);

    // 运行应用
    app.run()
}

/// 初始化完成-信号
fn setup_startup(app: &Application, config: &ConfigDataObject) {
    app.connect_startup(glib::clone!(
       #[weak]
        config,
         move |app| {
            setup_actions(app, &config);
            println!("*** 启动成功 ***");
            println!("全局快捷键配置：打开设置-键盘-键盘快捷键-查看及自定义快捷键-自定义快捷键-添加\n指令为：{} --toggle", get_executable_path());
        }
    ));
}

/// 参数处理-信号
fn setup_app_command_line_callback(app: &Application, window_ref: &WindowRef) {
    app.connect_command_line(glib::clone!(
        #[weak]
        window_ref,
        #[upgrade_or]
        ExitCode::FAILURE,
        move |app, cmdline| {
            let args = cmdline.arguments();
            println!("收到命令行参数：{:?}", args);

            let has_toggle = args
                .iter()
                .any(|arg| arg.to_string_lossy().contains("--toggle"));

            // 如果是主实例第一次启动（无 --toggle）
            if !has_toggle {
                app.activate();
            } else {
                // 如果是 --toggle 调用，切换窗口显示状态
                if let Some(window) = window_ref.borrow().as_ref() {
                    if window.is_visible() {
                        println!("隐藏窗口");
                        window.hide();
                    } else {
                        println!("显示窗口");
                        window.show();
                    }
                } else {
                    // 窗口还未创建，先激活创建窗口
                    app.activate();
                    if let Some(window) = window_ref.borrow().as_ref() {
                        window.show();
                    }
                }
            }

            ExitCode::SUCCESS
        }
    ));
}

/// 程序激活-信号
fn setup_app_activate_callback(
    app: &Application,
    window_ref: &WindowRef,
    config: &ConfigDataObject,
) {
    app.connect_activate(glib::clone!(
        #[weak]
        window_ref,
        #[weak]
        config,
        move |app| {
            if window_ref.borrow().is_none() {
                let window = build_ui(&app, &config);
                window.set_visible(false);
                *window_ref.borrow_mut() = Some(window);
            }
        }
    ));
}

/// 加载 CSS
fn load_css() {
    let css_paths = [
        "/org/zhoujing/jz-launcher/css/launcher.css",
        "/org/zhoujing/jz-launcher/css/settings.css",
    ];

    for path in css_paths.iter() {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource(path);
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("无法连接到显示器"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

/// 构建 UI
fn build_ui(app: &Application, config: &ConfigDataObject) -> LauncherWindow {
    // 创建并显示主窗口
    let window = LauncherWindow::new(app, &config);
    load_css();
    window
}

/// 配置 Actions
/// win.hide Actions 实现处：[`crate::launcher_window::LauncherWindow::setup_actions`]
fn setup_actions(app: &Application, config: &ConfigDataObject) {
    // 退出 Actions
    let quit_action = gio::SimpleAction::new(QUIT_ACTION, None);
    quit_action.connect_activate(glib::clone!(
        #[weak]
        app,
        move |_, _| {
            app.quit();
        }
    ));
    app.add_action(&quit_action);
    app.set_accels_for_action(&format!("app.{}", QUIT_ACTION), &[&config.quit()]);
    config.connect_quit_notify(glib::clone!(
        #[weak]
        app,
        move |config| {
            app.set_accels_for_action(&format!("app.{}", QUIT_ACTION), &[&config.quit()]);
        }
    ));
    // 隐藏 Actions
    app.set_accels_for_action(&format!("win.{}", HIDE_ACTION), &["Escape"]);
}

/// 配置属性变化时统一持久化
fn setup_config_auto_save(config: &ConfigDataObject) {
    config.connect_notify_local(None, |config, property| {
        if let Err(err) = config.save() {
            eprintln!("保存配置失败（属性：{}）：{}", property.name(), err);
        }
    });
}

/// 获取当前可执行文件的完整路径
/// 场景：当前用户需要配置全局快捷键时，需要知道绝对路径
fn get_executable_path() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "jz-launcher".to_string())
}
