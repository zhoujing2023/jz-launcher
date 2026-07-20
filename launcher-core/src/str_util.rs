use pinyin::ToPinyin;
pub struct StrUtil;

impl StrUtil {
    /// `contains_chinese` 判断字符串是否包含中文字符
    pub fn contains_chinese(text: &str) -> bool {
        text.chars().any(|c| matches!(c, '\u{4E00}'..='\u{9FFF}'))
    }

    /// 将字符串转换为拼音
    pub fn parse_pinyin(text: &str) -> String {
        text.to_pinyin().flatten().map(|c| c.plain()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::str_util::StrUtil;

    #[test]
    fn test_pinyin() {
        let text = "小明";
        let pinyin = StrUtil::parse_pinyin(text);
        assert_eq!("xiaoming", pinyin);
    }
}
