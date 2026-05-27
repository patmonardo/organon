//! Shell Help — doctrinal catalog of the GDS Shell vocabulary.
//!
//! The Shell sees itself through the nine-moment schema:
//!
//!   Being   → Frame : Series :: Expr
//!   Essence → Model : Feature :: Plan
//!   Concept → Corpus : Language :: Semantics
//!
//! Every macro, function, and type the Shell re-exports is placed in one of
//! these moments. `ShellHelp::new()` returns the full catalog. Use
//! `for_fold`, `for_moment`, or `find` to query it.

use std::fmt;

// ── Doctrinal address ────────────────────────────────────────────────────────

/// The three logical folds of the Shell vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellFold {
    /// Frame : Series :: Expr  (Being / Immediacy)
    FrameSeriesExpr,
    /// Model : Feature :: Plan  (Essence / Mediation)
    ModelFeaturePlan,
    /// Corpus : Language :: Semantics  (Concept / Return)
    CorpusLanguageSem,
}

impl fmt::Display for ShellFold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellFold::FrameSeriesExpr => write!(f, "Frame:Series::Expr"),
            ShellFold::ModelFeaturePlan => write!(f, "Model:Feature::Plan"),
            ShellFold::CorpusLanguageSem => write!(f, "Corpus:Language::Semantics"),
        }
    }
}

/// The nine moments — three per fold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellMomentKind {
    // Being fold
    Frame,
    Series,
    Expr,
    // Essence fold
    Model,
    Feature,
    Plan,
    // Concept fold
    Corpus,
    Language,
    Semantics,
}

impl ShellMomentKind {
    pub fn fold(self) -> ShellFold {
        match self {
            ShellMomentKind::Frame | ShellMomentKind::Series | ShellMomentKind::Expr => {
                ShellFold::FrameSeriesExpr
            }
            ShellMomentKind::Model | ShellMomentKind::Feature | ShellMomentKind::Plan => {
                ShellFold::ModelFeaturePlan
            }
            ShellMomentKind::Corpus | ShellMomentKind::Language | ShellMomentKind::Semantics => {
                ShellFold::CorpusLanguageSem
            }
        }
    }
}

impl fmt::Display for ShellMomentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellMomentKind::Frame => write!(f, "Frame"),
            ShellMomentKind::Series => write!(f, "Series"),
            ShellMomentKind::Expr => write!(f, "Expr"),
            ShellMomentKind::Model => write!(f, "Model"),
            ShellMomentKind::Feature => write!(f, "Feature"),
            ShellMomentKind::Plan => write!(f, "Plan"),
            ShellMomentKind::Corpus => write!(f, "Corpus"),
            ShellMomentKind::Language => write!(f, "Language"),
            ShellMomentKind::Semantics => write!(f, "Semantics"),
        }
    }
}

// ── Vocabulary kinds ─────────────────────────────────────────────────────────

/// Kind of Shell vocabulary item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellVocabKind {
    /// Declarative macro (invoked with `!`).
    Macro,
    /// Top-level free function.
    Function,
    /// Core type or trait.
    Type,
}

impl fmt::Display for ShellVocabKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellVocabKind::Macro => write!(f, "macro"),
            ShellVocabKind::Function => write!(f, "fn"),
            ShellVocabKind::Type => write!(f, "type"),
        }
    }
}

// ── Help entry ───────────────────────────────────────────────────────────────

/// One entry in the Shell vocabulary catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellHelpEntry {
    pub name: &'static str,
    pub kind: ShellVocabKind,
    pub moment: ShellMomentKind,
    pub synopsis: &'static str,
    pub example: &'static str,
}

impl ShellHelpEntry {
    const fn new(
        name: &'static str,
        kind: ShellVocabKind,
        moment: ShellMomentKind,
        synopsis: &'static str,
        example: &'static str,
    ) -> Self {
        Self {
            name,
            kind,
            moment,
            synopsis,
            example,
        }
    }
}

