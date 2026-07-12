use std::process::{Command, Stdio};

pub struct AppRunner;

impl AppRunner {

    /// `run` 启动应用程序
    pub fn run(exec: &str) {
        println!("打开的文件：{}", exec);
        let parts: Vec<&str> = exec.split_whitespace().collect();
        let Some(cmd) = parts.first() else {
            println!("exec为空，无法执行打开操作");
            return;
        };

        // 解析占位符（去除 %U，%F，……）
        let args: Vec<&str> = parts[1..]
            .iter()
            .filter(|arg| !arg.contains('%'))
            .copied()
            .collect();

        // 使用 setsid 让子进程在新的会话中运行，完全独立于父进程
        match Command::new("setsid")
            .arg("-f")  // fork 并立即退出，让进程在后台运行
            .arg(cmd)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(mut child) => {
                // 立即等待 setsid 命令完成（它会立即退出），实际的应用程序已经在后台独立运行了
                let _ = child.wait();
                println!("✅ 启动成功");
            }
            Err(err) => {
                eprintln!("❌ 启动失败：{}", err);
            }
        }
    }
}
