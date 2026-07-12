use crate::model::{AppEntry, Apps};

pub struct SearchEngine {
    apps: Apps,
}

impl SearchEngine {
    /// `new` 创建搜索实例
    pub fn new(apps: Apps) -> Self {
        Self { apps }
    }

    /// `get_apps` 获取 apps 属性
    pub fn get_apps(&self) -> &Apps {
        &self.apps
    }

    ///  `search` 搜索
    pub fn search(&self, keyword: &str) -> Vec<&AppEntry> {
        let keyword_lower = keyword.to_lowercase();
        let keyword_lower = keyword_lower.as_str();
        let mut app_entry_list: Vec<_> = self
            .apps
            .iter()
            .filter(|app| Self::fuzzy_match(&app.search_key, keyword_lower))
            .collect();
        // 根据 score 降序排序
        app_entry_list.sort_by(|a, b| b.score.borrow().cmp(&a.score.borrow()));
        app_entry_list
    }

    /// `fuzzy_match` 模糊匹配
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
}

#[cfg(test)]
mod tests {
    use crate::model::AppEntry;
    use crate::search_engine::SearchEngine;
    use std::cell::RefCell;

    /// 测试模糊匹配
    #[test]
    fn test_fuzzy_match_test() {
        assert!(SearchEngine::fuzzy_match("test", "te"));
        assert!(SearchEngine::fuzzy_match("wechat", "wt"));
        assert!(SearchEngine::fuzzy_match("krita", "kr"));
        assert!(!SearchEngine::fuzzy_match("wechat", "wx"));
    }

    #[test]
    fn test_fuzzy_match_is_case_sensitive() {
        assert!(!SearchEngine::fuzzy_match("test", "TE"));
        assert!(!SearchEngine::fuzzy_match("wechat", "WeChat"));
    }

    /// 测试排序
    #[test]
    fn test_search_returns_sorted_by_score() {
        let apps = vec![
            AppEntry {
                name: "微信".to_string(),
                search_key: "wechat,微信".to_string(),
                score: RefCell::new(5),
                ..Default::default()
            },
            AppEntry {
                name: "blender".to_string(),
                search_key: "blender,布兰德".to_string(),
                score: RefCell::new(3),
                ..Default::default()
            },
            AppEntry {
                name: "Godot".to_string(),
                search_key: "godot".to_string(),
                score: RefCell::new(1),
                ..Default::default()
            },
        ];
        let search_engine = SearchEngine::new(apps);
        let results = search_engine.search("e");
        assert_eq!(results.len(), 2);
        assert_eq!(*results[0].score.borrow(), 5);
        assert_eq!(*results[1].score.borrow(), 3);
    }
}
