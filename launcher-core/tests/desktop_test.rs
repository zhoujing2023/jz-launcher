use std::fs::read_to_string;

/// `test_read_desktop_entry_sections`
/// 只获取 \[Desktop Entry] 部分的数据
#[test]
fn test_read_desktop_entry_sections() {
    let file_path = String::from("/usr/share/applications/google-chrome.desktop");
    let content = read_to_string(file_path).unwrap();
    let sections = content.split("[Desktop");
    let desktop_entry_content: String = sections.filter(|s| s.starts_with(" Entry]")).collect();
    println!("{}", desktop_entry_content);
}

/// 模糊查询
fn fuzzy_match(text: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return false;
    }

    let mut pattern_chars = pattern.chars();
    let mut cur_pattern_char = pattern_chars.next();

    for char in text.chars() {
        if let Some(p_char) = cur_pattern_char {
            if char == p_char {
                cur_pattern_char = pattern_chars.next();
            }
        } else {
            return true;
        }
    }
    cur_pattern_char.is_none()
}

/// `test_fuzzy_match` 测试模糊搜索
#[test]
fn test_fuzzy_match() {
    let name = "WeChat".to_lowercase();
    println!("{}", fuzzy_match(&name, "we"));
    println!("{}", fuzzy_match(&name, "wc"));
    println!("{}", fuzzy_match(&name, "wt"));
    println!("{}", fuzzy_match(&name, "ca"));
    println!("{}", fuzzy_match(&name, "tw"));
}

/// `test_get_desktop_exec_cmd` 获取 desktop 文件中的执行指令
#[test]
fn test_get_desktop_exec_cmd() {
    let desktop_content = read_to_string("/usr/share/applications/google-chrome.desktop").unwrap();
    let exec_cmd = desktop_content.lines().find(|s| s.starts_with("Exec"));
    if let Some(exec_cmd) = exec_cmd {
        let cmd = exec_cmd.strip_prefix("Exec=");
        println!("{:?}", cmd);
    }
}
