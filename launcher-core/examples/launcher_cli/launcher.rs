use launcher_core::AppRunner;
use launcher_core::AppUsage;
use launcher_core::Env;
use launcher_core::SearchEngine;
use std::io::{Write, stdin, stdout};

pub struct Launcher {
    search_engine: SearchEngine,
}

impl Launcher {
    /// `new` 创建实例
    pub fn new(search_engine: SearchEngine) -> Self {
        Self { search_engine }
    }

    /// `run` 处理搜索
    pub fn run(&self, env: &Env) {
        loop {
            print!("请输入desktop名称（输入exit结束程序）：");
            match stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    eprint!("无法刷新输出：{}", e);
                    continue;
                }
            }
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("读取输入的内容失败：{}", err);
                    println!("请重新输入~");
                    continue;
                }
            }
            let input = input.trim();
            if input.is_empty() {
                println!("输入的内容为空");
                continue;
            }
            if input == "exit" {
                break;
            }
            let results = self.search_engine.search(input);
            if results.is_empty() {
                println!("没有查询到程序，请重新输入~");
                continue;
            }
            for (i, result) in results.iter().enumerate() {
                println!(
                    "序号:{}\t名称：{}\t分数：{}\t说明：{}",
                    i + 1,
                    result.name,
                    result.score.borrow(),
                    result.comment,
                )
            }
            println!("查询到的数量：{}", results.len());
            let mut application = None;
            loop {
                println!("请选择要打开的应用（序号，0=退出）：");
                let mut index = String::new();
                match stdin().read_line(&mut index) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("读取失败：{}", err);
                        println!("请重新输入~");
                        continue;
                    }
                }
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        eprintln!("转换失败，异常信息：{}", err);
                        println!("请重新输入～");
                        continue;
                    }
                };
                if index == 0 {
                    break;
                }
                if index < 1 {
                    println!("序号不能小于1，请重新选择~");
                    continue;
                }
                let app = results.get(index - 1);
                application = match app {
                    Some(app) => Some(app),
                    None => {
                        println!("无效的选择~");
                        continue;
                    }
                };
                break;
            }
            let application = match application {
                Some(app) => app,
                None => continue,
            };
            // 更新应用分数并保存
            let mut usage = AppUsage::default();
            if let Err(err) =
                usage.record_launch(&env, *application, &self.search_engine.get_apps())
            {
                eprintln!("更新应用分数失败：{}", err);
            }

            // 打开应用程序
            AppRunner::run(&application.exec);
        }
    }
}
