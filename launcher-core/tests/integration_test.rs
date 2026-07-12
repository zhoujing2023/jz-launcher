use launcher_core::{AppLoader, Env};

/// `test_app_loader_integration` 测试加载 desktop 目录
#[test]
fn test_app_loader_integration() {
    let env = Env::get_env().expect("获取环境信息失败");
    // 测试从实际目录加载
    let apps = AppLoader::load(&env);
    assert!(!apps.is_empty(), "加载应用程序完毕");
    println!("{:#?}", apps);
}