impl fmt::Display for ShellHelpEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{fold}/{moment}] {kind} {name} — {synopsis}",
            fold = self.moment.fold(),
            moment = self.moment,
            kind = self.kind,
            name = self.name,
            synopsis = self.synopsis,
        )
    }
}

// ── Full catalog ─────────────────────────────────────────────────────────────

/// Shell vocabulary catalog indexed by the nine-moment schema.
pub struct ShellHelp {
    entries: &'static [ShellHelpEntry],
}

impl ShellHelp {
    /// Return the full Shell vocabulary catalog.
    pub fn new() -> Self {
        Self { entries: CATALOG }
    }

    /// All entries.
    pub fn all(&self) -> &[ShellHelpEntry] {
        self.entries
    }

    /// Entries belonging to a specific fold.
    pub fn for_fold(&self, fold: ShellFold) -> Vec<&ShellHelpEntry> {
        self.entries
            .iter()
            .filter(|e| e.moment.fold() == fold)
            .collect()
    }

    /// Entries belonging to a specific moment.
    pub fn for_moment(&self, moment: ShellMomentKind) -> Vec<&ShellHelpEntry> {
        self.entries.iter().filter(|e| e.moment == moment).collect()
    }

    /// Find an entry by exact name (macro or function name, without `!`).
    pub fn find(&self, name: &str) -> Option<&ShellHelpEntry> {
        self.entries.iter().find(|e| e.name == name)
    }

    /// Print a formatted overview of all folds and moments to stdout.
    pub fn print_overview(&self) {
        let folds = [
            ShellFold::FrameSeriesExpr,
            ShellFold::ModelFeaturePlan,
            ShellFold::CorpusLanguageSem,
        ];
        for fold in folds {
            println!("── {} ──", fold);
            let moments = moments_for_fold(fold);
            for moment in moments {
                let entries = self.for_moment(*moment);
                if entries.is_empty() {
                    continue;
                }
                println!("  {}:", moment);
                for e in entries {
                    println!(
                        "    {:12} {:6}  {}",
                        e.name,
                        format!("({})", e.kind),
                        e.synopsis
                    );
                }
            }
            println!();
        }
    }

    /// Print the full help for a single named item.
    pub fn print_entry(&self, name: &str) {
        match self.find(name) {
            Some(e) => {
                println!("{}", e);
                println!("  example : {}", e.example);
            }
            None => println!("No entry found for '{name}'."),
        }
    }
}

impl Default for ShellHelp {
    fn default() -> Self {
        Self::new()
    }
}

fn moments_for_fold(fold: ShellFold) -> &'static [ShellMomentKind] {
    match fold {
        ShellFold::FrameSeriesExpr => &[
            ShellMomentKind::Frame,
            ShellMomentKind::Series,
            ShellMomentKind::Expr,
        ],
        ShellFold::ModelFeaturePlan => &[
            ShellMomentKind::Model,
            ShellMomentKind::Feature,
            ShellMomentKind::Plan,
        ],
        ShellFold::CorpusLanguageSem => &[
            ShellMomentKind::Corpus,
            ShellMomentKind::Language,
            ShellMomentKind::Semantics,
        ],
    }
}

// ── Static catalog ───────────────────────────────────────────────────────────

