//! N-gram preprocessing helpers.

pub fn pad_both_ends<T: AsRef<str>>(
    sentence: &[T],
    n: usize,
    left_pad: &str,
    right_pad: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    if n > 0 {
        for _ in 0..(n.saturating_sub(1)) {
            out.push(left_pad.to_string());
        }
    }
    out.extend(sentence.iter().map(|tok| tok.as_ref().to_string()));
    if n > 0 {
        for _ in 0..(n.saturating_sub(1)) {
            out.push(right_pad.to_string());
        }
    }
    out
}

pub fn pad_both_ends_default<T: AsRef<str>>(sentence: &[T], n: usize) -> Vec<String> {
    pad_both_ends(sentence, n, "<s>", "</s>")
}

pub fn ngrams<T: AsRef<str>>(tokens: &[T], n: usize) -> Vec<Vec<String>> {
    if n == 0 || tokens.len() < n {
        return Vec::new();
    }
    let mut out = Vec::new();
    for i in 0..=tokens.len() - n {
        let slice = tokens[i..i + n]
            .iter()
            .map(|t| t.as_ref().to_string())
            .collect();
        out.push(slice);
    }
    out
}

pub fn everygrams<T: AsRef<str>>(tokens: &[T], max_len: usize) -> Vec<Vec<String>> {
    if max_len == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    for start in 0..tokens.len() {
        for len in 1..=max_len {
            if start + len > tokens.len() {
                break;
            }
            let slice = tokens[start..start + len]
                .iter()
                .map(|t| t.as_ref().to_string())
                .collect();
            out.push(slice);
        }
    }
    out
}

pub fn padded_everygrams<T: AsRef<str>>(order: usize, sentence: &[T]) -> Vec<Vec<String>> {
    let padded = pad_both_ends_default(sentence, order);
    everygrams(&padded, order)
}

pub fn padded_everygram_pipeline(
    order: usize,
    text: &[Vec<String>],
) -> (Vec<Vec<Vec<String>>>, Vec<String>) {
    let mut train = Vec::new();
    let mut vocab = Vec::new();
    for sentence in text {
        let padded = pad_both_ends_default(sentence, order);
        vocab.extend(padded.iter().cloned());
        train.push(everygrams(&padded, order));
    }
    (train, vocab)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_both_ends_default_adds_padding() {
        let sentence = vec!["a", "b", "c"];
        let padded = pad_both_ends_default(&sentence, 2);
        assert_eq!(padded, vec!["<s>", "a", "b", "c", "</s>"]);
    }

    #[test]
    fn padded_everygrams_produces_unigrams_and_bigrams() {
        let sentence = vec!["a", "b"];
        let grams = padded_everygrams(2, &sentence);
        assert!(grams.contains(&vec!["<s>".to_string()]));
        assert!(grams.contains(&vec!["<s>".to_string(), "a".to_string()]));
        assert!(grams.contains(&vec!["a".to_string(), "b".to_string()]));
    }

    #[test]
    fn padded_everygram_pipeline_returns_train_and_vocab() {
        let text = vec![vec!["a".to_string(), "b".to_string()]];
        let (train, vocab) = padded_everygram_pipeline(2, &text);
        assert_eq!(train.len(), 1);
        assert!(vocab.contains(&"<s>".to_string()));
        assert!(vocab.contains(&"</s>".to_string()));
    }
}
