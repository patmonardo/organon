//! Tag utility helpers.

pub fn str2tuple(text: &str, sep: &str) -> (String, Option<String>) {
    if let Some(index) = text.rfind(sep) {
        let word = text[..index].to_string();
        let tag = text[index + sep.len()..].to_uppercase();
        (word, Some(tag))
    } else {
        (text.to_string(), None)
    }
}

pub fn tuple2str(tagged_token: (&str, Option<&str>), sep: &str) -> String {
    match tagged_token.1 {
        Some(tag) => {
            assert!(!tag.contains(sep), "tag may not contain sep");
            format!("{}{sep}{tag}", tagged_token.0)
        }
        None => tagged_token.0.to_string(),
    }
}

pub fn untag(tagged_sentence: &[(String, String)]) -> Vec<String> {
    tagged_sentence
        .iter()
        .map(|(word, _)| word.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str2tuple_splits_at_rightmost_separator() {
        assert_eq!(str2tuple("fly/NN", "/"), ("fly".to_string(), Some("NN".to_string())));
        assert_eq!(str2tuple("a/b/nn", "/"), ("a/b".to_string(), Some("NN".to_string())));
        assert_eq!(str2tuple("fly", "/"), ("fly".to_string(), None));
    }

    #[test]
    fn tuple2str_joins_word_and_tag() {
        assert_eq!(tuple2str(("fly", Some("NN")), "/"), "fly/NN");
        assert_eq!(tuple2str(("fly", None), "/"), "fly");
    }

    #[test]
    fn untag_extracts_words() {
        let sentence = vec![
            ("John".to_string(), "NNP".to_string()),
            ("walks".to_string(), "VBZ".to_string()),
        ];
        assert_eq!(untag(&sentence), vec!["John".to_string(), "walks".to_string()]);
    }
}
