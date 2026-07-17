use crate::model::{AppEntry, Apps};

#[derive(Default)]
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
        let keyword_lower = keyword.trim().to_lowercase();
        if keyword_lower.is_empty() {
            return Vec::new();
        }

        let mut results: Vec<_> = self
            .apps
            .iter()
            .filter_map(|app| {
                // search_key 由不同语言的应用名称以逗号分隔组成。分别计算每个名称，
                // 避免一次匹配跨越两个名称的边界。
                let rank = app
                    .search_key
                    .split(',')
                    .filter_map(|key| Self::match_rank(key.trim(), &keyword_lower))
                    .min()?;
                Some((app, rank))
            })
            .collect();

        results.sort_by(|(app_a, rank_a), (app_b, rank_b)| {
            rank_a
                .cmp(rank_b)
                // 匹配质量相同时，使用次数较高的应用优先。
                // .then_with(|| app_b.score.borrow().cmp(&app_a.score.borrow()))
                // 最后按名称升序，保证结果稳定且符合用户预期。
                .then_with(|| app_a.name.to_lowercase().cmp(&app_b.name.to_lowercase()))
        });

        results.into_iter().map(|(app, _)| app).collect()
    }

    /// 计算匹配质量，数值越小代表匹配越好。
    ///
    /// 排序级别：完全匹配、前缀匹配、连续子串、模糊字符序列。
    /// 后两个数值分别表示开始位置和匹配跨度。
    fn match_rank(text: &str, pattern: &str) -> Option<(u8, usize, usize)> {
        if pattern.is_empty() {
            return None;
        }

        if text == pattern {
            return Some((0, 0, pattern.chars().count()));
        }
        if text.starts_with(pattern) {
            return Some((1, 0, pattern.chars().count()));
        }
        if let Some(byte_index) = text.find(pattern) {
            let char_index = text[..byte_index].chars().count();
            return Some((2, char_index, pattern.chars().count()));
        }

        let mut pattern_chars = pattern.chars();
        let mut expected = pattern_chars.next()?;
        let mut first_index = None;

        for (index, character) in text.chars().enumerate() {
            if character != expected {
                continue;
            }

            first_index.get_or_insert(index);
            match pattern_chars.next() {
                Some(next) => expected = next,
                None => {
                    let first_index = first_index.unwrap_or(index);
                    return Some((3, first_index, index - first_index + 1));
                }
            }
        }

        None
    }

    /// `fuzzy_match` 模糊匹配
    #[cfg(test)]
    fn fuzzy_match(text: &str, pattern: &str) -> bool {
        Self::match_rank(text, pattern).is_some()
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

    #[test]
    fn test_prefix_match_precedes_fuzzy_match() {
        let apps = vec![
            AppEntry {
                name: "Clash Verge".to_string(),
                search_key: "clash verge".to_string(),
                score: RefCell::new(100),
                ..Default::default()
            },
            AppEntry {
                name: "Calculator".to_string(),
                search_key: "calculator".to_string(),
                score: RefCell::new(0),
                ..Default::default()
            },
        ];

        let search_engine = SearchEngine::new(apps);
        let results = search_engine.search("ca");

        assert_eq!(results[0].name, "Calculator");
        assert_eq!(results[1].name, "Clash Verge");
    }

    #[test]
    fn test_same_quality_results_are_sorted_by_name() {
        let apps = vec![
            AppEntry {
                name: "Camera".to_string(),
                search_key: "camera".to_string(),
                ..Default::default()
            },
            AppEntry {
                name: "Calculator".to_string(),
                search_key: "calculator".to_string(),
                ..Default::default()
            },
        ];

        let search_engine = SearchEngine::new(apps);
        let results = search_engine.search("ca");

        assert_eq!(results[0].name, "Calculator");
        assert_eq!(results[1].name, "Camera");
    }
}
