use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub type PosTaggedToken = (String, String);
pub type PosTaggedSentence = Vec<PosTaggedToken>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerceptronTaggerState {
    weights: HashMap<String, HashMap<String, f64>>,
    tagdict: HashMap<String, String>,
    classes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AveragedPerceptron {
    pub weights: HashMap<String, HashMap<String, f64>>,
    pub classes: BTreeSet<String>,
    totals: HashMap<(String, String), f64>,
    tstamps: HashMap<(String, String), usize>,
    i: usize,
}

impl AveragedPerceptron {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn predict(&self, features: &HashMap<String, f64>) -> String {
        if self.classes.is_empty() {
            return "UNK".to_string();
        }

        let mut scores = HashMap::<String, f64>::new();
        for class in &self.classes {
            scores.insert(class.clone(), 0.0);
        }

        for (feature, value) in features {
            if *value == 0.0 {
                continue;
            }
            if let Some(weights) = self.weights.get(feature) {
                for (label, weight) in weights {
                    if let Some(score) = scores.get_mut(label) {
                        *score += value * weight;
                    }
                }
            }
        }

        let mut best_label = None::<String>;
        let mut best_score = f64::NEG_INFINITY;
        for label in &self.classes {
            let score = *scores.get(label).unwrap_or(&0.0);
            let is_better = score > best_score
                || ((score - best_score).abs() < f64::EPSILON
                    && best_label
                        .as_ref()
                        .map(|current| label < current)
                        .unwrap_or(true));
            if is_better {
                best_score = score;
                best_label = Some(label.clone());
            }
        }

        best_label.unwrap_or_else(|| "UNK".to_string())
    }

    pub fn update(&mut self, truth: &str, guess: &str, features: &HashMap<String, f64>) {
        self.i += 1;
        if truth == guess {
            return;
        }

        for feature in features.keys() {
            self.adjust_weight(feature, truth, 1.0);
            self.adjust_weight(feature, guess, -1.0);
        }
    }

    pub fn average_weights(&mut self) {
        let mut pruned = HashMap::<String, HashMap<String, f64>>::new();
        for (feature, class_weights) in &self.weights {
            let mut new_class_weights = HashMap::<String, f64>::new();
            for (class, weight) in class_weights {
                let key = (feature.clone(), class.clone());
                let total = self.totals.get(&key).copied().unwrap_or(0.0)
                    + ((self
                        .i
                        .saturating_sub(self.tstamps.get(&key).copied().unwrap_or(0))
                        as f64)
                        * *weight);
                let averaged = if self.i == 0 {
                    *weight
                } else {
                    (total / self.i as f64 * 1000.0).round() / 1000.0
                };
                if averaged != 0.0 {
                    new_class_weights.insert(class.clone(), averaged);
                }
            }
            if !new_class_weights.is_empty() {
                pruned.insert(feature.clone(), new_class_weights);
            }
        }
        self.weights = pruned;
    }

    fn adjust_weight(&mut self, feature: &str, class: &str, delta: f64) {
        let key = (feature.to_string(), class.to_string());
        let current_weight = self
            .weights
            .get(feature)
            .and_then(|weights| weights.get(class))
            .copied()
            .unwrap_or(0.0);

        let last_update = self.tstamps.get(&key).copied().unwrap_or(0);
        let total = self.totals.entry(key.clone()).or_insert(0.0);
        *total += ((self.i.saturating_sub(last_update)) as f64) * current_weight;
        self.tstamps.insert(key, self.i);

        let weights = self.weights.entry(feature.to_string()).or_default();
        weights.insert(class.to_string(), current_weight + delta);
    }
}

#[derive(Debug, Clone, Default)]
pub struct PerceptronTagger {
    model: AveragedPerceptron,
    tagdict: HashMap<String, String>,
    classes: BTreeSet<String>,
}

impl PerceptronTagger {
    pub const TAGGER_NAME: &'static str = "averaged_perceptron_tagger";
    pub const START: [&'static str; 2] = ["-START-", "-START2-"];
    pub const END: [&'static str; 2] = ["-END-", "-END2-"];

    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_untrained() -> Self {
        Self::default()
    }

    pub fn new_from_json(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        Self::load_from_json(path)
    }

    pub fn default_model_path(lang: &str) -> PathBuf {
        std::env::temp_dir().join(format!("{}_{}.json", Self::TAGGER_NAME, lang))
    }

    pub fn train(&mut self, sentences: &[PosTaggedSentence], iterations: usize) {
        self.tagdict = self.make_tagdict(sentences);
        self.classes = collect_classes(sentences);
        self.model.classes = self.classes.clone();

        for _ in 0..iterations {
            for sentence in sentences {
                if sentence.is_empty() {
                    continue;
                }
                let words: Vec<&str> = sentence.iter().map(|(w, _)| w.as_str()).collect();
                let gold_tags: Vec<&str> = sentence.iter().map(|(_, t)| t.as_str()).collect();

                let mut prev = Self::START[0].to_string();
                let mut prev2 = Self::START[1].to_string();
                let context = build_context(&words);

                for (i, word) in words.iter().enumerate() {
                    let guess = self.tagdict.get(*word).cloned().unwrap_or_else(|| {
                        let feats = self.features(i, word, &context, &prev, &prev2);
                        self.model.predict(&feats)
                    });
                    let truth = gold_tags[i];

                    if guess != truth {
                        let feats = self.features(i, word, &context, &prev, &prev2);
                        self.model.update(truth, &guess, &feats);
                    }

                    prev2 = prev;
                    prev = guess;
                }
            }
        }

        self.model.average_weights();
    }

    pub fn tag(&self, tokens: &[String]) -> PosTaggedSentence {
        if tokens.is_empty() {
            return Vec::new();
        }

        let words: Vec<&str> = tokens.iter().map(|w| w.as_str()).collect();
        let mut out = Vec::with_capacity(tokens.len());
        let context = build_context(&words);
        let mut prev = Self::START[0].to_string();
        let mut prev2 = Self::START[1].to_string();

        for (i, word) in words.iter().enumerate() {
            let tag = self.tagdict.get(*word).cloned().unwrap_or_else(|| {
                let feats = self.features(i, word, &context, &prev, &prev2);
                self.model.predict(&feats)
            });
            out.push(((*word).to_string(), tag.clone()));
            prev2 = prev;
            prev = tag;
        }

        out
    }

    pub fn tag_sents(&self, sentences: &[Vec<String>]) -> Vec<PosTaggedSentence> {
        sentences.iter().map(|sent| self.tag(sent)).collect()
    }

    pub fn accuracy(&self, gold: &[PosTaggedSentence]) -> f64 {
        let mut correct = 0usize;
        let mut total = 0usize;

        for sentence in gold {
            let words = sentence.iter().map(|(w, _)| w.clone()).collect::<Vec<_>>();
            let predicted = self.tag(&words);
            for ((_, expected), (_, guess)) in sentence.iter().zip(predicted.iter()) {
                if expected == guess {
                    correct += 1;
                }
                total += 1;
            }
        }

        if total == 0 {
            0.0
        } else {
            correct as f64 / total as f64
        }
    }

    pub fn classes(&self) -> &BTreeSet<String> {
        &self.classes
    }

    pub fn save_to_json(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let state = PerceptronTaggerState {
            weights: self.model.weights.clone(),
            tagdict: self.tagdict.clone(),
            classes: self.classes.iter().cloned().collect(),
        };

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &state)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
    }

    pub fn save_default_json(&self, lang: &str) -> Result<PathBuf, std::io::Error> {
        let path = Self::default_model_path(lang);
        self.save_to_json(&path)?;
        Ok(path)
    }

    pub fn load_from_json(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let state: PerceptronTaggerState = serde_json::from_reader(reader)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;

        let classes = state.classes.into_iter().collect::<BTreeSet<String>>();
        let model = AveragedPerceptron {
            weights: state.weights,
            classes: classes.clone(),
            totals: HashMap::new(),
            tstamps: HashMap::new(),
            i: 0,
        };

        Ok(Self {
            model,
            tagdict: state.tagdict,
            classes,
        })
    }

    pub fn load_default_json(lang: &str) -> Result<Self, std::io::Error> {
        Self::load_from_json(Self::default_model_path(lang))
    }

    fn normalize(word: &str) -> String {
        if word.contains('-') && !word.starts_with('-') {
            return "!HYPHEN".to_string();
        }
        if word.chars().all(|c| c.is_ascii_digit()) && word.len() == 4 {
            return "!YEAR".to_string();
        }
        if word
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            return "!DIGITS".to_string();
        }
        word.to_lowercase()
    }

    fn features(
        &self,
        i: usize,
        word: &str,
        context: &[String],
        prev: &str,
        prev2: &str,
    ) -> HashMap<String, f64> {
        let index = i + Self::START.len();
        let mut features = HashMap::<String, f64>::new();

        add_feature(&mut features, "bias");
        add_feature(&mut features, &format!("i suffix {}", suffix(word, 3)));
        add_feature(&mut features, &format!("i pref1 {}", prefix(word, 1)));
        add_feature(&mut features, &format!("i-1 tag {prev}"));
        add_feature(&mut features, &format!("i-2 tag {prev2}"));
        add_feature(&mut features, &format!("i tag+i-2 tag {prev} {prev2}"));
        add_feature(&mut features, &format!("i word {}", context[index]));
        add_feature(
            &mut features,
            &format!("i-1 tag+i word {prev} {}", context[index]),
        );
        add_feature(&mut features, &format!("i-1 word {}", context[index - 1]));
        add_feature(
            &mut features,
            &format!("i-1 suffix {}", suffix(&context[index - 1], 3)),
        );
        add_feature(&mut features, &format!("i-2 word {}", context[index - 2]));
        add_feature(&mut features, &format!("i+1 word {}", context[index + 1]));
        add_feature(
            &mut features,
            &format!("i+1 suffix {}", suffix(&context[index + 1], 3)),
        );
        add_feature(&mut features, &format!("i+2 word {}", context[index + 2]));
        features
    }

    fn make_tagdict(&self, sentences: &[PosTaggedSentence]) -> HashMap<String, String> {
        let mut counts = HashMap::<String, HashMap<String, usize>>::new();

        for sentence in sentences {
            for (word, tag) in sentence {
                let freq = counts
                    .entry(word.clone())
                    .or_default()
                    .entry(tag.clone())
                    .or_insert(0);
                *freq += 1;
            }
        }

        let freq_threshold = 20usize;
        let ambiguity_threshold = 0.97f64;
        let mut tagdict = HashMap::<String, String>::new();

        for (word, tag_freqs) in counts {
            let mut best_tag = String::new();
            let mut best_count = 0usize;
            let mut total = 0usize;
            for (tag, count) in tag_freqs {
                total += count;
                if count > best_count {
                    best_count = count;
                    best_tag = tag;
                }
            }

            if total >= freq_threshold && (best_count as f64 / total as f64) >= ambiguity_threshold
            {
                tagdict.insert(word, best_tag);
            }
        }

        tagdict
    }
}

fn collect_classes(sentences: &[PosTaggedSentence]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for sentence in sentences {
        for (_, tag) in sentence {
            out.insert(tag.clone());
        }
    }
    out
}

fn build_context(words: &[&str]) -> Vec<String> {
    let mut out = Vec::with_capacity(words.len() + 4);
    out.push(PerceptronTagger::START[0].to_string());
    out.push(PerceptronTagger::START[1].to_string());
    out.extend(words.iter().map(|w| PerceptronTagger::normalize(w)));
    out.push(PerceptronTagger::END[0].to_string());
    out.push(PerceptronTagger::END[1].to_string());
    out
}

fn add_feature(features: &mut HashMap<String, f64>, name: &str) {
    let value = features.entry(name.to_string()).or_insert(0.0);
    *value += 1.0;
}

fn suffix(text: &str, n: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let start = chars.len().saturating_sub(n);
    chars[start..].iter().collect()
}

fn prefix(text: &str, n: usize) -> String {
    text.chars().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn averaged_perceptron_predicts_known_class() {
        let mut model = AveragedPerceptron::new();
        model.classes.insert("NN".to_string());
        model.classes.insert("VB".to_string());
        model.weights.insert(
            "i suffix ing".to_string(),
            HashMap::from([("VB".to_string(), 2.0), ("NN".to_string(), -1.0)]),
        );

        let mut feats = HashMap::new();
        feats.insert("i suffix ing".to_string(), 1.0);
        assert_eq!(model.predict(&feats), "VB");
    }

    #[test]
    fn perceptron_tagger_trains_and_tags() {
        let train = vec![
            vec![
                ("The".to_string(), "DT".to_string()),
                ("cat".to_string(), "NN".to_string()),
                ("runs".to_string(), "VBZ".to_string()),
            ],
            vec![
                ("A".to_string(), "DT".to_string()),
                ("dog".to_string(), "NN".to_string()),
                ("runs".to_string(), "VBZ".to_string()),
            ],
        ];

        let mut tagger = PerceptronTagger::new();
        tagger.train(&train, 3);

        let tagged = tagger.tag(&["The".to_string(), "dog".to_string(), "runs".to_string()]);
        assert_eq!(tagged.len(), 3);
        assert!(tagger.classes().contains("DT"));
        assert!(tagger.accuracy(&train) > 0.0);
    }

    #[test]
    fn perceptron_tagger_json_roundtrip() {
        let train = vec![
            vec![
                ("The".to_string(), "DT".to_string()),
                ("cat".to_string(), "NN".to_string()),
                ("runs".to_string(), "VBZ".to_string()),
            ],
            vec![
                ("A".to_string(), "DT".to_string()),
                ("dog".to_string(), "NN".to_string()),
                ("runs".to_string(), "VBZ".to_string()),
            ],
        ];

        let mut tagger = PerceptronTagger::new();
        tagger.train(&train, 3);

        let mut path = std::env::temp_dir();
        let unique = format!(
            "organon_perceptron_tagger_{}_{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        );
        path.push(unique);

        tagger.save_to_json(&path).expect("save model");
        let loaded = PerceptronTagger::load_from_json(&path).expect("load model");

        let sentence = vec!["The".to_string(), "dog".to_string(), "runs".to_string()];
        assert_eq!(tagger.tag(&sentence), loaded.tag(&sentence));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn perceptron_tagger_default_path_roundtrip() {
        let train = vec![vec![
            ("Bird".to_string(), "NNP".to_string()),
            ("flies".to_string(), "VBZ".to_string()),
        ]];

        let mut tagger = PerceptronTagger::new_untrained();
        tagger.train(&train, 2);

        let lang = format!(
            "test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        );
        let path = tagger.save_default_json(&lang).expect("save default");
        let loaded = PerceptronTagger::load_default_json(&lang).expect("load default");

        let sentence = vec!["Bird".to_string(), "flies".to_string()];
        assert_eq!(tagger.tag(&sentence), loaded.tag(&sentence));

        let _ = fs::remove_file(path);
    }
}
