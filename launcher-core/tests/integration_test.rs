use launcher_core::{AppEntry, AppLoader, Env};
use std::collections::HashMap;

/// `test_app_loader_integration` 测试加载 desktop 目录
#[test]
fn test_app_loader_integration() {
    let env = Env::load().expect("获取环境信息失败");
    // 测试从实际目录加载
    let default_paths = AppLoader::default_desktop_scan_paths(&env);
    let apps = AppLoader::load(&env, default_paths);
    assert!(!apps.is_empty(), "加载应用程序完毕");

    // 根据 exec_cmd 进行去重
    let mut seen = HashMap::new();
    let apps: Vec<AppEntry> = apps
        .into_iter()
        .filter(|app| seen.insert(app.exec_cmd.clone(), ()).is_none())
        .collect();

    println!("{:#?}", apps);
}
