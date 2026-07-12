/// build.rs 是“程序打包器”：这是一个 Cargo 构建脚本。它的任务是在执行“cargo build”时， 调用 glib_build_tools::compile_resources 函数，读取并处理 resources.gresource.xml 中列出的资源，将它们编译成一个二进制资源包，并静态链接到最终的可执行文件中
fn main() {
    glib_build_tools::compile_resources(
        &["resources"],                      // 资源目录
        "resources/resources.gresource.xml", // 资源清单路径
        "org.zhoujing.storage",              // 生成的资源包标识符
    );
}
