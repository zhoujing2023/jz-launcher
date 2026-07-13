use crate::launcher::Launcher;
use anyhow::Context;
use launcher_core::{AppLoader, Env, SearchEngine};

pub mod launcher;

fn main() -> anyhow::Result<()> {
    let env = Env::load().context("获取环境数据失败")?;
    println!("获取环境数据成功：\n{:#?}", env);
    println!("********* 开始检索 desktop *********");
    let apps = AppLoader::load(&env);
    println!("********* 检索完毕 *********");
    if apps.is_empty() {
        println!("没有找到应用程序。。。");
        return Ok(());
    }
    let search_engine = SearchEngine::new(apps);
    let launcher = Launcher::new(search_engine);
    launcher.run(&env);
    println!("程序结束");
    Ok(())
}