static CATALOG: &[ShellHelpEntry] = &[
    // ── Frame : Series :: Expr  (Being / Immediacy) ──────────────────────────

    // Frame — immediate tabular construction
    ShellHelpEntry::new(
        "tbl",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Build a GDSDataFrame from column literals.",
        r#"let df = tbl!((a: i64 => [1,2,3]), (b: ["x","y","z"]))?;"#,
    ),
    ShellHelpEntry::new(
        "tbl_def",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Build a GDSDataFrame with explicit schema definitions.",
        r#"let df = tbl_def!((id: i64 => [1,2]), (label: ["a","b"]))?;"#,
    ),
    ShellHelpEntry::new(
        "select",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Select columns from a lazy frame.",
        r#"let out = select!(df.lazy(), [col!("x"), col!("y")])?;"#,
    ),
    ShellHelpEntry::new(
        "select_columns",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Select columns by name strings.",
        r#"let out = select_columns!(df, "id", "label")?;"#,
    ),
    ShellHelpEntry::new(
        "sc",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Shorthand alias for select_columns.",
        r#"let out = sc!(df, "id")?;"#,
    ),
    ShellHelpEntry::new(
        "sel",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Select with typed selector expressions.",
        r#"let out = sel!(df, [col!("id")])?;"#,
    ),
    ShellHelpEntry::new(
        "filter",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Filter rows with a predicate expression.",
        r#"let out = filter!(df, col!("score").gt(lit!(10)))?;"#,
    ),
    ShellHelpEntry::new(
        "where_",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Alias for filter — where-clause style.",
        r#"let out = where_!(df, col!("active").eq(lit!(true)))?;"#,
    ),
    ShellHelpEntry::new(
        "mutate",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Add or overwrite columns with new expressions.",
        r#"let out = mutate!(df, [col!("x").alias("y")])?;"#,
    ),
    ShellHelpEntry::new(
        "arrange",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Sort rows by one or more columns.",
        r#"let out = arrange!(df, ["score"], [true])?;"#,
    ),
    ShellHelpEntry::new(
        "join",
        ShellVocabKind::Macro,
        ShellMomentKind::Frame,
        "Join two frames on a key column.",
        r#"let out = join!(left, right, on: "id")?;"#,
    ),
    // Series — column-level operations
    ShellHelpEntry::new(
        "col",
        ShellVocabKind::Macro,
        ShellMomentKind::Series,
        "Reference a column by name as an Expr.",
        r#"let e = col!("score");"#,
    ),
    ShellHelpEntry::new(
        "lit",
        ShellVocabKind::Macro,
        ShellMomentKind::Series,
        "Lift a scalar into a column Expr.",
        r#"let e = lit!(42);"#,
    ),
    ShellHelpEntry::new(
        "summarize",
        ShellVocabKind::Macro,
        ShellMomentKind::Series,
        "Aggregate a frame: mean, sum, min, max.",
        r#"let out = summarize!(df, [col!("x").mean()])?;"#,
    ),
    ShellHelpEntry::new(
        "group_by",
        ShellVocabKind::Macro,
        ShellMomentKind::Series,
        "Group a frame by key columns, then aggregate.",
        r#"let out = group_by!(df, ["category"], [col!("val").sum()])?;"#,
    ),
    ShellHelpEntry::new(
        "agg",
        ShellVocabKind::Macro,
        ShellMomentKind::Series,
        "Build an aggregation expression list.",
        r#"let aggs = agg![col!("x").sum(), col!("y").mean()];"#,
    ),
    // Expr — lazy-query + conditional
    ShellHelpEntry::new(
        "expr",
        ShellVocabKind::Macro,
        ShellMomentKind::Expr,
        "Lift a Polars Expr value directly.",
        r#"let e = expr!(polars_col("x"));"#,
    ),
    ShellHelpEntry::new(
        "q",
        ShellVocabKind::Macro,
        ShellMomentKind::Expr,
        "Quick lazy-frame query: select + filter in one step.",
        r#"let out = q!(df, select: [col!("x")], where: col!("x").gt(lit!(0)))?;"#,
    ),
    ShellHelpEntry::new(
        "when",
        ShellVocabKind::Macro,
        ShellMomentKind::Expr,
        "Begin a when/then/otherwise conditional expression.",
        r#"let e = when!(col!("x").gt(lit!(0))).then(lit!(1)).otherwise(lit!(-1));"#,
    ),
    ShellHelpEntry::new(
        "then",
        ShellVocabKind::Macro,
        ShellMomentKind::Expr,
        "The 'then' arm of a when expression.",
        r#"when!(cond).then(lit!(1)).otherwise(lit!(0))"#,
    ),
    ShellHelpEntry::new(
        "otherwise",
        ShellVocabKind::Macro,
        ShellMomentKind::Expr,
        "The 'otherwise' arm of a when expression.",
        r#"when!(cond).then(lit!(1)).otherwise(lit!(0))"#,
    ),
    // ── Model : Feature :: Plan  (Essence / Mediation) ───────────────────────

    // Model — dataset construction and projection
    ShellHelpEntry::new(
        "ds",
        ShellVocabKind::Macro,
        ShellMomentKind::Model,
        "Declare a Dataset with name and artifact kind.",
        r#"let dataset = ds!("my-ds", ProgramImage);"#,
    ),
    ShellHelpEntry::new(
        "td",
        ShellVocabKind::Macro,
        ShellMomentKind::Model,
        "Typed-declaration shorthand: wrap a DataFrame in a Dataset.",
        r#"let ds = td!(df, name: "typed-frame");"#,
    ),
    ShellHelpEntry::new(
        "io",
        ShellVocabKind::Macro,
        ShellMomentKind::Model,
        "Declare an IO descriptor (path + format) for a Dataset.",
        r#"let io_desc = io!("data/my.csv", "csv");"#,
    ),
    ShellHelpEntry::new(
        "get",
        ShellVocabKind::Macro,
        ShellMomentKind::Model,
        "Retrieve a column or field from a Dataset frame.",
        r#"let col = get!(ds, "label")?;"#,
    ),
    // Feature — structural feature expressions
    ShellHelpEntry::new(
        "fspec",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Build a flat feature specification from key-value pairs.",
        r#"let spec = fspec!{ "pos" => "NN", "lemma" => "run" };"#,
    ),
    ShellHelpEntry::new(
        "fspec_tree",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Build a nested feature specification (tree).",
        r#"let spec = fspec_tree!{ "np" => fspec!{"det"=>"the"} };"#,
    ),
    ShellHelpEntry::new(
        "fpos",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "POS-tag feature shorthand.",
        r#"let f = fpos!("NN");"#,
    ),
    ShellHelpEntry::new(
        "frange",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Numeric range feature.",
        r#"let f = frange!(0.0, 1.0);"#,
    ),
    ShellHelpEntry::new(
        "fcond",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Conditional feature guard.",
        r#"let f = fcond!("length > 3");"#,
    ),
    ShellHelpEntry::new(
        "ftemplate",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Named feature template reference.",
        r#"let f = ftemplate!("np-rule");"#,
    ),
    ShellHelpEntry::new(
        "fexpr",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Inline feature expression (Expr-level).",
        r#"let f = fexpr!("head.pos = NN && dep = nsubj");"#,
    ),
    ShellHelpEntry::new(
        "tree",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Build a tree node with label and children.",
        r#"let t = tree!("S", [tree!("NP", []), tree!("VP", [])]);"#,
    ),
    ShellHelpEntry::new(
        "tleaf",
        ShellVocabKind::Macro,
        ShellMomentKind::Feature,
        "Build a terminal (leaf) tree node.",
        r#"let leaf = tleaf!("word", "run");"#,
    ),
    // Plan — pipeline and execution declarations
    ShellHelpEntry::new(
        "pipeline",
        ShellVocabKind::Macro,
        ShellMomentKind::Plan,
        "Declare an ordered execution pipeline of Dataset operations.",
        r#"let p = pipeline![load_op, transform_op, emit_op];"#,
    ),
    ShellHelpEntry::new(
        "plan",
        ShellVocabKind::Macro,
        ShellMomentKind::Plan,
        "Declare a named plan with steps and dependencies.",
        r#"let pl = plan!{ name: "fit", steps: [step_a, step_b] };"#,
    ),
    ShellHelpEntry::new(
        "dop",
        ShellVocabKind::Macro,
        ShellMomentKind::Plan,
        "Declare a single dataset operation step.",
        r#"let op = dop!("normalize", normalize_fn);"#,
    ),
    // ── Corpus : Language :: Semantics  (Concept / Return) ───────────────────

    // Corpus — raw text and document intake
    ShellHelpEntry::new(
        "corpus",
        ShellVocabKind::Macro,
        ShellMomentKind::Corpus,
        "Build a corpus intake descriptor from file paths.",
        r#"let c = corpus!["data/a.txt", "data/b.txt"];"#,
    ),
    ShellHelpEntry::new(
        "PlaintextCorpusReader",
        ShellVocabKind::Type,
        ShellMomentKind::Corpus,
        "Read a corpus of plain-text files into tokens.",
        r#"let reader = PlaintextCorpusReader::new(files);"#,
    ),
    ShellHelpEntry::new(
        "WordListCorpusReader",
        ShellVocabKind::Type,
        ShellMomentKind::Corpus,
        "Read a corpus of word-list files (one word per line).",
        r#"let reader = WordListCorpusReader::new(files);"#,
    ),
    ShellHelpEntry::new(
        "XmlCorpusReader",
        ShellVocabKind::Type,
        ShellMomentKind::Corpus,
        "Read a corpus of XML-tagged files.",
        r#"let reader = XmlCorpusReader::new(files);"#,
    ),
    // Language — tokenization and parsing surface
    ShellHelpEntry::new(
        "WhitespaceTokenizer",
        ShellVocabKind::Type,
        ShellMomentKind::Language,
        "Tokenize text by whitespace.",
        r#"let tok = WhitespaceTokenizer; let tokens = tok.tokenize("hello world");"#,
    ),
    ShellHelpEntry::new(
        "MarkupTokenizer",
        ShellVocabKind::Type,
        ShellMomentKind::Language,
        "Tokenize HTML/XML markup into tagged tokens.",
        r#"let tok = MarkupTokenizer::new(); let tokens = tok.tokenize(src);"#,
    ),
    ShellHelpEntry::new(
        "MarkupParser",
        ShellVocabKind::Type,
        ShellMomentKind::Language,
        "Parse markup tokens into a tree.",
        r#"let parser = MarkupParser::default(); let trees = parser.parse_tokens(&tokens);"#,
    ),
    ShellHelpEntry::new(
        "MLE",
        ShellVocabKind::Type,
        ShellMomentKind::Language,
        "Maximum-likelihood estimation over a token corpus.",
        r#"let mle = MLE::new(); let freq = mle.freq_dist(&tokens);"#,
    ),
    // Semantics — LogicFrame and mediated Concept return
    ShellHelpEntry::new(
        "stream",
        ShellVocabKind::Macro,
        ShellMomentKind::Semantics,
        "Wrap a Dataset in a lazy streaming iterator.",
        r#"let iter = stream!(ds, batch_size: 32);"#,
    ),
    ShellHelpEntry::new(
        "LogicFrame",
        ShellVocabKind::Type,
        ShellMomentKind::Semantics,
        "Semantically mediated dataset — Concept-return unification.",
        r#"let sem = LogicFrame::from_texts(&["Being is Essence."])?;"#,
    ),
    ShellHelpEntry::new(
        "StreamingDataset",
        ShellVocabKind::Type,
        ShellMomentKind::Semantics,
        "Lazy streaming view over a Dataset with optional transform.",
        r#"let s = StreamingDataset::new(dataset, 32).with_transform(|lf| lf);"#,
    ),
    ShellHelpEntry::new(
        "DatasetPipeline",
        ShellVocabKind::Type,
        ShellMomentKind::Semantics,
        "Orchestrated multi-stage Dataset meta-pipeline.",
        r#"let pipe = DatasetPipeline::new(corpus, lm, sem);"#,
    ),
];
