const FILE_PATH: &str = "/home/zhoujing/.config/jz_tools/usage.json";
const FILE_DIR: &str = "/home/zhoujing/.config/jz_tools";

/// `test_file_read_or_write` 文件读取，如果目录不存在则创建，再写入
#[test]
fn test_file_read_or_write() {
    let content = std::fs::read_to_string(FILE_PATH);
    match content {
        Ok(content) => {
            println!("{}", content);
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                if let Err(err) = std::fs::create_dir_all(FILE_DIR) {
                    eprintln!("创建目录失败: {}", err);
                    return;
                }
                if let Err(err) = std::fs::write(FILE_PATH, "hello") {
                    eprintln!("写入文件失败：{}", err);
                    return;
                }
            }
            println!("读取文件失败：{}", err);
        }
    }
}

/// `test_file_write_or_create` 文件写入，如果目录不存在则创建，再写入
#[test]
fn test_file_write_or_create() {
    if let Err(err) = std::fs::write(FILE_PATH, "hello") {
        if err.kind() == std::io::ErrorKind::NotFound {
            if let Err(err) = std::fs::create_dir_all(FILE_DIR) {
                eprintln!("创建 {} 失败：{}", FILE_DIR, err);
                return;
            }
            let _ = std::fs::write(FILE_PATH, "hello");
            return;
        }
        eprintln!("写入文件 {} 失败：{}", FILE_PATH, err);
    }
}
